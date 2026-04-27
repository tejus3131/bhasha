# Bhasha

[![Rust](https://img.shields.io/badge/rust-2021-orange?logo=rust)](https://www.rust-lang.org/)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](LICENSE)

**Bhasha** is a small interpreter for a toy language whose keywords and operators read like **Hindi / Hinglish**. Programs are plain text files with the `.bhasha` extension.

The implementation is a classic three-stage pipeline in Rust: **lexer** ([Logos](https://github.com/maciejhirsz/logos)) → **recursive-descent parser** → **tree-walking interpreter**.

## Features

| Area | What works |
|------|------------|
| Variables | `mano` + `barabar` for declarations |
| Arithmetic | `ka yog`, `ka antar`, `ka guna`, `ka bhaag`, `%` |
| Types in runtime | Integers, floats, strings, booleans (`satya` / `asatya`) |
| Control flow | `agar` / `warna` / `aage`, `jabtak` loops |
| I/O | `likho` (print), `padho` with type names |
| Functions | `banao` … `jo le` … `fir` … `wapas karo`, `chalao` … `par` … `me` |
| Comments | Lines starting with `faltu` |
| Program end | `samapt` (CLI also appends this if omitted) |

## Requirements

- [Rust](https://www.rust-lang.org/tools/install) (stable), **2021 edition**

## Quick start

```bash
git clone https://github.com/<your-username>/bhasha.git
cd bhasha
cargo build --release
```

Run a program:

```bash
cargo run --release -- examples/hello.bhasha
# or
./target/release/bhasha examples/hello.bhasha
```

The CLI accepts exactly one argument: path to a `.bhasha` file.

## Project layout

| Path | Role |
|------|------|
| [`src/tokens.rs`](src/tokens.rs) | Logos-derived lexer; all keyword / operator spellings |
| [`src/ast.rs`](src/ast.rs) | AST: `Expression`, `Statement`, `Program` |
| [`src/parser.rs`](src/parser.rs) | Builds `Program` from token stream |
| [`src/interpreter.rs`](src/interpreter.rs) | Evaluates AST; global env + function table |
| [`src/main.rs`](src/main.rs) | Reads file, tokenizes, parses, runs |
| [`src/lib.rs`](src/lib.rs) | Re-exports `Token`, `Parser`, `Interpreter` for tests / embedders |
| [`examples/`](examples/) | Sample `.bhasha` programs |

## Language sketch

Binary expressions use **operand–operand–operator** token order (see parser), e.g. `10 5 ka yog` → 10 + 5.

**Declaration**

```text
mano naam barabar "Ram"
mano x barabar 20 5 ka bhaag
```

**Conditionals and loops**

Blocks close with **`aage`**. Optional `warna` branch after the first block.

```text
agar x 5 se bada hai
likho "bada"
warna
likho "chhota ya barabar"
aage

jabtak x 0 nhi hai
likho x
mano x barabar x 1 ka antar
aage
```

**Input** (`padho` then variable name then type keyword)

| Type keyword | Meaning |
|--------------|---------|
| `sankhya` | Integer |
| `dasamlav` | Float |
| `paath` | String (line) |
| `tark` | Boolean |

```text
padho sankhya n
likho n
```

**Functions** (minimal surface syntax)

```text
banao greet jo le naam fir
likho naam
wapas karo 0

chalao greet "Bhasha" par result me
```

**Comments**

```text
faltu yeh line ignore hoti hai
```

**End of program**

Use `samapt` at end of source, or rely on the CLI inserting it when missing.

Full token list lives in [`src/tokens.rs`](src/tokens.rs).

## Using as a library

```rust
use bhasha::{Interpreter, Parser, Token};

let source = "mano x barabar 1 2 ka yog samapt";
let mut tokens = Token::tokenize(source);
let mut parser = Parser::new(&mut tokens);
let program = parser.parse();
Interpreter::new().run(program);
```

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md) and [CODE_OF_CONDUCT.md](CODE_OF_CONDUCT.md). PRs welcome: docs, examples, tests, lexer/parser fixes, better errors.

## Security

See [SECURITY.md](SECURITY.md).

## License

[MIT](LICENSE).
