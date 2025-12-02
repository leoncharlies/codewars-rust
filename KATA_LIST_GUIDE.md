# 题目列表可视化系统

## 📖 概述

这个系统提供了一个可视化的方式来查看和管理你所有做过的 Codewars 题目。

## 📁 文件说明

### 1. `.kata_index` - 题目索引（机器可读）
- **格式**: `kata_id|题目名称|难度|路径|完成日期`
- **用途**: 内部索引，供脚本使用
- **生成**: 由 `add_kata.sh` 和 `sync_kata_list.sh` 自动维护
- **是否提交**: 建议提交到 Git

### 2. `KATA_LIST.md` - 题目列表（人类可读）
- **格式**: Markdown 格式，带有统计信息和进度条
- **用途**: 可视化展示所有题目
- **生成**: 由 `sync_kata_list.sh` 自动生成
- **是否提交**: 建议提交到 Git
- **特点**:
  - 📊 统计概览（总数、复习次数、平均值）
  - 📈 难度分布图表
  - 📝 详细题目列表（带链接）
  - 🔄 复习记录追踪
  - 🎯 学习进度展示

### 3. `sync_kata_list.sh` - 同步脚本
- **功能**:
  1. 扫描 `katas/` 下所有题目
  2. 重建 `.kata_index` 索引
  3. 生成 `KATA_LIST.md` 可视化文件
- **使用**: `./sync_kata_list.sh`

---

## 🚀 使用方法

### 自动同步（推荐）

添加题目或复习时会**自动同步**：

```bash
# 添加新题目 - 自动同步
./add_kata.sh <kata_id>

# 添加复习 - 自动同步
./add_kata.sh <kata_id> --review
```

### 手动同步

如果你手动修改了题目，需要手动同步：

```bash
./sync_kata_list.sh
```

**输出示例**：
```
开始同步题目列表...
1. 重建题目索引...
   已备份旧索引
   ✓ 索引已更新，共 12 题
2. 收集统计信息...
3. 生成 KATA_LIST.md...
   ✓ KATA_LIST.md 已生成

✅ 同步完成！

📊 统计信息:
   总题目数: 12
   总复习次数: 3
   已复习题目: 1

📖 查看: cat KATA_LIST.md
```

### 查看题目列表

```bash
# 在终端查看
cat KATA_LIST.md

# 或者用 Markdown 编辑器打开
code KATA_LIST.md  # VS Code
# 或在 GitHub/GitLab 上查看（自动渲染）
```

---

## 📊 KATA_LIST.md 内容说明

### 1. 统计概览
显示总题目数、复习次数、平均复习次数

```markdown
## 📊 统计概览

- **总题目数**: 12
- **总复习次数**: 3
- **平均复习次数**: 0.25
```

### 2. 难度分布
以表格和进度条形式显示

```markdown
| 难度 | 数量 | 占比 | 进度条 |
|------|------|------|--------|
| 4kyu | 3 | 25% | ██░░░░░░░░ |
| 5kyu | 6 | 50% | █████░░░░░ |
```

### 3. 题目列表
按难度分组，每个题目包含：
- 题目名称（有复习的会显示 🔄 标记）
- Kata ID
- Codewars 链接（可点击）
- 完成日期
- 本地路径
- 复习次数
- 复习记录（如果有）

```markdown
#### 6. product of consecutive fib numbers 🔄
- **Kata ID**: 5541f58a944b85ce6d00006a
- **链接**: https://www.codewars.com/kata/5541f58a944b85ce6d00006a
- **完成日期**: 2025-12-02
- **路径**: `katas/5kyu/005_product_of_consecutive_fib_numbers_5541f58a944b85ce6d00006a`
- **复习次数**: 3
- **复习记录**:
  - Review #1: 2025-12-02
  - Review #2: 2025-12-02
  - Review #3: 2025-12-02
```

### 4. 学习进度
显示各难度完成情况和复习统计

```markdown
## 🎯 学习进度

### 按难度完成情况

- ✅ 4kyu: 3 题完成
- ✅ 5kyu: 6 题完成

### 复习情况

- 📌 已复习题目: 1 题
- 📈 总复习次数: 3 次
- 🎖️ 最常复习: product of consecutive fib numbers (3次)
```

---

## 🔧 维护和检查

### 检查索引是否完整

```bash
# 方法 1: 比对数量
DISK_COUNT=$(find katas -name "lib.rs" -type f | wc -l)
INDEX_COUNT=$(grep -v "^#" .kata_index | grep -v "^$" | wc -l)
echo "磁盘题目: $DISK_COUNT"
echo "索引题目: $INDEX_COUNT"

# 方法 2: 直接重建（最安全）
./sync_kata_list.sh
```

### 手动重建索引

如果索引损坏或丢失：

```bash
# 备份旧索引（如果存在）
cp .kata_index .kata_index.backup

# 重建
./sync_kata_list.sh
```

脚本会：
1. 自动备份旧索引为 `.kata_index.bak`
2. 扫描所有 `katas/` 下的题目
3. 从 `lib.rs` 提取信息
4. 重建索引和列表

### 验证数据完整性

```bash
# 检查是否有题目没在索引中
for kata_dir in katas/*/*; do
  if [ -d "$kata_dir" ] && [ -f "$kata_dir/src/lib.rs" ]; then
    KATA_ID=$(grep "^//! Kata ID:" "$kata_dir/src/lib.rs" | sed 's/^\/\/! Kata ID: //' | tr -d ' ')
    if ! grep -q "^$KATA_ID|" .kata_index; then
      echo "缺失: $kata_dir"
    fi
  fi
done
```

如果发现缺失，运行 `./sync_kata_list.sh` 即可修复。

---

## 🎨 自定义

### 修改显示格式

编辑 `sync_kata_list.sh`，在生成 `KATA_LIST.md` 的部分修改：

```bash
# 大约在第 100-200 行
echo "#### $item_num. $name$review_marker" >> "$OUTPUT_FILE"
# ... 可以添加更多字段或改变顺序
```

### 添加新统计项

在脚本的统计部分添加新的计算逻辑：

```bash
# 例如：统计本周完成的题目
THIS_WEEK=$(grep "2025-12-" .kata_index | wc -l)
echo "- **本周完成**: $THIS_WEEK" >> "$OUTPUT_FILE"
```

---

## 📋 常见场景

### 场景 1: 分享你的刷题记录

```bash
# KATA_LIST.md 可以直接分享或推送到 GitHub
git add KATA_LIST.md .kata_index
git commit -m "Update kata list"
git push
```

在 GitHub 上，Markdown 会自动渲染，非常美观！

### 场景 2: 查找某个题目

```bash
# 在列表中搜索
grep -i "fibonacci" KATA_LIST.md

# 或使用管理工具
./manage_kata.sh search fibonacci
```

### 场景 3: 统计学习进度

```bash
# 查看统计
./manage_kata.sh stats

# 或查看详细的 KATA_LIST.md
cat KATA_LIST.md | head -30
```

### 场景 4: 导出到其他格式

```bash
# 转换为 HTML（需要 pandoc）
pandoc KATA_LIST.md -o kata_list.html

# 转换为 PDF（需要 pandoc 和 LaTeX）
pandoc KATA_LIST.md -o kata_list.pdf
```

---

## ⚙️ 自动化建议

### 定期同步

添加到 cron 或 Git hooks：

```bash
# .git/hooks/pre-commit
#!/bin/bash
./sync_kata_list.sh > /dev/null 2>&1
git add KATA_LIST.md .kata_index
```

### 每日统计

创建一个每日脚本：

```bash
#!/bin/bash
TODAY=$(date +%Y-%m-%d)
TODAY_COUNT=$(grep "$TODAY" .kata_index | wc -l)
echo "今日完成: $TODAY_COUNT 题"
```

---

## 🐛 故障排查

### 问题：索引和实际题目数量不符

**解决**：
```bash
./sync_kata_list.sh
```

### 问题：KATA_LIST.md 没有自动更新

**检查**：
```bash
# 确认脚本可执行
ls -l sync_kata_list.sh

# 手动运行
./sync_kata_list.sh
```

### 问题：某个题目信息缺失

**原因**：`lib.rs` 中的元信息格式不正确

**修复**：
1. 打开对应的 `lib.rs`
2. 确保有这些行：
   ```rust
   //! Title: xxx
   //! Kata ID: xxx
   //! Rank: xkyu
   //! Completed: 2025-12-02
   ```
3. 运行 `./sync_kata_list.sh`

---

## 💡 提示

1. **定期同步**：建议每完成几道题就运行一次 `./sync_kata_list.sh`
2. **提交到 Git**：`KATA_LIST.md` 和 `.kata_index` 都建议提交，方便跨设备同步
3. **备份**：索引文件会自动备份为 `.kata_index.bak`
4. **性能**：即使有上百道题，同步也只需几秒钟

---

## 📚 相关文档

- [README.md](README.md) - 完整使用文档
- [DUPLICATE_PROTECTION.md](DUPLICATE_PROTECTION.md) - 重复检测说明
- [QUICKSTART.md](QUICKSTART.md) - 快速开始

---

*生成时间: 2025-12-02*  
*版本: 1.0*
