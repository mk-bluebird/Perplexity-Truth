#ifndef PERPLEXITY_TRUTH_ENGINE_HPP
#define PERPLEXITY_TRUTH_ENGINE_HPP

#include <string>
#include <vector>
#include <memory>

namespace perplexity_truth {

/**
 * @brief Core verification engine for declassified document analysis
 * 
 * This engine provides fact-checking, evidence verification, and legal
 * framework mapping for neurorights advocacy and civil liberties defense.
 */
class Engine {
public:
    enum class VerificationStatus {
        VERIFIED,      // Tier 1: Declassified docs, court records, peer-reviewed
        PLAUSIBLE,     // Tier 2: Investigative journalism, expert testimony
        UNVERIFIED,    // Tier 3: Single-source, circumstantial
        CONTRADICTED   // Evidence contradicts claim
    };

    struct Evidence {
        std::string source;
        std::string citation;
        std::string content;
        VerificationStatus status;
    };

    struct VerificationResult {
        std::string claim;
        VerificationStatus overall_status;
        std::vector<Evidence> supporting_evidence;
        std::vector<Evidence> contradicting_evidence;
        std::vector<std::string> missing_evidence;
        std::vector<std::string> legal_frameworks;
    };

    Engine();
    ~Engine();

    // Disable copy, enable move
    Engine(const Engine&) = delete;
    Engine& operator=(const Engine&) = delete;
    Engine(Engine&&) noexcept;
    Engine& operator=(Engine&&) noexcept;

    /**
     * @brief Verify a factual claim against available evidence
     * @param claim The claim to verify
     * @return Structured verification result with evidence and legal context
     */
    VerificationResult verify_claim(const std::string& claim);

    /**
     * @brief Add evidence to the engine's knowledge base
     * @param evidence Evidence record to add
     */
    void add_evidence(const Evidence& evidence);

    /**
     * @brief Search for evidence matching query
     * @param query Search query
     * @return Matching evidence records
     */
    std::vector<Evidence> search_evidence(const std::string& query) const;

    /**
     * @brief Map claim to relevant legal frameworks
     * @param claim The claim to analyze
     * @return List of applicable constitutional/statutory provisions
     */
    std::vector<std::string> map_legal_frameworks(const std::string& claim) const;

    /**
     * @brief Get engine version
     * @return Version string
     */
    static std::string version();

private:
    class Impl;
    std::unique_ptr<Impl> pimpl_;
};

} // namespace perplexity_truth

#endif // PERPLEXITY_TRUTH_ENGINE_HPP
