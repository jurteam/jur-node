# Git hooks

Installation:

```shell
git config core.hooksPath contrib/githooks
```

## pre-commit

The hook automatically runs `cargo fmt`, to correctly format the
`.rs` files included in the commit, provided that rust toolchain's
program are installed and available in the user's search `$PATH`
environment variable.

### Environment variables

The pre-commit hook can be switched off by changing the value held by the
environvement variable `JUR_NODE_GIT_PRECOMMIT` as shown in the following example:

```shell
export JUR_NODE_GIT_PRECOMMIT=off
```

The hook also runs `cargo clippy` if the environment variable
`JUR_NODE_GIT_PRECOMMIT_FEATS` contains the word `clippy`:

```shell
export JUR_NODE_GIT_PRECOMMIT_FEATS=clippy
```
