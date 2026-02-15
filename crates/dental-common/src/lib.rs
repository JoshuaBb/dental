use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Role {
    Receptionist,
    Hygienist,
    Dentist,
    Admin,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Permission {
    PatientRead,
    PatientWrite,
    MedicalHistoryRead,
    MedicalHistoryWrite,
    ScheduleRead,
    ScheduleWrite,
    ChartRead,
    ChartWrite,
    PerioRead,
    PerioWrite,
    NotesRead,
    NotesWrite,
    TreatmentPlanRead,
    TreatmentPlanWrite,
    BillingRead,
    BillingWrite,
    InsuranceRead,
    InsuranceWrite,
    DocumentRead,
    DocumentWrite,
    AuditRead,
    UserManage,
}

impl Role {
    pub fn has_permission(self, perm: Permission) -> bool {
        if self == Role::Admin {
            return true;
        }
        matches!(
            (self, perm),
            (Role::Receptionist, Permission::PatientRead)
                | (Role::Receptionist, Permission::PatientWrite)
                | (Role::Receptionist, Permission::ScheduleRead)
                | (Role::Receptionist, Permission::ScheduleWrite)
                | (Role::Receptionist, Permission::DocumentRead)
                | (Role::Receptionist, Permission::DocumentWrite)
                | (Role::Receptionist, Permission::BillingRead)
                | (Role::Receptionist, Permission::BillingWrite)
                | (Role::Receptionist, Permission::InsuranceRead)
                | (Role::Receptionist, Permission::InsuranceWrite)
                | (Role::Hygienist, Permission::PatientRead)
                | (Role::Hygienist, Permission::MedicalHistoryRead)
                | (Role::Hygienist, Permission::MedicalHistoryWrite)
                | (Role::Hygienist, Permission::ChartRead)
                | (Role::Hygienist, Permission::ChartWrite)
                | (Role::Hygienist, Permission::PerioRead)
                | (Role::Hygienist, Permission::PerioWrite)
                | (Role::Hygienist, Permission::NotesRead)
                | (Role::Hygienist, Permission::NotesWrite)
                | (Role::Hygienist, Permission::ScheduleRead)
                | (Role::Dentist, Permission::PatientRead)
                | (Role::Dentist, Permission::MedicalHistoryRead)
                | (Role::Dentist, Permission::ChartRead)
                | (Role::Dentist, Permission::ChartWrite)
                | (Role::Dentist, Permission::PerioRead)
                | (Role::Dentist, Permission::NotesRead)
                | (Role::Dentist, Permission::NotesWrite)
                | (Role::Dentist, Permission::TreatmentPlanRead)
                | (Role::Dentist, Permission::TreatmentPlanWrite)
                | (Role::Dentist, Permission::ScheduleRead)
        )
    }
}

/// Maps an application role + route path to the appropriate Postgres NOLOGIN role
/// for `SET LOCAL ROLE` within a transaction.
pub fn db_role_for_route(app_role: Role, route: &str) -> &'static str {
    match app_role {
        Role::Receptionist
            if route.starts_with("/api/billing") || route.starts_with("/api/insurance") =>
        {
            "dental_billing"
        }
        Role::Receptionist => "dental_front_office",
        Role::Admin => "dental_auth",
        Role::Hygienist => "dental_clinical",
        Role::Dentist => "dental_treatment",
    }
}

#[derive(Debug, thiserror::Error)]
pub enum AppError {
    #[error("unauthorized")]
    Unauthorized,
    #[error("forbidden")]
    Forbidden,
    #[error("not found")]
    NotFound,
    #[error("internal error: {0}")]
    Internal(String),
}
