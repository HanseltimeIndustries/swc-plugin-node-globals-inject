# Development

If you would like to develop additional changes to this repository, please submit a Pull Request from your own fork.

It's also a good idea to start an Issue on the repository first if you would like feedback before spending development time.

## Change sets

This repository uses the [changesets library](https://github.com/changesets/changesets/blob/main/docs/intro-to-using-changesets.md)
for version control.  Because of this, any functional changes that should require a new release will require
a changeset file.

```shell
# Create a change set
yarn install
yarn changeset
```

## Makefile

All of the rust testing commmands that we use in our test workflow are listed as targets in the [Makefile](./Makefile).
Please take a look at them to get an understanding of what to run to verify your work.
