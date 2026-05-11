# Neurorights Product Evaluation Guide for Advocates

**Document ID**: NR-AE-001  
**Version**: 1.0.0  
**Effective Date**: 2026-05-11  
**Audience**: NGOs, journalists, community watchdogs, digital rights researchers  

## Purpose

This guide empowers civil society actors to systematically evaluate neurotechnology products and platform practices against neurorights standards. It provides a reproducible methodology for identifying unlawful or unethical neural-data collection, generating evidence-based critiques, and coordinating advocacy actions.

## Evaluation Framework

### Step 1: Product Identification & Scope Definition
- [ ] Identify the product/platform under review:
  - Name, developer, version, release date
  - Primary function and target user base
  - Jurisdictions where available
- [ ] Define evaluation scope:
  - Specific features involving neural data
  - User contexts of concern (home, VR, workplace, etc.)
  - Timeframe for data collection practices

### Step 2: Technical Reconnaissance
Gather publicly available information about the product's neural-data practices:

#### Documentation Review
- [ ] Privacy policy: Search for "neural," "brain," "EEG," "BCI," "cognitive," "emotion"
- [ ] Terms of service: Identify data ownership, sharing, and retention clauses
- [ ] Developer documentation: API specs, SDK guides, data schema definitions
- [ ] Marketing materials: Claims about data usage, AI training, or personalization

#### Technical Analysis (if accessible)
- [ ] Network traffic inspection: Identify data endpoints and payload structures
- [ ] App binary analysis: Search for neural-data processing libraries or keywords
- [ ] Consent flow testing: Document UI/UX of permission requests
- [ ] Data export request: Submit subject access request to obtain held data

#### Community Intelligence
- [ ] User forums and reviews: Search for privacy concerns or unexpected behaviors
- [ ] Academic literature: Check for independent security/privacy analyses
- [ ] Regulatory filings: Review FTC complaints, state AG actions, or EU DSA notifications

### Step 3: Scenario Classification
Use the framework's classifier logic to structure findings:

```yaml
scenario_description: |
  [Concise summary of the practice under review]

detected_context: home_private_use | virtual_platform_tracking | workplace_monitoring | other
detected_technology: eeg_headset | bci_implant | vr_neuro_wearable | ar_attention_tracker | unknown
data_types_observed:
  - raw_neural_signals
  - inferred_emotional_state
  - cognitive_load_metrics
  - intention_predictions
data_uses_identified:
  - primary_function
  - advertising
  - ai_training
  - third_party_sharing
jurisdiction_relevance:
  - US-CO  # If product available in Colorado
  - EU     # If product available in European Economic Area
```

### Step 4: Policy Mapping & Violation Assessment
Cross-reference observations against the policy database:

#### For Each Observed Practice:
1. **Identify applicable rules**:
   - Query `neural_policies` table for matching jurisdiction and topic
   - Prioritize Tier 1 (binding law) over Tier 2 (guidance)
   
2. **Assess compliance**:
   ```markdown
   Practice: [Description]
   Applicable Rules:
     - [Jurisdiction] [Law]: [Rule summary]
   Compliance Status:
     - [ ] Fully compliant
     - [ ] Partially compliant (gaps noted below)
     - [ ] Non-compliant (violations detailed below)
   Gap Analysis:
     - Missing explicit consent mechanism for [specific use]
     - Data repurposed for [unrelated purpose] without re-consent
     - No opt-out available for [high-risk inference]
   ```

3. **Risk characterization**:
   - Severity: Critical/High/Medium/Low based on harm potential
   - Scale: Individual/Group/Systemic impact
   - Reversibility: Can harm be mitigated post-collection?

### Step 5: Evidence Compilation
Document findings with verifiable evidence:

#### Required Evidence Types
- **Direct evidence**: Screenshots, network logs, code snippets, policy excerpts
- **Corroborating evidence**: User testimonials, expert analyses, regulatory precedents
- **Contextual evidence**: Market practices, technological feasibility, user expectations

#### Evidence Organization
```
evidence/
├── product_name/
│   ├── documentation/
│   │   ├── privacy_policy_vX.Y.pdf
│   │   ├── terms_of_service_vX.Y.pdf
│   │   └── marketing_claims_screenshots/
│   ├── technical/
│   │   ├── network_traffic_capture.pcap
│   │   ├── consent_flow_video.mp4
│   │   └── data_export_response.json
│   ├── community/
│   │   ├── user_forum_threads/
│   │   ├── academic_papers/
│   │   └── regulatory_filings/
│   └── analysis/
│       ├── policy_mapping_matrix.csv
│       ├── violation_assessment.md
│       └── risk_characterization.yaml
```

### Step 6: Output Generation
Produce one or more of the following deliverables:

#### A. Public-Facing Briefing Document
```markdown
# Neurorights Alert: [Product Name]

## Executive Summary
[2-3 sentence overview of concern and recommended actions]

## Key Findings
1. [Finding 1 with evidence reference]
2. [Finding 2 with evidence reference]
3. [Finding 3 with evidence reference]

## Legal & Ethical Concerns
- Violates [specific law] by [description]
- Contravenes [human rights principle] through [mechanism]
- Creates risk of [harm type] for [affected group]

## Recommended Actions
- For users: [Practical steps to protect themselves]
- For regulators: [Specific enforcement requests]
- For the developer: [Remediation requirements]

## Evidence Appendix
[Links to full evidence repository]
```

#### B. Regulatory Complaint Draft
- Use template from `docs/regulatory-templates/`
- Populate with scenario-specific facts and policy mappings
- Attach evidence index with verification instructions

#### C. Media Pitch Package
- Press release draft with quotable findings
- Visual assets: infographics, short video explainers
- Expert contact list for interviews and commentary

### Step 7: Advocacy Coordination
Maximize impact through strategic collaboration:

#### Stakeholder Mapping
- [ ] Identify aligned organizations (digital rights, disability justice, labor rights)
- [ ] Map regulatory touchpoints (FTC, state AGs, EU DPCs, UN special procedures)
- [ ] Engage affected communities for co-development of demands

#### Action Planning
- [ ] Set clear, measurable objectives (e.g., "Secure consent mechanism update within 90 days")
- [ ] Define escalation path: public comment → formal complaint → litigation support
- [ ] Establish communication protocol: who speaks, when, and through which channels

#### Impact Measurement
- Track outcomes: policy changes, product modifications, regulatory actions
- Document lessons learned for future evaluations
- Update evaluation guide based on new tactics or legal developments

## Ethical Guidelines for Advocates

### Do No Harm Principles
- **Minimize exposure**: Redact personal data of users in evidence; avoid doxxing
- **Verify before publishing**: Corroborate claims with multiple sources; allow right of reply
- **Center affected voices**: Prioritize input from users with lived experience of harm
- **Avoid surveillance replication**: Do not use invasive techniques to gather evidence

### Transparency & Accountability
- Disclose funding sources and potential conflicts of interest
- Publish methodology and evidence repository (with appropriate redactions)
- Establish correction protocol for errors or new information
- Submit to peer review by technical and legal experts before major releases

### Strategic Patience
- Recognize that regulatory change is often incremental
- Balance urgency with thoroughness: rushed claims can undermine credibility
- Build long-term capacity rather than chasing short-term wins

## Appendix A: Quick-Reference Policy Cheat Sheet

| Context | High-Risk Practice | Key Legal Hook | Advocacy Angle |
|---------|-------------------|----------------|----------------|
| Home Private Use | Covert neural data collection | CO/CA sensitive data rules | "Your thoughts are not a product" |
| Virtual Platforms | Emotion-based micro-targeting | EU AI Act Art. 5(1)(a) | "No brain-hacking in the metaverse" |
| Workplace | Cognitive performance monitoring | State mental-privacy bills | "Your mind is not a KPI" |
| Children | Behavioral profiling via neural data | COPPA + GDPR-K | "Protect kids' cognitive development" |

## Appendix B: Tooling Setup Checklist

To use the `neurorights-defense` framework for evaluations:

```bash
# 1. Clone and initialize repository
git clone https://github.com/your-org/neurorights-defense.git
cd neurorights-defense

# 2. Install dependencies
cargo build --release  # For Rust compliance engine
# OR use pre-built Docker image if available

# 3. Initialize policy database
sqlite3 data/neural_policies.db < schemas/neural_policies.sql
sqlite3 data/neural_policies.db < schemas/compliance_check_items.sql

# 4. Configure for advocacy use
cp config/jurisdictions.yaml config/jurisdictions.local.yaml
# Edit to prioritize jurisdictions relevant to your investigation

# 5. Test classification with sample scenario
cargo run --example classify_scenario -- --input tests/sample_home_scenario.yaml
```

## Appendix C: Evidence Verification Protocol

Before publishing findings:

1. **Technical verification**:
   - Reproduce network captures on clean test device
   - Validate code snippets against public repository commits
   - Confirm consent flow behavior across app versions

2. **Legal verification**:
   - Cross-check rule interpretations with primary legal sources
   - Consult with privacy law experts on jurisdictional nuances
   - Verify effective dates and sunset provisions for cited laws

3. **Ethical verification**:
   - Review evidence for unnecessary exposure of personal data
   - Assess potential for misuse of findings (e.g., enabling surveillance)
   - Obtain informed consent from any users quoted or depicted

4. **Strategic verification**:
   - Evaluate timing relative to regulatory windows or legislative sessions
   - Coordinate with allied organizations to avoid message fragmentation
   - Prepare response protocols for potential legal threats or smear campaigns
