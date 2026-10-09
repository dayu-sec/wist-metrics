# 更新日志

本文件记录 `wist-metrics` 的所有重要变更。格式遵循 [Keep a Changelog](https://keepachangelog.com/en/1.1.0/)，
版本号遵循[语义化版本](https://semver.org/lang/zh-CN/)。

## [0.2.0] - 2026-10-09

### 变更（不兼容）

- **对齐 `wist-contracts` 0.7**：依赖由 `0.3` 升到 `0.7`。本 crate 在公开 API 里用
  `wist_contracts::discovery::StringKeyValue`，**消费方需同样升到 `wist-contracts` 0.7** ——
  否则依赖图里出现两版 `wist_contracts`，跨边界类型对不上。

## [0.1.6] - 2026-10-05

### 变更

- 依赖 `wist-contracts` 由 `0.2` 升到 **`0.3`**：对齐「agent 注册/续期报文迁到 `wist-api`」后的
  契约版本。本 crate 的指标 spec / provider 抽象不变，使用者无需改动调用。
