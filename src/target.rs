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

#[cfg(test)]
mod tests {
    use super::MetricsTargetViewEntry;
    use serde_json::{Value, json};
    use wist_contracts::discovery::StringKeyValue;

    fn full_entry() -> MetricsTargetViewEntry {
        MetricsTargetViewEntry {
            candidate_id: "host-1".to_string(),
            collection_kind: "host_metrics".to_string(),
            target_ref: "host-1:host".to_string(),
            resource_ref: "host-1".to_string(),
            execution_hints: vec![
                StringKeyValue::new("host.name", "host-a"),
                StringKeyValue::new("host.region", "cn-north"),
            ],
        }
    }

    #[test]
    fn entry_serde_round_trip() {
        let entry = full_entry();
        let encoded = serde_json::to_string(&entry).expect("serialize entry");
        let decoded: MetricsTargetViewEntry =
            serde_json::from_str(&encoded).expect("deserialize entry");
        assert_eq!(decoded, entry);
    }

    #[test]
    fn entry_missing_optional_hints_defaults_to_empty() {
        let value = json!({
            "candidate_id": "host-1",
            "collection_kind": "host_metrics",
            "target_ref": "host-1:host",
            "resource_ref": "host-1",
        });
        let entry: MetricsTargetViewEntry =
            serde_json::from_value(value).expect("deserialize entry");
        assert!(entry.execution_hints.is_empty());
    }

    #[test]
    fn entry_preserves_hint_order_and_values() {
        let entry = full_entry();
        assert_eq!(entry.execution_hints[0].key, "host.name");
        assert_eq!(entry.execution_hints[0].value, "host-a");
        assert_eq!(entry.execution_hints[1].key, "host.region");
        assert_eq!(entry.execution_hints[1].value, "cn-north");
    }

    #[test]
    fn entry_accepts_empty_string_fields() {
        // 本层不做内容校验，空串只是普通取值。
        let value = json!({
            "candidate_id": "",
            "collection_kind": "",
            "target_ref": "",
            "resource_ref": "",
            "execution_hints": [],
        });
        let entry: MetricsTargetViewEntry =
            serde_json::from_value(value).expect("deserialize entry");
        assert!(entry.candidate_id.is_empty());
        assert!(entry.execution_hints.is_empty());
    }

    #[test]
    fn entry_rejects_unknown_field() {
        let mut value = json!({
            "candidate_id": "host-1",
            "collection_kind": "host_metrics",
            "target_ref": "host-1:host",
            "resource_ref": "host-1",
        });
        value["collection_target"] = json!("unexpected");
        let err = serde_json::from_value::<MetricsTargetViewEntry>(value)
            .expect_err("unknown field must be rejected");
        assert!(err.to_string().contains("collection_target"), "err: {err}");
    }

    #[test]
    fn entry_rejects_missing_required_field() {
        let value = json!({
            "collection_kind": "host_metrics",
            "target_ref": "host-1:host",
            "resource_ref": "host-1",
        });
        let err = serde_json::from_value::<MetricsTargetViewEntry>(value)
            .expect_err("missing candidate_id must be rejected");
        assert!(err.to_string().contains("candidate_id"), "err: {err}");
    }

    #[test]
    fn entry_rejects_null_execution_hints() {
        let value = json!({
            "candidate_id": "host-1",
            "collection_kind": "host_metrics",
            "target_ref": "host-1:host",
            "resource_ref": "host-1",
            "execution_hints": Value::Null,
        });
        assert!(serde_json::from_value::<MetricsTargetViewEntry>(value).is_err());
    }

    #[test]
    fn entry_rejects_duplicate_field() {
        let encoded = r#"{"candidate_id":"host-1","candidate_id":"host-2","collection_kind":"host_metrics","target_ref":"host-1:host","resource_ref":"host-1"}"#;
        let err = serde_json::from_str::<MetricsTargetViewEntry>(encoded)
            .expect_err("duplicate field must be rejected");
        assert!(err.to_string().contains("duplicate"), "err: {err}");
    }
}
