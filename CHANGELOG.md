# 更新日志

本文件记录 `wist-metrics` 的所有重要变更。格式遵循 [Keep a Changelog](https://keepachangelog.com/zh-CN/1.1.0/)，
版本号遵循[语义化版本](https://semver.org/lang/zh-CN/)。

## [0.13.0] - 2026-11-04

### 新增

- `names::label` 新增主机身份 label：`HOST_NAME` / `HOST_IP` / `OS` / `ARCH` / `KERNEL`，
  并纳入 `ALL_LABELS` 闭合表。
- `spec::METRIC_SPECS` 新增主机身份 info 指标 `system_host_info`（fact `host.info`，值恒为 1）：
  `host_name` / `host_ip` / `os` / `arch` / `kernel` 只在这一条序列上，测量序列身份不变。

### 为什么

主机名 / IP / 系统 / 架构 / 内核是「这是哪台机器」的必需信息，但以前只存在于控制面注册表（且
注册表只存 `node_id` / `hostname` / `ip_addresses`，`os` / `arch` / 内核根本没人存）。
把这些值平铺到 `system_*` 测量序列上会让历史在改名/换 IP/内核升级时断成两段，所以集中到一条
info 指标：变化只替换这一条序列。

## [0.12.0] - 2026-11-04

### 新增

- `kafka_log_dir_size_bytes`：broker 全部 log dir 的合计大小（V1.1 的容量面）。

### 为什么

「快写满了吗」需要容量。`DescribeLogDirs` v0 只给每个分区的 `size`，不给磁盘总量；
**分母不在这里编造** —— 同一节点上的 `system_filesystem_usage_ratio{mountpoint=…}`
本来就是权威口径（agentd 已在采）。Kafka 侧的职责只是回答「这个 broker 占了多少」。

## [0.11.0] - 2026-11-04

### 新增

- 注册 `kafka_broker_metrics` 采集域与 5 个指标声明：`kafka_broker_up` /
  `kafka_broker_role_is_controller` / `kafka_partition_count` /
  `kafka_partition_under_replicated` / `kafka_partition_offline`。
- 命名空间闭集增加 `kafka`（规范 §2.2）。

### 为什么

Kafka 指标改由 agentd 的协议探针采集（不依赖 Prometheus / exporter）。
broker 身份走 `instance = kafka:<cluster_key>:<broker_id>`，不新增 label ——
与规范 §3「`instance` 是被监控对象的稳定身份」一致，也不给 VM 线协议加一条只有 Kafka 用的键。

## [0.10.0] - 2026-11-03

### 新增

- `names::agent` 补齐全部自观测指标：`UPLINK_METRICS_FAILED_TOTAL` / `COLLECT_TARGETS`，
  并新增闭合表 `ALL_SELF_METRICS`（写入端用它断言「产出的名字都在这张表里」）。
- `names::label` 补齐注册表里出现的动态 label 键（`device` / `cpu` / `mode` / `state` /
  `target_kind` / `outcome`）与闭合表 `ALL_LABELS`。

### 为什么

上一版只收了趋势需要的五个名字，agentd 写侧仍在硬编码 `agent_uplink_metrics_failed_total` /
`agent_collect_targets` 与九个 label 键。补齐后写入端可以全面引用常量，由 `ALL_SELF_METRICS` /
`ALL_LABELS` 做闭合断言——新增名字时测试先红，而不是上线后“多了一条没人认识、也没人查的序列”。

## [0.9.0] - 2026-11-03

### 新增

- **`names` 模块：规范指标名与 label 键的唯一来源**。新增 `names::agent`（agent 自观测
  `agent_memory_bytes` / `agent_cpu_ratio` / `agent_cpu_cores` / `agent_admin_latency_seconds` /
  `agent_discovery_policy_version` 与趋势清单 `HISTORY`）、`names::label`（`agent` / `instance` /
  `disk_role` / `mountpoint` / `fstype`），以及单位换算 `ratio_to_percent` / `percent_to_ratio` /
  `seconds_to_millis` / `millis_to_seconds`。

### 为什么

写侧与读侧各自手写名字会漂：`wist-agentd` 0.21 改成规范下划线名后，管理面读趋势仍在查旧点分名
（`agent.cpu.percent`），两侧单测都能过，而“Agent 总览”的资源曲线一直是空的。名字收进本 crate 后，
改名只能改到这里；读取方（`wist-gateway` / `wist-center`）与写入方（`wist-agentd`）引用同一份常量。

## [0.8.0] - 2026-11-03

### 变更（不兼容）

- **对齐 `wist-contracts` 0.10**：注册表里的类型来自 contracts，消费方需同批升级，
  否则 cargo 会同时保留两份 contracts、跨边界类型对不上（mismatched types）。

## [0.7.0] - 2026-11-03

### 变更（不兼容）

- **删除旧名注册表**：`LEGACY_METRIC_SPECS` 整体移除，`find_metric_spec` 只返回规范名。
  旧名（`system.memory.total` 这类点分写法）不再被注册表认知，采集侧必须停止产出
  `legacy.*` fact。升级后旧看板的对应曲线停止新增。

## [0.6.0] - 2026-11-03

### 新增

- **指标注册表按指标规范 v1 收敛**：`MetricSpec` 提供 `name` / `unit` / `value_type` /
  `const_labels` / `dynamic_labels` / `semconv_ref`，消费方可以直接用注册表校验
  线上序列的名字、单位与 label，不用再各自维护一份名单。
- **`LEGACY_METRIC_SPECS` 旧名表**：双写期需要的旧名集中登记（`fact_key` 带 `legacy.` 前缀、
  保留旧单位），与规范名表物理隔离。
- **`MetricsCollectionOutcome.dropped_targets`**：超出采样上限的目标数，
  让“图里只有三个进程”这类截断从数据侧可见。

### 变更（不兼容）

- 指标名改为 Prometheus 形态（`snake_case` + 单位后缀，counter 以 `_total` 结尾），
  比例统一为 `*_ratio`（0–1）。按旧名查询或匹配的消费方需要同步切换。
- 用结构体字面量构造 `MetricsCollectionOutcome` 的消费方需要补 `dropped_targets`
  （反序列化旧 JSON 不受影响）。

## [0.5.0] - 2026-10-19

### 新增

- **带标签的采集事实 `MetricsLabeledFact`**：`key` / `value` / `labels`，用于同一 fact key
  下的多个实例（如不同挂载点）。`MetricsCollectionOutcome` 与
  `MetricsCollectionTargetSample` 新增 `labeled_facts`（可选，旧 JSON 兼容）。
- **逐挂载点容量指标 spec**：`host.filesystem.{usage_percent,total_kb,available_kb}` →
  `system.filesystem.{usage,total,available}`。

### 变更（不兼容）

- 两个结果结构体新增字段，使用结构体字面量构造的消费方需要补 `labeled_facts`。

## [0.4.0] - 2026-10-18

### 变更（不兼容）

- **对齐 `wist-contracts` 0.9**：依赖升到 `0.9`。消费方需同样升到 `wist-contracts` 0.9
  （否则 cargo 会同时保留 0.8 / 0.9，跨边界类型对不上）。

## [0.3.0] - 2026-10-11

### 变更（不兼容）

- **对齐 `wist-contracts` 0.8**（agent 本地运行配置移出，删 `agent_config` 模块）。本 crate 的指标
  spec / provider 抽象不变；但公开 API 里用 `wist_contracts` 的领域类型，**消费方需同样升到
  `wist-contracts` 0.8**。

## [0.2.0] - 2026-10-09

### 变更（不兼容）

- **对齐 `wist-contracts` 0.7**：依赖由 `0.3` 升到 `0.7`。本 crate 在公开 API 里用
  `wist_contracts::discovery::StringKeyValue`，**消费方需同样升到 `wist-contracts` 0.7** ——
  否则依赖图里出现两版 `wist_contracts`，跨边界类型对不上。

## [0.1.6] - 2026-10-05

### 变更

- 依赖 `wist-contracts` 由 `0.2` 升到 **`0.3`**：对齐「agent 注册/续期报文迁到 `wist-api`」后的
  契约版本。本 crate 的指标 spec / provider 抽象不变，使用者无需改动调用。
