# Features

`features/` 按业务域组织模块，是后续业务功能的唯一落点。当前仅包含 `system`
模块，用于展示运行环境，不包含任何时钟或收益计算逻辑。

## 目录约定

```text
features/
└── <feature-name>/
    ├── components/      # 该业务域私有的展示组件
    ├── composables/     # 该业务域私有的组合式函数（按需创建）
    ├── stores/          # 该业务域私有的 Pinia store（按需创建）
    ├── services/        # 该业务域私有的数据访问逻辑（按需创建）
    ├── types.ts         # 该业务域的领域类型
    └── index.ts         # 对外公开的入口，仅导出稳定 API
```

## 约束

1. 业务组件不得直接导入 `@tauri-apps/api`，系统能力统一走 `src/services`。
2. 跨业务域复用的展示组件放到 `src/components`，不要互相引用 feature 内部文件。
3. 每个 feature 只通过自身的 `index.ts` 对外暴露 API。
4. 业务状态放在 feature 自己的 store 中，不要写入 `stores/app.store.ts`。
