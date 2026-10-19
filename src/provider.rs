//! 采集层抽象：`MetricProvider` trait 与一次采集的结果类型。
//!
//! provider 只产出样本（runtime fact 键值），不做上报决策；规范化/上报在 `wist-agentd`
//! 的 samples 层完成。`MetricsCollectionOutcome` / `MetricsCollectionTargetSample` 是
//! collect → samples 之间的中转结构，这里与 `wist-agentd` 共享。

use serde::{Deserialize, Serialize};
use wist_contracts::discovery::StringKeyValue;

use crate::target::MetricsTargetViewEntry;

/// 一类指标采集的抽象（编译期注册 provider）。
pub trait MetricProvider {
    /// 采集 kind（与 `MetricsTargetViewEntry.collection_kind` 对应）。
    fn collection_kind(&self) -> &'static str;

    /// 对一组目标采集，产出该 kind 的采集结果。
    fn collect(&self, targets: Vec<&MetricsTargetViewEntry>) -> MetricsCollectionOutcome;
}

/// 一类指标的一次采集结果。
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, ::jumo_derive::Jumo)]
#[serde(deny_unknown_fields)]
#[jumo(kind = "struct", domain = "Discovery", module = "Discovery.Collect")]
pub struct MetricsCollectionOutcome {
    pub collection_kind: String,
    pub status: String,
    pub attempted_targets: usize,
    pub succeeded_targets: usize,
    pub failed_targets: usize,
    pub last_error: Option<String>,
    #[serde(default)]
    pub runtime_facts: Vec<StringKeyValue>,
    /// 带标签的采集事实（如按挂载点区分的文件系统容量）。
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub labeled_facts: Vec<MetricsLabeledFact>,
    #[serde(default)]
    pub sample_targets: Vec<MetricsCollectionTargetSample>,
}

/// 单个采集目标的采集结果（含该目标产出的 runtime fact）。
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, ::jumo_derive::Jumo)]
#[serde(deny_unknown_fields)]
#[jumo(kind = "struct", domain = "Discovery", module = "Discovery.Collect")]
pub struct MetricsCollectionTargetSample {
    pub candidate_id: String,
    pub target_ref: String,
    pub status: String,
    pub last_error: Option<String>,
    pub resource_ref: String,
    #[serde(default)]
    pub execution_hints: Vec<StringKeyValue>,
    #[serde(default)]
    pub runtime_facts: Vec<StringKeyValue>,
    /// 带标签的采集事实：同一个 fact key 的多个实例（如不同挂载点）用 `labels` 区分。
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub labeled_facts: Vec<MetricsLabeledFact>,
}

/// 带标签的采集事实。
///
/// 例如文件系统容量：`key = host.filesystem.usage_percent`，
/// `labels = [{mount: /var}, {fstype: xfs}]`。标签只用于区分同一指标下的多个实例，
/// 不参与 spec 的 `fact_key` 匹配。
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, ::jumo_derive::Jumo)]
#[serde(deny_unknown_fields)]
#[jumo(kind = "struct", domain = "Discovery", module = "Discovery.Collect")]
pub struct MetricsLabeledFact {
    pub key: String,
    pub value: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub labels: Vec<StringKeyValue>,
}

#[cfg(test)]
mod tests {
    use super::{
        MetricProvider, MetricsCollectionOutcome, MetricsCollectionTargetSample, MetricsLabeledFact,
    };
    use crate::target::MetricsTargetViewEntry;
    use serde_json::{Value, json};
    use wist_contracts::discovery::StringKeyValue;

    fn full_outcome() -> MetricsCollectionOutcome {
        MetricsCollectionOutcome {
            collection_kind: "host_metrics".to_string(),
            status: "partial".to_string(),
            attempted_targets: 2,
            succeeded_targets: 1,
            failed_targets: 1,
            last_error: Some("probe failed".to_string()),
            runtime_facts: vec![
                StringKeyValue::new("host.loadavg.1m", "0.25"),
                StringKeyValue::new("host.uptime.seconds", "3600"),
            ],
            labeled_facts: vec![MetricsLabeledFact {
                key: "host.filesystem.usage_percent".to_string(),
                value: "95".to_string(),
                labels: vec![
                    StringKeyValue::new("mount", "/var"),
                    StringKeyValue::new("fstype", "xfs"),
                ],
            }],
            sample_targets: vec![full_sample_target()],
        }
    }

    fn full_sample_target() -> MetricsCollectionTargetSample {
        MetricsCollectionTargetSample {
            candidate_id: "host-1".to_string(),
            target_ref: "host-1:host".to_string(),
            status: "succeeded".to_string(),
            last_error: None,
            resource_ref: "host-1".to_string(),
            execution_hints: vec![StringKeyValue::new("host.name", "host-a")],
            runtime_facts: vec![StringKeyValue::new("host.loadavg.1m", "0.25")],
            labeled_facts: vec![MetricsLabeledFact {
                key: "host.filesystem.available_kb".to_string(),
                value: "1048576".to_string(),
                labels: vec![StringKeyValue::new("mount", "/var")],
            }],
        }
    }

    fn minimal_outcome_json() -> Value {
        json!({
            "collection_kind": "process_metrics",
            "status": "idle",
            "attempted_targets": 0,
            "succeeded_targets": 0,
            "failed_targets": 0,
        })
    }

    #[test]
    fn outcome_serde_round_trip() {
        let outcome = full_outcome();
        let encoded = serde_json::to_string(&outcome).expect("serialize outcome");
        let decoded: MetricsCollectionOutcome =
            serde_json::from_str(&encoded).expect("deserialize outcome");
        assert_eq!(decoded, outcome);
    }

    #[test]
    fn target_sample_serde_round_trip() {
        let sample = full_sample_target();
        let encoded = serde_json::to_string(&sample).expect("serialize sample");
        let decoded: MetricsCollectionTargetSample =
            serde_json::from_str(&encoded).expect("deserialize sample");
        assert_eq!(decoded, sample);
    }

    #[test]
    fn outcome_serializes_absent_last_error_as_null() {
        let mut outcome = full_outcome();
        outcome.last_error = None;
        let value: Value = serde_json::to_value(&outcome).expect("serialize outcome");
        assert_eq!(value["last_error"], Value::Null);
        assert_eq!(value["runtime_facts"].as_array().map(Vec::len), Some(2));
    }

    #[test]
    fn outcome_minimal_json_applies_vec_defaults_and_none_last_error() {
        let outcome: MetricsCollectionOutcome =
            serde_json::from_value(minimal_outcome_json()).expect("deserialize minimal outcome");
        assert!(outcome.last_error.is_none());
        assert!(outcome.runtime_facts.is_empty());
        assert!(outcome.labeled_facts.is_empty());
        assert!(outcome.sample_targets.is_empty());
    }

    #[test]
    fn outcome_explicit_null_last_error_is_none() {
        let mut value = minimal_outcome_json();
        value["last_error"] = Value::Null;
        let outcome: MetricsCollectionOutcome =
            serde_json::from_value(value).expect("deserialize null last_error");
        assert!(outcome.last_error.is_none());
    }

    #[test]
    fn outcome_accepts_large_counts() {
        let value = json!({
            "collection_kind": "host_metrics",
            "status": "idle",
            "attempted_targets": usize::MAX,
            "succeeded_targets": usize::MAX,
            "failed_targets": 0,
        });
        let outcome: MetricsCollectionOutcome =
            serde_json::from_value(value).expect("deserialize large counts");
        assert_eq!(outcome.attempted_targets, usize::MAX);
        assert_eq!(outcome.succeeded_targets, usize::MAX);
    }

    #[test]
    fn outcome_rejects_negative_count() {
        let mut value = minimal_outcome_json();
        value["attempted_targets"] = json!(-1);
        assert!(serde_json::from_value::<MetricsCollectionOutcome>(value).is_err());
    }

    #[test]
    fn outcome_rejects_out_of_range_count() {
        // 超出 usize 的整数不能被静默截断。
        let encoded = r#"{"collection_kind":"host_metrics","status":"idle","attempted_targets":18446744073709551616,"succeeded_targets":0,"failed_targets":0}"#;
        assert!(serde_json::from_str::<MetricsCollectionOutcome>(encoded).is_err());
    }

    #[test]
    fn outcome_rejects_unknown_field() {
        let mut value = minimal_outcome_json();
        value["unexpected"] = json!(1);
        let err = serde_json::from_value::<MetricsCollectionOutcome>(value)
            .expect_err("unknown field must be rejected");
        assert!(err.to_string().contains("unexpected"), "err: {err}");
    }

    #[test]
    fn outcome_rejects_missing_required_field() {
        let mut value = minimal_outcome_json();
        value.as_object_mut().expect("object").remove("status");
        let err = serde_json::from_value::<MetricsCollectionOutcome>(value)
            .expect_err("missing status must be rejected");
        assert!(err.to_string().contains("status"), "err: {err}");
    }

    #[test]
    fn outcome_rejects_duplicate_field() {
        let encoded = r#"{"collection_kind":"host_metrics","collection_kind":"process_metrics","status":"idle","attempted_targets":0,"succeeded_targets":0,"failed_targets":0}"#;
        let err = serde_json::from_str::<MetricsCollectionOutcome>(encoded)
            .expect_err("duplicate field must be rejected");
        assert!(err.to_string().contains("duplicate"), "err: {err}");
    }

    #[test]
    fn outcome_rejects_null_for_required_string() {
        let mut value = minimal_outcome_json();
        value["status"] = Value::Null;
        assert!(serde_json::from_value::<MetricsCollectionOutcome>(value).is_err());
    }

    #[test]
    fn outcome_preserves_duplicate_fact_entries() {
        // 本层不去重：去重是采集侧（`push_fact_if_absent`）的责任，这里只保证不丢不损。
        let mut outcome = full_outcome();
        outcome.runtime_facts = vec![
            StringKeyValue::new("dup.key", "1"),
            StringKeyValue::new("dup.key", "2"),
        ];
        let encoded = serde_json::to_string(&outcome).expect("serialize outcome");
        let decoded: MetricsCollectionOutcome =
            serde_json::from_str(&encoded).expect("deserialize outcome");
        assert_eq!(decoded.runtime_facts, outcome.runtime_facts);
    }

    #[test]
    fn labeled_facts_round_trip_and_keep_labels() {
        let outcome = full_outcome();
        let encoded = serde_json::to_string(&outcome).expect("serialize outcome");
        let decoded: MetricsCollectionOutcome =
            serde_json::from_str(&encoded).expect("deserialize outcome");
        assert_eq!(decoded.labeled_facts, outcome.labeled_facts);
        assert_eq!(decoded.labeled_facts[0].labels.len(), 2);
    }

    #[test]
    fn empty_labeled_facts_are_omitted_from_json() {
        let mut outcome = full_outcome();
        outcome.labeled_facts.clear();
        let value: Value = serde_json::to_value(&outcome).expect("serialize outcome");
        assert!(value.get("labeled_facts").is_none());
    }

    #[test]
    fn labeled_fact_rejects_unknown_field() {
        let value = json!({
            "key": "host.filesystem.usage_percent",
            "value": "95",
            "unexpected": 1,
        });
        assert!(serde_json::from_value::<MetricsLabeledFact>(value).is_err());
    }

    #[test]
    fn labeled_fact_preserves_duplicate_labels() {
        let fact = MetricsLabeledFact {
            key: "host.filesystem.usage_percent".to_string(),
            value: "95".to_string(),
            labels: vec![
                StringKeyValue::new("mount", "/a"),
                StringKeyValue::new("mount", "/b"),
            ],
        };
        let encoded = serde_json::to_string(&fact).expect("serialize fact");
        let decoded: MetricsLabeledFact = serde_json::from_str(&encoded).expect("deserialize fact");
        assert_eq!(decoded.labels.len(), 2);
    }

    #[test]
    fn labeled_fact_labels_default_to_empty() {
        let fact: MetricsLabeledFact = serde_json::from_value(json!({
            "key": "host.filesystem.total_kb",
            "value": "1024",
        }))
        .expect("deserialize labeled fact");
        assert!(fact.labels.is_empty());
    }

    #[test]
    fn target_sample_minimal_json_applies_vec_defaults() {
        let value = json!({
            "candidate_id": "proc-1",
            "target_ref": "proc-1:process",
            "status": "succeeded",
            "resource_ref": "proc-1",
        });
        let sample: MetricsCollectionTargetSample =
            serde_json::from_value(value).expect("deserialize minimal sample");
        assert!(sample.last_error.is_none());
        assert!(sample.execution_hints.is_empty());
        assert!(sample.runtime_facts.is_empty());
        assert!(sample.labeled_facts.is_empty());
    }

    #[test]
    fn target_sample_rejects_unknown_field() {
        let value = json!({
            "candidate_id": "proc-1",
            "target_ref": "proc-1:process",
            "status": "succeeded",
            "resource_ref": "proc-1",
            "unexpected": true,
        });
        assert!(serde_json::from_value::<MetricsCollectionTargetSample>(value).is_err());
    }

    #[test]
    fn provider_trait_is_object_safe_and_dispatchable() {
        struct StaticProvider;

        impl MetricProvider for StaticProvider {
            fn collection_kind(&self) -> &'static str {
                "static_metrics"
            }

            fn collect(&self, targets: Vec<&MetricsTargetViewEntry>) -> MetricsCollectionOutcome {
                MetricsCollectionOutcome {
                    collection_kind: self.collection_kind().to_string(),
                    status: if targets.is_empty() {
                        "idle"
                    } else {
                        "succeeded"
                    }
                    .to_string(),
                    attempted_targets: targets.len(),
                    succeeded_targets: targets.len(),
                    failed_targets: 0,
                    last_error: None,
                    runtime_facts: Vec::new(),
                    labeled_facts: Vec::new(),
                    sample_targets: Vec::new(),
                }
            }
        }

        let provider: &dyn MetricProvider = &StaticProvider;
        let entry = MetricsTargetViewEntry {
            candidate_id: "host-1".to_string(),
            collection_kind: "static_metrics".to_string(),
            target_ref: "host-1:host".to_string(),
            resource_ref: "host-1".to_string(),
            execution_hints: Vec::new(),
        };

        assert_eq!(provider.collection_kind(), "static_metrics");
        let outcome = provider.collect(vec![&entry]);
        assert_eq!(outcome.collection_kind, provider.collection_kind());
        assert_eq!(outcome.attempted_targets, 1);
        assert_eq!(outcome.succeeded_targets, 1);
        assert_eq!(outcome.status, "succeeded");

        let idle = provider.collect(Vec::new());
        assert_eq!(idle.status, "idle");
        assert_eq!(idle.attempted_targets, 0);
    }
}
