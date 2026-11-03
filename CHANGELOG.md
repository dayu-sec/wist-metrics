# 更新日志

本文件记录 `wist-metrics` 的所有重要变更。格式遵循 [Keep a Changelog](https://keepachangelog.com/zh-CN/1.1.0/)，
版本号遵循[语义化版本](https://semver.org/lang/zh-CN/)。

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
