---
name: rust-validation
description: "Run the repository's complete Rust validation workflow after Rust changes or when investigating Cargo CI failures, clippy warnings, formatting errors, or test failures."
user-invocable: true
---

# Rust Validation

Run the bundled [verification script](./scripts/verify.sh) from the repository root:

```sh
sh .github/skills/rust-validation/scripts/verify.sh
```

Use the first failing command as the starting point for diagnosis. After a fix, run the script again and report the result.
