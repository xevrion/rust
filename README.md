
# Rust Learning & Exploration Workspace

[![Rust](https://img.shields.io/badge/rust-2024-orange.svg?style=flat-square&logo=rust)](https://www.rust-lang.org)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg?style=flat-square)](LICENSE)
[![Build Status](https://img.shields.io/badge/build-passing-brightgreen.svg?style=flat-square)]()

A curated collection of Rust projects, exercises, and applications designed to explore the Rust programming language. This repository serves as a structured learning journey, progressing from basic syntax and tooling to complex language features, memory management paradigms, and external crate integration.

All projects in this workspace are built using the modern **Rust 2024 Edition**, leveraging the latest compiler features, idioms, and performance optimizations.

---

## 📂 Repository Structure

The repository is organized as a multi-project workspace. Each directory represents an independent, runnable Rust application focusing on specific language concepts:


├── .gitignore
├── data_types/          # Deep dive into scalar and compound data types
│   ├── Cargo.toml
│   └── src/
│       └── main.rs
├── guessing_game/       # Interactive CLI game utilizing external crates & input handling
│   ├── Cargo.toml
│   └── src/
│       └── main.rs
├── hello_cargo/         # Introduction to Cargo, Rust's package manager and build system
│   ├── Cargo.toml
│   └── src/
│       └── main.rs
├── hello_world/         # Bare-metal Rust compilation using rustc directly
│   └── main.rs
└── variables/           # Exploration of mutability, variable shadowing, and scopes
    ├── Cargo.toml
    └── src/
        └── main.rs

## 🚀 Sub-Project Overview

### 1. `hello_world`
* **Focus**: Bare-metal compilation.
* **Description**: The classic entry point. This project bypasses Cargo to compile a single source file directly using `rustc`, demonstrating the fundamental compilation pipeline of the Rust compiler.

### 2. `hello_cargo`
* **Focus**: Build automation and dependency management.
* **Description**: A foundational project introducing Cargo. It demonstrates the standard directory layout, build targets (`debug` vs. `release`), and basic configuration via `Cargo.toml`.

### 3. `guessing_game`
* **Focus**: User input, error handling, control flow, and external dependencies.
* **Description**: An interactive command-line game where the player guesses a randomly generated number.
* **Key Concepts**:
  * Generating random numbers using the `rand` crate.
  * Handling standard input/output (`std::io`).
  * Pattern matching with `match` and handling `Result` enums (`Ok`/`Err`).
  * Loop control and string parsing.

### 4. `variables`
* **Focus**: Memory safety, mutability, and scoping.
* **Description**: Practical examples demonstrating how Rust manages memory safety at compile time.
* **Key Concepts**:
  * Immutable by default variables.
  * Mutable variables (`mut`).
  * Variable shadowing and scope boundaries.
  * Constants vs. variables.

### 5. `data_types`
* **Focus**: Rust's type system.
* **Description**: A comprehensive exploration of Rust's statically typed nature, covering both primitive and compound types.
* **Key Concepts**:
  * Scalar types: Integers, floating-point numbers, booleans, and characters.
  * Compound types: Tuples and Arrays.
  * Type annotations and implicit type inference.
## 🛠️ Tech Stack

* **Language**: [Rust](https://www.rust-lang.org/) (Edition 2024)
* **Build System & Package Manager**: [Cargo](https://doc.rust-lang.org/cargo/)
* **Key Dependencies**:
  * `rand` (v0.8.5) - Random number generation library used in `guessing_game`.

---

## ⚙️ Getting Started

### Prerequisites

To build and run these projects, you need the Rust toolchain installed on your system.

1. **Install Rustup** (the Rust installer and version manager):
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```
2. **Verify Installation**:
   Ensure you have the compiler and package manager available (Rust 1.85+ recommended for Edition 2024 support):
   ```bash
   rustc --version
   cargo --version
   ```

### Installation & Setup

Clone the repository to your local machine:

```bash
git clone https://github.com/xevrion/rust.git
cd rust
```

---

## 💻 Usage & Execution

Each project can be run independently. Navigate to the project directory or run them directly using Cargo's project-selection flags.

### Running a Specific Project

#### Method A: Direct Execution (From Root)
You can run any Cargo-managed project directly from the root directory:

bash
cargo run -p bounce


#### Method B: Directory Execution
Or navigate to the project directory and run:

bash
cd bounce
cargo run

# Run the Guessing Game
cargo run --manifest-path guessing_game/Cargo.toml

# Run the Variables demonstration
cargo run --manifest-path variables/Cargo.toml
```

#### Method B: Navigating to the Project Directory
Alternatively, navigate into the project directory to compile and run:

```bash
cd guessing_game
cargo run
```

### Compiling `hello_world` (Without Cargo)
Since `hello_world` does not use Cargo, compile it directly using the Rust compiler:

```bash
cd hello_world
rustc main.rs
./main
```

---

## 🧪 Development Workflow

### Code Formatting
Ensure all code conforms to the official Rust style guidelines:
```bash
# Check formatting
cargo fmt --all -- --check

# Apply formatting automatically
cargo fmt --all
```

### Static Analysis & Linting
Use Clippy to catch common mistakes and write idiomatic Rust:
```bash
# Run clippy on a specific project (e.g., guessing_game)
cd guessing_game
cargo clippy -- -D warnings
```

### Building for Production
To compile optimized binaries with full optimizations and no debug symbols:
```bash
cd guessing_game
cargo build --release
```
The optimized binary will be located in `target/release/guessing_game`.

---

## 🤝 Contributing

Contributions, issues, and feature requests are welcome! If you are also learning Rust and want to add an exercise or improve an existing implementation:

1. Fork the Project.
2. Create your Feature Branch (`git checkout -b feature/amazing-exercise`).
3. Commit your Changes (`git commit -m 'Add some amazing exercise'`).
4. Push to the Branch (`git push origin feature/amazing-exercise`).
5. Open a Pull Request.

---

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

---

## 🎓 Acknowledgments

* The official [Rust Programming Language Book](https://doc.rust-lang.org/book/) for providing the foundational concepts and structure for these exercises.
* The Rust community for maintaining an exceptional ecosystem and documentation.