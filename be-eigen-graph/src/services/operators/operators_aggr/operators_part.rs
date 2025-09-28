use crate::models::operators_aggr::{PageMeta, UniformOperator, UniformPage, UniformPosition};
use std::collections::BTreeMap;

pub fn partition_by_token(page: &UniformPage) -> BTreeMap<String, UniformPage> {
    let mut out: BTreeMap<String, Vec<UniformOperator>> = BTreeMap::new();

    for op in &page.operators {
        let mut buckets: BTreeMap<String, Vec<UniformPosition>> = BTreeMap::new();
        for p in &op.positions {
            if p.token_symbol.is_empty() {
                continue;
            }
            buckets
                .entry(p.token_symbol.clone())
                .or_default()
                .push(p.clone());
        }

        for (sym, positions) in buckets {
            if positions.is_empty() {
                continue;
            }
            let mut op_clone = op.clone();
            op_clone.positions = positions;
            out.entry(sym).or_default().push(op_clone);
        }
    }

    out.into_iter()
        .map(|(sym, ops)| {
            let token_page = UniformPage {
                operators: ops,
                page_meta: PageMeta {
                    first: page.page_meta.first,
                    skip: page.page_meta.skip,
                },
            };
            (sym, token_page)
        })
        .collect()
}
