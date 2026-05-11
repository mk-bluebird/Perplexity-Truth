# Building Perplexity-Truth Engine

## Prerequisites

- CMake 3.16+
- C++17 compatible compiler (GCC 7+, Clang 5+, MSVC 2017+)

## Quick Start

```bash
# From repository root
mkdir build && cd build
cmake ../cpp
make -j$(nproc)

# Run tests
./bin/perplexity_truth_engine_tests

# Run CLI example
./bin/perplexity_truth_engine_cli "Government conducted surveillance without warrants"
```

## Build Options

```bash
cmake ../cpp \
  -DPTE_BUILD_SHARED=ON \          # Build shared library
  -DPTE_BUILD_TESTS=ON \           # Build tests (default: ON)
  -DPTE_BUILD_EXAMPLES=ON \        # Build examples (default: ON)
  -DPTE_WARNINGS_AS_ERRORS=ON      # Strict compilation
```

## Integration with Other Projects

```cmake
find_package(perplexity_truth_engine REQUIRED)
target_link_libraries(your_target PRIVATE perplexity_truth::perplexity_truth_engine)
```
