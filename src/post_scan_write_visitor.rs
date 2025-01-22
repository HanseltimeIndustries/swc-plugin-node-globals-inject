use swc_core::{
    atoms::Atom,
    common::{SyntaxContext, DUMMY_SP},
    ecma::{
        ast::{
            BindingIdent, CallExpr, Callee, Decl, Expr, ExprOrSpread, Ident, IdentName, MemberExpr,
            MemberProp, MetaPropExpr, MetaPropKind, ModuleDecl, ModuleItem, Pat, Stmt, VarDecl,
            VarDeclKind, VarDeclarator,
        },
        visit::VisitMut,
    },
};

use crate::{
    constants::{DIRNAME_FUNC, PATH_PKG, URL_PKG, URL_TO_FILE_PATH_FUNC},
    scan_visitor::ScanResults,
    specifier_factory::SpecifierFactory,
};

pub struct PostScanWriteVisitor<'a> {
    pub scan_results: ScanResults<'a>,
    pub specifier_factory: SpecifierFactory,
}

impl PostScanWriteVisitor<'_> {
    fn create_import_meta_expr(&mut self) -> MemberExpr {
        MemberExpr {
            span: DUMMY_SP,
            obj: Box::new(Expr::MetaProp(MetaPropExpr {
                span: DUMMY_SP,
                kind: MetaPropKind::ImportMeta,
            })),
            prop: MemberProp::Ident(IdentName {
                span: DUMMY_SP,
                sym: Atom::new("url"),
            }),
        }
    }

    fn create_resolve_url_call(&mut self, url_to_filename_name: String) -> CallExpr {
        let import_meta_expr = self.create_import_meta_expr();
        CallExpr {
            span: DUMMY_SP,
            ctxt: SyntaxContext::empty(),
            callee: Callee::Expr(Box::new(Expr::Ident(Ident {
                span: DUMMY_SP,
                // Since this is one context nested
                ctxt: SyntaxContext::empty(),
                sym: Atom::new(url_to_filename_name),
                optional: false,
            }))),
            args: vec![ExprOrSpread {
                spread: None,
                expr: Box::new(Expr::Member(import_meta_expr)),
            }],
            type_args: None,
        }
    }

    fn create_file_name_var_statement(&mut self, url_to_filename_name: String) -> VarDecl {
        // urlToFileName(import.meta.url)
        let resolve_url_call = self.create_resolve_url_call(url_to_filename_name);
        // __filename = urlToFileName(import.meta.url)
        let global_filename = VarDeclarator {
            span: DUMMY_SP,
            name: Pat::Ident(BindingIdent {
                id: Ident {
                    span: DUMMY_SP,
                    // TODO: fix this not sure what the context is at this point - it was 2 when reviewing
                    ctxt: SyntaxContext::default(),
                    sym: Atom::new("__filename"),
                    optional: false,
                },
                type_ann: None,
            }),
            init: Some(Box::new(Expr::Call(resolve_url_call.clone()))),
            definite: false,
        };
        // const __filename = urlToFileName(import.meta.url);
        VarDecl {
            span: DUMMY_SP,
            ctxt: SyntaxContext::empty(),
            kind: VarDeclKind::Const,
            declare: false,
            decls: vec![global_filename],
        }
    }

    fn create_dirname_var_statement(
        &mut self,
        url_to_filename_name: String,
        dirname_name: String,
    ) -> VarDecl {
        // urlToFileName(import.meta.url)
        let resolve_url_call = self.create_resolve_url_call(url_to_filename_name);
        // dirname(urlToFileName(import.meta.url))
        let dirname_call = CallExpr {
            span: DUMMY_SP,
            ctxt: SyntaxContext::empty(),
            callee: Callee::Expr(Box::new(Expr::Ident(Ident {
                span: DUMMY_SP,
                ctxt: SyntaxContext::empty(),
                sym: Atom::new(dirname_name),
                optional: false,
            }))),
            args: vec![ExprOrSpread {
                spread: None,
                expr: Box::new(Expr::Call(resolve_url_call.clone())),
            }],
            type_args: None,
        };
        // __dirname = dirname(urlToFileName(import.meta.url))
        let global_dirname = VarDeclarator {
            span: DUMMY_SP,
            name: Pat::Ident(BindingIdent {
                id: Ident {
                    span: DUMMY_SP,
                    // TODO: fix this not sure what the context is at this point - it was 2 when reviewing
                    ctxt: SyntaxContext::default(),
                    sym: Atom::new("__dirname"),
                    optional: false,
                },
                type_ann: None,
            }),
            init: Some(Box::new(Expr::Call(dirname_call))),
            definite: false,
        };
        VarDecl {
            span: DUMMY_SP,
            ctxt: SyntaxContext::empty(),
            kind: VarDeclKind::Const,
            declare: false,
            decls: vec![global_dirname],
        }
    }
}

impl VisitMut for PostScanWriteVisitor<'_> {
    fn visit_mut_module_items(&mut self, n: &mut Vec<ModuleItem>) {
        if !self.scan_results.uses_dirname && !self.scan_results.uses_filename {
            return;
        }

        let path_dirname_parsed = &self.scan_results.path_import_dirname.parsed;
        let url_url_to_filename_parsed = &self.scan_results.url_import_url_to_file_path.parsed;

        let mut dirname_name: Option<String> = None; // this one is conditional since its only needed for __dirname
        let mut path_import: Option<ModuleItem> = None;
        let url_to_filename_name: String;
        let url_import: Option<ModuleItem>;
        let new_inserts: &mut Vec<ModuleItem> = &mut Vec::new();
        let mut insert_idx: usize = 0;
        match (path_dirname_parsed, url_url_to_filename_parsed) {
            (None, None) => {
                // Insert this at the top of the file since we can add everything there
                if self.scan_results.uses_dirname {
                    let path_decl = self
                        .specifier_factory
                        .create_new_import(PATH_PKG, DIRNAME_FUNC);
                    path_import = Some(ModuleItem::ModuleDecl(ModuleDecl::Import(path_decl.decl)));
                    dirname_name = Some(path_decl.alias);
                }
                let url_decl = self
                    .specifier_factory
                    .create_new_import(URL_PKG, URL_TO_FILE_PATH_FUNC);
                url_import = Some(ModuleItem::ModuleDecl(ModuleDecl::Import(url_decl.decl)));
                url_to_filename_name = url_decl.alias;
            }
            (Some(dirname_parsed), None) => {
                // Create a new url
                let url_decl = self
                    .specifier_factory
                    .create_new_import(URL_PKG, URL_TO_FILE_PATH_FUNC);
                url_import = Some(ModuleItem::ModuleDecl(ModuleDecl::Import(url_decl.decl)));
                path_import = None;
                url_to_filename_name = url_decl.alias;
                if self.scan_results.uses_dirname {
                    let (last_idx, var_names) = self
                        .specifier_factory
                        .get_modified_aliases(n, &vec![dirname_parsed]);
                    dirname_name = Some(var_names[0].clone());
                    insert_idx = last_idx + 1;
                }
            }
            (None, Some(url_to_filename_as)) => {
                if self.scan_results.uses_dirname {
                    let path_decl = self
                        .specifier_factory
                        .create_new_import(PATH_PKG, DIRNAME_FUNC);
                    path_import = Some(ModuleItem::ModuleDecl(ModuleDecl::Import(path_decl.decl)));
                    dirname_name = Some(path_decl.alias);
                }
                url_import = None;
                let (last_idx, var_names) = self
                    .specifier_factory
                    .get_modified_aliases(n, &vec![url_to_filename_as]);
                url_to_filename_name = var_names[0].clone();
                insert_idx = last_idx + 1;
            }
            (Some(dirname_as), Some(url_to_filename_as)) => {
                let existing_imports = if self.scan_results.uses_dirname {
                    vec![url_to_filename_as, dirname_as]
                } else {
                    vec![url_to_filename_as]
                };

                let (last_idx, var_names) = self
                    .specifier_factory
                    .get_modified_aliases(n, &existing_imports);
                url_to_filename_name = var_names[0].clone();
                if self.scan_results.uses_dirname {
                    dirname_name = Some(var_names[1].clone());
                }
                path_import = None;
                url_import = None;
                insert_idx = last_idx + 1;
            }
        }

        // We only need the path_import of dirname for dirname
        if self.scan_results.uses_dirname {
            if let Some(p_import) = path_import {
                new_inserts.push(p_import);
            }
        }
        if let Some(u_import) = url_import {
            new_inserts.push(u_import);
        }
        if self.scan_results.uses_filename {
            let filename_decl = self.create_file_name_var_statement(url_to_filename_name.clone());
            new_inserts.push(ModuleItem::Stmt(Stmt::Decl(Decl::Var(Box::new(
                filename_decl,
            )))));
        }
        if self.scan_results.uses_dirname {
            let dirname_decl =
                self.create_dirname_var_statement(url_to_filename_name, dirname_name.unwrap());
            new_inserts.push(ModuleItem::Stmt(Stmt::Decl(Decl::Var(Box::new(
                dirname_decl,
            )))));
        }

        n.splice(insert_idx..insert_idx, new_inserts.to_vec());
    }
}
