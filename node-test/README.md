# Node test project

This project installs the rust plugin in the above folder and then compiles a file as an esm module to verify
the plugin's functionality.

If you would like to use this project to test new features:

```shell
corepack enable
yarn install
yarn build
```

# TODO: tswc

I personnally like tswc since it allows for closer integration of typescript which IDES support and swc.  However,
the windows runners seem to have a bug with them.  In the future, it might be worth debugging and contributing back a fix
to the tswc repo.  Once that works, we can run a tswc build test as well.