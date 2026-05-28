# rage_flip_rust

A Rust CLI tool for text transformations: flip text upside-down, rage flip, table flip, apply sarcasm, strikethrough, underline, chaos mode, frustrated mode, and text substitution with kaomoji/emoji.

## Installation

### Prerequisites

- Rust and Cargo (install via [rustup](https://rustup.rs/))

### Build from Source

```bash
git clone <repo_url>
cd rage_flip_rust
cargo build --release
```

The binary will be available at `target/release/rage_flip`.

### Optional Dependencies (Clipboard Support)

Clipboard features require platform-specific tools:

| Platform | Dependency       | Install Command                  |
|----------|------------------|----------------------------------|
| macOS    | `pbcopy`         | Pre-installed                    |
| Linux    | `xclip` or `xsel`| `sudo apt install xclip`         |
| Windows  | `clip`           | Pre-installed                    |

## Usage

```bash
rage_flip <SUBCOMMAND> [text]
```

If no `text` argument is provided, the command reads from standard input.

### Subcommands

| Subcommand       | Description                                      |
|------------------|--------------------------------------------------|
| `flip`           | Flip text upside-down                            |
| `rage-flip`      | Flip text upside-down with capitalization        |
| `table-flip`     | Prepend a table flip emoji                       |
| `sarcasm`        | Apply alternating case (sarcasm effect)          |
| `strikethrough`  | Apply strikethrough formatting                   |
| `underline`      | Apply underline formatting                       |
| `double-underline` | Apply double underline formatting             |
| `chaos`          | Insert random characters for chaos effect        |
| `frustrated`     | Uppercase with period separators                 |
| `substitute`     | Replace words with kaomoji/emoji equivalents     |

### Examples

```bash
# Flip text
rage_flip flip "Hello, world!"

# Rage flip from stdin
echo "I can't believe this" | rage_flip rage-flip

# Apply sarcasm
rage_flip sarcasm "Oh great, another meeting"

# Chaos mode (default level 10)
rage_flip chaos "Everything is fine"

# Strikethrough
rage_flip strikethrough "This text is crossed out"

# Frustrated mode
rage_flip frustrated "This is so annoying"
```

### Chaos Level Configuration

The chaos level defaults to `10`. To customize it, create a file at `~/.chaos_level.txt` containing your desired level (1-100):

```bash
echo "50" > ~/.chaos_level.txt
```

## Project Structure

```
rage_flip_rust/
├── Cargo.toml
└── src/
    ├── main.rs
    ├── lib.rs
    ├── flipper.rs
    ├── sarcasm.rs
    ├── strikethrough.rs
    ├── underline.rs
    ├── chaos.rs
    ├── frustrated.rs
    ├── text_substitution.rs
    └── clipboard.rs
```

## License

MIT
