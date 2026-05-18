#!/usr/bin/env bash
set -e

# Initialize a new Rust binary+lib project with the desired skeleton.
# Run from the repo root: ./scripts/init-repo.sh

# Create core directories
mkdir -p config
mkdir -p src/invariants
mkdir -p src/rights
mkdir -p src/api
mkdir -p tests
mkdir -p docs/invariants
mkdir -p .github/workflows
mkdir -p examples
mkdir -p data
mkdir -p scripts

# Create placeholder Cargo.toml if missing
if [ ! -f Cargo.toml ]; then
  cat > Cargo.toml <<'EOF'
[package]
name = "perplexity_truth"
version = "0.1.0"
edition = "2021"

[dependencies]
serde = { version = "1", features = ["derive"] }
serde_json = "1"
tokio = { version = "1", features = ["macros", "rt-multi-thread"] }
warp = "0.3"
perplexity_truth_tiers = { path = "./perplexity_truth_tiers", optional = true }

[features]
default = []
EOF
fi

# Create src/lib.rs if missing
if [ ! -f src/lib.rs ]; then
  cat > src/lib.rs <<'EOF'
pub mod invariants;
pub mod rights;
pub mod api;
EOF
fi

# Create src/main.rs if missing
if [ ! -f src/main.rs ]; then
  cat > src/main.rs <<'EOF'
fn main() {
    println!("Perplexity-Truth CLI stub. Integrate with tiers and invariants here.");
}
EOF
fi

# Create basic LICENSE if missing (MIT as placeholder)
if [ ! -f LICENSE ]; then
  cat > LICENSE <<'EOF'
MIT License

Copyright (c)

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, subject to the conditions stated in this file.
EOF
fi

# Create basic README
if [ ! -f README.md ]; then
  cat > README.md <<'EOF'
# Perplexity-Truth

Evidence-tiered, invariants-driven analysis toolkit for civil liberties, neurorights, and documentation of complex incidents.
EOF
fi

# Create stub invariant module if missing
if [ ! -f src/invariants/mod.rs ]; then
  cat > src/invariants/mod.rs <<'EOF'
pub mod function_rules;
EOF
fi

# Create stub rights module if missing
if [ ! -f src/rights/mod.rs ]; then
  cat > src/rights/mod.rs <<'EOF'
pub mod neurorights;
EOF
fi

# Create stub API module if missing
if [ ! -f src/api/mod.rs ]; then
  cat > src/api/mod.rs <<'EOF'
pub mod invariant_validator;
EOF
fi

# Create basic test file if missing
if [ ! -f tests/invariants_tests.rs ]; then
  cat > tests/invariants_tests.rs <<'EOF'
#[test]
fn smoke_test() {
    assert_eq!(2 + 2, 4);
}
EOF
fi

echo "Repository skeleton initialized."

chmod +x scripts/init-repo.sh && ./scripts/init-repo.sh
