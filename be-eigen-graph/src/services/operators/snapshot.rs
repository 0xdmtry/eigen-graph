use crate::models::operators_snapshot::{OperatorsSnapshotData, OperatorsSnapshotVars};

pub trait OperatorsSnapshotFetcher {
    fn fetch(
        &self,
        vars: OperatorsSnapshotVars,
    ) -> futures::future::BoxFuture<'_, Result<OperatorsSnapshotData, anyhow::Error>>;
}
