use gritshield::GritSanitizer;
use gritshield::{http::response::HttpStatus, prelude::*};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;

use crate::repositories::issue::IssueRepository;
use crate::repositories::issue_type::IssueTypeRepository;
use crate::repositories::workflow::WorkflowRepository;
use crate::security::caps::{ProjectAdmin, ViewBoard};
use crate::services::project_service::ProjectService;

/// Jira-style default issue types (name, icon key, color) seeded per project.
const DEFAULT_ISSUE_TYPES: &[(&str, &str, &str)] = &[
    ("bug", "bug", "#eb5a46"),
    ("story", "story", "#65ba43"),
    ("task", "task", "#4bade9"),
    ("epic", "epic", "#a25dd8"),
    ("subtask", "subtask", "#8c9bab"),
    ("test", "test", "#ff8b45"),
    ("test execution", "test-execution", "#2daeb7"),
    ("test set", "test-set", "#f6b93b"),
    ("test plan", "test-plan", "#d9764f"),
    ("precondition", "precondition", "#a0a4b8"),
];

#[derive(Serialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub data: T,
}

#[derive(Deserialize, GritSanitizer)]
pub struct CreateProjectPayload {
    pub key: String,
    pub name: String,
    pub description: Option<String>,
}

#[derive(Deserialize, GritSanitizer)]
pub struct UpdateProjectPayload {
    pub name: Option<String>,
    pub description: Option<String>,
}

#[derive(Deserialize, GritSanitizer)]
pub struct CreateWorkflowStepPayload {
    #[serde(default)]
    pub name: Option<String>,
}

#[derive(Deserialize, GritSanitizer)]
pub struct UpdateWorkflowStepPayload {
    #[serde(default)]
    pub name: Option<String>,
}

#[derive(Deserialize, GritSanitizer)]
pub struct CreateIssueTypePayload {
    #[clean(trim, lowercase, html_escape)]
    pub name: String,

    #[clean(trim, lowercase)]
    #[serde(default)]
    pub icon_key: Option<String>,

    #[clean(trim, lowercase)]
    #[serde(default)]
    pub color: Option<String>,
}

#[derive(Deserialize, GritSanitizer)]
pub struct UpdateIssueTypePayload {
    #[clean(trim, lowercase, html_escape)]
    #[serde(default)]
    pub name: Option<String>,

    #[clean(trim, lowercase)]
    #[serde(default)]
    pub icon_key: Option<String>,

    #[clean(trim, lowercase)]
    #[serde(default)]
    pub color: Option<String>,
}

#[derive(Serialize)]
pub struct StatusBucket {
    pub step_id: i32,
    pub name: String,
    pub is_completed: bool,
    pub count: i32,
    pub points: i32,
}

#[derive(Serialize)]
pub struct TypeBucket {
    pub type_name: String,
    pub icon_key: String,
    pub color: String,
    pub count_open: i32,
    pub count_total: i32,
    pub percent: i32,
}

#[derive(Serialize)]
pub struct ProjectSummary {
    pub project_id: i32,
    pub total_issues: i32,
    pub open_issues: i32,
    pub done_issues: i32,
    pub total_points: i32,
    pub open_points: i32,
    pub by_status: Vec<StatusBucket>,
    pub by_type: Vec<TypeBucket>,
}

fn default_type_style(issue_type: &str) -> (String, String) {
    let key = issue_type.to_lowercase();
    match DEFAULT_ISSUE_TYPES
        .iter()
        .find(|(name, _, _)| *name == key)
    {
        Some((_, icon, color)) => (icon.to_string(), color.to_string()),
        None => ("task".to_string(), "#4bade9".to_string()),
    }
}

/// Seed the Jira-style default issue types when a project has none yet.
async fn ensure_default_issue_types(
    repo: &IssueTypeRepository,
    project_id: i32,
) -> Result<(), sea_orm::DbErr> {
    let existing = repo.find_by_project(project_id).await?;
    if !existing.is_empty() {
        return Ok(());
    }
    for (name, icon, color) in DEFAULT_ISSUE_TYPES {
        repo.create(project_id, name, icon, color).await?;
    }
    Ok(())
}

pub struct ProjectController;

#[controller("/api/v1/projects")]
impl ProjectController {
    /// GET /api/v1/projects - List the caller's projects
    #[get("")]
    #[cap(ViewBoard)]
    pub async fn list_projects(
        ctx: RequestContext,
        project_service: Arc<ProjectService>,
    ) -> Response {
        let user_id = ctx.get_session_data("user_id").and_then(|id| id.parse().ok());

        let result = match user_id {
            Some(uid) => project_service.list_projects_for_user(uid).await,
            None => project_service.list_projects().await,
        };

        match result {
            Ok(projects) => Response::json(
                HttpStatus::Ok,
                &ApiResponse {
                    success: true,
                    data: projects,
                },
            ),
            Err(e) => Response::internal_error(format!("Failed to fetch projects: {}", e)),
        }
    }

    /// GET /api/v1/projects/:id - Get project by ID
    #[get("/:id")]
    #[cap(ViewBoard)]
    pub async fn get_project(
        ctx: RequestContext,
        project_service: Arc<ProjectService>,
    ) -> Response {
        let project_id: i32 = match ctx.params.get("id").and_then(|p| p.parse().ok()) {
            Some(id) => id,
            None => return Response::bad_request("Invalid project ID"),
        };

        match project_service.get_project_by_id(project_id).await {
            Ok(Some(project)) => Response::json(
                HttpStatus::Ok,
                &ApiResponse {
                    success: true,
                    data: project,
                },
            ),
            Ok(None) => Response::not_found("Project not found"),
            Err(e) => Response::internal_error(format!("Failed to fetch project: {}", e)),
        }
    }

    /// POST /api/v1/projects - Create a new project
    #[post("")]
    #[cap(ProjectAdmin)]
    pub async fn create_project(
        ctx: RequestContext,
        project_service: Arc<ProjectService>,
    ) -> Response {
        let payload = match ctx.json::<CreateProjectPayload>().await {
            Ok(p) => p,
            Err(e) => return Response::bad_request(format!("Invalid request body: {:?}", e)),
        };

        // Validate key is not empty and alphanumeric
        if payload.key.is_empty()
            || !payload
                .key
                .chars()
                .all(|c| c.is_ascii_alphanumeric() || c == '-')
        {
            return Response::bad_request("Project key must be alphanumeric or contain hyphens");
        }

        match project_service
            .create_project(&payload.key, &payload.name, payload.description.as_deref())
            .await
        {
            Ok(project) => Response::json(
                HttpStatus::Created,
                &ApiResponse {
                    success: true,
                    data: project,
                },
            ),
            Err(e) => {
                if e.to_string().contains("duplicate") {
                    Response::bad_request("Project key already exists")
                } else {
                    Response::internal_error(format!("Failed to create project: {}", e))
                }
            }
        }
    }

    /// PATCH /api/v1/projects/:id - Update project
    #[patch("/:id")]
    #[cap(ProjectAdmin)]
    pub async fn update_project(
        ctx: RequestContext,
        project_service: Arc<ProjectService>,
    ) -> Response {
        let project_id: i32 = match ctx.params.get("id").and_then(|p| p.parse().ok()) {
            Some(id) => id,
            None => return Response::bad_request("Invalid project ID"),
        };

        let payload = match ctx.json::<UpdateProjectPayload>().await {
            Ok(p) => p,
            Err(e) => return Response::bad_request(format!("Invalid request body: {:?}", e)),
        };

        match project_service
            .update_project(
                project_id,
                payload.name.as_deref(),
                payload.description.as_deref(),
            )
            .await
        {
            Ok(project) => Response::json(
                HttpStatus::Ok,
                &ApiResponse {
                    success: true,
                    data: project,
                },
            ),
            Err(e) => {
                if e.to_string().contains("RecordNotFound") {
                    Response::not_found("Project not found")
                } else {
                    Response::internal_error(format!("Failed to update project: {}", e))
                }
            }
        }
    }

    /// DELETE /api/v1/projects/:id - Delete project
    #[delete("/:id")]
    #[cap(ProjectAdmin)]
    pub async fn delete_project(
        ctx: RequestContext,
        project_service: Arc<ProjectService>,
    ) -> Response {
        let project_id: i32 = match ctx.params.get("id").and_then(|p| p.parse().ok()) {
            Some(id) => id,
            None => return Response::bad_request("Invalid project ID"),
        };

        match project_service.delete_project(project_id).await {
            Ok(true) => Response::json(
                HttpStatus::Ok,
                &ApiResponse {
                    success: true,
                    data: "Project deleted successfully",
                },
            ),
            Ok(false) => Response::not_found("Project not found"),
            Err(e) => Response::internal_error(format!("Failed to delete project: {}", e)),
        }
    }

    /// GET /api/v1/projects/:id/issues - Get project issues
    #[get("/:id/issues")]
    #[cap(ViewBoard)]
    pub async fn get_project_issues(
        ctx: RequestContext,
        project_service: Arc<ProjectService>,
    ) -> Response {
        let project_id: i32 = match ctx.params.get("id").and_then(|p| p.parse().ok()) {
            Some(id) => id,
            None => return Response::bad_request("Invalid project ID"),
        };

        match project_service.get_project_with_issues(project_id).await {
            Ok(Some((project, issues))) => Response::json(
                HttpStatus::Ok,
                &ApiResponse {
                    success: true,
                    data: serde_json::json!({
                        "project": project,
                        "issues": issues
                    }),
                },
            ),
            Ok(None) => Response::not_found("Project not found"),
            Err(e) => Response::internal_error(format!("Failed to fetch project issues: {}", e)),
        }
    }

    /// GET /api/v1/projects/search - Search projects
    #[get("/search")]
    #[cap(ViewBoard)]
    pub async fn search_projects(
        ctx: RequestContext,
        project_service: Arc<ProjectService>,
    ) -> Response {
        let query = ctx
            .query
            .get("q")
            .and_then(|v| v.first())
            .map(|s| s.to_string())
            .unwrap_or_default();

        if query.is_empty() {
            return Response::bad_request("Search query is required");
        }

        match project_service.search_projects(&query).await {
            Ok(projects) => Response::json(
                HttpStatus::Ok,
                &ApiResponse {
                    success: true,
                    data: projects,
                },
            ),
            Err(e) => Response::internal_error(format!("Failed to search projects: {}", e)),
        }
    }

    // ======================================================
    // Workflow API (per project)
    // ======================================================

    /// GET /api/v1/projects/:id/workflow - List workflow steps (ordered by position)
    #[get("/:id/workflow")]
    #[cap(ViewBoard)]
    pub async fn get_workflow(
        ctx: RequestContext,
        workflow_repo: Arc<WorkflowRepository>,
    ) -> Response {
        let project_id: i32 = match ctx.params.get("id").and_then(|p| p.parse().ok()) {
            Some(id) => id,
            None => return Response::bad_request("Invalid project ID"),
        };

        match workflow_repo.find_steps_by_project(project_id).await {
            Ok(steps) => Response::json(
                HttpStatus::Ok,
                &ApiResponse {
                    success: true,
                    data: steps,
                },
            ),
            Err(e) => Response::internal_error(format!("Failed to fetch workflow: {}", e)),
        }
    }

    /// POST /api/v1/projects/:id/workflow/steps - Add a workflow step (appended)
    #[post("/:id/workflow/steps")]
    #[cap(ProjectAdmin)]
    pub async fn add_workflow_step(
        ctx: RequestContext,
        workflow_repo: Arc<WorkflowRepository>,
    ) -> Response {
        let project_id: i32 = match ctx.params.get("id").and_then(|p| p.parse().ok()) {
            Some(id) => id,
            None => return Response::bad_request("Invalid project ID"),
        };

        let payload = match ctx.json::<CreateWorkflowStepPayload>().await {
            Ok(p) => p,
            Err(_) => return Response::bad_request("Invalid workflow step payload"),
        };

        let name = payload.name.as_deref().unwrap_or("New Step");

        match workflow_repo.create_step_with_name(project_id, name).await {
            Ok(step) => Response::json(
                HttpStatus::Created,
                &ApiResponse {
                    success: true,
                    data: step,
                },
            ),
            Err(e) => Response::bad_request(format!("Failed to add step: {}", e)),
        }
    }

    /// POST /api/v1/projects/:id/workflow/default - Seed the default workflow
    /// columns (To Do / In Progress / In Review / Done) if the project has none.
    #[post("/:id/workflow/default")]
    #[cap(ProjectAdmin)]
    pub async fn ensure_default_workflow(
        ctx: RequestContext,
        workflow_repo: Arc<WorkflowRepository>,
        project_service: Arc<ProjectService>,
    ) -> Response {
        let project_id: i32 = match ctx.params.get("id").and_then(|p| p.parse().ok()) {
            Some(id) => id,
            None => return Response::bad_request("Invalid project ID"),
        };

        let existing = match workflow_repo.find_steps_by_project(project_id).await {
            Ok(steps) => steps,
            Err(e) => return Response::internal_error(format!("Failed to fetch workflow: {}", e)),
        };

        if !existing.is_empty() {
            return Response::json(
                HttpStatus::Ok,
                &ApiResponse {
                    success: true,
                    data: existing,
                },
            );
        }

        match project_service
            .create_default_workflow_steps(project_id)
            .await
        {
            Ok(()) => match workflow_repo.find_steps_by_project(project_id).await {
                Ok(steps) => Response::json(
                    HttpStatus::Ok,
                    &ApiResponse {
                        success: true,
                        data: steps,
                    },
                ),
                Err(e) => Response::internal_error(format!("Failed to fetch workflow: {}", e)),
            },
            Err(e) => Response::bad_request(format!("Failed to create default workflow: {}", e)),
        }
    }

    /// PATCH /api/v1/projects/:project_id/workflow/:step_id - Rename a workflow step
    #[patch("/:project_id/workflow/:step_id")]
    #[cap(ProjectAdmin)]
    pub async fn update_workflow_step(
        ctx: RequestContext,
        workflow_repo: Arc<WorkflowRepository>,
    ) -> Response {
        let step_id: i32 = match ctx.params.get("step_id").and_then(|p| p.parse().ok()) {
            Some(id) => id,
            None => return Response::bad_request("Invalid step ID"),
        };

        let payload = match ctx.json::<UpdateWorkflowStepPayload>().await {
            Ok(p) => p,
            Err(_) => return Response::bad_request("Invalid workflow step payload"),
        };

        match workflow_repo.update_step(step_id, payload.name.as_deref()).await {
            Ok(Some(step)) => Response::json(
                HttpStatus::Ok,
                &ApiResponse {
                    success: true,
                    data: step,
                },
            ),
            Ok(None) => Response::not_found("Step not found"),
            Err(e) => Response::bad_request(format!("Failed to update step: {}", e)),
        }
    }

    /// POST /api/v1/projects/:project_id/workflow/:step_id/toggle - Toggle
    /// the "completed" flag of a workflow step
    #[post("/:project_id/workflow/:step_id/toggle")]
    #[cap(ProjectAdmin)]
    pub async fn toggle_workflow_step(
        ctx: RequestContext,
        workflow_repo: Arc<WorkflowRepository>,
    ) -> Response {
        let step_id: i32 = match ctx.params.get("step_id").and_then(|p| p.parse().ok()) {
            Some(id) => id,
            None => return Response::bad_request("Invalid step ID"),
        };

        match workflow_repo.toggle_completed(step_id).await {
            Ok(Some(step)) => Response::json(
                HttpStatus::Ok,
                &ApiResponse {
                    success: true,
                    data: step,
                },
            ),
            Ok(None) => Response::not_found("Step not found"),
            Err(e) => Response::bad_request(format!("Failed to toggle step: {}", e)),
        }
    }

    /// DELETE /api/v1/projects/:project_id/workflow/:step_id - Delete a workflow step
    #[delete("/:project_id/workflow/:step_id")]
    #[cap(ProjectAdmin)]
    pub async fn delete_workflow_step(
        ctx: RequestContext,
        workflow_repo: Arc<WorkflowRepository>,
    ) -> Response {
        let step_id: i32 = match ctx.params.get("step_id").and_then(|p| p.parse().ok()) {
            Some(id) => id,
            None => return Response::bad_request("Invalid step ID"),
        };

        match workflow_repo.delete_step(step_id).await {
            Ok(true) => Response::json(
                HttpStatus::Ok,
                &ApiResponse {
                    success: true,
                    data: "Workflow step deleted",
                },
            ),
            Ok(false) => Response::not_found("Step not found"),
            Err(e) => Response::bad_request(format!("Failed to delete step: {}", e)),
        }
    }

    // ======================================================
    // Issue Types API (per project)
    // ======================================================

    /// GET /api/v1/projects/:id/issue-types - List issue types (seeding the
    /// Jira-style defaults the first time a project has none).
    #[get("/:id/issue-types")]
    #[cap(ViewBoard)]
    pub async fn get_issue_types(
        ctx: RequestContext,
        issue_type_repo: Arc<IssueTypeRepository>,
    ) -> Response {
        let project_id: i32 = match ctx.params.get("id").and_then(|p| p.parse().ok()) {
            Some(id) => id,
            None => return Response::bad_request("Invalid project ID"),
        };

        if let Err(e) = ensure_default_issue_types(&issue_type_repo, project_id).await {
            return Response::internal_error(format!("Failed to seed issue types: {}", e));
        }

        match issue_type_repo.find_by_project(project_id).await {
            Ok(types) => Response::json(
                HttpStatus::Ok,
                &ApiResponse {
                    success: true,
                    data: types,
                },
            ),
            Err(e) => Response::internal_error(format!("Failed to fetch issue types: {}", e)),
        }
    }

    /// POST /api/v1/projects/:id/issue-types - Add a custom issue type
    #[post("/:id/issue-types")]
    #[cap(ProjectAdmin)]
    pub async fn create_issue_type(
        ctx: RequestContext,
        issue_type_repo: Arc<IssueTypeRepository>,
    ) -> Response {
        let project_id: i32 = match ctx.params.get("id").and_then(|p| p.parse().ok()) {
            Some(id) => id,
            None => return Response::bad_request("Invalid project ID"),
        };

        let payload = match ctx.json::<CreateIssueTypePayload>().await {
            Ok(p) => p,
            Err(e) => return Response::bad_request(format!("Invalid issue type payload: {:?}", e)),
        };

        if payload.name.trim().is_empty() {
            return Response::bad_request("Issue type name is required");
        }

        let (icon_key, color) = (
            payload.icon_key.as_deref().unwrap_or("task"),
            payload.color.as_deref().unwrap_or("#4bade9"),
        );

        match issue_type_repo
            .create(project_id, &payload.name, icon_key, color)
            .await
        {
            Ok(issue_type) => Response::json(
                HttpStatus::Created,
                &ApiResponse {
                    success: true,
                    data: issue_type,
                },
            ),
            Err(e) => {
                if e.to_string().contains("duplicate") {
                    Response::bad_request("An issue type with this name already exists")
                } else {
                    Response::internal_error(format!("Failed to create issue type: {}", e))
                }
            }
        }
    }

    /// PATCH /api/v1/projects/:project_id/issue-types/:type_id - Update an issue type
    #[patch("/:project_id/issue-types/:type_id")]
    #[cap(ProjectAdmin)]
    pub async fn update_issue_type(
        ctx: RequestContext,
        issue_type_repo: Arc<IssueTypeRepository>,
    ) -> Response {
        let type_id: i32 = match ctx.params.get("type_id").and_then(|p| p.parse().ok()) {
            Some(id) => id,
            None => return Response::bad_request("Invalid issue type ID"),
        };

        let payload = match ctx.json::<UpdateIssueTypePayload>().await {
            Ok(p) => p,
            Err(_) => return Response::bad_request("Invalid issue type payload"),
        };

        match issue_type_repo
            .update_type(
                type_id,
                payload.name.as_deref(),
                payload.icon_key.as_deref(),
                payload.color.as_deref(),
            )
            .await
        {
            Ok(Some(issue_type)) => Response::json(
                HttpStatus::Ok,
                &ApiResponse {
                    success: true,
                    data: issue_type,
                },
            ),
            Ok(None) => Response::not_found("Issue type not found"),
            Err(e) => Response::bad_request(format!("Failed to update issue type: {}", e)),
        }
    }

    /// DELETE /api/v1/projects/:project_id/issue-types/:type_id - Delete an issue type
    #[delete("/:project_id/issue-types/:type_id")]
    #[cap(ProjectAdmin)]
    pub async fn delete_issue_type(
        ctx: RequestContext,
        issue_type_repo: Arc<IssueTypeRepository>,
    ) -> Response {
        let type_id: i32 = match ctx.params.get("type_id").and_then(|p| p.parse().ok()) {
            Some(id) => id,
            None => return Response::bad_request("Invalid issue type ID"),
        };

        match issue_type_repo.delete_type(type_id).await {
            Ok(true) => Response::json(
                HttpStatus::Ok,
                &ApiResponse {
                    success: true,
                    data: "Issue type deleted",
                },
            ),
            Ok(false) => Response::not_found("Issue type not found"),
            Err(e) => Response::bad_request(format!("Failed to delete issue type: {}", e)),
        }
    }

    // ======================================================
    // Project Summary API
    // ======================================================

    /// GET /api/v1/projects/:id/summary - Status + "Types of work" aggregation
    #[get("/:id/summary")]
    #[cap(ViewBoard)]
    pub async fn get_project_summary(
        ctx: RequestContext,
        workflow_repo: Arc<WorkflowRepository>,
        issue_repo: Arc<IssueRepository>,
        issue_type_repo: Arc<IssueTypeRepository>,
    ) -> Response {
        let project_id: i32 = match ctx.params.get("id").and_then(|p| p.parse().ok()) {
            Some(id) => id,
            None => return Response::bad_request("Invalid project ID"),
        };

        // Seed defaults so the summary always shows the full type catalog.
        let _ = ensure_default_issue_types(&issue_type_repo, project_id).await;

        let steps = match workflow_repo.find_steps_by_project(project_id).await {
            Ok(steps) => steps,
            Err(e) => return Response::internal_error(format!("Failed to fetch workflow: {}", e)),
        };
        let issues = match issue_repo.find_by_project(project_id).await {
            Ok(issues) => issues,
            Err(e) => return Response::internal_error(format!("Failed to fetch issues: {}", e)),
        };
        let types = match issue_type_repo.find_by_project(project_id).await {
            Ok(types) => types,
            Err(e) => return Response::internal_error(format!("Failed to fetch issue types: {}", e)),
        };

        // step_id -> completion flag. Fall back to the last column as "done"
        // when the workflow has no step flagged completed (mirrors burndown).
        let mut step_map: HashMap<i32, bool> = HashMap::new();
        let has_completed = steps.iter().any(|s| s.is_completed);
        for step in &steps {
            step_map.insert(step.id, step.is_completed);
        }
        if !has_completed {
            if let Some(last) = steps.iter().max_by_key(|s| (s.position, s.id)) {
                step_map.insert(last.id, true);
            }
        }

        // by_status buckets for every column (zero-filled)
        let mut by_status: Vec<StatusBucket> = steps
            .iter()
            .map(|s| StatusBucket {
                step_id: s.id,
                name: s.name.clone(),
                is_completed: s.is_completed,
                count: 0,
                points: 0,
            })
            .collect();

        // by_type buckets keyed by the stored issue_type string
        let mut type_buckets: HashMap<String, TypeBucket> = HashMap::new();

        let mut total_issues = 0;
        let mut open_issues = 0;
        let mut total_points = 0;
        let mut open_points = 0;

        for issue in &issues {
            let is_done = step_map.get(&issue.step_id).copied().unwrap_or(false);
            let points = issue.story_points.unwrap_or(0);

            total_issues += 1;
            total_points += points;

            if let Some(bucket) = by_status.iter_mut().find(|b| b.step_id == issue.step_id) {
                bucket.count += 1;
                bucket.points += points;
            }

            let (icon_key, color) = default_type_style(&issue.issue_type);
            let bucket = type_buckets
                .entry(issue.issue_type.clone())
                .or_insert_with(|| TypeBucket {
                    type_name: issue.issue_type.clone(),
                    icon_key,
                    color,
                    count_open: 0,
                    count_total: 0,
                    percent: 0,
                });
            bucket.count_total += 1;
            if !is_done {
                open_issues += 1;
                open_points += points;
                bucket.count_open += 1;
            }
        }

        // Overlay catalog icon/color for known types and fold in empty catalog
        // types so the "Types of work" gadget lists the configured set.
        for t in &types {
            if let Some(bucket) = type_buckets.get_mut(&t.name) {
                bucket.icon_key = t.icon_key.clone();
                bucket.color = t.color.clone();
            }
        }
        for t in &types {
            if !type_buckets.contains_key(&t.name) {
                type_buckets.insert(
                    t.name.clone(),
                    TypeBucket {
                        type_name: t.name.clone(),
                        icon_key: t.icon_key.clone(),
                        color: t.color.clone(),
                        count_open: 0,
                        count_total: 0,
                        percent: 0,
                    },
                );
            }
        }

        let mut by_type: Vec<TypeBucket> = type_buckets.into_values().collect();
        by_type.sort_by(|a, b| {
            let ac = a.count_open + a.count_total;
            let bc = b.count_open + b.count_total;
            bc.cmp(&ac).then_with(|| a.type_name.cmp(&b.type_name))
        });

        let open_total = open_issues.max(1);
        for bucket in by_type.iter_mut() {
            bucket.percent = ((bucket.count_open as f32 / open_total as f32) * 100.0).round() as i32;
        }

        let summary = ProjectSummary {
            project_id,
            total_issues,
            open_issues,
            done_issues: total_issues - open_issues,
            total_points,
            open_points,
            by_status,
            by_type,
        };

        Response::json(
            HttpStatus::Ok,
            &ApiResponse {
                success: true,
                data: summary,
            },
        )
    }
}