use node_globals_inject::{config::Config, NodeGlobalsInjector};
use swc_core::{
    common::Mark,
    ecma::{
        ast::Pass,
        transforms::{base::resolver, testing::test_inline},
    },
};

const DEFAULT_CONFIG_STR: &str = r#"{}"#;

fn test_with_config(config: Config) -> impl Pass {
    let unresolved_mark = Mark::new();
    let top_level_mark = Mark::new();
    (
        // resolver is applied by swc and can modify global variables
        resolver(unresolved_mark, top_level_mark, false),
        NodeGlobalsInjector {
            config,
            unresolved_mark,
        },
    )
}

test_inline!(
    Default::default(),
    |_| test_with_config(serde_json::from_str(DEFAULT_CONFIG_STR).unwrap()),
    no_shared_imports_both,
    // Input codes
    r#"import { something } from 'modA';
import * as somethingElse from 'modB';
console.log(`${__dirname} and ${JSON.stringify(something)}`);
const v = __filename;"#,
    // Output codes after transformed with plugin
    r#"import { dirname as __swc_shim_dirname } from "path";
import { fileURLToPath as __swc_shim_fileURLToPath } from "url";
const __filename = __swc_shim_fileURLToPath(import.meta.url);
const __dirname = __swc_shim_dirname(__swc_shim_fileURLToPath(import.meta.url));
import { something } from 'modA';
import * as somethingElse from 'modB';
console.log(`${__dirname} and ${JSON.stringify(something)}`);
const v = __filename;"#
);

test_inline!(
    Default::default(),
    // TODO: need to use both plugins if we do it this way...
    |_| test_with_config(serde_json::from_str(DEFAULT_CONFIG_STR).unwrap()),
    no_shared_imports_dirname_only,
    // Input codes
    r#"import { something } from 'modA';
import * as somethingElse from 'modB';
console.log(`${__dirname} and ${JSON.stringify(something)}`);"#,
    // Output codes after transformed with plugin
    r#"import { dirname as __swc_shim_dirname } from "path";
import { fileURLToPath as __swc_shim_fileURLToPath } from "url";
const __dirname = __swc_shim_dirname(__swc_shim_fileURLToPath(import.meta.url));
import { something } from 'modA';
import * as somethingElse from 'modB';
console.log(`${__dirname} and ${JSON.stringify(something)}`);"#
);

test_inline!(
    Default::default(),
    // TODO: need to use both plugins if we do it this way...
    |_| test_with_config(serde_json::from_str(DEFAULT_CONFIG_STR).unwrap()),
    no_shared_imports_filename_only,
    // Input codes
    r#"import { something } from 'modA';
import * as somethingElse from 'modB';
console.log(`${__filename} and ${JSON.stringify(something)}`);"#,
    // Output codes after transformed with plugin
    r#"import { fileURLToPath as __swc_shim_fileURLToPath } from "url";
const __filename = __swc_shim_fileURLToPath(import.meta.url);
import { something } from 'modA';
import * as somethingElse from 'modB';
console.log(`${__filename} and ${JSON.stringify(something)}`);"#
);

test_inline!(
    Default::default(),
    // TODO: need to use both plugins if we do it this way...
    |_| test_with_config(serde_json::from_str(DEFAULT_CONFIG_STR).unwrap()),
    existing_path_dirname_import_both,
    // Input codes
    r#"import { something } from 'modA';
import { join, dirname } from 'path';
import * as somethingElse from 'modB';
console.log(`${__dirname} and ${JSON.stringify(something)}`);
const v = __filename;"#,
    // Output codes after transformed with plugin
    r#"import { something } from 'modA';
import { join, dirname } from 'path';
import { fileURLToPath as __swc_shim_fileURLToPath } from "url";
const __filename = __swc_shim_fileURLToPath(import.meta.url);
const __dirname = dirname(__swc_shim_fileURLToPath(import.meta.url));
import * as somethingElse from 'modB';
console.log(`${__dirname} and ${JSON.stringify(something)}`);
const v = __filename;"#
);

test_inline!(
    Default::default(),
    // TODO: need to use both plugins if we do it this way...
    |_| test_with_config(serde_json::from_str(DEFAULT_CONFIG_STR).unwrap()),
    existing_path_dirname_import_dirname_only,
    // Input codes
    r#"import { something } from 'modA';
import { join, dirname } from 'path';
import * as somethingElse from 'modB';
console.log(`${__dirname} and ${JSON.stringify(something)}`);"#,
    // Output codes after transformed with plugin
    r#"import { something } from 'modA';
import { join, dirname } from 'path';
import { fileURLToPath as __swc_shim_fileURLToPath } from "url";
const __dirname = dirname(__swc_shim_fileURLToPath(import.meta.url));
import * as somethingElse from 'modB';
console.log(`${__dirname} and ${JSON.stringify(something)}`);"#
);

test_inline!(
    Default::default(),
    // TODO: need to use both plugins if we do it this way...
    |_| test_with_config(serde_json::from_str(DEFAULT_CONFIG_STR).unwrap()),
    existing_path_dirname_import_filename_only,
    // Input codes
    r#"import { something } from 'modA';
import { join, dirname } from 'path';
import * as somethingElse from 'modB';
console.log(`${__filename} and ${JSON.stringify(something)}`);"#,
    // Output codes after transformed with plugin
    r#"import { fileURLToPath as __swc_shim_fileURLToPath } from "url";
const __filename = __swc_shim_fileURLToPath(import.meta.url);
import { something } from 'modA';
import { join, dirname } from 'path';
import * as somethingElse from 'modB';
console.log(`${__filename} and ${JSON.stringify(something)}`);"#
);

test_inline!(
    Default::default(),
    // TODO: need to use both plugins if we do it this way...
    |_| test_with_config(serde_json::from_str(DEFAULT_CONFIG_STR).unwrap()),
    existing_path_no_dirname_import_both,
    // Input codes
    r#"import { something } from 'modA';
import { join } from 'path';
import * as somethingElse from 'modB';
console.log(`${__dirname} and ${JSON.stringify(something)}`);
const v = __filename;"#,
    // Output codes after transformed with plugin
    r#"import { something } from 'modA';
import { join, dirname as __swc_shim_dirname } from 'path';
import { fileURLToPath as __swc_shim_fileURLToPath } from "url";
const __filename = __swc_shim_fileURLToPath(import.meta.url);
const __dirname = __swc_shim_dirname(__swc_shim_fileURLToPath(import.meta.url));
import * as somethingElse from 'modB';
console.log(`${__dirname} and ${JSON.stringify(something)}`);
const v = __filename;"#
);

test_inline!(
    Default::default(),
    // TODO: need to use both plugins if we do it this way...
    |_| test_with_config(serde_json::from_str(DEFAULT_CONFIG_STR).unwrap()),
    existing_path_no_dirname_import_dirname_only,
    // Input codes
    r#"import { something } from 'modA';
import { join } from 'path';
import * as somethingElse from 'modB';
console.log(`${__dirname} and ${JSON.stringify(something)}`);"#,
    // Output codes after transformed with plugin
    r#"import { something } from 'modA';
import { join, dirname as __swc_shim_dirname } from 'path';
import { fileURLToPath as __swc_shim_fileURLToPath } from "url";
const __dirname = __swc_shim_dirname(__swc_shim_fileURLToPath(import.meta.url));
import * as somethingElse from 'modB';
console.log(`${__dirname} and ${JSON.stringify(something)}`);"#
);

test_inline!(
    Default::default(),
    // TODO: need to use both plugins if we do it this way...
    |_| test_with_config(serde_json::from_str(DEFAULT_CONFIG_STR).unwrap()),
    existing_path_no_dirname_import_filename_only,
    // Input codes
    r#"import { something } from 'modA';
import { join } from 'path';
import * as somethingElse from 'modB';
console.log(`${__filename} and ${JSON.stringify(something)}`);"#,
    // Output codes after transformed with plugin
    r#"import { fileURLToPath as __swc_shim_fileURLToPath } from "url";
const __filename = __swc_shim_fileURLToPath(import.meta.url);
import { something } from 'modA';
import { join } from 'path';
import * as somethingElse from 'modB';
console.log(`${__filename} and ${JSON.stringify(something)}`);"#
);

test_inline!(
    Default::default(),
    // TODO: need to use both plugins if we do it this way...
    |_| test_with_config(serde_json::from_str(DEFAULT_CONFIG_STR).unwrap()),
    existing_path_star_import_both,
    // Input codes
    r#"import { something } from 'modA';
import * as p from 'path';
import * as somethingElse from 'modB';
console.log(`${__dirname} and ${JSON.stringify(something)}`);
const v = __filename;"#,
    // Output codes after transformed with plugin
    r#"import { something } from 'modA';
import * as p from 'path';
import { fileURLToPath as __swc_shim_fileURLToPath } from "url";
const __filename = __swc_shim_fileURLToPath(import.meta.url);
const __dirname = p.dirname(__swc_shim_fileURLToPath(import.meta.url));
import * as somethingElse from 'modB';
console.log(`${__dirname} and ${JSON.stringify(something)}`);
const v = __filename;"#
);

test_inline!(
    Default::default(),
    // TODO: need to use both plugins if we do it this way...
    |_| test_with_config(serde_json::from_str(DEFAULT_CONFIG_STR).unwrap()),
    existing_path_star_import_dirname_only,
    // Input codes
    r#"import { something } from 'modA';
import * as p from 'path';
import * as somethingElse from 'modB';
console.log(`${__dirname} and ${JSON.stringify(something)}`);"#,
    // Output codes after transformed with plugin
    r#"import { something } from 'modA';
import * as p from 'path';
import { fileURLToPath as __swc_shim_fileURLToPath } from "url";
const __dirname = p.dirname(__swc_shim_fileURLToPath(import.meta.url));
import * as somethingElse from 'modB';
console.log(`${__dirname} and ${JSON.stringify(something)}`);"#
);

test_inline!(
    Default::default(),
    // TODO: need to use both plugins if we do it this way...
    |_| test_with_config(serde_json::from_str(DEFAULT_CONFIG_STR).unwrap()),
    existing_path_star_import_filename_only,
    // Input codes
    r#"import { something } from 'modA';
import * as p from 'path';
import * as somethingElse from 'modB';
console.log(`${__filename} and ${JSON.stringify(something)}`);"#,
    // Output codes after transformed with plugin
    r#"import { fileURLToPath as __swc_shim_fileURLToPath } from "url";
const __filename = __swc_shim_fileURLToPath(import.meta.url);
import { something } from 'modA';
import * as p from 'path';
import * as somethingElse from 'modB';
console.log(`${__filename} and ${JSON.stringify(something)}`);"#
);

test_inline!(
    Default::default(),
    // TODO: need to use both plugins if we do it this way...
    |_| test_with_config(serde_json::from_str(DEFAULT_CONFIG_STR).unwrap()),
    existing_url_url_to_filename_import_both,
    // Input codes
    r#"import { something } from 'modA';
import { huh, fileURLToPath } from 'url';
import * as somethingElse from 'modB';
console.log(`${__dirname} and ${JSON.stringify(something)}`);
const v = __filename;"#,
    // Output codes after transformed with plugin
    r#"import { something } from 'modA';
import { huh, fileURLToPath } from 'url';
import { dirname as __swc_shim_dirname } from "path";
const __filename = fileURLToPath(import.meta.url);
const __dirname = __swc_shim_dirname(fileURLToPath(import.meta.url));
import * as somethingElse from 'modB';
console.log(`${__dirname} and ${JSON.stringify(something)}`);
const v = __filename;"#
);

test_inline!(
    Default::default(),
    // TODO: need to use both plugins if we do it this way...
    |_| test_with_config(serde_json::from_str(DEFAULT_CONFIG_STR).unwrap()),
    existing_url_url_to_filename_import_dirname_only,
    // Input codes
    r#"import { something } from 'modA';
import { huh, fileURLToPath } from 'url';
import * as somethingElse from 'modB';
console.log(`${__dirname} and ${JSON.stringify(something)}`);"#,
    // Output codes after transformed with plugin
    r#"import { something } from 'modA';
import { huh, fileURLToPath } from 'url';
import { dirname as __swc_shim_dirname } from "path";
const __dirname = __swc_shim_dirname(fileURLToPath(import.meta.url));
import * as somethingElse from 'modB';
console.log(`${__dirname} and ${JSON.stringify(something)}`);"#
);

test_inline!(
    Default::default(),
    // TODO: need to use both plugins if we do it this way...
    |_| test_with_config(serde_json::from_str(DEFAULT_CONFIG_STR).unwrap()),
    existing_url_url_to_filename_import_filename_only,
    // Input codes
    r#"import { something } from 'modA';
import { huh, fileURLToPath } from 'url';
import * as somethingElse from 'modB';
console.log(`${__filename} and ${JSON.stringify(something)}`);"#,
    // Output codes after transformed with plugin
    r#"import { something } from 'modA';
import { huh, fileURLToPath } from 'url';
const __filename = fileURLToPath(import.meta.url);
import * as somethingElse from 'modB';
console.log(`${__filename} and ${JSON.stringify(something)}`);"#
);

test_inline!(
    Default::default(),
    // TODO: need to use both plugins if we do it this way...
    |_| test_with_config(serde_json::from_str(DEFAULT_CONFIG_STR).unwrap()),
    existing_url_no_dirname_import_both,
    // Input codes
    r#"import { something } from 'modA';
import { huh } from 'url';
import * as somethingElse from 'modB';
console.log(`${__dirname} and ${JSON.stringify(something)}`);
const v = __filename;"#,
    // Output codes after transformed with plugin
    r#"import { something } from 'modA';
import { huh, fileURLToPath as __swc_shim_fileURLToPath } from 'url';
import { dirname as __swc_shim_dirname } from "path";
const __filename = __swc_shim_fileURLToPath(import.meta.url);
const __dirname = __swc_shim_dirname(__swc_shim_fileURLToPath(import.meta.url));
import * as somethingElse from 'modB';
console.log(`${__dirname} and ${JSON.stringify(something)}`);
const v = __filename;"#
);

test_inline!(
    Default::default(),
    // TODO: need to use both plugins if we do it this way...
    |_| test_with_config(serde_json::from_str(DEFAULT_CONFIG_STR).unwrap()),
    existing_url_no_dirname_import_dirname_only,
    // Input codes
    r#"import { something } from 'modA';
import { huh } from 'url';
import * as somethingElse from 'modB';
console.log(`${__dirname} and ${JSON.stringify(something)}`);"#,
    // Output codes after transformed with plugin
    r#"import { something } from 'modA';
import { huh, fileURLToPath as __swc_shim_fileURLToPath } from 'url';
import { dirname as __swc_shim_dirname } from "path";
const __dirname = __swc_shim_dirname(__swc_shim_fileURLToPath(import.meta.url));
import * as somethingElse from 'modB';
console.log(`${__dirname} and ${JSON.stringify(something)}`);"#
);

test_inline!(
    Default::default(),
    // TODO: need to use both plugins if we do it this way...
    |_| test_with_config(serde_json::from_str(DEFAULT_CONFIG_STR).unwrap()),
    existing_url_no_dirname_import_filename_only,
    // Input codes
    r#"import { something } from 'modA';
import { huh } from 'url';
import * as somethingElse from 'modB';
console.log(`${__filename} and ${JSON.stringify(something)}`);"#,
    // Output codes after transformed with plugin
    r#"import { something } from 'modA';
import { huh, fileURLToPath as __swc_shim_fileURLToPath } from 'url';
const __filename = __swc_shim_fileURLToPath(import.meta.url);
import * as somethingElse from 'modB';
console.log(`${__filename} and ${JSON.stringify(something)}`);"#
);

test_inline!(
    Default::default(),
    // TODO: need to use both plugins if we do it this way...
    |_| test_with_config(serde_json::from_str(DEFAULT_CONFIG_STR).unwrap()),
    existing_url_star_import_both,
    // Input codes
    r#"import { something } from 'modA';
import * as u from 'url';
import * as somethingElse from 'modB';
console.log(`${__dirname} and ${JSON.stringify(something)}`);
const v = __filename;"#,
    // Output codes after transformed with plugin
    r#"import { something } from 'modA';
import * as u from 'url';
import { dirname as __swc_shim_dirname } from "path";
const __filename = u.fileURLToPath(import.meta.url);
const __dirname = __swc_shim_dirname(u.fileURLToPath(import.meta.url));
import * as somethingElse from 'modB';
console.log(`${__dirname} and ${JSON.stringify(something)}`);
const v = __filename;"#
);

test_inline!(
    Default::default(),
    // TODO: need to use both plugins if we do it this way...
    |_| test_with_config(serde_json::from_str(DEFAULT_CONFIG_STR).unwrap()),
    existing_url_star_import_dirname_only,
    // Input codes
    r#"import { something } from 'modA';
import * as u from 'url';
import * as somethingElse from 'modB';
console.log(`${__dirname} and ${JSON.stringify(something)}`);"#,
    // Output codes after transformed with plugin
    r#"import { something } from 'modA';
import * as u from 'url';
import { dirname as __swc_shim_dirname } from "path";
const __dirname = __swc_shim_dirname(u.fileURLToPath(import.meta.url));
import * as somethingElse from 'modB';
console.log(`${__dirname} and ${JSON.stringify(something)}`);"#
);

test_inline!(
    Default::default(),
    // TODO: need to use both plugins if we do it this way...
    |_| test_with_config(serde_json::from_str(DEFAULT_CONFIG_STR).unwrap()),
    existing_url_star_import_filename_only,
    // Input codes
    r#"import { something } from 'modA';
import * as u from 'url';
import * as somethingElse from 'modB';
console.log(`${__filename} and ${JSON.stringify(something)}`);"#,
    // Output codes after transformed with plugin
    r#"import { something } from 'modA';
import * as u from 'url';
const __filename = u.fileURLToPath(import.meta.url);
import * as somethingElse from 'modB';
console.log(`${__filename} and ${JSON.stringify(something)}`);"#
);

test_inline!(
    Default::default(),
    // TODO: need to use both plugins if we do it this way...
    |_| test_with_config(serde_json::from_str(DEFAULT_CONFIG_STR).unwrap()),
    mixed_star_and_import_both,
    // Input codes
    r#"import { something } from 'modA';
import * as u from 'url';
import * as somethingElse from 'modB';
import { dirname, join } from 'path';
console.log(`${__dirname} and ${JSON.stringify(something)}`);
const v = __filename;"#,
    // Output codes after transformed with plugin
    r#"import { something } from 'modA';
import * as u from 'url';
import * as somethingElse from 'modB';
import { dirname, join } from 'path';
const __filename = u.fileURLToPath(import.meta.url);
const __dirname = dirname(u.fileURLToPath(import.meta.url));
console.log(`${__dirname} and ${JSON.stringify(something)}`);
const v = __filename;"#
);

test_inline!(
    Default::default(),
    |_| test_with_config(serde_json::from_str(DEFAULT_CONFIG_STR).unwrap()),
    mixed_star_and_import_dirname_only,
    // Input codes
    r#"import { something } from 'modA';
import * as u from 'url';
import * as somethingElse from 'modB';
import { dirname, join } from 'path';
console.log(`${__dirname} and ${JSON.stringify(something)}`);"#,
    // Output codes after transformed with plugin
    r#"import { something } from 'modA';
import * as u from 'url';
import * as somethingElse from 'modB';
import { dirname, join } from 'path';
const __dirname = dirname(u.fileURLToPath(import.meta.url));
console.log(`${__dirname} and ${JSON.stringify(something)}`);"#
);

test_inline!(
    Default::default(),
    |_| test_with_config(serde_json::from_str(DEFAULT_CONFIG_STR).unwrap()),
    mixed_star_and_import_filename_only,
    // Input codes
    r#"import { something } from 'modA';
import * as u from 'url';
import * as somethingElse from 'modB';
import { dirname, join } from 'path';
console.log(`${__filename} and ${JSON.stringify(something)}`);"#,
    // Output codes after transformed with plugin
    r#"import { something } from 'modA';
import * as u from 'url';
const __filename = u.fileURLToPath(import.meta.url);
import * as somethingElse from 'modB';
import { dirname, join } from 'path';
console.log(`${__filename} and ${JSON.stringify(something)}`);"#
);

test_inline!(
    Default::default(),
    |_| test_with_config(serde_json::from_str(DEFAULT_CONFIG_STR).unwrap()),
    both_aliased_imports_both,
    // Input codes
    r#"import { something } from 'modA';
import { join, dirname as funkyDirname } from 'path';
import * as somethingElse from 'modB';
import { fileURLToPath as utfp } from 'url';
console.log(`${__dirname} and ${JSON.stringify(something)}`);
const v = __filename;"#,
    // Output codes after transformed with plugin
    r#"import { something } from 'modA';
import { join, dirname as funkyDirname } from 'path';
import * as somethingElse from 'modB';
import { fileURLToPath as utfp } from 'url';
const __filename = utfp(import.meta.url);
const __dirname = funkyDirname(utfp(import.meta.url));
console.log(`${__dirname} and ${JSON.stringify(something)}`);
const v = __filename;"#
);

test_inline!(
    Default::default(),
    |_| test_with_config(serde_json::from_str(DEFAULT_CONFIG_STR).unwrap()),
    both_aliased_imports_dirname_only,
    // Input codes
    r#"import { something } from 'modA';
import { join, dirname as funkyDirname } from 'path';
import * as somethingElse from 'modB';
import { fileURLToPath as utfp } from 'url';
console.log(`${__dirname} and ${JSON.stringify(something)}`);"#,
    // Output codes after transformed with plugin
    r#"import { something } from 'modA';
import { join, dirname as funkyDirname } from 'path';
import * as somethingElse from 'modB';
import { fileURLToPath as utfp } from 'url';
const __dirname = funkyDirname(utfp(import.meta.url));
console.log(`${__dirname} and ${JSON.stringify(something)}`);"#
);

test_inline!(
    Default::default(),
    |_| test_with_config(serde_json::from_str(DEFAULT_CONFIG_STR).unwrap()),
    both_aliased_imports_filename_only,
    // Input codes
    r#"import { something } from 'modA';
import { join, dirname as funkyDirname } from 'path';
import * as somethingElse from 'modB';
import { fileURLToPath as utfp } from 'url';
console.log(`${__filename} and ${JSON.stringify(something)}`);"#,
    // Output codes after transformed with plugin
    r#"import { something } from 'modA';
import { join, dirname as funkyDirname } from 'path';
import * as somethingElse from 'modB';
import { fileURLToPath as utfp } from 'url';
const __filename = utfp(import.meta.url);
console.log(`${__filename} and ${JSON.stringify(something)}`);"#
);

test_inline!(
    Default::default(),
    |_| test_with_config(serde_json::from_str(r#"{"funcAliasPrefix": "__custom_"}"#).unwrap()),
    uses_custom_prefix,
    // Input codes
    // Input codes
    r#"import { something } from 'modA';
import * as somethingElse from 'modB';
console.log(`${__dirname} and ${JSON.stringify(something)}`);
const v = __filename;"#,
    // Output codes after transformed with plugin
    r#"import { dirname as __custom_dirname } from "path";
import { fileURLToPath as __custom_fileURLToPath } from "url";
const __filename = __custom_fileURLToPath(import.meta.url);
const __dirname = __custom_dirname(__custom_fileURLToPath(import.meta.url));
import { something } from 'modA';
import * as somethingElse from 'modB';
console.log(`${__dirname} and ${JSON.stringify(something)}`);
const v = __filename;"#
);
