#pragma once

extern "C" {
    // Return a simple confidence or plausibility score.
    // 0.0–1.0, purely a stub, deterministic and cheap.
    float pt_engine_score_claim(const char* claim);
}
