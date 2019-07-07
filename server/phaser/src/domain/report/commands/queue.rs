use crate::{domain::report, domain::scan};
use diesel::{
    r2d2::{ConnectionManager, PooledConnection},
    PgConnection,
};
use eventsourcing::{Event, EventTs};
use kernel::KernelError;

#[derive(Clone, Debug)]
pub struct Queue {
    pub scan_id: uuid::Uuid,
    pub targets: Vec<String>,
    pub profile: scan::ScanProfile,
    pub trigger: scan::ReportTrigger,
}

impl eventsourcing::Command for Queue {
    type Aggregate = report::Report;
    type Event = report::Event;
    type Context = PooledConnection<ConnectionManager<PgConnection>>;
    type Error = KernelError;
    type NonStoredData = ();

    fn validate(
        &self,
        _ctx: &Self::Context,
        _aggregate: &Self::Aggregate,
    ) -> Result<(), Self::Error> {
        return Ok(());
    }

    fn build_event(
        &self,
        _ctx: &Self::Context,
        _aggregate: &Self::Aggregate,
    ) -> Result<Self::Event, Self::Error> {
        return Ok(Queued {
            id: uuid::Uuid::new_v4(),
            timestamp: chrono::Utc::now(),
            scan_id: self.scan_id,
            targets: self.targets.clone(),
            profile: self.profile.clone(),
            trigger: self.trigger.clone(),
        });
    }
}

// Event
#[derive(Clone, Debug, EventTs)]
pub struct Queued {
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub id: uuid::Uuid,
    pub scan_id: uuid::Uuid,
    pub targets: Vec<String>,
    pub profile: ScanProfile,
    pub trigger: ReportTrigger,
}

impl Event for Queued {
    type Aggregate = report::Report;

    fn apply(&self, aggregate: Self::Aggregate) -> Self::Aggregate {
        return Self::Aggregate {
            id: selfid,
            created_at: self.timestamp,
            updated_at: self.timestamp,
            deleted_at: None,
            version: 0,
            completed_at: None,
            error: None,
            findings: None,
            high_level_findings: 0,
            information_findings: 0,
            low_level_findings: 0,
            medium_level_findings: 0,
            profile: selfprofile.clone(),
            started_at: None,
            status: report::ReportStatus::Queued,
            targets: self.targets.clone(),
            trigger: selftrigger.clone(),
            total_findings: 0,
            scan_id: selfscan_id,
        };
    }
}
