# Contributing

Contributions are welcome. You can:

- Open an **issue** for bugs or feature ideas.
- Open a **pull request** with code changes.

By contributing, you agree that your contributions will be licensed under the same license as this project (MIT).

## Development

```bash
git clone https://github.com/lohit-dev/broken-link-checker
cd broken_link_checker
cargo test
cargo build --release
```

Run the binary with a URL or file to test:

```bash
cargo run -- https://example.com
# or
cargo run -- --file index.html
```
