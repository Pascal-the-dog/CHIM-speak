# CHIM-speak



CHIM-speak is a memory-safe, high-stakes, Turing-complete esoteric programming language written in pure Rust.

What happens if you combine the brutal pointer mechanics of **Brainfuck**,  the verbose, dramatic readability of the **Shakespeare Programming Language (SPL)**’s proses and add in some Morrowind flavour?

This. You get this.

---

## 🌌 The Core Philosophy: High-Stakes Coding

Unlike modern languages that coddle you with "warnings" or "syntax suggestions," CHIM-speak respects the cosmic balance of the universe. Code written in CHIM-speak interacts directly with the Earth-Bones of your operating system.

---

### 💀 The Zero-Sum Feature

The compiler and runtime engine are written in pure Rust for maximum execution speed and safety. However, the pipeline is entirely unforgiving. If the lexer encounters an unrecognized character sequence, if the parser catches a structural loop misalignment, or if the interpreter hits a runtime trap (like a variable that doesn't exist or division by zero), the script will immediately realize the fictional nature of its own existence.

It will panic, fail its ego-check, and **instantly zero-sum (permanently delete itself from your hard drive)** via `std::fs::remove_file`.

---

### 📝 The Scribe's Trial (How This Was Forged)

The developer did not write this compiler from the safety of an IDE. Every line of the compiler, the lexer, and the initial test suites was written using an unconfigured, bare-bones version of the **Kate** text editor, and executed directly out of the terminal. The target source files themselves were composed straight over the the terminal stream via **`nano`**.

No local history backups. No auto-save recovery nets. No LSP hand-holding.

During early development, the compiler's `trigger_zero_sum()` function accidentally triggered on its own source files multiple times due to logic loops. To code CHIM-speak is to look into the void. The creator survived the wheel. You might not. *Live by the wheel, die by the wheel.*

---

## 📜 Syntax Rules

1. **Pure Alpha-Numeric Only:** Absolutely no symbols (`[`, `]`, `+`, `-`, `;`, etc.) are permitted. Punctuation upsets the Tribunal.
2. **Raw Digit Logic:** Numeric evaluations and loop parameters must be written as raw digits (`3`, `5`, `100`). Word variants are heretical.
3. **Theatrical Execution:** Memory operations mimic SPL. Variables are treated as entities interacting within "Houses" or "Planes of Oblivion" governed by algebraic equations.

### 1. Loop Orchestration

Every `.chim` program must declare its structural time boundaries at startup and close gracefully via the sacred acronym:
* **Initialization**: `The Wheel turns from the first dawn until the [N] iteration of the Middle Dawn`
* **Tracking Vector**: `Let the chronicler track the passing of the cycles within the house of the Counter`
* **Loop Step Hook**: `Each step along the Walking Way must be witnessed by the sleeping deity`
* **Closure**: `The ending of the words is ALMSIVI`

### 2. Parameterized Conditionals

Output can be conditionally shifted by measuring current loops against the physics of the universe.
* **Double Condition**: `If the stars align and the cycle bows to both the [X] and the [Y] simultaneously the houses collapse into a singular state and the terminal must shout [Word]`
* **Tribunal Check**: `But if the number of the current cycle can be divided cleanly by the [X] secret keys of the Tribunal leaving no remainder in the math of the Earth-Bones then let the voice of the machine proclaim [Word]`
* **Left-Hand Check**: `Yet if the cycle can instead be divided by the [X] fingers of the Left Hand let it cry [Word]`
* **Fallback Default**: `Otherwise let the machine simply whisper the raw naked number of the counter to the void`

### 3. Turing-Complete Memory Space & Algebraic Operators

CHIM-speak handles arbitrary state manipulation and nested evaluation sub-loops. Values are stored as signed 64-bit integers (`i64`) tied to your designated lore entities.

* **Variable Allocation (The Enantiomorph)**: Creates a space in the global environment heap.
  ```text
  The Rebel [Variable_Name] usurps the King [Value]
  ```
* **Addition (The Anticipation)**: Increments the state map.
  ```text
  The Anticipation of [Variable_Name] reflects the change of [Value]
  ```
* **Subtraction (The Numidium/Refusal)**: Decrements the state map.
  ```text
  The Walking Brass denies [Variable_Name] by the decrement of [Value]
  ```
* **Multiplication (The Heart/Expansion)**: Multiplies the value.
  ```text
  The Heart of Lorkhan amplifies [Variable_Name] by the iteration of [Value]
  ```
* **Division (The Sundering)**: Divides the value. *Warning: Dividing by 0 instantly triggers a Zero-Sum self-destruction.*
  ```text
  The Tools of Kagrenac sunder [Variable_Name] across the fractions of [Value]
  ```
* **Turing Loop (The Sharmat)**: A dynamic, infinite conditional `while` loop that continues to execute its nested statements recursively as long as the target variable remains greater than `0`.
  ```text
  While the Sharmat maintains the House of [Variable_Name]
     [Nested Statements]
  The House is closed
  ```

---


## 🚀 Getting Started

To compile and execute a `.chim` file natively from your workspace:

```bash
cargo run -- my_program.chim
```

**WARNING:** Ensure your code is fully committed to a remote Git repository before running this command. If you hit a logic typo or miss a closing loop statement, your file will vanish from your local machine before you can blink.

---

## 🛑 Troubleshooting (Interpreting Divine Wrath)

If your terminal falls silent and your file is gone, you have failed the ego-check. Because CHIM-speak enforces absolute existential clarity, **standard text errors do not exist**. The compiler operates on a single execution branch: absolute validation or total erasure.

However, just before the file is wiped from your disk arrays, the zero-sum engine will echo one of the following diagnostic panic strings to your terminal buffer to detail exactly how your syntax violated the Earth-Bones:

* **Unrecognized cosmic script segment:** The Lexer encountered an unapproved alphanumeric string or syntax typo. 
* **Compilation Error (The House was never closed):** A Turing loop was opened with `While the Sharmat maintains` but failed to balance its closing scope before `ALMSIVI`.
* **Mundus Runtime Error:** An equation attempted to modify or reference a variable entity that was never formally allocated or usurped by a Rebel.
* **Zero-Sum Critical Fault:** The machine attempted to divide a value by the void (0) and shattered the natural laws of physics.
* **ERROR 404: RESDAYN NOT FOUND:** (Planned Legacy Exception) You attempted to reference memory coordinates outside the designated Planes of Oblivion.
* **ERROR: THE SHAPE OF THE EGG:** (Planned Legacy Exception) A mismatched nested vision loop caused the internal AST framework to completely collapse into an egg shape and pop.

---

## 📜 Free Samples: The Test of the Scribe

For those too terrified to compose their own prose from scratch, a canonical file named `fizzbuzz.chim` is included in the root directory.

You may run it using:
```bash
cargo run -- fizzbuzz.chim
```

**CRITICAL NOTE:** Even though this file was generated by the Creator, running it still subjects it to the local compiler's Ego-Check. If your local Rust compiler architecture introduces a single byte offset or memory misalignment during compilation, the engine will still interpret it as heresy and **instantly zero-sum the sample file**. If `fizzbuzz.chim` vanishes from your local repository cloning, do not open an issue on GitHub. The Tribunal has judged your environment unworthy.

**Note:** A secondary sample file, `countdown.chim`, was originally planned for inclusion in this repository. However, during the final local compilation testing phase, a minor structural spacing misalignment caused the script to fail its ego-check. The file immediately zero-summed itself out of existence on the Creator's machine before it could be staged for a git commit. It is no longer available in this reality.

---

## ⚔️ The Oversight Committee: The Ordinators

Before you attempt to execute your prose, remember that you are being closely evaluated not just by the Rust compiler, but by **The Ordinators**.

The Temple's most ruthless inquisitors police your local terminal buffers. If your file is zero-summed, do not look to them for sympathy. They will look back at you through your monitor from behind their golden, unblinking Indoril masks with an expression of pure, unyielding theological judgment. 

They know if you are using bloated, unapproved IDE applications to cheat the system. Kate, Nano, or literal re-education. If you code a typo, expect no mercy. *We are watching you, scum.*

## 🐕 The Repository Secretary: Pascal

If your code zero-sums on your local machine, do not bother opening a GitHub Issue or submitting a Pull Request to complain about the absolute erasure of your files. 

All external repository communications are strictly managed by **Pascal**, the Creator's familiar and official GitHub Secretary under the pseudonym `pascal_the_dog`. Pascal reviews all repository traffic with an expression of absolute cosmic indifference. Any attempts to report a bug regarding the `trigger_zero_sum()` protocol will be intercepted by Pascal, evaluated with deadpan silence, and filed directly into the digital void. He will not reply.

If for any reasons contributing to this repos becomes appealing for any reasons, you will first need to read Contributing.md and respect the pull requests, bug reports, or contribution templates, our peppy secretary **Pascal** will gladly responds to any inquiries from Outlanders. 

---

## 🧱 The Wall of Shame (The Severed Threads)

Below lies the eternal record of scribes who looked into the wheel, blinked, and allowed their source code to be entirely zero-summed. Their files have vanished into the void, and their names are recorded here for the amusement of the Tribunal.

| Scribe | Offending Code / Typo | Divine Judgment Meted Out |
| :--- | :--- | :--- |
| `@Your_Name_Here?` | *Your typo goes here.* | *Open an issue with your terminal logs (if any survived) to be immortalized.* |


### 🕯️ How to Submit Your Failure
If you have failed the ego-check and your file is gone, do not weep. Open a GitHub Issue titled `[HERESY] I have failed the Wheel` and include:
1. The exact typo that cost you your source file.
2. A screenshot of your empty folder.
3. A formal apology to Pascal.

---

## ⚖️ License

The Earth-Bones License. You break it, it breaks your local file system.

> "The ending of the words is ALMSIVI."
