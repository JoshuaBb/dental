# Prompt: main-ai-2026-02-15-4

## Input
Using the results in main-ai-2026-02-15-1.md, main-ai-2026-02-15-2.md, and main-ai-2026-02-15-3.md I am starting a business. What would be good requirements for MVP

## Output

Based on the inputs across all three roles (hygienist, dentist, receptionist), here are MVP requirements for a dental practice management application:

### Core MVP Requirements

#### 1. Patient Management
- Create, view, edit patient records (name, DOB, contact info, emergency contact)
- Medical history tracking (medications, allergies, conditions)
- Patient search and lookup
- Document/ID scanning and attachment

#### 2. Scheduling
- Appointment calendar with daily/weekly views
- Book, reschedule, and cancel appointments
- Assign provider and operatory
- Appointment types with default durations
- Patient check-in / check-out workflow
- Basic recall/recare tracking

#### 3. Clinical Charting
- Tooth chart (adult and primary dentition)
- Mark existing conditions (caries, missing teeth, restorations)
- Periodontal charting (pocket depths, bleeding on probing)
- Soft/hard tissue notes

#### 4. Treatment Planning
- Create treatment plans with sequenced procedures
- Associate ADA CDT codes (D-codes) with procedures
- Track treatment status (planned, in-progress, completed)
- Record patient acceptance/consent

#### 5. Progress Notes
- Per-visit clinical notes (free text or templated)
- Associate notes with procedures performed
- Notes accessible by all providers on the care team

#### 6. Billing & Insurance (Basic)
- Enter insurance carrier and subscriber info
- Attach CDT codes to visits for billing
- Record payments (cash, card, check)
- Generate simple patient invoices/statements
- Track outstanding balances

#### 7. Imaging Integration (Minimal)
- Attach and view radiograph images per patient
- Basic annotation or notes on images

### What to Defer Post-MVP

| Feature | Why Defer |
|---|---|
| E-prescribing | Requires DEA/EPCS compliance, third-party integration |
| Insurance claim submission (EDI) | Complex clearinghouse integrations |
| Automated reminders (SMS/email) | Third-party messaging APIs |
| Lab case management | Secondary workflow |
| Detailed reporting/analytics | Nice-to-have, not day-one |
| Multi-location support | Adds complexity early |
| Patient portal | Separate frontend effort |
| CBCT/3D imaging | Specialized hardware integration |

### Cross-Cutting Concerns for MVP

- **Role-based access**: Receptionist, hygienist, and dentist see different views/permissions
- **HIPAA compliance**: Encryption at rest and in transit, audit logging, access controls
- **Data integrity**: Dental records are legal documents — no silent deletions, maintain edit history
- **Desktop-first UI**: Optimize for desktop (keyboard-heavy workflows, multi-panel layouts)
- **Offline resilience**: Dental offices can't stop working if the internet drops briefly

### MVP Scope Summary

Build a system where a **receptionist** can register patients and schedule appointments, a **hygienist** can chart perio and document cleanings, and a **dentist** can diagnose, treatment plan, and document procedures — all tied together with basic billing. That covers the daily workflow end-to-end without drowning in integrations.
