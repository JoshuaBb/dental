SET ROLE dental_auth;
CREATE TABLE shared.cdt_codes (
    code VARCHAR(10) PRIMARY KEY, category VARCHAR(100) NOT NULL,
    description TEXT NOT NULL, is_active BOOLEAN NOT NULL DEFAULT true);
