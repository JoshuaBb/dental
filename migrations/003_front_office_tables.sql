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
