# Prompt: milestone-0-chief-architect-ai-2026-02-15-3

## Input
Based on the output of milestone-0.md create a more concise version that only references meaningful changes. Make this optimized for LLM Tokens.

## Output

# Dental PMS — M0 Architecture (Consolidated)

## Tech Stack
Rust, Axum, Tokio, SQLx (compile-time checked), PostgreSQL 16, Leptos (WASM), Argon2id+JWT auth, S3-compatible storage behind `StorageBackend` trait, GitHub Actions CI/CD, Docker Compose.

## Workspace Layout
5 crates: `dental-server` (Axum routes+middleware), `dental-core` (domain models, zero framework deps), `dental-db` (all SQL, one module per PG schema), `dental-frontend` (Leptos WASM), `dental-common` (shared DTOs+validation).

## Postgres Schema-Per-Role Design

7 schemas, each owned by a NOLOGIN Postgres role. App connects as `dental_app` (LOGIN, member of all roles) and issues `SET LOCAL ROLE` per request based on JWT claims. Defense in depth: app-layer RBAC + DB-layer GRANT/REVOKE.

| Schema | DB Role | App Role | Tables |
|---|---|---|---|
| `auth` | `dental_auth` | Admin | users, sessions, providers |
| `front_office` | `dental_front_office` | Receptionist | patients, appointments, operatories, documents, appointment_types |
| `clinical` | `dental_clinical` | Hygienist | medical_histories, tooth_conditions, perio_exams, perio_measurements, progress_notes |
| `treatment` | `dental_treatment` | Dentist | treatment_plans, treatment_plan_procedures |
| `billing` | `dental_billing` | Receptionist (billing routes) | insurance_policies, ledger_entries |
| `shared` | `dental_auth` | All (read-only) | cdt_codes |
| `audit` | `dental_audit` | All INSERT, Admin SELECT | audit_log |

### Cross-Schema Access Matrix

| DB Role → Schema | auth | front_office | clinical | treatment | billing | shared | audit |
|---|---|---|---|---|---|---|---|
| dental_auth | **CRUD** | SELECT | SELECT | SELECT | SELECT | **CRUD** | SELECT |
| dental_front_office | SELECT | **CRUD** | — | — | SELECT | SELECT | INSERT |
| dental_clinical | SELECT | SELECT | **CRUD** | SELECT | — | SELECT | INSERT |
| dental_treatment | SELECT | SELECT | SELECT | **CRUD** | — | SELECT | INSERT |
| dental_billing | SELECT | SELECT | — | SELECT | **CRUD** | SELECT | INSERT |

**CRUD**=owner. SELECT=read-only grant. —=no access. INSERT=audit append only.

### Receptionist Dual-Role
Route-aware middleware: `/api/billing` and `/api/insurance` routes use `dental_billing`; all other receptionist routes use `dental_front_office`. Never writes both schemas in one connection.

## Connection Strategy

```rust
// Single pool, SET LOCAL ROLE scoped to transaction, auto-resets on pool return
pub fn db_role_for_route(app_role: Role, route: &str) -> &'static str {
    match app_role {
        Role::Receptionist if route.starts_with("/api/billing")
                           || route.starts_with("/api/insurance") => "dental_billing",
        Role::Receptionist => "dental_front_office",
        Role::Admin        => "dental_auth",
        Role::Hygienist    => "dental_clinical",
        Role::Dentist      => "dental_treatment",
    }
}
```

## Middleware Stack
`Auth (JWT+Role)` → `DB Role (SET LOCAL ROLE)` → `Audit (PHI access logging)` → Route handlers.

## RBAC (Application Layer)

```rust
pub enum Role { Receptionist, Hygienist, Dentist, Admin }
pub enum Permission {
    PatientRead, PatientWrite, MedicalHistoryRead, MedicalHistoryWrite,
    ScheduleRead, ScheduleWrite, ChartRead, ChartWrite,
    PerioRead, PerioWrite, NotesRead, NotesWrite,
    TreatmentPlanRead, TreatmentPlanWrite, BillingRead, BillingWrite,
    InsuranceRead, InsuranceWrite, DocumentRead, DocumentWrite,
    AuditRead, UserManage,
}
// Single match expression; Admin matches all.
```

## Schema SQL (All Tables)

All tables use UUIDs, TIMESTAMPTZ, created_at/updated_at/created_by/updated_by. Clinical tables add `version INT` for optimistic concurrency.

### 000_bootstrap_schemas.sql
```sql
CREATE ROLE dental_auth NOLOGIN;
CREATE ROLE dental_front_office NOLOGIN;
CREATE ROLE dental_clinical NOLOGIN;
CREATE ROLE dental_treatment NOLOGIN;
CREATE ROLE dental_billing NOLOGIN;
CREATE ROLE dental_audit NOLOGIN;
CREATE ROLE dental_app LOGIN PASSWORD 'changeme_in_env';
GRANT dental_auth, dental_front_office, dental_clinical,
      dental_treatment, dental_billing, dental_audit TO dental_app;

CREATE SCHEMA auth AUTHORIZATION dental_auth;
CREATE SCHEMA front_office AUTHORIZATION dental_front_office;
CREATE SCHEMA clinical AUTHORIZATION dental_clinical;
CREATE SCHEMA treatment AUTHORIZATION dental_treatment;
CREATE SCHEMA billing AUTHORIZATION dental_billing;
CREATE SCHEMA shared AUTHORIZATION dental_auth;
CREATE SCHEMA audit AUTHORIZATION dental_audit;
REVOKE ALL ON SCHEMA public FROM PUBLIC;
```

### 001_auth_tables.sql
```sql
SET ROLE dental_auth;
CREATE TYPE auth.user_role AS ENUM ('receptionist','hygienist','dentist','admin');
CREATE TABLE auth.users (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    username VARCHAR(100) UNIQUE NOT NULL, password_hash VARCHAR(255) NOT NULL,
    role auth.user_role NOT NULL, is_active BOOLEAN NOT NULL DEFAULT true,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(), updated_at TIMESTAMPTZ NOT NULL DEFAULT now());
CREATE TABLE auth.sessions (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES auth.users(id),
    refresh_token VARCHAR(500) NOT NULL, expires_at TIMESTAMPTZ NOT NULL,
    revoked BOOLEAN NOT NULL DEFAULT false, created_at TIMESTAMPTZ NOT NULL DEFAULT now());
CREATE TABLE auth.providers (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES auth.users(id),
    npi VARCHAR(10), license_no VARCHAR(50), specialty VARCHAR(100),
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(), updated_at TIMESTAMPTZ NOT NULL DEFAULT now());
```

### 002_shared_tables.sql
```sql
SET ROLE dental_auth;
CREATE TABLE shared.cdt_codes (
    code VARCHAR(10) PRIMARY KEY, category VARCHAR(100) NOT NULL,
    description TEXT NOT NULL, is_active BOOLEAN NOT NULL DEFAULT true);
```

### 003_front_office_tables.sql
```sql
SET ROLE dental_front_office;
CREATE TABLE front_office.operatories (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name VARCHAR(50) NOT NULL, is_active BOOLEAN NOT NULL DEFAULT true,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now());
CREATE TABLE front_office.patients (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    first_name VARCHAR(100) NOT NULL, last_name VARCHAR(100) NOT NULL,
    date_of_birth DATE NOT NULL, phone VARCHAR(20), email VARCHAR(255),
    address_line1 VARCHAR(255), address_line2 VARCHAR(255),
    city VARCHAR(100), state VARCHAR(2), zip VARCHAR(10),
    emergency_contact JSONB, guarantor_id UUID REFERENCES front_office.patients(id),
    is_active BOOLEAN NOT NULL DEFAULT true,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(), updated_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    created_by UUID NOT NULL REFERENCES auth.users(id),
    updated_by UUID NOT NULL REFERENCES auth.users(id));
CREATE TABLE front_office.appointment_types (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name VARCHAR(100) NOT NULL, default_duration INT NOT NULL DEFAULT 30,
    color VARCHAR(7), is_active BOOLEAN NOT NULL DEFAULT true);
CREATE TABLE front_office.appointments (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    patient_id UUID NOT NULL REFERENCES front_office.patients(id),
    provider_id UUID NOT NULL REFERENCES auth.providers(id),
    operatory_id UUID REFERENCES front_office.operatories(id),
    appointment_type_id UUID REFERENCES front_office.appointment_types(id),
    starts_at TIMESTAMPTZ NOT NULL, ends_at TIMESTAMPTZ NOT NULL,
    status VARCHAR(20) NOT NULL DEFAULT 'scheduled', notes TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(), updated_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    created_by UUID NOT NULL REFERENCES auth.users(id),
    updated_by UUID NOT NULL REFERENCES auth.users(id));
CREATE TABLE front_office.documents (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    patient_id UUID NOT NULL REFERENCES front_office.patients(id),
    doc_type VARCHAR(50) NOT NULL, file_name VARCHAR(255) NOT NULL,
    storage_key VARCHAR(500) NOT NULL, mime_type VARCHAR(100), file_size BIGINT,
    notes TEXT, uploaded_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    uploaded_by UUID NOT NULL REFERENCES auth.users(id));
```

### 004_clinical_tables.sql
```sql
SET ROLE dental_clinical;
CREATE TABLE clinical.medical_histories (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    patient_id UUID NOT NULL REFERENCES front_office.patients(id),
    medications JSONB NOT NULL DEFAULT '[]', allergies JSONB NOT NULL DEFAULT '[]',
    conditions JSONB NOT NULL DEFAULT '[]', version INT NOT NULL DEFAULT 1,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(), updated_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_by UUID NOT NULL REFERENCES auth.users(id));
CREATE TABLE clinical.tooth_conditions (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    patient_id UUID NOT NULL REFERENCES front_office.patients(id),
    tooth_num SMALLINT NOT NULL, surface VARCHAR(10),
    condition VARCHAR(50) NOT NULL, material VARCHAR(50), notes TEXT,
    recorded_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    recorded_by UUID NOT NULL REFERENCES auth.users(id), version INT NOT NULL DEFAULT 1);
CREATE TABLE clinical.perio_exams (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    patient_id UUID NOT NULL REFERENCES front_office.patients(id),
    exam_date DATE NOT NULL, recorded_by UUID NOT NULL REFERENCES auth.users(id),
    created_at TIMESTAMPTZ NOT NULL DEFAULT now());
CREATE TABLE clinical.perio_measurements (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    exam_id UUID NOT NULL REFERENCES clinical.perio_exams(id),
    tooth_num SMALLINT NOT NULL, site VARCHAR(5) NOT NULL,
    pocket_depth SMALLINT, recession SMALLINT,
    bleeding BOOLEAN DEFAULT false, suppuration BOOLEAN DEFAULT false);
CREATE TABLE clinical.progress_notes (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    patient_id UUID NOT NULL REFERENCES front_office.patients(id),
    visit_date DATE NOT NULL, author_id UUID NOT NULL REFERENCES auth.users(id),
    content TEXT NOT NULL, template_id UUID, version INT NOT NULL DEFAULT 1,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(), updated_at TIMESTAMPTZ NOT NULL DEFAULT now());
```

### 005_treatment_tables.sql
```sql
SET ROLE dental_treatment;
CREATE TABLE treatment.treatment_plans (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    patient_id UUID NOT NULL REFERENCES front_office.patients(id),
    created_by UUID NOT NULL REFERENCES auth.users(id),
    status VARCHAR(20) NOT NULL DEFAULT 'proposed',
    consent_date DATE, notes TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(), updated_at TIMESTAMPTZ NOT NULL DEFAULT now());
CREATE TABLE treatment.treatment_plan_procedures (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    plan_id UUID NOT NULL REFERENCES treatment.treatment_plans(id),
    cdt_code VARCHAR(10) NOT NULL REFERENCES shared.cdt_codes(code),
    tooth_num SMALLINT, surface VARCHAR(10), sequence_order INT NOT NULL,
    status VARCHAR(20) NOT NULL DEFAULT 'planned', fee DECIMAL(10,2),
    notes TEXT, completed_at TIMESTAMPTZ, completed_by UUID REFERENCES auth.users(id));
```

### 006_billing_tables.sql
```sql
SET ROLE dental_billing;
CREATE TABLE billing.insurance_policies (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    patient_id UUID NOT NULL REFERENCES front_office.patients(id),
    carrier_name VARCHAR(200) NOT NULL, group_number VARCHAR(50),
    subscriber_id VARCHAR(50) NOT NULL, subscriber_name VARCHAR(200),
    relationship VARCHAR(20), is_primary BOOLEAN NOT NULL DEFAULT true,
    effective_date DATE, expiry_date DATE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(), updated_at TIMESTAMPTZ NOT NULL DEFAULT now());
CREATE TABLE billing.ledger_entries (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    patient_id UUID NOT NULL REFERENCES front_office.patients(id),
    entry_type VARCHAR(20) NOT NULL, amount DECIMAL(10,2) NOT NULL,
    cdt_code VARCHAR(10) REFERENCES shared.cdt_codes(code),
    procedure_id UUID, payment_method VARCHAR(20), reference_no VARCHAR(100),
    notes TEXT, entry_date DATE NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    created_by UUID NOT NULL REFERENCES auth.users(id));
```

### 007_audit_tables.sql
```sql
SET ROLE dental_audit;
CREATE TABLE audit.audit_log (
    id BIGSERIAL PRIMARY KEY, user_id UUID, db_role VARCHAR(50) NOT NULL,
    action VARCHAR(50) NOT NULL, schema_name VARCHAR(50) NOT NULL,
    entity_type VARCHAR(50) NOT NULL, entity_id UUID,
    old_value JSONB, new_value JSONB, ip_address INET,
    occurred_at TIMESTAMPTZ NOT NULL DEFAULT now());
CREATE INDEX idx_audit_log_entity ON audit.audit_log(entity_type, entity_id);
CREATE INDEX idx_audit_log_user ON audit.audit_log(user_id, occurred_at);
CREATE INDEX idx_audit_log_schema ON audit.audit_log(schema_name, occurred_at);
GRANT USAGE ON SCHEMA audit TO dental_front_office, dental_clinical, dental_treatment, dental_billing;
GRANT INSERT ON audit.audit_log TO dental_front_office, dental_clinical, dental_treatment, dental_billing;
GRANT USAGE, SELECT ON SEQUENCE audit.audit_log_id_seq TO dental_front_office, dental_clinical, dental_treatment, dental_billing;
GRANT USAGE ON SCHEMA audit TO dental_auth;
GRANT SELECT ON audit.audit_log TO dental_auth;
```

### 008_cross_schema_grants.sql
```sql
-- auth: all domain roles read users/providers
GRANT USAGE ON SCHEMA auth TO dental_front_office, dental_clinical, dental_treatment, dental_billing;
GRANT SELECT ON auth.users, auth.providers TO dental_front_office, dental_clinical, dental_treatment, dental_billing;
-- shared: all read cdt_codes
GRANT USAGE ON SCHEMA shared TO dental_front_office, dental_clinical, dental_treatment, dental_billing;
GRANT SELECT ON ALL TABLES IN SCHEMA shared TO dental_front_office, dental_clinical, dental_treatment, dental_billing;
-- front_office: clinical, treatment, billing read patients+appointments
GRANT USAGE ON SCHEMA front_office TO dental_clinical, dental_treatment, dental_billing;
GRANT SELECT ON front_office.patients TO dental_clinical, dental_treatment, dental_billing;
GRANT SELECT ON front_office.appointments TO dental_clinical, dental_treatment, dental_billing;
GRANT SELECT ON front_office.documents TO dental_clinical, dental_treatment;
-- clinical: dentist reads all for treatment planning
GRANT USAGE ON SCHEMA clinical TO dental_treatment;
GRANT SELECT ON ALL TABLES IN SCHEMA clinical TO dental_treatment;
-- treatment: billing reads for charges, clinical reads for context
GRANT USAGE ON SCHEMA treatment TO dental_billing, dental_clinical;
GRANT SELECT ON treatment.treatment_plans, treatment.treatment_plan_procedures TO dental_billing, dental_clinical;
-- billing: front_office reads for balance display
GRANT USAGE ON SCHEMA billing TO dental_front_office;
GRANT SELECT ON ALL TABLES IN SCHEMA billing TO dental_front_office;
```

## HIPAA Infrastructure
- TLS termination at reverse proxy; pgcrypto + full-disk encryption at rest; S3 SSE for files
- Append-only audit.audit_log (no UPDATE/DELETE grants); DB triggers + middleware logging
- JWT 15min expiry, refresh 8hr, idle timeout 10min, Argon2id passwords (12+ chars)

## Offline Architecture
Frontend writes through command queue (dental-common `Command` trait) → IndexedDB → sync engine. M0 defines types; M5 delivers sync UI. All writes use same path online/offline.

## Microservice Extraction Path
Each schema module = future service boundary. To extract: `ALTER ROLE dental_X LOGIN`, pull schema/*.rs + routes + models into standalone service, replace cross-schema SELECTs with API calls. Cross-schema grants document exactly which inter-service calls are needed.

## Architecture Diagram

```mermaid
graph TB
    subgraph "Client — Leptos WASM"
        UI[UI Layer] --> CQ[Command Queue] --> IDB[(IndexedDB)] --> SYNC[Sync Engine]
    end
    SYNC -- "HTTPS" --> LB[Reverse Proxy/TLS]
    LB --> AXUM[Axum Router]
    subgraph "Middleware"
        AXUM --> MW_AUTH[Auth JWT+Role] --> MW_DBROLE[SET LOCAL ROLE] --> MW_AUDIT[Audit PHI Log]
    end
    subgraph "Routes"
        MW_AUDIT --> R_AUTH[/auth] & R_FO[/patients /appointments] & R_CLIN[/charting] & R_TX[/treatment] & R_BILL[/billing] & R_IMG[/imaging]
    end
    subgraph "dental-core"
        DC[Domain Models — zero deps]
    end
    R_AUTH & R_FO & R_CLIN & R_TX & R_BILL & R_IMG --> DC
    subgraph "dental-db schema modules"
        S_A[auth.rs] & S_FO[front_office.rs] & S_C[clinical.rs] & S_T[treatment.rs] & S_B[billing.rs] & S_SH[shared.rs] & S_AU[audit.rs]
    end
    DC --> S_A & S_FO & S_C & S_T & S_B & S_SH & S_AU
    subgraph "PostgreSQL 16"
        PG_A[(auth)] & PG_SH[(shared)] & PG_FO[(front_office)] & PG_CL[(clinical)] & PG_TX[(treatment)] & PG_BI[(billing)] & PG_AU[(audit)]
    end
    S_A --> PG_A
    S_SH --> PG_SH
    S_FO --> PG_FO
    S_C --> PG_CL
    S_T --> PG_TX
    S_B --> PG_BI
    S_AU --> PG_AU
    PG_A -. SELECT .-> PG_FO & PG_CL & PG_TX & PG_BI
    PG_FO -. "SELECT patients" .-> PG_CL & PG_TX & PG_BI
    PG_CL -. SELECT .-> PG_TX
    PG_TX -. SELECT .-> PG_BI & PG_CL
    PG_BI -. SELECT .-> PG_FO
    PG_FO & PG_CL & PG_TX & PG_BI -. INSERT .-> PG_AU
    PG_SH -. SELECT .-> PG_FO & PG_CL & PG_TX & PG_BI
    S_FO --> S3[(S3 Object Storage)]
    style AXUM fill:#D94A4A,color:#fff
    style PG_A fill:#336791,color:#fff
    style PG_SH fill:#336791,color:#fff
    style PG_FO fill:#336791,color:#fff
    style PG_CL fill:#336791,color:#fff
    style PG_TX fill:#336791,color:#fff
    style PG_BI fill:#336791,color:#fff
    style PG_AU fill:#336791,color:#fff
```

## Key Risks
| Risk | Mitigation |
|---|---|
| Leptos immaturity | Isolated in own crate; swappable for JS frontend against same API |
| CDT code licensing | Seed codes only, add descriptions if licensed |
| SET ROLE overhead | Negligible (metadata op); benchmark in M0 |
| Cross-schema FK ordering | Bootstrap creates all schemas first; tables follow dependency order |
| SQLx + qualified names | `cargo sqlx prepare` runs as `dental_app` with all role memberships |
| Receptionist dual-role routing | Documented in middleware; consider splitting to own app role if complex |
