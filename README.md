# PUT Language

PUT (Proficient Universal Transformer) is an experimental programming language designed for machine learning and scientific computing tasks. It aims to combine the simplicity of high-level languages with the performance needed for intensive computations.

## Features

Currently, PUT supports:

- Basic arithmetic operations
- Variable declarations and assignments
- Parenthesized expressions
- Tensor operations (creation, addition)
- Project configuration via `.zom` files

## Prerequisites

- Rust (latest stable version)
- Cargo (comes with Rust)

## Setup

1. Clone the repository:
   ```
   git clone https://github.com/yourusername/put-lang.git
   cd put-lang
   ```

2. Build the project:
   ```
   cargo build
   ```

## Usage

To run the PUT interpreter:

```
cargo run
```

This will execute the sample code in `main.rs` and demonstrate basic language features and tensor operations.

## Project Structure

- `src/main.rs`: Entry point and demo code
- `src/ast.rs`: Abstract Syntax Tree definitions
- `src/lexer.rs`: Lexical analysis
- `src/parser.rs`: Parsing logic
- `src/token.rs`: Token definitions
- `src/zom_parser.rs`: Parser for .zom configuration files
- `src/tensor.rs`: Tensor operations for machine learning

## Examples

Here's a simple PUT program:

```
var x = (42 + 5) * 2 - 3 / 1.5;
```

Tensor operations:

```rust
let t1 = Tensor::new(vec![1.0, 2.0, 3.0, 4.0], vec![2, 2]);
let t2 = Tensor::new(vec![5.0, 6.0, 7.0, 8.0], vec![2, 2]);
let t3 = &t1 + &t2;
```

## Testing

Run the test suite with:

```
cargo test
```

## Future Plans

- Implement more tensor operations (subtraction, multiplication, division)
- Add support for machine learning algorithms (linear regression, neural networks)
- Enhance language syntax for ML-specific operations
- Implement a REPL (Read-Eval-Print Loop) for interactive use
- Add GPU acceleration for tensor operations

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

This project is licensed under the GNU General Public License v2.0 - see the [LICENSE](LICENSE) file for details.

This means you are free to:
- Use the software for any purpose
- Change the software to suit your needs
- Share the software with your friends and neighbors
- Share the changes you make

Under the following conditions:
- If you distribute this software, you must provide the source code
- Any modifications you make must also be licensed under the GNU GPLv2
- You must include a copy of the license with the software

For more details, see the [GNU General Public License v2.0](https://www.gnu.org/licenses/old-licenses/gpl-2.0.html).

## Acknowledgments

- Rust programming language and community
- Inspiration from Python, Julia, and TensorFlow