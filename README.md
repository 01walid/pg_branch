# pg_branch
[![Tests](https://github.com/01walid/pg_branch/actions/workflows/tests.yml/badge.svg)](https://github.com/01walid/pg_branch/actions/workflows/tests.yml)
[![Clippy](https://github.com/01walid/pg_branch/actions/workflows/clippy.yml/badge.svg)](https://github.com/01walid/pg_branch/actions/workflows/clippy.yml)
[![Rustfmt](https://github.com/01walid/pg_branch/actions/workflows/rustfmt.yml/badge.svg)](https://github.com/01walid/pg_branch/actions/workflows/rustfmt.yml)

<img align="left" width="320" alt="Branch your postgres database like you do in Git" src="https://raw.githubusercontent.com/01walid/pg_branch/main/assets/logo-dark.png" />

<ul>
  <li> :elephant: Branch your <strong>development</strong> Postgres DB like you do in Git!
  <li> 🚀 Fast DB copying, no <code>pg_dump</code> / <code>pg_restore</code> (by default)!
  <li> :computer: Rust-based single binary, supports Windows, Linux & Mac.
  <li> :open_file_folder: Auto-discovers your Git setup & branches.
</ul>

<br>

**Warning**: This is a work in progress.


## Demo

![pg_branch](https://user-images.githubusercontent.com/983020/153002119-3c846525-f4bc-4514-9d38-0b51f7e416a0.svg)

TODO: 
- More tests
- Git hooks / Husky support
- Pre-built binaries for Mac/M1, Linux & Windows.
- npm package
- switch --no-rename mode
- `pg_branch run <shell|os> command>` for the no-rename mode.
- VCS agnostic?
