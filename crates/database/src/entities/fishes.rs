//! SeaORM Entity. Generated by sea-orm-codegen 0.9.2

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "fishes")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub name: String,
    pub html_name: String,
    pub count: i32,
    pub base_value: f32,
    pub max_weight: f32,
    pub min_weight: f32,
    pub is_trash: bool,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::catches::Entity")]
    Catches,
}

impl Related<super::catches::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Catches.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}