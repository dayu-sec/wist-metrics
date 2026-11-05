//! 指标契约（spec）：把采集侧 runtime fact 规范化成指标的声明式映射。
//!
//! 这里是《指标数据规范》的机器可读副本：指标名、单位、值类型、label 只在此定义一次。
//! 新增一个指标 = 加一行，不改采集与规范化逻辑。
//!
//! 规范要点（由本模块的测试固化）：
//! - 名字全小写、下划线分隔，单位用后缀承载（`_bytes` / `_seconds` / `_ratio`）；
//! - 单位闭集 `By` / `s` / `1`；计数器名以 `_total` 结尾；
//! - 域由 `collection_kind` 决定，进名字首段，不再用 label 重复；
//! - label 白名单由 `const_labels` / `dynamic_labels` 声明。

/// 一个指标的声明：某个采集域的某个 runtime fact 规范化成什么指标。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MetricSpec {
    /// 采集域（`host_metrics` / `process_metrics` / `container_metrics` / `kafka_broker_metrics`）。
    pub collection_kind: &'static str,
    /// 采集侧 runtime fact 键（必须单位中立，值已是 `unit` 单位）。
    pub fact_key: &'static str,
    /// 规范化后的指标名（snake_case，单位后缀在后）。
    pub name: &'static str,
    /// 单位：`By` / `s` / `1`。
    pub unit: &'static str,
    /// 值类型：`gauge_i64` / `gauge_f64` / `counter_i64` / `counter_f64`。
    pub value_type: &'static str,
    /// 规范化层写入的常量 label（如 `disk_role=system`）。
    pub const_labels: &'static [(&'static str, &'static str)],
    /// 来自采集事实的动态 label 键（如 `mountpoint` / `device` / `state`）。
    pub dynamic_labels: &'static [&'static str],
    /// OpenTelemetry 语义约定对齐项；无对应时为空串。
    pub semconv_ref: &'static str,
}

const NO_CONST: &[(&str, &str)] = &[];
const NO_LABELS: &[&str] = &[];

/// 指标注册表（新增指标 = 在这里加一行）。
pub const METRIC_SPECS: &[MetricSpec] = &[
    // ---- 主机：身份（info 指标）----
    // 一条 1 值序列承载「这是哪台机器」：主机名 / 主 IP / 系统 / 架构 / 内核。
    // 这些值会变（改名、换 IP、内核升级），所以**只**放在 info 指标上；测量序列的
    // 身份仍是 `(name, agent, instance)`，变一个 IP 只替换这一条序列，不会切断历史。
    MetricSpec {
        collection_kind: "host_metrics",
        fact_key: "host.info",
        name: "system_host_info",
        unit: "1",
        value_type: "gauge_i64",
        const_labels: NO_CONST,
        dynamic_labels: &["host_name", "host_ip", "os", "arch", "kernel"],
        semconv_ref: "host.name / host.ip",
    },
    // ---- 主机：CPU ----
    MetricSpec {
        collection_kind: "host_metrics",
        fact_key: "host.loadavg.1m",
        name: "system_load_average_1m",
        unit: "1",
        value_type: "gauge_f64",
        const_labels: NO_CONST,
        dynamic_labels: NO_LABELS,
        semconv_ref: "system.cpu.load_average.1m",
    },
    MetricSpec {
        collection_kind: "host_metrics",
        fact_key: "host.loadavg.5m",
        name: "system_load_average_5m",
        unit: "1",
        value_type: "gauge_f64",
        const_labels: NO_CONST,
        dynamic_labels: NO_LABELS,
        semconv_ref: "system.cpu.load_average.5m",
    },
    MetricSpec {
        collection_kind: "host_metrics",
        fact_key: "host.loadavg.15m",
        name: "system_load_average_15m",
        unit: "1",
        value_type: "gauge_f64",
        const_labels: NO_CONST,
        dynamic_labels: NO_LABELS,
        semconv_ref: "system.cpu.load_average.15m",
    },
    MetricSpec {
        collection_kind: "host_metrics",
        fact_key: "host.cpu.seconds",
        name: "system_cpu_seconds_total",
        unit: "s",
        value_type: "counter_f64",
        const_labels: NO_CONST,
        dynamic_labels: &["cpu", "mode"],
        semconv_ref: "system.cpu.time",
    },
    MetricSpec {
        collection_kind: "host_metrics",
        fact_key: "host.uptime.seconds",
        name: "system_uptime_seconds",
        unit: "s",
        value_type: "gauge_f64",
        const_labels: NO_CONST,
        dynamic_labels: NO_LABELS,
        semconv_ref: "system.uptime",
    },
    // ---- 主机：内存 ----
    MetricSpec {
        collection_kind: "host_metrics",
        fact_key: "host.memory.total_bytes",
        name: "system_memory_total_bytes",
        unit: "By",
        value_type: "gauge_i64",
        const_labels: NO_CONST,
        dynamic_labels: NO_LABELS,
        semconv_ref: "system.memory.limit",
    },
    MetricSpec {
        collection_kind: "host_metrics",
        fact_key: "host.memory.available_bytes",
        name: "system_memory_available_bytes",
        unit: "By",
        value_type: "gauge_i64",
        const_labels: NO_CONST,
        dynamic_labels: NO_LABELS,
        semconv_ref: "system.memory.usage",
    },
    MetricSpec {
        collection_kind: "host_metrics",
        fact_key: "host.memory.usage_ratio",
        name: "system_memory_usage_ratio",
        unit: "1",
        value_type: "gauge_f64",
        const_labels: NO_CONST,
        dynamic_labels: NO_LABELS,
        semconv_ref: "system.memory.utilization",
    },
    MetricSpec {
        collection_kind: "host_metrics",
        fact_key: "host.memory.swap_total_bytes",
        name: "system_memory_swap_total_bytes",
        unit: "By",
        value_type: "gauge_i64",
        const_labels: NO_CONST,
        dynamic_labels: NO_LABELS,
        semconv_ref: "system.paging.limit",
    },
    MetricSpec {
        collection_kind: "host_metrics",
        fact_key: "host.memory.swap_free_bytes",
        name: "system_memory_swap_free_bytes",
        unit: "By",
        value_type: "gauge_i64",
        const_labels: NO_CONST,
        dynamic_labels: NO_LABELS,
        semconv_ref: "system.paging.usage",
    },
    // ---- 主机：磁盘容量（按角色汇总） ----
    MetricSpec {
        collection_kind: "host_metrics",
        fact_key: "host.disk.system.usage_ratio",
        name: "system_disk_usage_ratio",
        unit: "1",
        value_type: "gauge_f64",
        const_labels: &[("disk_role", "system")],
        dynamic_labels: NO_LABELS,
        semconv_ref: "",
    },
    MetricSpec {
        collection_kind: "host_metrics",
        fact_key: "host.disk.system.total_bytes",
        name: "system_disk_total_bytes",
        unit: "By",
        value_type: "gauge_i64",
        const_labels: &[("disk_role", "system")],
        dynamic_labels: NO_LABELS,
        semconv_ref: "",
    },
    MetricSpec {
        collection_kind: "host_metrics",
        fact_key: "host.disk.system.available_bytes",
        name: "system_disk_available_bytes",
        unit: "By",
        value_type: "gauge_i64",
        const_labels: &[("disk_role", "system")],
        dynamic_labels: NO_LABELS,
        semconv_ref: "",
    },
    MetricSpec {
        collection_kind: "host_metrics",
        fact_key: "host.disk.data.usage_ratio",
        name: "system_disk_usage_ratio",
        unit: "1",
        value_type: "gauge_f64",
        const_labels: &[("disk_role", "data")],
        dynamic_labels: NO_LABELS,
        semconv_ref: "",
    },
    MetricSpec {
        collection_kind: "host_metrics",
        fact_key: "host.disk.data.total_bytes",
        name: "system_disk_total_bytes",
        unit: "By",
        value_type: "gauge_i64",
        const_labels: &[("disk_role", "data")],
        dynamic_labels: NO_LABELS,
        semconv_ref: "",
    },
    MetricSpec {
        collection_kind: "host_metrics",
        fact_key: "host.disk.data.available_bytes",
        name: "system_disk_available_bytes",
        unit: "By",
        value_type: "gauge_i64",
        const_labels: &[("disk_role", "data")],
        dynamic_labels: NO_LABELS,
        semconv_ref: "",
    },
    MetricSpec {
        collection_kind: "host_metrics",
        fact_key: "host.disk.data.mounts",
        name: "system_disk_mounts",
        unit: "1",
        value_type: "gauge_i64",
        const_labels: &[("disk_role", "data")],
        dynamic_labels: NO_LABELS,
        semconv_ref: "",
    },
    // ---- 主机：磁盘 I/O（按设备） ----
    MetricSpec {
        collection_kind: "host_metrics",
        fact_key: "host.disk.read_bytes",
        name: "system_disk_read_bytes_total",
        unit: "By",
        value_type: "counter_i64",
        const_labels: NO_CONST,
        dynamic_labels: &["device"],
        semconv_ref: "system.disk.io",
    },
    MetricSpec {
        collection_kind: "host_metrics",
        fact_key: "host.disk.written_bytes",
        name: "system_disk_written_bytes_total",
        unit: "By",
        value_type: "counter_i64",
        const_labels: NO_CONST,
        dynamic_labels: &["device"],
        semconv_ref: "system.disk.io",
    },
    MetricSpec {
        collection_kind: "host_metrics",
        fact_key: "host.disk.reads",
        name: "system_disk_reads_total",
        unit: "1",
        value_type: "counter_i64",
        const_labels: NO_CONST,
        dynamic_labels: &["device"],
        semconv_ref: "system.disk.operations",
    },
    MetricSpec {
        collection_kind: "host_metrics",
        fact_key: "host.disk.writes",
        name: "system_disk_writes_total",
        unit: "1",
        value_type: "counter_i64",
        const_labels: NO_CONST,
        dynamic_labels: &["device"],
        semconv_ref: "system.disk.operations",
    },
    MetricSpec {
        collection_kind: "host_metrics",
        fact_key: "host.disk.io_time_seconds",
        name: "system_disk_io_time_seconds_total",
        unit: "s",
        value_type: "counter_f64",
        const_labels: NO_CONST,
        dynamic_labels: &["device"],
        semconv_ref: "system.disk.io_time",
    },
    // ---- 主机：逐挂载点文件系统 ----
    MetricSpec {
        collection_kind: "host_metrics",
        fact_key: "host.filesystem.usage_ratio",
        name: "system_filesystem_usage_ratio",
        unit: "1",
        value_type: "gauge_f64",
        const_labels: NO_CONST,
        dynamic_labels: &["mountpoint", "fstype"],
        semconv_ref: "system.filesystem.utilization",
    },
    MetricSpec {
        collection_kind: "host_metrics",
        fact_key: "host.filesystem.total_bytes",
        name: "system_filesystem_total_bytes",
        unit: "By",
        value_type: "gauge_i64",
        const_labels: NO_CONST,
        dynamic_labels: &["mountpoint", "fstype"],
        semconv_ref: "",
    },
    MetricSpec {
        collection_kind: "host_metrics",
        fact_key: "host.filesystem.available_bytes",
        name: "system_filesystem_available_bytes",
        unit: "By",
        value_type: "gauge_i64",
        const_labels: NO_CONST,
        dynamic_labels: &["mountpoint", "fstype"],
        semconv_ref: "system.filesystem.usage",
    },
    MetricSpec {
        collection_kind: "host_metrics",
        fact_key: "host.filesystem.inodes",
        name: "system_filesystem_inodes",
        unit: "1",
        value_type: "gauge_i64",
        const_labels: NO_CONST,
        dynamic_labels: &["mountpoint", "fstype"],
        semconv_ref: "",
    },
    MetricSpec {
        collection_kind: "host_metrics",
        fact_key: "host.filesystem.inodes_free",
        name: "system_filesystem_inodes_free",
        unit: "1",
        value_type: "gauge_i64",
        const_labels: NO_CONST,
        dynamic_labels: &["mountpoint", "fstype"],
        semconv_ref: "",
    },
    // ---- 进程 ----
    MetricSpec {
        collection_kind: "process_metrics",
        fact_key: "process.memory.rss_bytes",
        name: "process_memory_rss_bytes",
        unit: "By",
        value_type: "gauge_i64",
        const_labels: NO_CONST,
        dynamic_labels: NO_LABELS,
        semconv_ref: "process.memory.usage",
    },
    MetricSpec {
        collection_kind: "process_metrics",
        fact_key: "process.cpu.seconds",
        name: "process_cpu_seconds_total",
        unit: "s",
        value_type: "counter_f64",
        const_labels: NO_CONST,
        dynamic_labels: NO_LABELS,
        semconv_ref: "process.cpu.time",
    },
    MetricSpec {
        collection_kind: "process_metrics",
        fact_key: "process.state",
        name: "process_state",
        unit: "1",
        value_type: "gauge_i64",
        const_labels: NO_CONST,
        dynamic_labels: &["state"],
        semconv_ref: "",
    },
    MetricSpec {
        collection_kind: "process_metrics",
        fact_key: "process.pid",
        name: "process_pid",
        unit: "1",
        value_type: "gauge_i64",
        const_labels: NO_CONST,
        dynamic_labels: NO_LABELS,
        semconv_ref: "",
    },
    // ---- 容器 ----
    MetricSpec {
        collection_kind: "container_metrics",
        fact_key: "process.memory.rss_bytes",
        name: "container_memory_rss_bytes",
        unit: "By",
        value_type: "gauge_i64",
        const_labels: NO_CONST,
        dynamic_labels: NO_LABELS,
        semconv_ref: "container.memory.usage",
    },
    MetricSpec {
        collection_kind: "container_metrics",
        fact_key: "process.cpu.seconds",
        name: "container_cpu_seconds_total",
        unit: "s",
        value_type: "counter_f64",
        const_labels: NO_CONST,
        dynamic_labels: NO_LABELS,
        semconv_ref: "container.cpu.time",
    },
    MetricSpec {
        collection_kind: "container_metrics",
        fact_key: "process.state",
        name: "container_state",
        unit: "1",
        value_type: "gauge_i64",
        const_labels: NO_CONST,
        dynamic_labels: &["state"],
        semconv_ref: "",
    },
    MetricSpec {
        collection_kind: "container_metrics",
        fact_key: "container.pid",
        name: "container_pid",
        unit: "1",
        value_type: "gauge_i64",
        const_labels: NO_CONST,
        dynamic_labels: NO_LABELS,
        semconv_ref: "",
    },
    // ---- Kafka broker（协议探针，不依赖 exporter）----
    // 名字空间 `kafka` 见规范 §2.2。目标身份走 `instance`
    // （`kafka:<cluster_key>:<broker_id>`），所以这几条都不带动态 label。
    MetricSpec {
        collection_kind: "kafka_broker_metrics",
        fact_key: "kafka.broker.up",
        name: "kafka_broker_up",
        unit: "1",
        value_type: "gauge_i64",
        const_labels: NO_CONST,
        dynamic_labels: NO_LABELS,
        semconv_ref: "",
    },
    MetricSpec {
        collection_kind: "kafka_broker_metrics",
        fact_key: "kafka.broker.is_controller",
        name: "kafka_broker_role_is_controller",
        unit: "1",
        value_type: "gauge_i64",
        const_labels: NO_CONST,
        dynamic_labels: NO_LABELS,
        semconv_ref: "",
    },
    MetricSpec {
        collection_kind: "kafka_broker_metrics",
        fact_key: "kafka.partition.count",
        name: "kafka_partition_count",
        unit: "1",
        value_type: "gauge_i64",
        const_labels: NO_CONST,
        dynamic_labels: NO_LABELS,
        semconv_ref: "",
    },
    MetricSpec {
        collection_kind: "kafka_broker_metrics",
        fact_key: "kafka.partition.under_replicated",
        name: "kafka_partition_under_replicated",
        unit: "1",
        value_type: "gauge_i64",
        const_labels: NO_CONST,
        dynamic_labels: NO_LABELS,
        semconv_ref: "",
    },
    MetricSpec {
        collection_kind: "kafka_broker_metrics",
        fact_key: "kafka.log_dir.size_bytes",
        name: "kafka_log_dir_size_bytes",
        unit: "By",
        value_type: "gauge_i64",
        const_labels: NO_CONST,
        dynamic_labels: NO_LABELS,
        semconv_ref: "",
    },
    MetricSpec {
        collection_kind: "kafka_broker_metrics",
        // 集群级：离线分区没有 leader，归到具体 broker 会重复计数。
        fact_key: "kafka.partition.offline",
        name: "kafka_partition_offline",
        unit: "1",
        value_type: "gauge_i64",
        const_labels: NO_CONST,
        dynamic_labels: NO_LABELS,
        semconv_ref: "",
    },
    MetricSpec {
        collection_kind: "kafka_broker_metrics",
        // 消费积压：`log_end_offset - committed_offset`，按 topic 求和。
        // 组名与 topic 都是业务标识，只能做 label（instance 仍是 `kafka:<cluster>`）。
        fact_key: "kafka.consumergroup.lag",
        name: "kafka_consumergroup_lag",
        unit: "1",
        value_type: "gauge_i64",
        const_labels: NO_CONST,
        dynamic_labels: &["group", "topic"],
        semconv_ref: "",
    },
    MetricSpec {
        collection_kind: "kafka_broker_metrics",
        // 一热：取值（`stable` / `rebalancing` / `empty` / `dead`）进 `state` label，样本值恒 1。
        fact_key: "kafka.consumergroup.state",
        name: "kafka_consumergroup_state",
        unit: "1",
        value_type: "gauge_i64",
        const_labels: NO_CONST,
        dynamic_labels: &["group", "state"],
        semconv_ref: "",
    },
    MetricSpec {
        collection_kind: "kafka_broker_metrics",
        // 可用空间：`DescribeLogDirs` v4 的 `usable_bytes` 之和。
        // 仍然是「Kafka 自己的卷」，整盘分母仍由 `system_filesystem_*` 负责。
        fact_key: "kafka.log_dir.usable_bytes",
        name: "kafka_log_dir_usable_bytes",
        unit: "By",
        value_type: "gauge_i64",
        const_labels: NO_CONST,
        dynamic_labels: NO_LABELS,
        semconv_ref: "",
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

    const COLLECTION_KINDS: &[&str] = &[
        "host_metrics",
        "process_metrics",
        "container_metrics",
        "kafka_broker_metrics",
    ];
    const UNITS: &[&str] = &["By", "s", "1"];
    const VALUE_TYPES: &[&str] = &["gauge_i64", "gauge_f64", "counter_i64", "counter_f64"];

    fn is_valid_name(name: &str) -> bool {
        if name.is_empty() || name.starts_with('_') || name.ends_with('_') || name.contains("__") {
            return false;
        }
        // 首段必须是字母开头；后续段允许窗口后缀（如 `_1m`）以数字开头。
        let mut segments = name.split('_');
        match segments.next() {
            Some(first) if first.starts_with(|c: char| c.is_ascii_lowercase()) => {}
            _ => return false,
        }
        segments.all(|segment| {
            !segment.is_empty()
                && segment
                    .chars()
                    .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit())
        })
    }

    #[test]
    fn finds_spec_by_kind_and_fact_key() {
        let spec = find_metric_spec("host_metrics", "host.loadavg.1m").expect("spec");
        assert_eq!(spec.name, "system_load_average_1m");
        assert_eq!(spec.unit, "1");
        assert_eq!(spec.value_type, "gauge_f64");
    }

    #[test]
    fn unknown_key_is_none() {
        assert!(find_metric_spec("host_metrics", "no.such.fact").is_none());
        assert!(find_metric_spec("no_such_kind", "host.loadavg.1m").is_none());
        assert!(find_metric_spec("", "").is_none());
    }

    #[test]
    fn lookup_matches_every_table_entry() {
        for spec in METRIC_SPECS {
            let found = find_metric_spec(spec.collection_kind, spec.fact_key).expect("spec");
            assert_eq!(found, spec);
        }
    }

    #[test]
    fn table_is_not_empty_and_fields_are_present() {
        assert!(!METRIC_SPECS.is_empty());
        for spec in METRIC_SPECS {
            for field in [
                spec.collection_kind,
                spec.fact_key,
                spec.name,
                spec.unit,
                spec.value_type,
            ] {
                assert!(!field.trim().is_empty(), "empty field in {spec:?}");
            }
        }
    }

    #[test]
    fn names_follow_the_snake_case_grammar() {
        for spec in METRIC_SPECS {
            assert!(is_valid_name(spec.name), "bad metric name `{}`", spec.name);
        }
    }

    #[test]
    fn no_legacy_names_remain() {
        // 旧名（点分写法 + `legacy.` fact 前缀）已整体下线：注册表只允许规范名。
        for spec in METRIC_SPECS {
            assert!(!spec.name.contains('.'), "legacy name `{}`", spec.name);
            assert!(
                !spec.fact_key.starts_with("legacy."),
                "legacy fact key `{}`",
                spec.fact_key
            );
        }
    }

    #[test]
    fn collection_kind_matches_name_namespace() {
        let namespace = |kind: &str| match kind {
            "host_metrics" => "system",
            "process_metrics" => "process",
            "container_metrics" => "container",
            "kafka_broker_metrics" => "kafka",
            other => panic!("unexpected collection kind {other}"),
        };
        for spec in METRIC_SPECS {
            let expected = namespace(spec.collection_kind);
            assert!(
                spec.name.starts_with(&format!("{expected}_")),
                "`{}` must start with `{expected}_`",
                spec.name
            );
        }
    }

    #[test]
    fn unit_and_name_suffix_agree() {
        for spec in METRIC_SPECS {
            let name = spec.name;
            match spec.unit {
                "By" => assert!(
                    name.ends_with("_bytes") || name.ends_with("_bytes_total"),
                    "unit By requires `_bytes` suffix: {name}"
                ),
                "s" => assert!(
                    name.ends_with("_seconds") || name.ends_with("_seconds_total"),
                    "unit s requires `_seconds` suffix: {name}"
                ),
                "1" => {}
                other => panic!("unknown unit `{other}` in {spec:?}"),
            }
        }
    }

    #[test]
    fn counters_end_with_total_and_gauges_do_not() {
        for spec in METRIC_SPECS {
            if spec.value_type.starts_with("counter") {
                assert!(
                    spec.name.ends_with("_total"),
                    "counter must end with `_total`: {}",
                    spec.name
                );
            } else {
                assert!(
                    !spec.name.ends_with("_total"),
                    "gauge must not end with `_total`: {}",
                    spec.name
                );
            }
        }
    }

    #[test]
    fn units_and_types_are_from_closed_sets() {
        for spec in METRIC_SPECS {
            assert!(UNITS.contains(&spec.unit), "bad unit `{}`", spec.unit);
            assert!(
                VALUE_TYPES.contains(&spec.value_type),
                "bad value type `{}`",
                spec.value_type
            );
        }
    }

    #[test]
    fn collection_kinds_are_from_closed_set() {
        for spec in METRIC_SPECS {
            assert!(
                COLLECTION_KINDS.contains(&spec.collection_kind),
                "bad collection kind `{}`",
                spec.collection_kind
            );
        }
    }

    #[test]
    fn fact_key_is_unit_neutral() {
        for spec in METRIC_SPECS {
            for banned in ["_kb", "_pages", "_percent"] {
                assert!(
                    !spec.fact_key.ends_with(banned),
                    "fact key must be unit neutral: {}",
                    spec.fact_key
                );
            }
        }
    }

    #[test]
    fn kafka_broker_metrics_keep_the_documented_names() {
        // 线名是查询与看板的契约：改名不会报错，只会让既有面板静默变空。
        for (fact_key, name) in [
            ("kafka.broker.up", "kafka_broker_up"),
            (
                "kafka.broker.is_controller",
                "kafka_broker_role_is_controller",
            ),
            ("kafka.partition.count", "kafka_partition_count"),
            (
                "kafka.partition.under_replicated",
                "kafka_partition_under_replicated",
            ),
            ("kafka.partition.offline", "kafka_partition_offline"),
            ("kafka.log_dir.size_bytes", "kafka_log_dir_size_bytes"),
            ("kafka.log_dir.usable_bytes", "kafka_log_dir_usable_bytes"),
            ("kafka.consumergroup.lag", "kafka_consumergroup_lag"),
            ("kafka.consumergroup.state", "kafka_consumergroup_state"),
        ] {
            let spec = find_metric_spec("kafka_broker_metrics", fact_key).expect("spec");
            assert_eq!(spec.name, name, "`{fact_key}` 的线名被改动了");
            if name.ends_with("_bytes") {
                assert_eq!(spec.unit, "By", "`{name}` 是字节量");
            } else {
                assert_eq!(spec.unit, "1", "`{name}` 是无量纲计数");
            }
            assert_eq!(spec.value_type, "gauge_i64", "`{name}` 是可上可下的读数");
        }
    }

    #[test]
    fn kafka_consumergroup_series_are_labeled_by_group_and_topic() {
        // lag 必须按 topic 拆开才能定位积压源；state 是枚举一热（group + state）。
        let lag = find_metric_spec("kafka_broker_metrics", "kafka.consumergroup.lag").expect("lag");
        assert_eq!(lag.dynamic_labels, &["group", "topic"]);
        let state =
            find_metric_spec("kafka_broker_metrics", "kafka.consumergroup.state").expect("state");
        assert_eq!(state.dynamic_labels, &["group", "state"]);
        // 身份仍走 instance：这两条都不参与 instance 的构成。
        assert!(lag.const_labels.is_empty());
        assert!(state.const_labels.is_empty());
    }

    #[test]
    fn registry_keys_and_names_are_unique() {
        // 同一个指标名可以因常量 label（如 `disk_role`）而有多行，
        // 因此唯一键是 `(name, const_labels)`，不是单独的名字。
        let mut keys = BTreeSet::new();
        let mut series = BTreeSet::new();
        for spec in METRIC_SPECS {
            assert!(
                keys.insert((spec.collection_kind, spec.fact_key)),
                "duplicate key {spec:?}"
            );
            assert!(
                series.insert((spec.name, spec.const_labels)),
                "duplicate series key {spec:?}"
            );
        }
    }

    #[test]
    fn one_name_has_one_unit_per_collection_kind() {
        let mut units: BTreeMap<(&str, &str), BTreeSet<&str>> = BTreeMap::new();
        for spec in METRIC_SPECS {
            units
                .entry((spec.collection_kind, spec.name))
                .or_default()
                .insert(spec.unit);
        }
        for ((kind, name), seen) in units {
            assert_eq!(seen.len(), 1, "{kind}/{name} has multiple units: {seen:?}");
        }
    }

    #[test]
    fn const_labels_are_closed_and_dynamic_labels_are_declared() {
        const ALLOWED_CONST: &[&str] = &["disk_role"];
        for spec in METRIC_SPECS {
            for (key, value) in spec.const_labels {
                assert!(ALLOWED_CONST.contains(key), "unknown const label `{key}`");
                assert!(!value.is_empty(), "empty const label value for `{key}`");
            }
            for key in spec.dynamic_labels {
                assert!(
                    matches!(
                        *key,
                        "cpu"
                            | "mode"
                            | "device"
                            | "mountpoint"
                            | "fstype"
                            | "state"
                            | "group"
                            | "topic"
                            | "host_name"
                            | "host_ip"
                            | "os"
                            | "arch"
                            | "kernel"
                    ),
                    "unknown dynamic label `{key}`"
                );
            }
        }
    }

    #[test]
    fn const_and_dynamic_labels_never_share_a_key() {
        // 同名 label 既被常量写入又来自事实，会得到两条身份相同的序列。
        for spec in METRIC_SPECS {
            for (key, _) in spec.const_labels {
                assert!(
                    !spec.dynamic_labels.contains(key),
                    "`{key}` is both const and dynamic in {}",
                    spec.name
                );
            }
        }
    }

    #[test]
    fn ratio_metrics_are_dimensionless_ratio_suffix() {
        for spec in METRIC_SPECS {
            if !spec.name.ends_with("_ratio") {
                continue;
            }
            assert_eq!(spec.unit, "1", "ratio must be dimensionless: {}", spec.name);
        }
    }
}
