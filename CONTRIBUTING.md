Thank you for contributing to VeriLog Audit Engine!

Guidelines:
- Fork the repo and make changes in a feature branch.
- Run local checks: cargo fmt, cargo clippy, cargo test.
- Keep changes small and focused; include tests for new functionality.
- Open a PR against main/master and reference the issue it fixes.

Development environment:
- Copy .env.example to .env and adjust settings.
- To run the agent locally: cargo run -p agent -- --help

CI:
- CI runs cargo fmt --check, cargo build, cargo test, and cargo clippy.
