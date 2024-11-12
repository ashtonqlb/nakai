use ::entity::{file, file::Entity as File};
use sea_orm::*;

pub struct Mutation;

impl Mutation {
    pub async fn create_file(
        db: &DbConn,
        form_data: file::Model,
    ) -> Result<file::ActiveModel, DbErr> {
        file::ActiveModel {
            ..Default::default()
        }
        .save(db)
        .await
    }
}
