# 快速开始指南

## 安装

克隆或创建此工作空间后，确保已安装 Rust：

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

## 第一个题目

### 1. 从 Codewars 获取题目 ID

访问你想做的题目，例如：
`https://www.codewars.com/kata/5541f58a944b85ce6d00006a`

题目 ID 就是 URL 的最后一段：`5541f58a944b85ce6d00006a`

### 2. 添加题目

```bash
./add_kata.sh 5541f58a944b85ce6d00006a
```

输出示例：
```
正在获取题目信息...
创建新题目: katas/5kyu/006_product_of_consecutive_fib_numbers_5541f58a944b85ce6d00006a
✓ 题目创建成功!
  路径: katas/5kyu/006_product_of_consecutive_fib_numbers_5541f58a944b85ce6d00006a
  题目: Product of consecutive Fib numbers
  难度: 5kyu
```

### 3. 编辑代码

打开生成的文件：
```bash
$EDITOR katas/5kyu/006_product_of_consecutive_fib_numbers_5541f58a944b85ce6d00006a/src/lib.rs
```

### 4. 粘贴测试用例

从 Codewars 复制测试代码到 `#[cfg(test)] mod tests` 模块中。

### 5. 实现解决方案

编写你的代码实现。

### 6. 运行测试

```bash
cargo test -p kata_006_product_of_consecutive_fib_numbers_5541f58a944b85ce6d00006a
```

### 7. 提交到 Codewars

测试通过后，复制你的代码到 Codewars 并提交。

## 复习题目

当你想复习已完成的题目时：

```bash
./add_kata.sh 5541f58a944b85ce6d00006a --review
```

这会创建 `review_1.rs`，你可以在不修改原始代码的情况下重新实现。

## 管理题目

查看所有题目：
```bash
./manage_kata.sh list
```

查看特定难度：
```bash
./manage_kata.sh list 5kyu
```

搜索题目：
```bash
./manage_kata.sh search fibonacci
```

查看统计：
```bash
./manage_kata.sh stats
```

查看题目详情：
```bash
./manage_kata.sh info 5541f58a944b85ce6d00006a
```

## 常见问题

**Q: 如何测试特定的测试函数？**
```bash
cargo test -p <package_name> <test_name>
```

**Q: 如何只测试某个复习版本？**
```bash
cargo test -p <package_name> review_1
```

**Q: 题目需要外部依赖怎么办？**

编辑题目的 `Cargo.toml`，在 `[dependencies]` 下添加：
```toml
[dependencies]
serde = "1.0"
```

**Q: 如何删除题目？**

直接删除对应的目录即可：
```bash
rm -rf katas/5kyu/006_product_of_consecutive_fib_numbers_5541f58a944b85ce6d00006a
```

**Q: API 获取失败怎么办？**

检查网络连接，或者手动创建目录和文件。

## 提示

- 使用 `cargo test` 运行所有测试
- 使用 `cargo test --workspace` 确保整个工作空间没问题
- 定期运行 `cargo clean` 清理构建缓存
- 使用 `rustfmt` 格式化代码：`cargo fmt`
- 使用 `clippy` 检查代码：`cargo clippy`

祝你刷题愉快！🦀
