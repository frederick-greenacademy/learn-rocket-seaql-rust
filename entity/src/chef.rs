use rocket::serde::{Deserialize, Serialize};
use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
#[sea_orm(table_name = "chef")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub name: String,
    #[sea_orm(column_name = "BakeryId")]
    pub bakeryid: i32,
    #[sea_orm(column_name = "ContactDetails")]
    pub contactdetails: Json,
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {
    Bakery,
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Self::Bakery => Entity::has_many(super::bakery::Entity).into(),
        }
    }
}

impl Related<super::bakery::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Bakery.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
