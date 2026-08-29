use gritshield::{GritModel, GritRelation};
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use chrono::NaiveDateTime;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize, GritModel)]
#[sea_orm(table_name = "issues")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub project_id: i32,
    pub sprint_id: Option<i32>,
    pub step_id: i32,
    pub reporter_id: i32,
    pub assignee_id: Option<i32>,
    pub position: i32,
    #[sea_orm(unique)]
    pub key: String,
    pub summary: String,
    pub description: Option<String>,
    pub priority: String,
    pub issue_type: String,
    pub story_points: Option<i32>,
    pub time_estimate_minutes: Option<i32>,
    pub time_spent_minutes: i32,
    pub created_at: NaiveDateTime,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation, GritRelation)]
#[grit(table = "issues")]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::project::Entity",
        from = "Column::ProjectId",
        to = "super::project::Column::Id"
    )]
    Project,
    #[sea_orm(
        belongs_to = "super::sprint::Entity",
        from = "Column::SprintId",
        to = "super::sprint::Column::Id"
    )]
    Sprint,
    #[sea_orm(
        belongs_to = "super::workflow::Entity",
        from = "Column::StepId",
        to = "super::workflow::Column::Id"
    )]
    WorkflowStep,
    #[sea_orm(
        belongs_to = "super::user::Entity",
        from = "Column::ReporterId",
        to = "super::user::Column::Id"
    )]
    Reporter,
    #[sea_orm(
        belongs_to = "super::user::Entity",
        from = "Column::AssigneeId",
        to = "super::user::Column::Id"
    )]
    Assignee,
    #[sea_orm(has_many = "super::comment::Entity")]
    Comments,
}

impl Related<super::project::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Project.def()
    }
}

impl Related<super::sprint::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Sprint.def()
    }
}

impl Related<super::workflow::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::WorkflowStep.def()
    }
}

impl Related<super::comment::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Comments.def()
    }
}
impl Related<super::user::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Assignee.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}