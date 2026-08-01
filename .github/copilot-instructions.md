# Repository Guidance

- This is a Rust + Yew + WebAssembly app for GitHub Pages.
- Keep changes small and focused. Avoid broad refactors unless they are required for the task.
- Prefer repository-local context over speculation. Use the existing `src/` modules and current UI structure.
- Keep the questionnaire flow deterministic: one `most`, one `next`, and up to two `slight` selections per axis.
- Preserve `rustfmt` formatting and keep code compatible with the current stable toolchain.
- When editing logic, favor pure functions and small unit tests over ad hoc UI-driven behavior.
- For verification, use the cheapest relevant checks first: `cargo fmt --all --check`, `cargo clippy --all-targets --all-features -- -D warnings`, and `cargo test` when applicable.
- Avoid adding new dependencies unless they materially reduce complexity or improve correctness.
- Keep README and workflow updates consistent with the actual build and deploy process.
