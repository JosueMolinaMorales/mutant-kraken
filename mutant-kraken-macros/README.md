# Mutant-Kraken Macros

This crate contains the procedural macros used to generate the `KotlinTypes` enum used in the `mutant-kraken` project. The `KotlinTypes` enum is used to represent the different types of nodes in the `kotlin-tree-sitter` AST.

## Usage

To use the `KotlinTypes` enum in your project, add the following to your `Cargo.toml`:

```toml
[dependencies]
mutant-kraken-macros = "0.1.0"
```

Then, add the following to your Rust file:

```rust
use mutant_kraken_macros;

mutant_kraken_macros::generate_kotlin_types_enum!();
```

This will generate the `KotlinTypes` enum in your project, which you can then use to represent the different types of nodes in the `kotlin-tree-sitter` AST.

## Mutant-Kraken

This project is part of the `mutant-kraken` project. For more information, please see the [main project repository](https://github.com/JosueMolinaMorales/mutant-kraken).
