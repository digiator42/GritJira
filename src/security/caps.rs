use gritshield::declare_security_caps;

// ============================================================
// System Role Tokens
// ============================================================
pub struct Admin;
pub struct Manager;
pub struct Developer;
pub struct Tester;
pub struct Viewer;

// ============================================================
// Security Capability Tokens
// ============================================================
pub struct IssueEdit;
pub struct IssueCreate;
pub struct IssueDelete;
pub struct ProjectAdmin;
pub struct ViewBoard;

// ============================================================
// Single Source of Truth Capability Registry
// ============================================================
declare_security_caps! {
    IssueEdit    => [Admin, Manager, Developer],
    IssueCreate  => [Admin, Manager, Developer, Tester],
    IssueDelete  => [Admin, Manager],
    ProjectAdmin => [Admin],
    ViewBoard    => [Admin, Manager, Developer, Tester, Viewer],
}