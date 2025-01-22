pub mod config;
mod constants;
mod import_data_parser;
mod post_scan_write_visitor;
mod scan_visitor;
mod specifier_factory;

use config::Config;
use constants::PLUGIN_NAME;
use post_scan_write_visitor::PostScanWriteVisitor;
use scan_visitor::{ScanResults, ScanVisitor};
use specifier_factory::SpecifierFactory;

use swc_core::ecma::{
    ast::{Pass, Program},
    visit::{visit_mut_pass, VisitMutWith},
};
use swc_core::plugin::{plugin_transform, proxies::TransformPluginProgramMetadata};

pub struct NodeGlobalsInjector {
    pub config: Config,
}

/// Compound Pass object that can call the first scan plugin and then the second injection plugin
impl Pass for NodeGlobalsInjector {
    fn process(&mut self, program: &mut Program) {
        let mut scan_results = ScanResults::new();
        let scan_visitor = ScanVisitor {
            scan_results: &mut scan_results,
        };

        program.visit_mut_with(&mut visit_mut_pass(scan_visitor));
        program.visit_mut_with(&mut visit_mut_pass(PostScanWriteVisitor {
            scan_results,
            specifier_factory: SpecifierFactory {
                insert_alias_prefix: self.config.func_alias_prefix.clone(),
            },
        }));
    }
}

#[plugin_transform]
pub fn process_transform(program: Program, metadata: TransformPluginProgramMetadata) -> Program {
    let config: Config = serde_json::from_str(
        &metadata
            .get_transform_plugin_config()
            .expect(&format!("failed to get plugin config for {PLUGIN_NAME}")[..]),
    )
    .expect(&format!("invalid {PLUGIN_NAME} configuration")[..]);

    program.apply(NodeGlobalsInjector { config })
}
