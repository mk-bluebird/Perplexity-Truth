#include "engine.hpp"
#include <algorithm>
#include <sstream>

namespace perplexity_truth {

// Private implementation (pimpl idiom for ABI stability)
class Engine::Impl {
public:
    std::vector<Engine::Evidence> evidence_db;

    std::vector<std::string> extract_keywords(const std::string& text) const {
        std::vector<std::string> keywords;
        std::istringstream stream(text);
        std::string word;
        while (stream >> word) {
            if (word.length() > 3) {
                std::transform(word.begin(), word.end(), word.begin(), ::tolower);
                keywords.push_back(word);
            }
        }
        return keywords;
    }
};

Engine::Engine() : pimpl_(std::make_unique<Impl>()) {}

Engine::~Engine() = default;

Engine::Engine(Engine&&) noexcept = default;
Engine& Engine::operator=(Engine&&) noexcept = default;

Engine::VerificationResult Engine::verify_claim(const std::string& claim) {
    VerificationResult result;
    result.claim = claim;

    auto evidence_matches = search_evidence(claim);

    for (const auto& ev : evidence_matches) {
        if (ev.status == VerificationStatus::VERIFIED || 
            ev.status == VerificationStatus::PLAUSIBLE) {
            result.supporting_evidence.push_back(ev);
        } else if (ev.status == VerificationStatus::CONTRADICTED) {
            result.contradicting_evidence.push_back(ev);
        }
    }

    if (!result.supporting_evidence.empty()) {
        auto max_status = std::max_element(
            result.supporting_evidence.begin(),
            result.supporting_evidence.end(),
            [](const Evidence& a, const Evidence& b) {
                return static_cast<int>(a.status) < static_cast<int>(b.status);
            }
        );
        result.overall_status = max_status->status;
    } else if (!result.contradicting_evidence.empty()) {
        result.overall_status = VerificationStatus::CONTRADICTED;
    } else {
        result.overall_status = VerificationStatus::UNVERIFIED;
        result.missing_evidence.push_back("No evidence found in database");
    }

    result.legal_frameworks = map_legal_frameworks(claim);

    return result;
}

void Engine::add_evidence(const Evidence& evidence) {
    pimpl_->evidence_db.push_back(evidence);
}

std::vector<Engine::Evidence> Engine::search_evidence(const std::string& query) const {
    std::vector<Evidence> results;
    auto keywords = pimpl_->extract_keywords(query);

    for (const auto& evidence : pimpl_->evidence_db) {
        std::string content_lower = evidence.content;
        std::transform(content_lower.begin(), content_lower.end(), 
                      content_lower.begin(), ::tolower);

        for (const auto& keyword : keywords) {
            if (content_lower.find(keyword) != std::string::npos) {
                results.push_back(evidence);
                break;
            }
        }
    }

    return results;
}

std::vector<std::string> Engine::map_legal_frameworks(const std::string& claim) const {
    std::vector<std::string> frameworks;
    std::string claim_lower = claim;
    std::transform(claim_lower.begin(), claim_lower.end(), claim_lower.begin(), ::tolower);

    if (claim_lower.find("surveillance") != std::string::npos ||
        claim_lower.find("privacy") != std::string::npos) {
        frameworks.push_back("Fourth Amendment (unreasonable search/seizure)");
        frameworks.push_back("ECPA (Electronic Communications Privacy Act)");
    }

    if (claim_lower.find("neural") != std::string::npos ||
        claim_lower.find("brain") != std::string::npos ||
        claim_lower.find("neuro") != std::string::npos) {
        frameworks.push_back("Mental Privacy (proposed under neurorights frameworks)");
        frameworks.push_back("Fourteenth Amendment (bodily autonomy/liberty interest)");
    }

    if (claim_lower.find("experiment") != std::string::npos ||
        claim_lower.find("consent") != std::string::npos) {
        frameworks.push_back("Nuremberg Code (informed consent)");
        frameworks.push_back("Common Rule (45 CFR 46 - human subjects research)");
    }

    if (claim_lower.find("discrimination") != std::string::npos) {
        frameworks.push_back("Equal Protection Clause (Fourteenth Amendment)");
        frameworks.push_back("Civil Rights Act of 1964");
    }

    if (frameworks.empty()) {
        frameworks.push_back("Due Process Clause (Fifth/Fourteenth Amendment)");
    }

    return frameworks;
}

std::string Engine::version() {
    return "0.1.0";
}

} // namespace perplexity_truth
