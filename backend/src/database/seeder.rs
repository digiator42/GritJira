use crate::models::{comment, issue, project, sprint, user, workflow};
use crate::security::password::hash_password;
use chrono::Utc;
use sea_orm::{ActiveModelTrait, DatabaseConnection, EntityTrait, Set};

pub async fn seed_database(db: &DatabaseConnection) -> Result<(), sea_orm::DbErr> {
    // 1. Check if database is already seeded
    if user::Entity::find().one(db).await?.is_some() {
        println!("[SEEDER] Database already contains data. Skipping seed.");
        return Ok(());
    }

    println!("[SEEDER] Seeding initial GritJira data...");

    // ============================================================
    // 2. Seed Users
    // ============================================================
    let admin_user = user::ActiveModel {
        username: Set("admin".to_string()),
        email: Set("admin@gritjira.local".to_string()),
        password: Set(hash_password("admin123").expect("argon2 hashing")),
        role: Set("Admin".to_string()),
        avatar_url: Set(Some(
            "https://api.dicebear.com/7.x/avataaars/svg?seed=admin".to_string(),
        )),
        created_at: Set(Utc::now().naive_utc()),
        ..Default::default()
    }
    .insert(db)
    .await?;

    let dev_user = user::ActiveModel {
        username: Set("alex_dev".to_string()),
        email: Set("alex@gritjira.local".to_string()),
        password: Set(hash_password("alex123").expect("argon2 hashing")),
        role: Set("Developer".to_string()),
        avatar_url: Set(Some(
            "https://api.dicebear.com/7.x/avataaars/svg?seed=alex".to_string(),
        )),
        created_at: Set(Utc::now().naive_utc()),
        ..Default::default()
    }
    .insert(db)
    .await?;

    // ============================================================
    // 3. Seed Project
    // ============================================================
    let project = project::ActiveModel {
        key: Set("GRIT".to_string()),
        name: Set("GritShield Engine".to_string()),
        description: Set(Some(
            "Security Framework and Jira Clone core platform".to_string(),
        )),
        created_at: Set(Utc::now().naive_utc()),
        ..Default::default()
    }
    .insert(db)
    .await?;

    // ============================================================
    // 4. Seed Workflow Steps (Columns for the Kanban Board)
    // ============================================================
    let todo_step = workflow::ActiveModel {
        project_id: Set(project.id),
        name: Set("To Do".to_string()),
        position: Set(0),
        is_completed: Set(false),
        ..Default::default()
    }
    .insert(db)
    .await?;

    let in_progress_step = workflow::ActiveModel {
        project_id: Set(project.id),
        name: Set("In Progress".to_string()),
        position: Set(1),
        is_completed: Set(false),
        ..Default::default()
    }
    .insert(db)
    .await?;

    let review_step = workflow::ActiveModel {
        project_id: Set(project.id),
        name: Set("In Review".to_string()),
        position: Set(2),
        is_completed: Set(false),
        ..Default::default()
    }
    .insert(db)
    .await?;

    let done_step = workflow::ActiveModel {
        project_id: Set(project.id),
        name: Set("Done".to_string()),
        position: Set(3),
        is_completed: Set(true),
        ..Default::default()
    }
    .insert(db)
    .await?;

    // ============================================================
    // 5. Seed Sprint
    // ============================================================
    let sprint = sprint::ActiveModel {
        project_id: Set(project.id),
        name: Set("Sprint 1 - Foundation".to_string()),
        goal: Set(Some(
            "Build initial framework and setup Kanban board".to_string(),
        )),
        status: Set("active".to_string()),
        start_date: Set(Some(Utc::now().naive_utc())),
        end_date: Set(Some((Utc::now() + chrono::Duration::days(14)).naive_utc())),
        ..Default::default()
    }
    .insert(db)
    .await?;

    // ============================================================
    // 6. Seed Issues
    // ============================================================
    let issue_1 = issue::ActiveModel {
        project_id: Set(project.id),
        sprint_id: Set(Some(sprint.id)),
        step_id: Set(todo_step.id),
        reporter_id: Set(admin_user.id),
        assignee_id: Set(Some(dev_user.id)),
        key: Set("GRIT-1".to_string()),
        summary: Set("Configure eBPF packet filter rules".to_string()),
        description: Set(Some(
            "Integrate XDP network firewall routines into kernel pipeline.".to_string(),
        )),
        priority: Set("High".to_string()),
        issue_type: Set("Task".to_string()),
        story_points: Set(Some(5)),
        created_at: Set(Utc::now().naive_utc()),
        ..Default::default()
    }
    .insert(db)
    .await?;

    let issue_2 = issue::ActiveModel {
        project_id: Set(project.id),
        sprint_id: Set(Some(sprint.id)),
        step_id: Set(in_progress_step.id),
        reporter_id: Set(dev_user.id),
        assignee_id: Set(Some(admin_user.id)),
        key: Set("GRIT-2".to_string()),
        summary: Set("Refactor GritRepository DSL macro expansions".to_string()),
        description: Set(Some(
            "Clean up query builder traits and pagination support.".to_string(),
        )),
        priority: Set("Medium".to_string()),
        issue_type: Set("Story".to_string()),
        story_points: Set(Some(3)),
        created_at: Set(Utc::now().naive_utc()),
        ..Default::default()
    }
    .insert(db)
    .await?;

    let _issue_3 = issue::ActiveModel {
        project_id: Set(project.id),
        sprint_id: Set(Some(sprint.id)),
        step_id: Set(review_step.id),
        reporter_id: Set(admin_user.id),
        assignee_id: Set(Some(dev_user.id)),
        key: Set("GRIT-3".to_string()),
        summary: Set("Fix HTMX drag-and-drop target swap".to_string()),
        description: Set(Some(
            "Ensure sortable-column events dispatch proper step_id updates.".to_string(),
        )),
        priority: Set("High".to_string()),
        issue_type: Set("Bug".to_string()),
        story_points: Set(Some(2)),
        created_at: Set(Utc::now().naive_utc()),
        ..Default::default()
    }
    .insert(db)
    .await?;

    let _issue_4 = issue::ActiveModel {
        project_id: Set(project.id),
        sprint_id: Set(Some(sprint.id)),
        step_id: Set(done_step.id),
        reporter_id: Set(admin_user.id),
        assignee_id: Set(Some(dev_user.id)),
        key: Set("GRIT-4".to_string()),
        summary: Set("Initial PostgreSQL migrations setup".to_string()),
        description: Set(Some(
            "Create base tables for users, projects, sprints, and issues.".to_string(),
        )),
        priority: Set("Low".to_string()),
        issue_type: Set("Task".to_string()),
        story_points: Set(Some(1)),
        created_at: Set(Utc::now().naive_utc()),
        ..Default::default()
    }
    .insert(db)
    .await?;

    // ============================================================
    // 7. Seed Comments
    // ============================================================
    comment::ActiveModel {
        issue_id: Set(issue_1.id),
        author_id: Set(admin_user.id),
        body: Set("Make sure to verify kernel loading permissions on x86_64 arch.".to_string()),
        created_at: Set(Utc::now().naive_utc()),
        ..Default::default()
    }
    .insert(db)
    .await?;

    comment::ActiveModel {
        issue_id: Set(issue_2.id),
        author_id: Set(dev_user.id),
        body: Set("Working on unifying the QueryBuilder and sea_orm Entity traits.".to_string()),
        created_at: Set(Utc::now().naive_utc()),
        ..Default::default()
    }
    .insert(db)
    .await?;

    println!("[SEEDER] Database successfully seeded!");
    Ok(())
}
