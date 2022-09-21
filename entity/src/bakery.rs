use rocket::serde::{Deserialize, Serialize};
use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Deserialize, Serialize, FromForm)]
#[serde(crate = "rocket::serde")]
#[sea_orm(table_name = "bakery")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub name: String,
    #[sea_orm(column_type = "Double")]
    pub profitmargin: f64,
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {
    Chef,
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Self::Chef => Entity::belongs_to(super::chef::Entity)
                .from(Column::Id)
                .to(super::chef::Column::Id)
                .into(),
        }
    }
}

impl Related<super::chef::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Chef.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
