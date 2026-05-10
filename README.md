Perplexity-Truth/
  README.md
  Cargo.toml
  src/
    main.rs               # Rust entrypoint (CLI / service)
    debunk.rs             # Rust facade: calls into C++ and Lua
    db.rs                 # SQLite helpers
  cpp/
    CMakeLists.txt
    engine.cpp            # C++ stubs for “reasoning / ranking”
    engine.hpp
  lua/
    classify.lua          # basic claim classification stub
    normalize.lua         # text normalization stub
  db/
    schema.sql            # SQLite schema
  .github/
    workflows/
      ci.yml              # Simple build/test CI
