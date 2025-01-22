use swc_core::{
    atoms::Atom,
    common::{SyntaxContext, DUMMY_SP},
    ecma::ast::{
        Ident, ImportDecl, ImportNamedSpecifier, ImportPhase, ImportSpecifier, ModuleDecl,
        ModuleExportName, ModuleItem, Str,
    },
};

use crate::import_data_parser::{ImportAs, ParseResult};

pub struct NewSpecifierInfo {
    pub alias: String,
    pub specifier: ImportSpecifier,
}

pub struct NewDeclInfo {
    pub alias: String,
    pub decl: ImportDecl,
}

pub struct SpecifierFactory {
    /// The prefix we should use when renaming a function we import
    /// (used to avoid namespace collisions)
    pub insert_alias_prefix: String,
}

impl SpecifierFactory {
    fn create_new_specifier(&mut self, var_name: &str) -> NewSpecifierInfo {
        let alias = format!("{}{}", self.insert_alias_prefix, var_name);
        let import_named_alias = Ident {
            span: DUMMY_SP,
            sym: Atom::new(alias.as_str()),
            ctxt: SyntaxContext::empty(), // TODO: make sure this is right
            optional: false,
        };
        let func_import = ImportNamedSpecifier {
            span: DUMMY_SP,
            local: import_named_alias,
            is_type_only: false,
            imported: Some(ModuleExportName::Ident(Ident {
                span: DUMMY_SP,
                ctxt: SyntaxContext::empty(),
                sym: Atom::new(var_name),
                optional: false,
            })),
        };
        NewSpecifierInfo {
            alias,
            specifier: ImportSpecifier::Named(func_import),
        }
    }

    pub fn create_new_import(&mut self, pkg: &str, var_name: &str) -> NewDeclInfo {
        let pkg_src = Box::new(Str {
            span: DUMMY_SP,
            value: Atom::new(pkg),
            raw: Some(Atom::new(format!("\"{}\"", pkg))),
        });
        let spec = self.create_new_specifier(var_name);
        NewDeclInfo {
            alias: spec.alias,
            decl: ImportDecl {
                span: DUMMY_SP,
                specifiers: vec![spec.specifier],
                src: pkg_src,
                type_only: false,
                phase: ImportPhase::Evaluation,
                with: None,
            },
        }
    }

    pub fn apply_parse_result(
        &mut self,
        decl: &mut ImportDecl,
        parse_result: &ParseResult,
    ) -> Option<String> {
        if decl.src.value != parse_result.pkg_name {
            return None;
        }
        let var_name = parse_result.var_name.as_str();
        match &parse_result.import_as {
            ImportAs::DestructuredNone() => {
                // Inserts an aliased specifier for us to use
                let spec = self.create_new_specifier(var_name);
                decl.specifiers.push(spec.specifier);
                Some(spec.alias)
                // TODO: return the alias name
            }
            ImportAs::Named(n) => {
                // No import but return the n.var_name
                Some(format!("{}.{}", n, var_name))
            }
            ImportAs::Destructured(n) => {
                // No import but return n
                Some(n.to_string())
            }
        }
    }

    /// Given a list of parse results for different imports, this will return the import aliases (maybe just the original name)
    /// and it will also return the last index found
    pub fn get_modified_aliases(
        &mut self,
        n: &mut Vec<ModuleItem>,
        existing_imports: &Vec<&ParseResult>,
    ) -> (usize, Vec<String>) {
        let mut idx = -1;
        let size = existing_imports.len();
        let mut aliases: Vec<String> = vec!["".to_string(); existing_imports.len()];
        let mut num_modified = 0;
        for item in n {
            if num_modified == size {
                // Don't iterate any more to avoid additional cycles
                break;
            }
            idx += 1;
            match item {
                ModuleItem::ModuleDecl(v) => {
                    match v {
                        ModuleDecl::Import(h) => {
                            for (idx, imp) in existing_imports.iter().enumerate() {
                                let export_result = self.apply_parse_result(h, imp);
                                if let Some(alias) = export_result {
                                    aliases[idx] = alias;
                                    num_modified += 1;
                                    break;
                                }
                            }
                        }
                        _ => {
                            // We only care about imports
                            continue;
                        }
                    }
                }
                _ => {
                    // We don't need care about Statements
                    continue;
                }
            }
        }

        // Should never happen but we keep this for completeness
        if num_modified != size {
            let mut missing_packages: Vec<&str> = vec![];
            for import in existing_imports {
                if !aliases.contains(&import.var_name) {
                    missing_packages.push(&import.var_name)
                }
            }
            panic!(
                "Unable to match parsed imports to existing imports: {:#?}",
                missing_packages
            );
        }

        (idx as usize, aliases)
    }
}
