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
