use crate::{domain::file, FOLDER_TYPE};
use diesel::{
    r2d2::{ConnectionManager, PooledConnection},
    PgConnection,
};
use kernel::{events::EventMetadata, KernelError};

#[derive(Clone, Debug)]
pub struct Move {
    pub to: uuid::Uuid,
    pub metadata: EventMetadata,
}

impl eventsourcing::Command for Move {
    type Aggregate = file::File;
    type Event = file::Event;
    type Context = PooledConnection<ConnectionManager<PgConnection>>;
    type Error = KernelError;
    type NonStoredData = ();

    fn validate(
        &self,
        ctx: &Self::Context,
        aggregate: &Self::Aggregate,
    ) -> Result<(), Self::Error> {
        use diesel::prelude::*;
        use kernel::db::schema::drive_files;

        // fetch new parent
        let new_parent: file::File = drive_files::dsl::drive_files
            .filter(drive_files::dsl::id.eq(self.to))
            .filter(drive_files::dsl::owner_id.eq(aggregate.owner_id))
            .filter(drive_files::dsl::deleted_at.is_null())
            .filter(drive_files::dsl::trashed_at.is_null())
            .first(ctx)?;

        if new_parent.type_ != FOLDER_TYPE {
            return Err(KernelError::Validation(
                "Destination must be a file".to_string(),
            ));
        }

        if new_parent.id == aggregate.id {
            return Err(KernelError::Validation(
                "Destination can't be source".to_string(),
            ));
        }

        return Ok(());
    }

    fn build_event(
        &self,
        _ctx: &Self::Context,
        aggregate: &Self::Aggregate,
    ) -> Result<(Self::Event, Self::NonStoredData), Self::Error> {
        let event_data = file::EventData::MovedV1(file::MovedV1 { to: self.to });;

        return Ok((
            file::Event {
                id: uuid::Uuid::new_v4(),
                timestamp: chrono::Utc::now(),
                data: event_data,
                aggregate_id: aggregate.id,
                metadata: self.metadata.clone(),
            },
            (),
        ));
    }
}
