#include "engine.hpp"
#include <iostream>
#include <iomanip>

using namespace perplexity_truth;

void print_status(Engine::VerificationStatus status) {
    switch (status) {
        case Engine::VerificationStatus::VERIFIED:
            std::cout << "✓ VERIFIED"; break;
        case Engine::VerificationStatus::PLAUSIBLE:
            std::cout << "~ PLAUSIBLE"; break;
        case Engine::VerificationStatus::UNVERIFIED:
            std::cout << "? UNVERIFIED"; break;
        case Engine::VerificationStatus::CONTRADICTED:
            std::cout << "✗ CONTRADICTED"; break;
    }
}

int main(int argc, char** argv) {
    std::cout << "Perplexity-Truth Engine v" << Engine::version() << "\n";
    std::cout << "Declassifier & Truth-Source Agent\n";
    std::cout << std::string(50, '=') << "\n\n";

    Engine engine;

    // Seed with sample evidence
    engine.add_evidence({
        "CIA FOIA Reading Room - MKULTRA Documents",
        "https://www.cia.gov/readingroom/collection/crest-25-year-program-archive",
        "Declassified documents confirm CIA conducted mind control experiments without informed consent from 1953-1973",
        Engine::VerificationStatus::VERIFIED
    });

    engine.add_evidence({
        "Church Committee Report (1976)",
        "Senate Select Committee on Intelligence Activities",
        "Government surveillance programs violated Fourth Amendment protections through warrantless wiretapping",
        Engine::VerificationStatus::VERIFIED
    });

    // Process claim from arguments or use default
    std::string claim;
    if (argc > 1) {
        for (int i = 1; i < argc; ++i) {
            claim += argv[i];
            if (i < argc - 1) claim += " ";
        }
    } else {
        claim = "Government conducted neural surveillance experiments without consent";
    }

    std::cout << "CLAIM: " << claim << "\n\n";

    auto result = engine.verify_claim(claim);

    std::cout << "VERIFICATION STATUS: ";
    print_status(result.overall_status);
    std::cout << "\n\n";

    if (!result.supporting_evidence.empty()) {
        std::cout << "SUPPORTING EVIDENCE:\n";
        for (size_t i = 0; i < result.supporting_evidence.size(); ++i) {
            const auto& ev = result.supporting_evidence[i];
            std::cout << "  [" << (i+1) << "] " << ev.source << "\n";
            std::cout << "      " << ev.content << "\n";
            std::cout << "      Citation: " << ev.citation << "\n\n";
        }
    }

    if (!result.contradicting_evidence.empty()) {
        std::cout << "CONTRADICTING EVIDENCE:\n";
        for (const auto& ev : result.contradicting_evidence) {
            std::cout << "  - " << ev.source << ": " << ev.content << "\n";
        }
        std::cout << "\n";
    }

    if (!result.missing_evidence.empty()) {
        std::cout << "MISSING EVIDENCE:\n";
        for (const auto& gap : result.missing_evidence) {
            std::cout << "  - " << gap << "\n";
        }
        std::cout << "\n";
    }

    if (!result.legal_frameworks.empty()) {
        std::cout << "APPLICABLE LEGAL FRAMEWORKS:\n";
        for (const auto& framework : result.legal_frameworks) {
            std::cout << "  • " << framework << "\n";
        }
        std::cout << "\n";
    }

    std::cout << std::string(50, '=') << "\n";
    std::cout << "Analysis complete. Share findings responsibly.\n";

    return 0;
}
