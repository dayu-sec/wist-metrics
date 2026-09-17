//! 采集目标视图条目：provider 采集的输入（一个「采什么」的候选目标）。

use serde::{Deserialize, Serialize};
use wist_contracts::discovery::StringKeyValue;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MetricsTargetViewEntry {
    pub candidate_id: String,
    pub collection_kind: String,
    pub target_ref: String,
    pub resource_ref: String,
    #[serde(default)]
    pub execution_hints: Vec<StringKeyValue>,
}
