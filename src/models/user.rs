use gritshield::{GritModel, GritRelation};
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use chrono::NaiveDateTime;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize, GritModel)]
#[sea_orm(table_name = "users")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    #[sea_orm(unique)]
    pub username: String,
    #[sea_orm(unique)]
    pub email: String,
    pub role: String,
    pub password: String,
    pub avatar_url: Option<String>,
    pub created_at: NaiveDateTime,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation, GritRelation)]
#[grit(table = "users")]
pub enum Relation {
    #[sea_orm(has_many = "super::issue::Entity")]
    AssignedIssues,
    #[sea_orm(has_many = "super::comment::Entity")]
    Comments,
}

impl Related<super::issue::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::AssignedIssues.def()
    }
}

impl Related<super::comment::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Comments.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}