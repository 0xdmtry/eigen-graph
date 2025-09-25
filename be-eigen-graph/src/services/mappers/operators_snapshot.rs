use crate::models::operator::OperatorRiskRow;
use crate::models::operators_snapshot::{OperatorDto, OperatorsSnapshotData};

pub fn map_operators_snapshot(data: &OperatorsSnapshotData) -> Vec<OperatorRiskRow> {
    data.operators.iter().map(map_operator).collect()
}

fn map_operator(o: &OperatorDto) -> OperatorRiskRow {
    todo!()
}
