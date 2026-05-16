# SQL 美化规范

## 1. 目标

本规范用于统一团队 SQL 写法，提升可读性、可维护性和代码评审效率。  
适用于项目中所有手写 SQL、SQL 模板、查询语句、批处理脚本等场景。

---

## 2. 基本原则

- SQL 关键字统一大写，如 `SELECT`、`FROM`、`WHERE`、`JOIN`、`GROUP BY`、`ORDER BY`。
- 表名、字段名、别名统一使用一种命名风格，建议使用小写加下划线。
- 一个 SQL 只表达一个清晰的业务意图，避免把多层逻辑堆在一条语句里。
- 复杂查询优先拆成 CTE 或子查询分段处理。
- 尽量避免 `SELECT *`，仅选择实际需要的字段。
- 所有 SQL 在提交前应通过格式化工具统一处理。

---

## 3. 排版规范

### 3.1 关键字换行

以下子句必须单独成行：

- `SELECT`
- `FROM`
- `JOIN`
- `ON`
- `WHERE`
- `GROUP BY`
- `HAVING`
- `ORDER BY`
- `LIMIT`

### 3.2 字段排版

- `SELECT` 后的字段建议每行一个。
- 字段较少时可保持同行；字段较多时必须分行。
- 字段之间使用统一缩进，推荐 2 空格或 4 空格，项目内保持一致。

### 3.3 条件排版

- 多个条件使用换行展示。
- `AND`、`OR` 建议放在行首，便于扫描。
- 复杂表达式、`CASE WHEN`、嵌套判断必须分行对齐。

### 3.4 JOIN 规范

- `JOIN` 与 `ON` 分开书写。
- 每个 `JOIN` 关联条件独立成块。
- 关联字段两侧应语义清晰，避免无意义的隐式关联。

---

## 4. 命名规范

### 4.1 表与字段

- 表名统一使用小写下划线风格，如 `user_order`。
- 字段名统一使用小写下划线风格，如 `create_time`、`user_id`。
- 同一项目中不要混用驼峰和下划线。

### 4.2 别名

- 表别名应简短且有语义，如 `u`、`o`、`p`。
- 别名避免纯随机字母组合。
- 派生字段必须显式命名，如 `COUNT(*) AS order_count`。

### 4.3 CTE 名称

- CTE 名称要表达阶段含义，如 `base_user`、`order_stat`、`final_result`。
- 不要使用 `tmp1`、`a1`、`t2` 这类无意义命名。

---

## 5. 查询结构规范

### 5.1 推荐结构

推荐按以下顺序组织 SQL：

1. `WITH` / CTE。
2. 主查询 `SELECT`。
3. `FROM`。
4. `JOIN`。
5. `WHERE`。
6. `GROUP BY`。
7. `HAVING`。
8. `ORDER BY`。
9. `LIMIT`。

### 5.2 复杂逻辑拆分

当 SQL 满足以下任一情况时，应拆分为 CTE：

- 嵌套层级过深。
- 过滤条件过多。
- 聚合逻辑复杂。
- 需要复用中间结果。
- 一段 SQL 难以在 30 秒内读懂。

---

## 6. 分组与排序规范

- `GROUP BY` 和 `ORDER BY` 优先使用字段名，不推荐依赖序号。
- 如团队允许 `GROUP BY 1, 2`，必须在规范中明确适用范围。
- `ORDER BY` 必须显式声明排序方向，如 `ASC` 或 `DESC`。
- 多字段排序要按业务优先级顺序书写。

---

## 7. 注释规范

- 注释只写“为什么这么写”，不要复述 SQL 表面含义。
- 复杂业务规则、临时兼容逻辑、性能优化原因应补充注释。
- 注释应简短明确，避免大段说明影响阅读。
- 建议为大段 SQL 增加分段注释，说明每一段的作用。

---

## 8. 推荐示例

### 8.1 标准写法

```sql
WITH user_base AS (
  SELECT
    u.id,
    u.name,
    u.city
  FROM user u
  WHERE u.status = 'active'
),
order_stat AS (
  SELECT
    o.user_id,
    COUNT(*) AS order_count
  FROM orders o
  GROUP BY o.user_id
)
SELECT
  ub.id,
  ub.name,
  ub.city,
  os.order_count
FROM user_base ub
LEFT JOIN order_stat os
  ON ub.id = os.user_id
WHERE ub.city IS NOT NULL
ORDER BY ub.id DESC;
```

### 8.2 条件较多的写法

```sql
SELECT
  u.id,
  u.name,
  u.status
FROM user u
WHERE u.status = 'active'
  AND u.city IS NOT NULL
  AND u.create_time >= '2026-01-01'
ORDER BY u.create_time DESC
LIMIT 100;
```

### 8.3 CASE WHEN 写法

```sql
SELECT
  u.id,
  u.name,
  CASE
    WHEN u.status = 'active' THEN '有效'
    WHEN u.status = 'disabled' THEN '禁用'
    ELSE '未知'
  END AS status_desc
FROM user u;
```

---

## 9. 禁止项

以下写法默认不推荐或禁止：

- `SELECT *`。
- 关键字大小写混用。
- 多个字段、条件、JOIN 条件写在同一行。
- 使用无意义别名，如 `a1`、`b2`、`tmp`。
- 依赖 `ORDER BY 1`、`GROUP BY 2` 等不直观写法。
- 超长 SQL 不做分段处理。
- 在业务 SQL 中混入大量临时调试内容。

---

## 10. 代码评审检查清单

- SQL 关键字是否统一大写。
- 字段、条件、JOIN 是否按规则分行。
- 是否存在 `SELECT *`。
- 别名是否清晰、统一、可读。
- 复杂 SQL 是否已拆分为 CTE。
- 分组、排序是否显式、稳定。
- 是否有必要的业务注释。
- 是否可以通过格式化工具自动统一。

---

## 11. 落地要求

- 所有新写 SQL 必须遵循本规范。
- 旧 SQL 可按迭代优先级逐步改造。
- 提交前应执行 SQL 格式化工具。
- 代码评审中，如发现明显不符合规范的 SQL，应要求修改。
- 团队可补充不同数据库方言的特殊约定，但不得覆盖本规范的基础规则。

---

## 12. 维护说明

本规范由团队统一维护，新增规则需满足以下要求：

- 具有明确的可执行性。
- 能被格式化工具或 review 检查。
- 不与现有规则冲突。
- 新增示例应优先覆盖高频业务场景。

---

## 13. 附录：简版口诀

- 关键字大写。
- 字段分行。
- 条件对齐。
- 别名清晰。
- 复杂拆分。
- 少用星号。
- 统一格式。