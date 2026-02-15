# Prompt: cto-ai-2026-02-15-1

## Input
I am a product manager pairing with the CTO, using the requirements in mvp/mvp.md develop a milestone plan for deliverables for the MVP

## Output

# MVP Milestone Plan — Dental Practice Management System

## Milestone 0: Foundation (Weeks 1–2)
**Goal:** Project scaffolding, core data models, and dev environment

**Deliverables:**
- Tech stack decisions and project setup (Rust backend, database, frontend framework)
- Database schema design for core entities: patients, providers, operatories, appointments, procedures
- Authentication and role-based access control (receptionist, hygienist, dentist)
- HIPAA-aligned infrastructure: encryption at rest/in transit, audit log table
- CI/CD pipeline, dev/staging environments
- ADA CDT code reference table seeded

**Exit Criteria:** A developer can log in with a role, and the schema supports all MVP entities.

---

## Milestone 1: Patient Management + Scheduling (Weeks 3–5)
**Goal:** The receptionist can register patients and manage the schedule

**Deliverables:**
- Patient CRUD (name, DOB, contact, emergency contact, guarantor)
- Medical history form (medications, allergies, conditions)
- Patient search and lookup
- Document/ID attachment (file upload)
- Appointment calendar (daily + weekly views)
- Book, reschedule, cancel appointments
- Provider and operatory assignment
- Appointment types with default durations
- Check-in / check-out workflow

**Exit Criteria:** A receptionist can register a new patient, book them for a cleaning, check them in, and check them out.

---

## Milestone 2: Clinical Charting (Weeks 6–8)
**Goal:** The hygienist can chart perio and document findings

**Deliverables:**
- Interactive tooth chart (adult + primary dentition)
- Mark existing conditions (caries, missing, restorations)
- Periodontal charting (pocket depths, bleeding on probing, recession)
- Soft tissue and hard tissue notes
- Per-visit progress notes (free text + templates)
- Associate notes with procedures performed
- Notes visible across care team

**Exit Criteria:** A hygienist can open a patient's chart, record full perio charting, mark existing conditions, and write a progress note for a prophylaxis visit.

---

## Milestone 3: Treatment Planning + Diagnosis (Weeks 9–10)
**Goal:** The dentist can diagnose, plan treatment, and document procedures

**Deliverables:**
- Create treatment plans with sequenced procedures
- Associate ADA CDT codes (D-codes) with procedures
- Track treatment status (planned, in-progress, completed)
- Record patient acceptance/consent
- Dentist progress notes (procedure narratives)
- Tooth/surface-level procedure documentation

**Exit Criteria:** A dentist can review a patient's chart, create a treatment plan (e.g., crown on #14, filling on #19), get patient consent recorded, complete a procedure, and document it with the correct D-code.

---

## Milestone 4: Basic Billing & Insurance (Weeks 11–13)
**Goal:** The front office can track payments, insurance, and balances

**Deliverables:**
- Enter insurance carrier and subscriber information per patient
- Attach CDT codes from completed procedures to visits for billing
- Record payments (cash, card, check) against patient account
- Generate simple patient invoices/statements
- Track outstanding balances per patient
- Ledger view of charges, payments, and adjustments

**Exit Criteria:** After a patient visit, the receptionist can see charges generated from completed procedures, record a copay payment, print/export a statement, and verify the remaining balance.

---

## Milestone 5: Imaging & Polish (Weeks 14–15)
**Goal:** Attach radiographs, harden the UI, and prepare for pilot

**Deliverables:**
- Attach and view radiograph images per patient (file upload/viewer)
- Basic annotation or notes on images
- Recall/recare tracking (flag patients due for hygiene)
- Desktop-first UI polish (keyboard shortcuts, multi-panel layouts)
- Offline resilience (queue writes when connectivity drops, sync on reconnect)
- Edit history / audit trail for clinical records (no silent deletions)

**Exit Criteria:** A full patient visit flow works end-to-end — from scheduling through check-in, clinical charting, treatment, billing, and check-out — with images attached and an audit trail maintained.

---

## Milestone 6: Integration Testing & Pilot Prep (Week 16)
**Goal:** Harden for real-world use

**Deliverables:**
- End-to-end integration tests for the three role workflows
- Security review: access controls verified per role, audit log coverage
- HIPAA compliance checklist walkthrough
- Performance testing with realistic data volume
- User acceptance testing with a pilot dental office (or simulated)
- Bug fixes and UX refinements from testing

**Exit Criteria:** The system is ready for a controlled pilot deployment at a single practice.

---

## Summary Timeline

| Milestone | Weeks | What Ships |
|---|---|---|
| M0: Foundation | 1–2 | Auth, schema, infra |
| M1: Patients + Scheduling | 3–5 | Receptionist workflow |
| M2: Clinical Charting | 6–8 | Hygienist workflow |
| M3: Treatment Planning | 9–10 | Dentist workflow |
| M4: Billing & Insurance | 11–13 | Front office billing |
| M5: Imaging & Polish | 14–15 | Imaging, offline, UX |
| M6: Testing & Pilot | 16 | Production-ready pilot |

## Key Risks

| Risk | Mitigation |
|---|---|
| HIPAA compliance gaps | Engage compliance advisor early; encrypt everything by default; audit log from day 1 |
| Tooth chart UI complexity | Prototype interactive chart in M0 spike; consider existing open-source SVG charts |
| CDT code licensing | Verify ADA licensing requirements for embedding D-code descriptions |
| Offline resilience | Design write-ahead queue in M0; test sync conflicts in M5 |
| Scope creep into deferred features | Strict milestone exit criteria; anything not listed is post-MVP |

## Deferred (Post-MVP)
Per the requirements doc: e-prescribing, EDI claim submission, automated reminders, lab case management, reporting/analytics, multi-location, patient portal, CBCT/3D imaging.
