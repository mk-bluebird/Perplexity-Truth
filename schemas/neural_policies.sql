-- schemas/neural_policies.sql
-- Machine-readable policy rules for neural data compliance
-- Version: 1.0.0 | Last Updated: 2026-05-11

CREATE TABLE IF NOT EXISTS neural_policies (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    jurisdiction TEXT NOT NULL CHECK (jurisdiction IN (
        'US-CO', 'US-CA', 'US-MT', 'US-CT', 'US-FEDERAL',
        'EU', 'GLOBAL_HR', 'CHILE'
    )),
    law_or_instrument TEXT NOT NULL,
    topic TEXT NOT NULL CHECK (topic IN (
        'consent', 'purpose_limitation', 'home_use', 'workplace',
        'school', 'children', 'subliminal_manipulation',
        'emotion_inference', 'data_retention', 'third_party_sharing',
        'sensitive_classification', 'mental_privacy', 'freedom_of_thought'
    )),
    rule_text TEXT NOT NULL,
    source_url TEXT NOT NULL,
    tier INTEGER NOT NULL CHECK (tier IN (1, 2)),
    effective_date DATE,
    sunset_date DATE,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    UNIQUE(jurisdiction, law_or_instrument, topic)
);

CREATE INDEX IF NOT EXISTS idx_policies_jurisdiction_topic 
ON neural_policies(jurisdiction, topic) WHERE tier <= 2;

CREATE INDEX IF NOT EXISTS idx_policies_tier 
ON neural_policies(tier) WHERE tier = 1;

-- Pre-populate with foundational rules (sample entries)
INSERT INTO neural_policies (
    jurisdiction, law_or_instrument, topic, rule_text, source_url, tier, effective_date
) VALUES
('US-CO', 'Colorado Privacy Act', 'sensitive_classification', 
 'Neural data is classified as sensitive personal data requiring explicit consent and purpose limitation.',
 'https://leg.colorado.gov/bills/sb21-190', 1, '2023-07-01'),

('US-CA', 'California Consumer Privacy Act', 'sensitive_classification',
 'Neural data falls under sensitive personal information with enhanced consumer rights and opt-out requirements.',
 'https://oag.ca.gov/privacy/ccpa', 1, '2023-01-01'),

('EU', 'AI Act', 'subliminal_manipulation',
 'AI systems using subliminal techniques to significantly distort behavior are prohibited.',
 'https://artificialintelligenceact.eu/article/5', 1, '2025-02-02'),

('EU', 'AI Act', 'emotion_inference',
 'Emotion recognition AI is banned in workplace and educational settings except for narrow safety/medical exceptions.',
 'https://artificialintelligenceact.eu/article/5', 1, '2025-02-02'),

('GLOBAL_HR', 'ICCPR Article 18', 'freedom_of_thought',
 'Freedom of thought, conscience, and religion is non-derogable and protects against coercive interference with mental processes.',
 'https://www.ohchr.org/en/instruments-mechanisms/instruments/international-covenant-civil-and-political-rights', 2, '1976-03-23'),

('GLOBAL_HR', 'UNESCO AI Ethics Recommendation', 'mental_privacy',
 'Neurotechnology must empower individuals to make free and informed decisions about their nervous system and mental health.',
 'https://unesdoc.unesco.org/ark:/48223/pf0000381137', 2, '2021-11-24');

CREATE TRIGGER IF NOT EXISTS update_timestamp 
AFTER UPDATE ON neural_policies
BEGIN
    UPDATE neural_policies SET updated_at = CURRENT_TIMESTAMP WHERE id = NEW.id;
END;
