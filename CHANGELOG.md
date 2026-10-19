# 更新日志

本文件记录 `wist-metrics` 的所有重要变更。格式遵循 [Keep a Changelog](https://keepachangelog.com/en/1.1.0/)，
版本号遵循[语义化版本](https://semver.org/lang/zh-CN/)。

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
