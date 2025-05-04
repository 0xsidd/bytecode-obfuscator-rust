# EVM Bytecode Obfuscator (Rust)

This project implements a simple EVM bytecode obfuscator written in Rust. The primary goal is to make static analysis of Ethereum smart contract bytecode more difficult by obscuring direct control flow transfers (`PUSH-JUMP` sequences).

## How it Works

The obfuscation technique employed targets sequences of `PUSHn <destination>` followed immediately by a `JUMP` instruction. For each such sequence found in the input bytecode, the obfuscator performs the following steps:

1.  **Append `JUMPDEST`**: A new `JUMPDEST` opcode (`5b`) is appended to the end of the current bytecode.
2.  **Modify `PUSH` Value**: The original `PUSHn <destination>` instruction's `<destination>` value is replaced with the byte offset of the newly appended `JUMPDEST`. This redirects the original jump.
3.  **Inject Dead Code**: A randomly selected snippet of valid-but-semantically-useless bytecode ("dead code") is appended after the modified `PUSH` and the original `JUMP`. Any `PUSH-JUMP` sequences *within* this dead code snippet are adjusted to ensure their relative jump targets remain correct after insertion.
4.  **Append Trampoline `PUSH-JUMP`**: A new `PUSH1 <original_destination>` followed by a `JUMP` (`56`) is appended to the very end of the bytecode.

**The overall effect:**

*   The original `PUSH-JUMP` now jumps to the newly added `JUMPDEST` at the end of the modified `PUSH` and dead code block.
*   Execution falls through the dead code (which should ideally have no side effects other than consuming gas).
*   Execution eventually reaches the appended `PUSH-JUMP` trampoline.
*   This trampoline pushes the *original* intended jump destination and jumps to it, restoring the original control flow.

This redirection through dead code and trampolines makes it harder for static analysis tools to directly identify the true control flow graph of the contract.

## Project Structure

```
obfuscator_rs/
├── src/
│   ├── main.rs           # Main executable entry point
│   ├── lib.rs            # Library entry point (if used as a library)
│   │   ├── analysis/
│   │   │   ├── mod.rs
│   │   │   └── jump_seq.rs   # Logic for finding PUSH-JUMP sequences
│   │   │   ├── constant/
│   │   │   │   ├── mod.rs
│   │   │   │   └── opcodes.rs    # EVM opcode definitions and sizes
│   │   │   │   ├── helper/
│   │   │   │   │   ├── mod.rs
│   │   │   │   │   └── bytecode.rs   # Utility functions for manipulating bytecode strings
│   │   │   │   │   └── obfuscation/
│   │   │   │   │       ├── mod.rs
│   │   │   │   │       └── obfuscate.rs  # Core obfuscation logic
│   │   │   │   └── Cargo.toml            # Rust project manifest
│   │   └── README.md             # This file
```

## How to Use

### Prerequisites

*   Rust programming language and Cargo package manager installed. ([https://www.rust-lang.org/tools/install](https://www.rust-lang.org/tools/install))

### Building

1.  Clone the repository (if you haven't already):
    ```bash
    git clone <repository_url>
    cd obfuscator_rs
    ```
2.  Build the project using Cargo:
    ```bash
    cargo build
    ```
    For a release build (optimized):
    ```bash
    cargo build --release
    ```

### Running

The current `main.rs` contains a hardcoded example bytecode string.

1.  Run the project using Cargo:
    ```bash
    cargo run
    ```
    This command compiles and runs the project in one step. For an optimized release build, you can run:
    ```bash
    cargo run --release
    ```

    Alternatively, after building (see `Building` section), you can run the compiled executable directly:
    *   Development build:
        ```bash
        ./target/debug/obfuscator_rs
        ```
    *   Release build:
        ```bash
        ./target/release/obfuscator_rs
        ```

2.  The program will:
    *   Take the hardcoded bytecode string.
    *   Apply the obfuscation steps.
    *   Print the resulting obfuscated bytecode hex string to the console.

### Modifying Input Bytecode

To obfuscate your own bytecode:

1.  Open `src/main.rs`.
2.  Replace the multi-line string assigned to the `bytecode` variable with your own EVM bytecode hex string (ensure it starts with `0x` or modify the code to handle raw hex).
3.  Rebuild and run the project as described above.

## Future Improvements / Considerations

*   **Command-Line Arguments**: Modify `main.rs` to accept bytecode input via command-line arguments or file input instead of hardcoding it. Libraries like `clap` can be used for this.
*   **More Obfuscation Techniques**: Implement additional obfuscation methods (e.g., opaque predicates, arithmetic obfuscation, data encoding).
*   **Sophistication of Dead Code**: Generate more complex or varied dead code dynamically instead of using pre-defined snippets.
*   **Gas Analysis**: The current method adds significant overhead (extra `JUMPDEST`, dead code, `PUSH`, `JUMP`). Analyze the gas impact of the obfuscation.
*   **Testing**: Add comprehensive unit and integration tests.
*   **Error Handling**: Improve error handling (e.g., for invalid input bytecode).
*   **Library Usage**: Refactor the code to be more easily usable as a library in other Rust projects. 