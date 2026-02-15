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
