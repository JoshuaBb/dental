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
