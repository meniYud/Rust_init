# Rust Import/Export Cheatsheet

## Declaring Modules

- In-file module:
  ```rust
  mod my_module {
      // Module contents
  }
  ```

- Separate file module:
  - Create `my_module.rs` in the same directory
  - In parent file: `mod my_module;`

## Visibility

- Make an item public: `pub`
  ```rust
  pub struct MyStruct { ... }
  pub fn my_function() { ... }
  ```

- Make struct fields public:
  ```rust
  pub struct MyStruct {
      pub field1: i32,
      pub field2: String,
  }
  ```

## Importing

- Basic import:
  ```rust
  use my_module::MyStruct;
  ```

- Import multiple items:
  ```rust
  use my_module::{MyStruct, my_function};
  ```

- Import all public items:
  ```rust
  use my_module::*;
  ```

- Import with alias:
  ```rust
  use my_module::MyLongStructName as MyStruct;
  ```

- Nested imports:
  ```rust
  use my_module::nested::{Struct1, Struct2};
  ```

## Re-exporting

- Re-export an item:
  ```rust
  pub use my_module::MyStruct;
  ```

## Importing External Crates

- In `Cargo.toml`:
  ```toml
  [dependencies]
  rand = "0.8.5"
  ```

- In your Rust file:
  ```rust
  use rand::Rng;
  ```

## Path Clarification

- Absolute path (from crate root):
  ```rust
  use crate::my_module::MyStruct;
  ```

- Relative path:
  ```rust
  use self::my_submodule::MyStruct;
  use super::sibling_module::SiblingStruct;
  ```

## Organizing Large Projects

- Create a `lib.rs` file for library crates
- Use `mod` declarations in `lib.rs` to define the module tree
- Use `pub use` in `lib.rs` to re-export important items

## Troubleshooting

- Item not found: Check if it's marked as `pub`
- Module not found: Ensure `mod` declaration exists and file naming is correct
- Naming conflicts: Use aliases or fully qualified paths
- Circular dependencies: Restructure your modules or use forward declarations

Remember: The `use` keyword brings items into scope, while `mod` declares the existence of a module.
