#include "engine.hpp"
#include <iostream>
#include <cassert>

using namespace perplexity_truth;

void test_version() {
    std::cout << "Testing version()... ";
    assert(!Engine::version().empty());
    std::cout << "✓\n";
}

void test_add_and_search_evidence() {
    std::cout << "Testing add_evidence() and search_evidence()... ";
    
    Engine engine;
    Engine::Evidence ev{
        "Test Source",
        "test:1",
        "This document discusses neural surveillance technology",
        Engine::VerificationStatus::VERIFIED
    };
    
    engine.add_evidence(ev);
    auto results = engine.search_evidence("neural surveillance");
    
    assert(results.size() == 1);
    assert(results[0].source == "Test Source");
    
    std::cout << "✓\n";
}

void test_verify_claim_verified() {
    std::cout << "Testing verify_claim() with verified evidence... ";
    
    Engine engine;
    engine.add_evidence({
        "Declassified CIA Document",
        "cia:mkultra:001",
        "Project MKULTRA involved non-consensual experiments on human subjects",
        Engine::VerificationStatus::VERIFIED
    });
    
    auto result = engine.verify_claim("CIA conducted non-consensual experiments");
    
    assert(result.overall_status == Engine::VerificationStatus::VERIFIED);
    assert(!result.supporting_evidence.empty());
    
    std::cout << "✓\n";
}

void test_verify_claim_unverified() {
    std::cout << "Testing verify_claim() with no evidence... ";
    
    Engine engine;
    auto result = engine.verify_claim("Random unsupported claim about nothing");
    
    assert(result.overall_status == Engine::VerificationStatus::UNVERIFIED);
    assert(result.supporting_evidence.empty());
    assert(!result.missing_evidence.empty());
    
    std::cout << "✓\n";
}

void test_legal_framework_mapping() {
    std::cout << "Testing map_legal_frameworks()... ";
    
    Engine engine;
    auto frameworks = engine.map_legal_frameworks("Government surveillance violated privacy rights");
    
    assert(!frameworks.empty());
    bool found_fourth = false;
    for (const auto& fw : frameworks) {
        if (fw.find("Fourth Amendment") != std::string::npos) {
            found_fourth = true;
            break;
        }
    }
    assert(found_fourth);
    
    std::cout << "✓\n";
}

void test_neurorights_framework() {
    std::cout << "Testing neurorights framework detection... ";
    
    Engine engine;
    auto frameworks = engine.map_legal_frameworks("Neural data collection without consent");
    
    bool found_neurorights = false;
    for (const auto& fw : frameworks) {
        if (fw.find("Mental Privacy") != std::string::npos || 
            fw.find("neurorights") != std::string::npos) {
            found_neurorights = true;
            break;
        }
    }
    assert(found_neurorights);
    
    std::cout << "✓\n";
}

int main() {
    std::cout << "Running Perplexity-Truth Engine Tests\n";
    std::cout << std::string(50, '=') << "\n\n";
    
    test_version();
    test_add_and_search_evidence();
    test_verify_claim_verified();
    test_verify_claim_unverified();
    test_legal_framework_mapping();
    test_neurorights_framework();
    
    std::cout << "\n" << std::string(50, '=') << "\n";
    std::cout << "All tests passed ✓\n";
    
    return 0;
}
