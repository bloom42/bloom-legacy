use crate::domain::contact;
use diesel::{
    r2d2::{ConnectionManager, PooledConnection},
    PgConnection,
};
use kernel::{events::EventMetadata, KernelError};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct UpdateNotes {
    pub notes: Option<String>,
    pub metadata: EventMetadata,
}

impl eventsourcing::Command for UpdateNotes {
    type Aggregate = contact::Contact;
    type Event = contact::Event;
    type Context = PooledConnection<ConnectionManager<PgConnection>>;
    type Error = KernelError;
    type NonStoredData = ();

    fn validate(
        &self,
        _ctx: &Self::Context,
        aggregate: &Self::Aggregate,
    ) -> Result<(), Self::Error> {
        if aggregate.deleted_at.is_some() {
            return Err(KernelError::NotFound("Note not found".to_string()));
        }

        return Ok(());
    }

    fn build_event(
        &self,
        _ctx: &Self::Context,
        aggregate: &Self::Aggregate,
    ) -> Result<(Self::Event, Self::NonStoredData), Self::Error> {
        let data = contact::EventData::NotesUpdatedV1(contact::NotesUpdatedV1 {
            notes: self.notes.clone(),
        });

        return Ok((
            contact::Event {
                id: uuid::Uuid::new_v4(),
                timestamp: chrono::Utc::now(),
                data,
                aggregate_id: aggregate.id,
                metadata: self.metadata.clone(),
            },
            (),
        ));
    }
}
