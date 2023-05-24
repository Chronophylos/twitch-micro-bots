//! `SeaORM` Entity. Generated by sea-orm-codegen 0.10.4

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "seasons")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub name: String,
    pub start: DateTimeWithTimeZone,
    pub end: Option<DateTimeWithTimeZone>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::catches::Entity")]
    Catches,
    #[sea_orm(has_many = "super::season_data::Entity")]
    SeasonData,
}

impl Related<super::catches::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Catches.def()
    }
}

impl Related<super::season_data::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::SeasonData.def()
    }
}

impl Related<super::fishes::Entity> for Entity {
    fn to() -> RelationDef {
        super::fishes_seasons::Relation::Fishes.def()
    }
    fn via() -> Option<RelationDef> {
        Some(super::fishes_seasons::Relation::Seasons.def().rev())
    }
}

impl ActiveModelBehavior for ActiveModel {}
