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
