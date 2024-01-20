use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug,Clone,PartialEq,Eq,DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name="user")]
pub struct Model {

    #[sea_orm(primary_key)]
    #[serde(skip_deserializing)]
    pub id: i32,
    pub username: String,
    pub password: String,

}
#[derive(Debug,Clone,Copy,EnumIter,DeriveRelation)]
pub enum Relation {
    
}

impl ActiveModelBehavior for ActiveModel {
    
}
 
  