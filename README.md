# aicommit-rs 🔨 (in progress)

Using Google Gemini to generate git comments.

Read `examples/aicommit-template` to understand what exactly is generated.

## Installation

Clone the repository and build the project:

```bash
cargo build --path .
```

Add `~/.cargo/bin/` into your PATH.

Copy `aicommit.toml` and `aicommit-template` into your home directory:

```bash
cp examples/aicommit.toml ~/.aicommit.toml
cp examples/aicommit-template ~/.aicommit-template
```

Replace `~/.aicommit.toml` with your own configuration file.

```bash
gemini_api_key = ""
gemini_api_url = "https://generativelanguage.googleapis.com/v1beta/openai"
model_name = "gemini-2.0-flash"

```

Follow instructions to get your API key from [Google Gemini](<https://ai.google.dev/gemini-api/docs/quickstart>)

## Usage

Read [docs/usage.md](docs/usage.md) for more information.
