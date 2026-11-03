//! 规范指标名 / label 键 / 单位换算的**常量表**。
//!
//! 为什么要有这个模块：名字漂移是「线上页面空白、而两侧单测都能过」的那类故障。真实事故：
//! `wist-agentd` 0.21 把 agent 自观测指标改成规范下划线名（`agent_cpu_ratio`），管理面读趋势
//! 时还在查旧点分名（`agent.cpu.percent`），于是「Agent 总览」的资源曲线一直是空的 —— 写侧与
//! 读侧各自都有测试，但没人比较过两个名字。把名字收进**共用的 crate** 是唯一能从结构上消灭
//! 这类漂移的办法：改名只可能改到这里。
//!
//! 分工：
//! - [`spec::METRIC_SPECS`](crate::spec::METRIC_SPECS)：fact 派生指标（`system_*` / `process_*` /
//!   `container_*`）的「fact_key → 名字/单位/类型」映射，供采集与规范化使用；
//! - 本模块：**不是** fact 派生的名字（agent 自观测 `agent_*`）以及序列 label 键。这些名字
//!   由「写入端（agentd）」与「读取端（网关 / 中心）」共同引用，不走 spec 表。

/// agent 自观测指标（规范 `metrics-data-spec.md` §self-observation）。
///
/// 单位口径（规范 §4）：内存是 `By`，CPU 是**单核比例**（`1.0` = 占满一个核，多线程进程可 >1），
/// 时延是 `s`，核数是 `1`。这些指标**没有** `instance` label —— 它们描述 agent 进程自身。
pub mod agent {
    /// 进程常驻内存（`By`）。
    pub const MEMORY_BYTES: &str = "agent_memory_bytes";
    /// CPU 占用（单核比例，`1`）。
    pub const CPU_RATIO: &str = "agent_cpu_ratio";
    /// 逻辑核数（`1`）：把单核比例换算成整机占比的分母。
    pub const CPU_CORES: &str = "agent_cpu_cores";
    /// 到管理面的往返时延（`s`）。
    pub const ADMIN_LATENCY_SECONDS: &str = "agent_admin_latency_seconds";
    /// 当前生效的发现策略版本（`1`，阶标，不进趋势图）。
    pub const DISCOVERY_POLICY_VERSION: &str = "agent_discovery_policy_version";

    /// 趋势图要拉的四个名字（策略版本是阶标，不进趋势）。
    pub const HISTORY: &[&str] = &[MEMORY_BYTES, CPU_RATIO, CPU_CORES, ADMIN_LATENCY_SECONDS];
}

/// 序列 label 键（规范 §3.3：`agent` 是 agent 身份，**不得**与 `agent_id` 混用）。
pub mod label {
    /// agent 身份（= 注册表里的 `agent_id` 值）。序列上只用这一个键。
    pub const AGENT: &str = "agent";
    /// 目标对象的稳定身份（主机资源 id / 进程身份 / 容器 id）；agent 自观测指标没有它。
    pub const INSTANCE: &str = "instance";
    /// 磁盘角色（`system` / `data` …），规范化层写入的常量 label。
    pub const DISK_ROLE: &str = "disk_role";
    /// 文件系统挂载点（绝对路径）。
    pub const MOUNTPOINT: &str = "mountpoint";
    /// 文件系统类型。
    pub const FSTYPE: &str = "fstype";
}

/// 单核比例 → 百分比（页面口径）。`0.5` → `50.0`。
pub fn ratio_to_percent(ratio: f64) -> f64 {
    ratio * 100.0
}

/// 百分比 → 单核比例（写入口径）。
pub fn percent_to_ratio(percent: f64) -> f64 {
    percent / 100.0
}

/// 秒 → 毫秒（页面口径），四舍五入，不截断（`0.0153s` → `15ms` 而不是 `15.3` 被切掉）。
pub fn seconds_to_millis(seconds: f64) -> u64 {
    (seconds * 1000.0).round() as u64
}

/// 毫秒 → 秒（写入口径）。
pub fn millis_to_seconds(millis: u64) -> f64 {
    millis as f64 / 1000.0
}

#[cfg(test)]
mod tests {
    use super::*;

    /// 名字必须是规范形状：小写 + 下划线，且**没有**点分名残留（旧名曾把读取侧拖成静默空白）。
    #[test]
    fn self_observation_names_are_snake_case_and_not_dotted() {
        for name in agent::HISTORY
            .iter()
            .chain(std::iter::once(&agent::DISCOVERY_POLICY_VERSION))
        {
            assert!(!name.contains('.'), "{name} 不得是点分名");
            assert!(
                name.chars()
                    .all(|ch| ch.is_ascii_lowercase() || ch.is_ascii_digit() || ch == '_'),
                "{name} 必须是 snake_case"
            );
            assert!(name.starts_with("agent_"), "{name} 应以 agent_ 开头");
        }
    }

    /// 趋势清单与单独常量保持一致（漏掉一项 = 图上少一条线，且不会有任何报错）。
    #[test]
    fn history_lists_exactly_the_four_trend_metrics() {
        assert_eq!(agent::HISTORY.len(), 4);
        assert!(agent::HISTORY.contains(&agent::MEMORY_BYTES));
        assert!(agent::HISTORY.contains(&agent::CPU_RATIO));
        assert!(agent::HISTORY.contains(&agent::CPU_CORES));
        assert!(agent::HISTORY.contains(&agent::ADMIN_LATENCY_SECONDS));
        assert!(!agent::HISTORY.contains(&agent::DISCOVERY_POLICY_VERSION));
    }

    #[test]
    fn unit_helpers_round_trip() {
        assert_eq!(ratio_to_percent(0.5), 50.0);
        assert_eq!(percent_to_ratio(50.0), 0.5);
        assert_eq!(seconds_to_millis(0.25), 250);
        assert_eq!(seconds_to_millis(0.0153), 15);
        assert_eq!(millis_to_seconds(250), 0.25);
    }

    /// `agent` 是唯一的身份 label 键；`agent_id` 只作为**报文**字段存在，不得出现在序列上。
    #[test]
    fn agent_identity_label_is_not_agent_id() {
        assert_eq!(label::AGENT, "agent");
        assert_ne!(label::AGENT, "agent_id");
    }
}
