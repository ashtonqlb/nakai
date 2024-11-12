use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Files::Table)
                    .if_not_exists()
                    .col(pk_auto(Files::Id))
                    .col(string(Files::Name))
                    .col(date(Files::Date))
                    .col(string(Files::Hash))
                    .col(integer(Files::IP))
                    .col(integer(Files::Size))
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Denylist::Table)
                    .if_not_exists()
                    .col(pk_auto(Denylist::Id))
                    .col(string(Denylist::Name))
                    .col(date(Denylist::Date))
                    .col(string(Denylist::Hash))
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Ratelimit::Table)
                    .if_not_exists()
                    .col(pk_auto(Ratelimit::Id))
                    .col(string(Ratelimit::IpHash))
                    .col(integer(Ratelimit::Files))
                    .col(date(Ratelimit::Date))
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Passkeys::Table)
                    .if_not_exists()
                    .col(pk_auto(Passkeys::Id))
                    .col(string(Passkeys::PublicKey))
                    .col(string(Passkeys::Credential))
                    .col(integer(Passkeys::Level))
                    .to_owned(),
            )
            .await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Posts::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Posts {
    Table,
}

#[derive(DeriveIden)]
enum Files {
    Table,
    Id,
    Name,
    Date,
    Hash,
    IP,
    Size,
}

#[derive(DeriveIden)]
enum Denylist {
    Table,
    Id,
    Name,
    Date,
    Hash,
}

#[derive(DeriveIden)]
enum Ratelimit {
    Table,
    Id,
    IpHash,
    Files,
    Date,
}

#[derive(DeriveIden)]
enum Passkeys {
    Table,
    Id,
    PublicKey,
    Credential,
    Level,
}
