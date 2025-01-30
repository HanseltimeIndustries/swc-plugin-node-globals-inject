# node_globals_inject plugin

This plugin supports commonjs -> esm syntax for commonjs node constants like `__filename` and `__dirname`.

In effect, this plugin will add the following declarations to any file where we find `__filename` or `__dirname`.

```typescript
import { fileURLToPath } from "url"
import { dirname } from "path"

const __dirname = dirname(fileURLToPath(import.meta.url))
const __filename = fileURLToPath(import.meta.url)
```

## Usage

### Configuration

```json
plugins: [
    [
        "@swc/plugin-relay",
        {
            // Optional - This will change how we do function imports (only change this is you need to avoid name collisions from the default - see below)
            "funcAliasPrefix": "__from_plugin_",
        },
    ],
],
```

## Design Details

This plugin will:

1. Scan each file to determine if `__dirname` or `__filename` are used in the file
2. For files with those usages, it will inject the necessary library imports and const declations:

### Examples of injection
1. If both `dirname` and `fileURLToPath` are imported, it will inject the declarations right after the last import
   ```typescript
   import something from 'something';
   import { fileURLToPath } from 'url';
   import else from 'else';
   import { dirname } from 'path';
   // Injected here
   const __dirname = dirname(fileURLToPath(import.meta.url))
   const __filename = fileURLToPath(import.meta.url)
   ```
2. If only 1 function is imported, it will inject an aliased import of the other function and then the declarations after that import
   ```typescript
   import something from 'something';
   import { fileURLToPath } from 'url';
   // injected here
   import { dirname as __swc_shim_dirname } from 'path';
   const __dirname = __swc_shim_dirname(fileURLToPath(import.meta.url))
   const __filename = fileURLToPath(import.meta.url)
   import else from 'else';
   ```
3. If star imports (`* as foo from 'path'`) are used, it will use the star object notation and just inject the declarations
   ```typescript
   import something from 'something';
   import * as url from 'url';
   import else from 'else';
   import { dirname } from 'path';
   // Injected here
   const __dirname = dirname(url.fileURLToPath(import.meta.url))
   const __filename = url.fileURLToPath(import.meta.url)
   ```
4. If no imports already exists, it will inject aliased imports and then the declarations
   ```typescript
   // Injected here
   import { dirname as __swc_shim_dirname } from 'path';
   import { fileURLToPath as __swc_shim_fileURLToPath } from 'path';
   const __dirname = __swc_shim_dirname(__swc_shim_fileURLToPath(import.meta.url))
   const __filename = __swc_shim_fileURLToPath(import.meta.url)
   import something from 'something';
   import else from 'else';
   ```
