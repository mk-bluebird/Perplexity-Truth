#include "engine.hpp"
#include <cstring>

// Extremely cheap stub: longer claims → slightly higher “confidence”,
// but always bounded and deterministic. Replace later with real logic.
float pt_engine_score_claim(const char* claim) {
    if (!claim) return 0.0f;
    std::size_t len = std::strlen(claim);
    if (len == 0) return 0.0f;

    // Example: map length to 0.2–0.9
    float base = 0.2f;
    float bonus = static_cast<float>(len % 70) / 100.0f; // 0.0–0.69
    float score = base + bonus;
    if (score > 0.9f) score = 0.9f;
    return score;
}
