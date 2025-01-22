use swc_core::{
    ecma::ast::{ImportDecl, ImportSpecifier, ModuleExportName},
    plugin::errors::HANDLER,
};

pub enum ImportAs {
    /**
     * The entire library is imported as a "* as name" from 'x"
     */
    Named(String),
    /**
     * The library is imported in parts as a "{ } from 'x'"
     * and we found our desired varName import which is either aliased or it's own name
     */
    Destructured(String),
    /**
     * The import of the package is destructured but doesn't have the varName we were looking for
     */
    DestructuredNone(),
}

pub struct ParseResult {
    /// This is the actual pkg_name that got parsed
    pub pkg_name: String,
    /// Populated after running eval on a particular import_decl
    pub import_as: ImportAs,
    /// This is the export var_name that we were parsing for
    pub var_name: String,
}

pub struct ImportDataParser<'a> {
    /// The variable that we are parsing for
    var_name: &'a str,
    /// The name variants of the imported package that we expect to parse for
    pkg_names: Vec<&'a str>,
    /**
     * The result of a parse that works
     */
    pub parsed: Option<ParseResult>,
}

impl<'a> ImportDataParser<'a> {
    pub fn new(pkg_names: Vec<&'a str>, var_name: &'a str) -> Self {
        Self {
            pkg_names,
            var_name,
            parsed: None,
        }
    }
}

impl ImportDataParser<'_> {
    pub fn parse_import_decl(&mut self, decl: &mut ImportDecl) -> bool {
        let import_pkg: &str = decl.src.value.as_str();
        let _match = &mut false;
        for pkg_name in &self.pkg_names {
            if *pkg_name == import_pkg {
                *_match = true;
                break;
            }
        }
        if !(*_match) {
            return false;
        }
        if self.parsed.is_some() {
            // Throw an error because we somehow provided multiple declarations
            HANDLER.with(|handler| {
                handler
                    .struct_span_err(
                        decl.span,
                        &format!("Multiple import statements detected for `{}`", import_pkg),
                    )
                    .emit();
            });
        }

        // If we don't find a better parse this is the default
        let mut parse_result: ParseResult = ParseResult {
            pkg_name: String::from(import_pkg),
            import_as: ImportAs::DestructuredNone(),
            var_name: String::from(self.var_name),
        };

        for spec in &decl.specifiers {
            match spec {
                // For importing named variables, we need to see if there are aliases, etc.
                ImportSpecifier::Named(spec_stmt) => {
                    let name = spec_stmt.local.sym.to_string();
                    // ensure this is targeting the varName from the package
                    if let Some(aliased) = &spec_stmt.imported {
                        let alias_for = match aliased {
                            ModuleExportName::Ident(i) => &i.sym,
                            ModuleExportName::Str(s) => &s.value,
                        };
                        if alias_for == self.var_name {
                            parse_result.import_as = ImportAs::Destructured(name);
                        }
                    } else if name == self.var_name {
                        parse_result.import_as = ImportAs::Destructured(name);
                    }
                }
                ImportSpecifier::Namespace(spec_stmt) => {
                    let name = spec_stmt.local.sym.to_string();
                    parse_result.import_as = ImportAs::Named(name);
                }
                // We don't expect default paths for this type of parser
                _ => {
                    HANDLER.with(|handler| {
                        handler
                            .struct_span_err(
                                decl.span,
                                &format!("`{}` import statement must be either star import or named import", decl.src.value),
                            )
                            .emit();
                    });
                }
            }
        }
        self.parsed = Some(parse_result);
        true
    }
}
