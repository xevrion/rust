# Rust Projects Monorepo

[![Rust Version](https://img.shields.io/badge/rust-1.75%2B-blue.svg)](https://github.com/xevrion/rust)
[![License](https://img.shields.io/badge/license-MIT-green.svg)](LICENSE)
[![Platform](https://img.shields.io/badge/platform-Linux%20%7C%20macOS%20%7C%20Windows-lightgrey.svg)](https://github.com/xevrion/rust)

A curated collection of Rust projects demonstrating a progressive journey from fundamental language concepts to interactive, real-time terminal applications. This monorepo serves as both a learning portfolio and a showcase of idiomatic Rust design patterns, systems programming, and terminal user interface (TUI) engineering.

---

## 📂 Repository Structure

The repository is organized as a set of independent Cargo packages, allowing each project to be built, tested, and run in isolation:

```
├── data_types/          # Deep dive into Rust's scalar and compound types
├── guessing_game/       # Interactive CLI game utilizing external crates (rand)
├── hello_cargo/         # Standard Cargo project initialization template
├── hello_world/         # Minimalist, dependency-free Rust entry point
├── snake/               # Flagship terminal-based Snake game (Crossterm, non-blocking I/O)
└── variables/           # Exploration of mutability, shadowing, and scoping
```

---

## 🚀 Project Portfolio

### 1. 🐍 Snake (`snake`)
* **Edition**: 2021
* **Key Dependencies**: `crossterm` (v0.28), `rand` (v0.8)
* **Complexity**: Intermediate (Systems / TUI)

A fully-featured, real-time Snake game running entirely in the terminal. It features raw-mode terminal manipulation, non-blocking keyboard event polling, double-buffered rendering, and deterministic collision physics.

#### Key Architectural Highlights:
* **Efficient Memory Layout**: Uses a `VecDeque<Point>` to represent the snake's body, allowing $O(1)$ time complexity for head insertion and tail removal during movement.
* **Non-Blocking Game Loop**: Utilizes `crossterm::event::poll` with a 100ms timeout to handle real-time physics updates independently of user input.
* **Terminal Safety**: Implements raw mode switching and cursor hiding with guaranteed cleanup on exit to prevent terminal corruption.

---

### 2. 🎲 Guessing Game (`guessing_game`)
* **Edition**: 2024
* **Key Dependencies**: `rand` (v0.8.5)
* **Complexity**: Beginner (CLI / Control Flow)

An interactive command-line game where the player guesses a randomly generated number. It demonstrates core Rust concepts such as pattern matching (`match`), error handling (`Result`), console I/O, and loop control.

---

### 3. 🧠 Language Fundamentals (`variables` & `data_types`)
* **Edition**: 2024
* **Complexity**: Beginner (Language Syntax)

These modules explore the core mechanics of the Rust compiler:
* **`variables`**: Demonstrates variable mutability, variable shadowing, scoping rules, and memory allocation on the stack.
* **`data_types`**: Explores scalar types (integers, floats, booleans, characters) and compound types (tuples, arrays) with strict compile-time type checking.

---

### 4. 📦 Boilerplates (`hello_world` & `hello_cargo`)
* **Complexity**: Beginner (Tooling)

The foundational entry points of the repository. `hello_world` compiles directly via `rustc` without package manager overhead, while `hello_cargo` showcases the standard structure of a Cargo-managed binary application.

---

## 🛠️ Tech Stack

* **Language**: Rust (Editions 2021 & 2024)
* **Terminal Manipulation**: [Crossterm](https://crates.io/crates/crossterm) (Cross-platform terminal rendering and event handling)
* **Randomness**: [Rand](https://crates.io/crates/rand) (Fast, secure pseudo-random number generation)
* **Build System**: Cargo (Rust's native package manager and build tool)

---

## 📐 Deep Dive: Snake Architecture

The `snake` game is built around a clean state-machine architecture that decouples state representation, game physics, and rendering.

```
                  ┌────────────────────────┐
                  │      Terminal Input    │
                  └───────────┬────────────┘
                              │ (Arrow Keys / 'q')
                              ▼
┌─────────────┐   ┌────────────────────────┐   ┌─────────────┐
│             │──►│       Game State       │──►│             │
│ Render Loop │   │  - Snake (VecDeque)    │   │ Physics Engine│
│ (Stdout)    │◄──│  - Food (Point)        │◄──│ (Collisions)│
└─────────────┘   │  - Score / Game Over   │   └─────────────┘
                  └────────────────────────┘
```

### State Representation
```rust
struct Point {
    x: i16,
    y: i16,
}

struct Snake {
    body: VecDeque<Point>,
    direction: Direction,
}

struct Game {
    snake: Snake,
    food: Point,
    width: i16,
    height: i16,
    score: u32,
    game_over: bool,
}
```

### The Rendering Pipeline
To prevent screen flickering, the game utilizes a double-buffered rendering approach. Instead of clearing and redrawing individual characters, the entire frame is queued into standard output and flushed atomically:

```rust
fn render(game: &Game) -> std::io::Result<()> {
    let mut out = stdout();
    out.queue(Clear(ClearType::All))?;
    out.queue(MoveTo(0, 0))?;

    // Render Top Border
    print!("{}", "┏".to_string() + &"━".repeat(game.width as usize) + "┓\r\n");

    // Render Play Area & Entities
    for y in 1..=game.height {
        print!("┃");
        for x in 1..=game.width {
            let p = Point::new(x, y);
            if p == game.snake.head() {
                print!("■"); // Snake Head
            } else if game.snake.body.contains(&p) {
                print!("□"); // Snake Body Segment
            } else if p == game.food {
                print!("●"); // Food
            } else {
                print!(" "); // Empty Space
            }
        }
        println!("┃");
    }
    // ... Render Bottom Border & Score ...
    out.flush()?;
    Ok(())
}
```

---

## 🚦 Getting Started

### Prerequisites

To build and run these projects, you must have the Rust toolchain installed on your machine.

1. Install Rust via `rustup` (recommended):
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```
2. Verify your installation:
   ```bash
   rustc --version
   cargo --version
   ```

### Installation & Execution

Clone the repository and navigate to the project you wish to run:

```bash
# Clone the repository
git clone https://github.com/xevrion/rust.git
cd rust
```

#### Running the Snake Game
```bash
cd snake
cargo run --release
```

#### Running the Guessing Game
```bash
cd ../guessing_game
cargo run
```

#### Compiling Raw Hello World (No Cargo)
```bash
cd ../hello_world
rustc main.rs
./main
```

---

## 🎮 Gameplay & Controls (Snake)

* **Objective**: Eat the food pellets (`●`) to grow your snake and increase your score. Avoid crashing into the walls (`┏ ━ ┓ ┃ ┗ ┛`) or your own tail (`□`).
* **Controls**:
  * **Up Arrow**: Move Up
  * **Down Arrow**: Move Down
  * **Left Arrow**: Move Left
  * **Right Arrow**: Move Right
  * **Q / Esc**: Quit Game

---

## 🛠️ Development & Contribution

Contributions are welcome! If you want to add new projects, optimize existing algorithms, or improve documentation, please follow these steps:

1. **Fork the Repository**
2. **Create a Feature Branch**:
   ```bash
   git checkout -b feature/amazing-feature
   ```
3. **Format Your Code**: Ensure your code conforms to the Rust style guidelines:
   ```bash
   cargo fmt
   ```
4. **Run Lints**: Check for idiomatic Rust issues using Clippy:
   ```bash
   cargo clippy
   ```
5. **Commit Your Changes**: Use descriptive, conventional commit messages.
6. **Push and Open a Pull Request**

---

## 🔍 Troubleshooting

### Terminal remains messed up after exiting `snake`
If the game crashes or terminates unexpectedly, your terminal might be left in "raw mode" (cursor hidden, input not echoing). You can restore your terminal to its default state by running:

* **Linux / macOS**:
  ```bash
  reset
  # or
  stty sane
  ```
* **Windows (PowerShell)**:
  Simply close and reopen your terminal window, or run:
  ```powershell
  Clear-Host
  ```

### Rendering Glitches / Weird Characters
The `snake` game uses Unicode characters (`┏`, `━`, `■`, `●`) for high-fidelity terminal graphics. If these characters do not render correctly:
1. Ensure your terminal emulator supports UTF-8 encoding.
2. Use a modern terminal emulator such as **Windows Terminal**, **Alacritty**, **iTerm2**, or the integrated terminal in **VS Code**.
3. Use a font that supports Unicode glyphs (e.g., Fira Code, JetBrains Mono, or any Nerd Font).

---

## 📄 License

This repository is open-source software licensed under the **MIT License**. See the [LICENSE](LICENSE) file for more details.