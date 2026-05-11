-- schemas/compliance_check_items.sql
-- Human-review checklist items for neurorights compliance
-- Version: 1.0.0 | Last Updated: 2026-05-11

CREATE TABLE IF NOT EXISTS compliance_check_items (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    topic TEXT NOT NULL CHECK (topic IN (
        'consent', 'purpose_limitation', 'home_use', 'workplace',
        'school', 'children', 'subliminal_manipulation',
        'emotion_inference', 'data_retention', 'third_party_sharing'
    )),
    context TEXT NOT NULL CHECK (context IN (
        'home_private_use', 'virtual_platform_tracking',
        'workplace_monitoring', 'medical_research', 'children_interaction'
    )),
    checklist_text TEXT NOT NULL,
    verification_method TEXT CHECK (verification_method IN (
        'document_review', 'code_audit', 'user_interview', 'technical_test'
    )),
    severity TEXT NOT NULL CHECK (severity IN ('critical', 'high', 'medium', 'low')),
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    UNIQUE(topic, context, checklist_text)
);

CREATE INDEX IF NOT EXISTS idx_checks_topic_context 
ON compliance_check_items(topic, context);

-- Pre-populate with foundational checklist items
INSERT INTO compliance_check_items (
    topic, context, checklist_text, verification_method, severity
) VALUES
('consent', 'home_private_use', 
 'Confirm neural data collection is disclosed in plain language at the point of capture, not buried in terms of service.',
 'document_review', 'critical'),

('consent', 'home_private_use',
 'Verify consent mechanism is granular (per data use), affirmative (opt-in), and revocable without service degradation.',
 'technical_test', 'critical'),

('purpose_limitation', 'virtual_platform_tracking',
 'Ensure neural data collected for primary function (e.g., game control) is not repurposed for advertising or AI training without separate consent.',
 'code_audit', 'high'),

('subliminal_manipulation', 'virtual_platform_tracking',
 'Audit UX flows for non-obvious cues that leverage neural feedback to influence user choices (e.g., dynamic pricing based on attention metrics).',
 'technical_test', 'critical'),

('emotion_inference', 'workplace_monitoring',
 'Confirm emotion or cognitive state inference is not used for employment decisions, performance evaluations, or disciplinary actions.',
 'document_review', 'critical'),

('children', 'children_interaction',
 'Verify enhanced safeguards for users under 18: parental consent, data minimization, and prohibition of behavioral profiling.',
 'document_review', 'critical'),

('third_party_sharing', 'home_private_use',
 'Review data sharing agreements to ensure third parties adhere to same neurorights standards and purpose limitations.',
 'document_review', 'high'),

('data_retention', 'medical_research',
 'Confirm automated deletion schedules for neural data post-study completion, with exceptions only for legally mandated retention.',
 'code_audit', 'medium');
