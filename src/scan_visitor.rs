use swc_core::ecma::{
    ast::{Ident, Import, ImportDecl, ImportSpecifier},
    visit::{VisitMut, VisitMutWith},
};

use crate::{
    constants::{
        DIRNAME_FUNC, DIRNAME_TOKEN, FILENAME_TOKEN, NODE_PATH_PKG, NODE_URL_PKG, PATH_PKG,
        URL_PKG, URL_TO_FILE_PATH_FUNC,
    },
    import_data_parser::ImportDataParser,
};

pub struct ScanResults<'a> {
    pub uses_dirname: bool,
    pub uses_filename: bool,

    pub path_import_dirname: ImportDataParser<'a>,
    pub url_import_url_to_file_path: ImportDataParser<'a>,
}

impl ScanResults<'_> {
    pub fn new() -> Self {
        Self {
            uses_dirname: false,
            uses_filename: false,
            path_import_dirname: ImportDataParser::new(vec![PATH_PKG, NODE_PATH_PKG], DIRNAME_FUNC),
            url_import_url_to_file_path: ImportDataParser::new(
                vec![URL_PKG, NODE_URL_PKG],
                URL_TO_FILE_PATH_FUNC,
            ),
        }
    }
}

pub struct ScanVisitor<'a, 'b> {
    pub scan_results: &'a mut ScanResults<'b>,
}

impl VisitMut for ScanVisitor<'_, '_> {
    /// Looks at symbol usage for commonjs global constructs
    fn visit_mut_ident(&mut self, e: &mut Ident) {
        // Check to see if we have any imports of the path already
        e.visit_mut_children_with(self);
        if e.sym == DIRNAME_TOKEN {
            self.scan_results.uses_dirname = true
        } else if e.sym == FILENAME_TOKEN {
            self.scan_results.uses_filename = true
        }
    }
    fn visit_mut_import(&mut self, node: &mut Import) {
        node.visit_mut_children_with(self);
    }

    /// Evaluates imports to see which imports already exist in code
    /// This is so that we can determine if we need to inject imports or adapt
    /// our shim code to handle namned imports etc.
    fn visit_mut_import_decl(&mut self, node: &mut ImportDecl) {
        // Skip if type-only - our modifications need to be actual js
        if node.type_only {
            return;
        }

        if self
            .scan_results
            .path_import_dirname
            .parse_import_decl(node)
        {
            return;
        }
        if self
            .scan_results
            .url_import_url_to_file_path
            .parse_import_decl(node)
        {}
    }
    fn visit_mut_import_specifier(&mut self, node: &mut ImportSpecifier) {
        node.visit_mut_children_with(self);
    }
}
