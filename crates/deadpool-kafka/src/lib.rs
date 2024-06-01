//! Deadpool for Kafka
//! 
//! This crate implements a [deadpool] manager for [kafka].

use deadpool::managed;
use kafka::{
    producer::{Producer, RequiredAcks},
    Error,
};
use std::{convert::Infallible, time::Duration};

pub use deadpool::managed::reexports::*;
pub use kafka;

/// Type alias for [managed::Object]
pub type Connection = managed::Object<Manager>;

type RecycleResult = managed::RecycleResult<Error>;
type ConfigError = Infallible;

deadpool::managed_reexports!(
    "kafka",
    Manager,
    managed::Object<Manager>,
    Error,
    ConfigError
);

#[derive(Debug)]
/// [`Manager`] for creating and recycling [`kafka::Producer`].
pub struct Manager {
    hosts: Vec<String>,
}

impl Manager {
    /// Creates a new [`Manager`] using the given Kafka hosts.
    #[must_use]
    pub fn new(hosts: Vec<String>) -> Self {
        Self { hosts }
    }
}

impl managed::Manager for Manager {
    type Type = Producer;
    type Error = Error;

    async fn create(&self) -> Result<Producer, Error> {
        #[cfg(not(feature = "compression"))]
        let conn = Producer::from_hosts(self.hosts.clone())
            .with_ack_timeout(Duration::from_secs(1))
            .with_required_acks(RequiredAcks::One)
            .create();

        #[cfg(feature = "compression")]
        let conn = {
            use kafka::producer::Compression;

            Producer::from_hosts(self.hosts.clone())
                .with_ack_timeout(Duration::from_secs(1))
                .with_required_acks(RequiredAcks::One)
                .with_compression(Compression::SNAPPY)
                .create()
        };

        conn
    }

    async fn recycle(
        &self,
        _conn: &mut Producer,
        _: &Metrics,
    ) -> RecycleResult {
        Ok(())
    }
}
