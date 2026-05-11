cmake_minimum_required(VERSION 3.16)

# Project configuration
project(perplexity_truth_engine
    VERSION 0.1.0
    LANGUAGES CXX
)

# C++ standard configuration
set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)
set(CMAKE_CXX_EXTENSIONS OFF)

# Build options
option(PTE_BUILD_SHARED    "Build perplexity_truth_engine as a shared library" OFF)
option(PTE_BUILD_TESTS     "Build unit tests for perplexity_truth_engine" ON)
option(PTE_BUILD_EXAMPLES  "Build example binaries for perplexity_truth_engine" ON)
option(PTE_WARNINGS_AS_ERRORS "Treat compiler warnings as errors" OFF)

# Output directories (for clarity when used in Spaces/containers)
set(CMAKE_ARCHIVE_OUTPUT_DIRECTORY "${CMAKE_BINARY_DIR}/lib")
set(CMAKE_LIBRARY_OUTPUT_DIRECTORY "${CMAKE_BINARY_DIR}/lib")
set(CMAKE_RUNTIME_OUTPUT_DIRECTORY "${CMAKE_BINARY_DIR}/bin")

# Collect sources
# Adjust this list as you add more implementation files
set(PTE_SOURCES
    engine.cpp
)

set(PTE_HEADERS
    engine.hpp
)

# Library type selection
if(PTE_BUILD_SHARED)
    add_library(perplexity_truth_engine SHARED ${PTE_SOURCES} ${PTE_HEADERS})
else()
    add_library(perplexity_truth_engine STATIC ${PTE_SOURCES} ${PTE_HEADERS})
endif()

# Public include directories
target_include_directories(perplexity_truth_engine
    PUBLIC
        ${CMAKE_CURRENT_SOURCE_DIR}
)

# Compiler warnings configuration
if(MSVC)
    target_compile_options(perplexity_truth_engine PRIVATE
        /W4
        $<$<BOOL:${PTE_WARNINGS_AS_ERRORS}>:/WX>
    )
else()
    target_compile_options(perplexity_truth_engine PRIVATE
        -Wall
        -Wextra
        -Wpedantic
        $<$<BOOL:${PTE_WARNINGS_AS_ERRORS}>:-Werror>
    )
endif()

# Position independent code for better compatibility with shared libs / plugins
set_property(TARGET perplexity_truth_engine PROPERTY POSITION_INDEPENDENT_CODE ON)

# Exported properties for integration into larger CMake projects / Spaces
set_target_properties(perplexity_truth_engine PROPERTIES
    OUTPUT_NAME "perplexity_truth_engine"
    VERSION     ${PROJECT_VERSION}
    SOVERSION   ${PROJECT_VERSION_MAJOR}
    CXX_STANDARD 17
)

# Install rules (useful for containerized builds and Spaces that reuse artifacts)
include(GNUInstallDirs)

install(TARGETS perplexity_truth_engine
    EXPORT perplexity_truth_engineTargets
    ARCHIVE DESTINATION ${CMAKE_INSTALL_LIBDIR}
    LIBRARY DESTINATION ${CMAKE_INSTALL_LIBDIR}
    RUNTIME DESTINATION ${CMAKE_INSTALL_BINDIR}
)

install(FILES ${PTE_HEADERS}
    DESTINATION ${CMAKE_INSTALL_INCLUDEDIR}/perplexity_truth_engine
)

install(EXPORT perplexity_truth_engineTargets
    FILE perplexity_truth_engineTargets.cmake
    NAMESPACE perplexity_truth::
    DESTINATION ${CMAKE_INSTALL_LIBDIR}/cmake/perplexity_truth_engine
)

include(CMakePackageConfigHelpers)

write_basic_package_version_file(
    "${CMAKE_CURRENT_BINARY_DIR}/perplexity_truth_engineConfigVersion.cmake"
    VERSION ${PROJECT_VERSION}
    COMPATIBILITY SameMajorVersion
)

configure_package_config_file(
    "${CMAKE_CURRENT_SOURCE_DIR}/cmake/perplexity_truth_engineConfig.cmake.in"
    "${CMAKE_CURRENT_BINARY_DIR}/perplexity_truth_engineConfig.cmake"
    INSTALL_DESTINATION ${CMAKE_INSTALL_LIBDIR}/cmake/perplexity_truth_engine
)

install(FILES
    "${CMAKE_CURRENT_BINARY_DIR}/perplexity_truth_engineConfig.cmake"
    "${CMAKE_CURRENT_BINARY_DIR}/perplexity_truth_engineConfigVersion.cmake"
    DESTINATION ${CMAKE_INSTALL_LIBDIR}/cmake/perplexity_truth_engine
)

# Optional: tests (expects a tests/ directory with CMakeLists or sources)
if(PTE_BUILD_TESTS)
    enable_testing()

    # Example simple test executable (adjust filenames as needed)
    # File suggestion: cpp/tests/engine_tests.cpp
    add_executable(perplexity_truth_engine_tests
        tests/engine_tests.cpp
    )
    target_link_libraries(perplexity_truth_engine_tests
        PRIVATE
            perplexity_truth_engine
    )
    target_include_directories(perplexity_truth_engine_tests
        PRIVATE
            ${CMAKE_CURRENT_SOURCE_DIR}
    )

    add_test(
        NAME perplexity_truth_engine_basic
        COMMAND perplexity_truth_engine_tests
    )
endif()

# Optional: examples / tools (for Spaces integration sanity-checks)
if(PTE_BUILD_EXAMPLES)
    # File suggestion: cpp/examples/engine_cli.cpp
    add_executable(perplexity_truth_engine_cli
        examples/engine_cli.cpp
    )
    target_link_libraries(perplexity_truth_engine_cli
        PRIVATE
            perplexity_truth_engine
    )
    target_include_directories(perplexity_truth_engine_cli
        PRIVATE
            ${CMAKE_CURRENT_SOURCE_DIR}
    )
endif()
