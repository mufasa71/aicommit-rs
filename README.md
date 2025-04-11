# aicomment-rs (in development)

Using Google Gemini to generate git comments.

Read `examples/aicomment-template` to understand what exactly is generated.

## Installation

Clone the repository and build the project:

```bash
cargo build --path .
```

Add `~/.cargo/bin/` into your PATH.

Copy `aicomment.toml` and `aicomment-template` into your home directory:

```bash
cp examples/aicomment.toml ~/.aicomment.toml
cp examples/aicomment-template ~/.aicomment-template
```

Replace `~/.aicomment.toml` with your own configuration file.

```bash
gemini_api_key = ""

```

Follow instructions to get your API key from [Google Gemini] (<https://ai.google.dev/gemini-api/docs/quickstart>)
