//! 指标声明（spec）：把采集侧 runtime fact 规范化成 OTel 风格指标的声明式映射。
//!
//! 这是 W4 三层结构的「契约层」——声明「采什么、叫什么、什么单位/类型」。新增一个指标 =
//! 在 [`METRIC_SPECS`] 里加一行，不改采集（provider）与规范化逻辑。

/// 一个指标的声明：某个采集 kind 的某个 runtime fact 规范化成什么指标。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MetricSpec {
    /// 所属采集 kind（与 `MetricProvider::collection_kind` 对应，即 provider 维度）。
    pub collection_kind: &'static str,
    /// 采集侧 runtime fact 键（如 `host.loadavg.1m`）。
    pub fact_key: &'static str,
    /// 规范化后的指标名（如 `system.load_average.1m`）。
    pub name: &'static str,
    /// 单位。
    pub unit: &'static str,
    /// 值类型（`gauge_i64` / `gauge_f64` / `gauge_string`）。
    pub value_type: &'static str,
}

/// Batch A 指标声明表（新增指标 = 在这里加一行）。
pub const METRIC_SPECS: &[MetricSpec] = &[
    MetricSpec {
        collection_kind: "host_metrics",
        fact_key: "host.target.count",
        name: "system.target.count",
        unit: "1",
        value_type: "gauge_i64",
    },
    MetricSpec {
        collection_kind: "host_metrics",
        fact_key: "host.loadavg.1m",
        name: "system.load_average.1m",
        unit: "1",
        value_type: "gauge_f64",
    },
    MetricSpec {
        collection_kind: "host_metrics",
        fact_key: "host.loadavg.5m",
        name: "system.load_average.5m",
        unit: "1",
        value_type: "gauge_f64",
    },
    MetricSpec {
        collection_kind: "host_metrics",
        fact_key: "host.loadavg.15m",
        name: "system.load_average.15m",
        unit: "1",
        value_type: "gauge_f64",
    },
    MetricSpec {
        collection_kind: "host_metrics",
        fact_key: "host.uptime.seconds",
        name: "system.uptime",
        unit: "s",
        value_type: "gauge_f64",
    },
    MetricSpec {
        collection_kind: "host_metrics",
        fact_key: "host.memory.total_kb",
        name: "system.memory.total",
        unit: "KiBy",
        value_type: "gauge_i64",
    },
    MetricSpec {
        collection_kind: "host_metrics",
        fact_key: "host.memory.available_kb",
        name: "system.memory.available",
        unit: "KiBy",
        value_type: "gauge_i64",
    },
    MetricSpec {
        collection_kind: "host_metrics",
        fact_key: "host.disk.usage_percent",
        name: "system.disk.usage",
        unit: "percent",
        value_type: "gauge_f64",
    },
    MetricSpec {
        collection_kind: "host_metrics",
        fact_key: "host.disk.total_kb",
        name: "system.disk.total",
        unit: "KiBy",
        value_type: "gauge_i64",
    },
    MetricSpec {
        collection_kind: "host_metrics",
        fact_key: "host.disk.available_kb",
        name: "system.disk.available",
        unit: "KiBy",
        value_type: "gauge_i64",
    },
    // Linux procfs 的 rss 是「页」，其它 unix 的 `ps -o rss` 是 KiB；两者平台互斥（同一
    // kind 下只会出现其一），故都规范化为同一个 `*.memory.rss`，单位随平台不同。
    MetricSpec {
        collection_kind: "process_metrics",
        fact_key: "process.memory.rss_pages",
        name: "process.memory.rss",
        unit: "pages",
        value_type: "gauge_i64",
    },
    MetricSpec {
        collection_kind: "process_metrics",
        fact_key: "process.memory.rss_kb",
        name: "process.memory.rss",
        unit: "KiBy",
        value_type: "gauge_i64",
    },
    MetricSpec {
        collection_kind: "process_metrics",
        fact_key: "process.state",
        name: "process.state",
        unit: "state",
        value_type: "gauge_string",
    },
    MetricSpec {
        collection_kind: "container_metrics",
        fact_key: "process.memory.rss_pages",
        name: "container.memory.rss",
        unit: "pages",
        value_type: "gauge_i64",
    },
    MetricSpec {
        collection_kind: "container_metrics",
        fact_key: "process.memory.rss_kb",
        name: "container.memory.rss",
        unit: "KiBy",
        value_type: "gauge_i64",
    },
    MetricSpec {
        collection_kind: "container_metrics",
        fact_key: "process.state",
        name: "container.state",
        unit: "state",
        value_type: "gauge_string",
    },
    MetricSpec {
        collection_kind: "container_metrics",
        fact_key: "container.pid",
        name: "container.pid",
        unit: "1",
        value_type: "gauge_i64",
    },
];

/// 按 `(collection_kind, fact_key)` 查找指标声明。
pub fn find_metric_spec(collection_kind: &str, fact_key: &str) -> Option<&'static MetricSpec> {
    METRIC_SPECS
        .iter()
        .find(|spec| spec.collection_kind == collection_kind && spec.fact_key == fact_key)
}

#[cfg(test)]
mod tests {
    use std::collections::{BTreeMap, BTreeSet};

    use super::{METRIC_SPECS, find_metric_spec};

    #[test]
    fn finds_spec_by_kind_and_fact_key() {
        let spec = find_metric_spec("host_metrics", "host.loadavg.1m").expect("spec");
        assert_eq!(spec.name, "system.load_average.1m");
        assert_eq!(spec.unit, "1");
        assert_eq!(spec.value_type, "gauge_f64");
    }

    #[test]
    fn unknown_fact_key_is_none() {
        assert!(find_metric_spec("host_metrics", "no.such.fact").is_none());
    }

    #[test]
    fn specs_are_unique_per_kind_and_fact_key() {
        // 声明表不能有重复的 (collection_kind, fact_key)，否则规范化结果不确定。
        let mut seen = std::collections::HashSet::new();
        for spec in METRIC_SPECS {
            let key = (spec.collection_kind, spec.fact_key);
            assert!(seen.insert(key), "duplicate spec key: {key:?}");
        }
    }

    #[test]
    fn finds_specs_across_all_collection_kinds() {
        let host = find_metric_spec("host_metrics", "host.loadavg.5m").expect("host spec");
        assert_eq!(host.name, "system.load_average.5m");

        let process = find_metric_spec("process_metrics", "process.state").expect("process spec");
        assert_eq!(process.name, "process.state");
        assert_eq!(process.value_type, "gauge_string");

        let container =
            find_metric_spec("container_metrics", "container.pid").expect("container spec");
        assert_eq!(container.name, "container.pid");
    }

    #[test]
    fn fact_key_is_scoped_by_collection_kind() {
        // 同一个 fact_key 在不同 kind 下可以映射到不同指标名。
        let process = find_metric_spec("process_metrics", "process.memory.rss_pages")
            .expect("process rss spec");
        let container = find_metric_spec("container_metrics", "process.memory.rss_pages")
            .expect("container rss spec");
        assert_eq!(process.name, "process.memory.rss");
        assert_eq!(container.name, "container.memory.rss");
        assert_ne!(process.name, container.name);
    }

    #[test]
    fn unknown_collection_kind_is_none() {
        assert!(find_metric_spec("no_such_kind", "host.loadavg.1m").is_none());
    }

    #[test]
    fn empty_inputs_are_none() {
        assert!(find_metric_spec("", "").is_none());
        assert!(find_metric_spec("host_metrics", "").is_none());
        assert!(find_metric_spec("", "host.loadavg.1m").is_none());
    }

    #[test]
    fn lookup_is_exact_not_prefix_or_case_insensitive() {
        assert!(find_metric_spec("host_metrics", "host.loadavg").is_none());
        assert!(find_metric_spec("host", "host.loadavg.1m").is_none());
        assert!(find_metric_spec("host_metrics", "HOST.LOADAVG.1M").is_none());
    }

    #[test]
    fn table_is_not_empty() {
        assert!(!METRIC_SPECS.is_empty());
    }

    #[test]
    fn every_spec_field_is_non_empty() {
        for spec in METRIC_SPECS {
            for field in [
                spec.collection_kind,
                spec.fact_key,
                spec.name,
                spec.unit,
                spec.value_type,
            ] {
                assert!(!field.trim().is_empty(), "empty field in spec: {spec:?}");
            }
        }
    }

    #[test]
    fn every_value_type_is_one_the_pipeline_supports() {
        // 与 `wist-agentd` 的 `is_numeric_sample` / `sample_value` 支持的集合保持一致。
        const SUPPORTED: &[&str] = &[
            "gauge_i64",
            "gauge_f64",
            "gauge_string",
            "counter_i64",
            "counter_f64",
        ];
        for spec in METRIC_SPECS {
            assert!(
                SUPPORTED.contains(&spec.value_type),
                "unsupported value_type `{}` in {spec:?}",
                spec.value_type
            );
        }
    }

    #[test]
    fn every_collection_kind_is_known() {
        const KINDS: &[&str] = &["host_metrics", "process_metrics", "container_metrics"];
        for spec in METRIC_SPECS {
            assert!(
                KINDS.contains(&spec.collection_kind),
                "unknown collection_kind `{}` in {spec:?}",
                spec.collection_kind
            );
        }
    }

    #[test]
    fn lookup_matches_every_table_entry() {
        // `METRIC_SPECS` 是 const，每个使用点可能是一份独立拷贝，故按字段比较而不是指针。
        for spec in METRIC_SPECS {
            let found = find_metric_spec(spec.collection_kind, spec.fact_key).expect("spec");
            assert_eq!(found.name, spec.name);
            assert_eq!(found.unit, spec.unit);
            assert_eq!(found.value_type, spec.value_type);
        }
    }

    #[test]
    fn normalized_name_has_one_unit_except_the_rss_platform_split() {
        // 契约：同一 collection_kind 下，一个规范化指标名只应有一个单位。
        // 已知例外：process/container 的 rss，Linux 用 procfs 得 pages、其它 unix 用
        // `ps -o rss` 得 KiB，二者平台互斥，这里显式固定住，避免新增别的歧义映射。
        let mut units_by_name: BTreeMap<(&str, &str), BTreeSet<&str>> = BTreeMap::new();
        for spec in METRIC_SPECS {
            units_by_name
                .entry((spec.collection_kind, spec.name))
                .or_default()
                .insert(spec.unit);
        }
        let ambiguous: Vec<((&str, &str), Vec<&str>)> = units_by_name
            .iter()
            .filter(|(_, units)| units.len() > 1)
            .map(|(key, units)| (*key, units.iter().copied().collect()))
            .collect();
        assert_eq!(
            ambiguous,
            vec![
                (
                    ("container_metrics", "container.memory.rss"),
                    vec!["KiBy", "pages"]
                ),
                (
                    ("process_metrics", "process.memory.rss"),
                    vec!["KiBy", "pages"]
                ),
            ],
            "同名指标只允许平台互斥的 rss pages/KiBy 例外"
        );
    }
}
