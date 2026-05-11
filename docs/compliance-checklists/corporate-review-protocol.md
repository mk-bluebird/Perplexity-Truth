# Corporate Neurorights Compliance Review Protocol

**Document ID**: NR-CR-001  
**Version**: 1.0.0  
**Effective Date**: 2026-05-11  
**Review Cycle**: Quarterly or upon major product changes  

## Purpose

This protocol ensures all products or features involving neural data collection undergo rigorous pre-launch review against emerging neurorights standards. It operationalizes U.S. state laws, EU AI Act provisions, and human-rights doctrine into actionable engineering and policy requirements.

## Scope

Applies to:
- Consumer neurotechnology (EEG headsets, BCIs, neuro-wearables)
- VR/AR platforms with neural data integration
- Workplace or educational tools using cognitive/emotional inference
- Any system processing raw neural signals or derived mental state metrics

## Review Workflow

### Phase 1: Intake (Product Team)
- [ ] Submit `Scenario Intake Form` with:
  - Technology description and data flow diagram
  - List of neural data types collected (raw/inferred)
  - Primary and secondary data use cases
  - Target jurisdictions and user demographics
- [ ] Attach privacy impact assessment (PIA) draft

### Phase 2: Automated Classification (Compliance Engine)
- [ ] Run scenario through `neurorights-defense` classifier
- [ ] Review generated `ClassificationResult` for:
  - Context tag accuracy (home/virtual/workplace/etc.)
  - Policy theme coverage (consent, manipulation risks, etc.)
  - Risk level assignment (Low/Medium/High/Critical)
- [ ] Flag any misclassifications for manual override

### Phase 3: Policy Lookup & Gap Analysis
- [ ] Query `neural_policies` database for applicable rules
- [ ] Map each rule to implementation requirements:
  ```yaml
  rule_id: US-CO-CPA-001
  requirement: "Explicit consent for neural data collection"
  implementation:
    - UI: Granular opt-in toggle at first data capture
    - Backend: Consent state stored with neural data records
    - Audit: Consent logs retained for 7 years
  ```
- [ ] Identify gaps between current implementation and requirements

### Phase 4: Human Review Checklist
Complete all items below before launch approval:

#### Consent & Transparency
- [ ] Neural data collection disclosed in plain language at point of capture (not in EULA)
- [ ] Consent mechanism is:
  - Granular: Separate toggles per data use case
  - Affirmative: Opt-in required, no pre-checked boxes
  - Revocable: One-click withdrawal without service degradation
  - Documented: Consent state logged with timestamp and version
- [ ] Privacy notice includes:
  - Specific neural data types collected
  - Exact purposes for each data use
  - Retention period and deletion process
  - Third-party sharing recipients and purposes

#### High-Risk Practice Avoidance
- [ ] No subliminal manipulation techniques detected:
  - Audit UX flows for non-obvious cues leveraging neural feedback
  - Verify dynamic content/pricing not influenced by real-time brain metrics
  - Confirm A/B tests do not exploit cognitive vulnerabilities
- [ ] Emotion/cognitive inference not used for:
  - Employment decisions (hiring, promotion, termination)
  - Educational grading or disciplinary actions
  - Credit scoring or insurance underwriting
  - Law enforcement profiling without warrant
- [ ] Children's data (under 18) receives enhanced safeguards:
  - Parental consent mechanism compliant with COPPA/GDPR-K
  - Data minimization: collect only what's essential for primary function
  - Prohibition of behavioral profiling or targeted advertising

#### Data Governance
- [ ] Neural data classified as "sensitive" in data inventory and catalog
- [ ] Encryption at rest and in transit using FIPS 140-2 validated modules
- [ ] Access controls: role-based permissions with least-privilege principle
- [ ] Retention policy: automated deletion after purpose fulfillment
- [ ] Third-party agreements include:
  - Neurorights compliance clauses
  - Purpose limitation enforcement
  - Audit rights and breach notification terms

### Phase 5: Sign-off & Documentation
- [ ] Privacy Officer approval signature
- [ ] Engineering Lead confirmation of implemented controls
- [ ] Legal Counsel review of jurisdictional compliance
- [ ] Store completed checklist in compliance repository with:
  - Product version identifier
  - Review date and next review trigger
  - Link to supporting evidence (code commits, config files, test results)

## Escalation Path

If critical gaps are identified:
1. Halt launch timeline immediately
2. Convene cross-functional remediation team (Engineering, Privacy, Legal)
3. Document remediation plan with timelines and owners
4. Re-run compliance review after fixes implemented
5. Escalate to CISO/General Counsel if unresolved after 14 days

## Maintenance

- Update this protocol quarterly based on:
  - New state/federal neural-data legislation
  - EU AI Act implementing acts and guidance
  - Human-rights jurisprudence developments
- Retrain classification engine lexicon with new scenario examples
- Refresh policy database with latest rule texts and source URLs

## Appendix A: Scenario Intake Form Template

```yaml
product_name: string
version: string
technology_description: string
data_flow_diagram_url: string  # Link to architecture diagram
neural_data_types:
  - raw_neural_signals
  - inferred_emotional_state
  - cognitive_load_metrics
  - intention_predictions
data_uses:
  - primary_function
  - advertising
  - ai_training
  - third_party_sharing
target_jurisdictions:
  - US-CO
  - US-CA
  - EU
user_demographics:
  - adults_18_plus
  - minors_under_18
  - vulnerable_populations  # Specify if applicable
privacy_impact_assessment_url: string
submitted_by: string  # Name and role
submission_date: YYYY-MM-DD
```

## Appendix B: Evidence Collection Guidelines

For each checklist item, collect:
- **Documentary**: Policy documents, consent UI screenshots, data flow diagrams
- **Technical**: Code commits implementing controls, config files, test results
- **Process**: Meeting notes, approval emails, training records
- **Audit**: Logs demonstrating control operation (e.g., consent revocation events)

Store evidence in version-controlled repository with:
```
evidence/
├── product_name/
│   ├── version/
│   │   ├── consent_ui_screenshots/
│   │   ├── code_diffs/
│   │   ├── test_reports/
│   │   └── approval_records/
```

## Appendix C: Glossary

- **Neural Data**: Any information derived from the measurement or analysis of brain activity, including raw signals (EEG, fNIRS) and inferred states (emotion, attention, intention).
- **Subliminal Manipulation**: Techniques that influence user behavior below the threshold of conscious awareness, particularly when leveraging real-time neural feedback.
- **Purpose Limitation**: Principle that data collected for one specified purpose cannot be repurposed for unrelated uses without new, specific consent.
- **Granular Consent**: Consent mechanism allowing users to approve/reject individual data uses rather than accepting a blanket policy.
