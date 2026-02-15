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
