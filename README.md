# EVM Bytecode Obfuscator (Rust)

This project implements a simple EVM bytecode obfuscator written in Rust. The primary goal is to make static analysis of Ethereum smart contract bytecode more difficult by obscuring direct control flow transfers (`PUSH-JUMP` sequences).

## How it Works

This project implements an EVM bytecode obfuscator in Rust. Its primary goal is to make static analysis of Ethereum smart contract bytecode more difficult by obscuring direct control flow transfers and modifying how contract code is structured and deployed.

**Core Obfuscation Technique: `PUSH-JUMP` Redirection**

The foundational obfuscation technique targets sequences of `PUSHn <destination>` followed immediately by a `JUMP` instruction within the **runtime bytecode** of a smart contract. For each such sequence found in the input bytecode, the obfuscator performs the following steps:

1.  **Append `JUMPDEST`**: A new `JUMPDEST` opcode (`5b`) is appended to the end of the current bytecode block being processed.
2.  **Modify `PUSH` Value**: The original `PUSHn <destination>` instruction's `<destination>` value is replaced with the byte offset of this newly appended `JUMPDEST`. This effectively redirects the original jump to a temporary location.
3.  **Inject Dead Code**: A randomly selected snippet of valid-but-semantically-useless bytecode ("dead code") is inserted immediately after the modified `PUSH` and the original `JUMP`. Any `PUSH-JUMP` sequences *within* this injected dead code are recursively analyzed and their relative jump targets adjusted to remain correct after insertion into the main bytecode.
4.  **Append Trampoline `PUSH-JUMP`**: A new `PUSH1 <original_destination>` followed by a `JUMP` (`56`) is appended after the dead code. This trampoline's purpose is to restore the original control flow.

**The Overall Effect on Runtime Bytecode:**

*   The original `PUSH-JUMP` sequence no longer jumps directly to its intended logical target. Instead, it jumps to the newly added `JUMPDEST` that precedes the injected dead code.
*   Execution then falls through the dead code. This dead code is designed to consume gas and perform operations that do not alter the contract's state or intended logic but can confuse analysis tools.
*   After the dead code, execution reaches the appended `PUSH-JUMP` trampoline.
*   This trampoline pushes the *original* intended jump destination onto the stack and executes a `JUMP`, thereby transferring control to the actual target of the original, pre-obfuscation jump.

This multi-step redirection through dead code and a trampoline significantly complicates the control flow graph (CFG) as perceived by static analysis tools. It makes it harder to directly identify the true destinations of jumps and the overall logical structure of the contract.

**Handling `initcode` and `runtime_bytecode`**

Modern smart contracts are deployed using "creation code," which typically consists of two main parts:

1.  **`initcode` (Constructor Code)**: This code executes only once, during contract deployment. Its responsibilities include setting initial storage values (i.e., running the constructor logic) and, crucially, returning the `runtime_bytecode`. The obfuscator processes this part to identify where the runtime bytecode begins.
2.  **`runtime_bytecode` (On-Chain Code)**: This is the actual code that resides on the blockchain at the contract's address and is executed during subsequent interactions with the contract. This is the primary target for obfuscation.

The obfuscator intelligently separates the `creation_code` into its `initcode` and `runtime_bytecode` components. The core obfuscation techniques, such as `PUSH-JUMP` redirection and dead code injection, are applied predominantly to the `runtime_bytecode`.

**Updating Runtime Offsets in `initcode` (The `update_runtime_offset` Logic)**

A critical step after obfuscating the `runtime_bytecode` is ensuring the `initcode` remains consistent and functional. The `initcode` often uses opcodes like `CODECOPY` to copy the `runtime_bytecode` from the creation code payload into memory, and then `RETURN` to specify the location and size of this `runtime_bytecode` for the EVM to store on-chain.

The obfuscation process inevitably modifies the `runtime_bytecode`, most notably changing its length due to the injection of dead code, `JUMPDESTs`, and trampolines. If the `initcode` is not updated to reflect these changes, it will attempt to copy and return an incorrect segment or length of code, leading to deployment failure or a malformed/non-functional contract.

The `update_runtime_offset` function (or similar logic) addresses this by:

*   **Purpose**: To dynamically adjust specific `PUSH` instructions within the `initcode` that define the *length* of the `runtime_bytecode`. These `PUSH` instructions are typically part of the sequence that prepares arguments for `CODECOPY` (which copies the runtime code) and `RETURN` (which returns the runtime code for deployment).
*   **Mechanism (Conceptual)**:
    *   The `initcode` is parsed to locate the pattern responsible for copying and returning the runtime code. This usually involves finding the `CODECOPY` opcode and the final `RETURN` opcode of the `initcode`.
    *   The `PUSH` instruction that specifies the *length* of the runtime code to be copied by `CODECOPY` (and subsequently returned by `RETURN`) is identified.
    *   The original value pushed by this instruction (representing the length of the original `runtime_bytecode`) is replaced with the new length of the *obfuscated* `runtime_bytecode`.
    *   This ensures that the `initcode`, when executed by the EVM during deployment, correctly copies and returns the complete, obfuscated `runtime_bytecode`.
*   **Importance**: This adjustment is vital for the successful deployment of the obfuscated contract. It guarantees that the contract's on-chain footprint accurately reflects the transformations applied during obfuscation.

**Theoretical Impact on Static Analysis**

The combined obfuscation strategies aim to:

*   **Obscure Control Flow Graph (CFG)**: By introducing indirect jumps, dead code, and trampolines, the CFG becomes significantly more complex. Edges in the graph may not represent direct logical succession, making automated CFG reconstruction and analysis challenging.
*   **Hinder Disassembly and Decompilation**: Disassemblers might misinterpret jump targets or the boundaries of basic blocks. Decompilers relying on a clean CFG will produce less readable or incorrect high-level code.
*   **Disrupt Pattern Matching**: Automated tools searching for known vulnerabilities or specific code patterns (e.g., function signatures, standard library calls) can be easily misled by the injected code and altered instruction sequences.
*   **Increase Manual Analysis Effort**: While not making analysis impossible, these techniques substantially increase the time, effort, and expertise required for a human analyst to understand the contract's true behavior.

## Project Structure

```
obfuscator_rs/
├── Cargo.toml              # Rust project manifest
├── README.md               # This file
└── src/
    ├── main.rs             # Main executable entry point
    ├── lib.rs              # Library entry point (if used as a library)
    ├── analysis/
    │   ├── mod.rs
    │   └── jump_seq.rs     # Logic for finding PUSH-JUMP sequences
    │   └── push_codecopy_seq.rs # Logic for finding PUSH-CODECOPY sequences
    ├── constants/
    │   ├── mod.rs
    │   └── opcodes.rs      # EVM opcode definitions and sizes
    ├── bytecode_utils/
    │   ├── mod.rs
    │   └── bytecode.rs     # Utility functions for manipulating bytecode strings
    └── obfuscation/
        ├── mod.rs
        └── obfuscate.rs    # Core obfuscation logic
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

The `main.rs` is configured to process an EVM creation code string, typically hardcoded for demonstration or read from a source.

1.  Run the project using Cargo:
    ```bash
    cargo run
    ```
    This command compiles and runs the project in one step. For an optimized release build:
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

2.  The program will perform the following sequence:
    *   Accept the input EVM creation bytecode.
    *   Separate the `initcode` from the `runtime_bytecode`.
    *   Apply the suite of obfuscation techniques (primarily targeting the `runtime_bytecode`).
    *   Critically, update length parameters within the `initcode` (e.g., via `update_runtime_offset` logic) to ensure it correctly deploys the now-modified (obfuscated) `runtime_bytecode`.
    *   Prepare the final obfuscated creation bytecode.

### Output

The resulting obfuscated creation bytecode (comprising the adjusted `initcode` and the transformed `runtime_bytecode`) is written to a text file located in the project's root directory. This file is typically named `obfuscated_bytecode.txt`. The hex string representation of the complete, deployable bytecode is stored in this file, offering a convenient way to access the obfuscated output for deployment or further examination, an improvement over console-only output.

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