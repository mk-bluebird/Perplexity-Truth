fn main() {
    cc::Build::new()
        .cpp(true)
        .file("cpp/engine.cpp")
        .flag_if_supported("-std=c++17")
        .compile("perplexity_truth_engine");
}
