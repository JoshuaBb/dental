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
