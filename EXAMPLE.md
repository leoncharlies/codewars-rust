# 使用示例

## 示例 1：添加并完成一个新题目

假设你想做这道题：https://www.codewars.com/kata/5541f58a944b85ce6d00006a

### 步骤 1：添加题目

```bash
./add_kata.sh 5541f58a944b85ce6d00006a
```

输出：
```
正在获取题目信息...
创建新题目: katas/5kyu/006_product_of_consecutive_fib_numbers_5541f58a944b85ce6d00006a
✓ 题目创建成功!
  路径: katas/5kyu/006_product_of_consecutive_fib_numbers_5541f58a944b85ce6d00006a
  题目: Product of consecutive Fib numbers
  难度: 5kyu
```

### 步骤 2：查看生成的模板

```bash
cat katas/5kyu/006_product_of_consecutive_fib_numbers_5541f58a944b85ce6d00006a/src/lib.rs
```

看到：
```rust
//! Title: Product of consecutive Fib numbers
//! Link: https://www.codewars.com/kata/5541f58a944b85ce6d00006a
//! Kata ID: 5541f58a944b85ce6d00006a
//! Rank: 5kyu
//! Completed: 2025-12-02

pub fn solution() {
    unimplemented!()
}

#[cfg(test)]
mod tests {
    use super::*;

    // 在这里粘贴从 Codewars 复制的测试代码
}
```

### 步骤 3：从 Codewars 复制测试用例

假设 Codewars 给了这样的测试：

```rust
#[test]
fn basic_tests() {
    assert_eq!(product_fib(4895), (55, 89, true));
    assert_eq!(product_fib(5895), (89, 144, false));
}
```

### 步骤 4：编辑文件，粘贴测试并实现

```rust
//! Title: Product of consecutive Fib numbers
//! Link: https://www.codewars.com/kata/5541f58a944b85ce6d00006a
//! Kata ID: 5541f58a944b85ce6d00006a
//! Rank: 5kyu
//! Completed: 2025-12-02

pub fn product_fib(prod: u64) -> (u64, u64, bool) {
    let (mut a, mut b) = (0, 1);
    while a * b < prod {
        let temp = a + b;
        a = b;
        b = temp;
    }
    (a, b, a * b == prod)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_tests() {
        assert_eq!(product_fib(4895), (55, 89, true));
        assert_eq!(product_fib(5895), (89, 144, false));
    }
}
```

### 步骤 5：运行测试

```bash
cargo test -p kata_006_product_of_consecutive_fib_numbers_5541f58a944b85ce6d00006a
```

输出：
```
running 1 test
test tests::basic_tests ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

### 步骤 6：提交到 Codewars

测试通过！复制代码到 Codewars 提交。

---

## 示例 2：复习已完成的题目

三个月后，Codewars 建议你复习这道题...

### 步骤 1：创建复习版本

```bash
./add_kata.sh 5541f58a944b85ce6d00006a --review
```

输出：
```
正在获取题目信息...
创建复习版本: review_1.rs
✓ 复习版本创建成功: katas/5kyu/006_product_of_consecutive_fib_numbers_5541f58a944b85ce6d00006a/src/review_1.rs
运行测试: cargo test -p kata_006_product_of_consecutive_fib_numbers_5541f58a944b85ce6d00006a review_1
```

### 步骤 2：查看目录结构

```bash
ls katas/5kyu/006_product_of_consecutive_fib_numbers_5541f58a944b85ce6d00006a/src/
```

看到：
```
lib.rs  review_1.rs
```

原始代码还在 `lib.rs`，新的复习文件是 `review_1.rs`。

### 步骤 3：编辑复习文件

打开 `review_1.rs`，看到空模板：

```rust
//! Review #1
//! Date: 2025-12-02

pub fn solution() {
    unimplemented!()
}

#[cfg(test)]
mod tests {
    use super::*;

    // 在这里粘贴测试代码
}
```

从 `lib.rs` 复制测试用例，然后用不同的方法实现：

```rust
//! Review #1
//! Date: 2025-12-02

// 这次用递归方式实现
pub fn product_fib(prod: u64) -> (u64, u64, bool) {
    fn fib_pair(a: u64, b: u64, prod: u64) -> (u64, u64, bool) {
        match a * b {
            p if p == prod => (a, b, true),
            p if p > prod => (a, b, false),
            _ => fib_pair(b, a + b, prod),
        }
    }
    fib_pair(0, 1, prod)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_tests() {
        assert_eq!(product_fib(4895), (55, 89, true));
        assert_eq!(product_fib(5895), (89, 144, false));
    }
}
```

### 步骤 4：测试复习版本

```bash
cargo test -p kata_006_product_of_consecutive_fib_numbers_5541f58a944b85ce6d00006a review_1
```

通过！现在你有两种不同的实现可以对比学习。

---

## 示例 3：管理大量题目

### 查看所有题目

```bash
./manage_kata.sh list
```

### 只看 5kyu 的题目

```bash
./manage_kata.sh list 5kyu
```

### 搜索包含 "fib" 的题目

```bash
./manage_kata.sh search fib
```

输出：
```
搜索包含 'fib' 的题目...

  [5kyu] Product of consecutive Fib numbers
      ID: 5541f58a944b85ce6d00006a
      路径: katas/5kyu/006_product_of_consecutive_fib_numbers_5541f58a944b85ce6d00006a

找到 1 个匹配的题目
```

### 查看统计信息

```bash
./manage_kata.sh stats
```

输出：
```
=== Codewars Kata 统计 ===

  3kyu: 2 题
  4kyu: 5 题
  5kyu: 12 题
  6kyu: 3 题

总题目数: 22
总复习次数: 5
平均复习次数: 0.23
```

### 查看题目详情

```bash
./manage_kata.sh info 5541f58a944b85ce6d00006a
```

输出：
```
=== 题目信息 ===

//! Title: Product of consecutive Fib numbers
//! Link: https://www.codewars.com/kata/5541f58a944b85ce6d00006a
//! Kata ID: 5541f58a944b85ce6d00006a
//! Rank: 5kyu
//! Completed: 2025-12-02

路径: katas/5kyu/006_product_of_consecutive_fib_numbers_5541f58a944b85ce6d00006a
包名: kata_006_product_of_consecutive_fib_numbers_5541f58a944b85ce6d00006a

复习版本:
  - review_1.rs
//! Date: 2025-12-02

测试命令:
  cargo test -p kata_006_product_of_consecutive_fib_numbers_5541f58a944b85ce6d00006a
```

---

## 高级使用

### 测试所有 5kyu 题目

```bash
cargo test --workspace | grep "5kyu"
```

### 同时测试多个复习版本

```bash
cargo test -p kata_006_product_of_consecutive_fib_numbers_5541f58a944b85ce6d00006a
```

这会测试 `lib.rs` 和所有 `review_N.rs`。

### 添加外部依赖

编辑题目的 `Cargo.toml`：

```toml
[package]
name = "kata_006_product_of_consecutive_fib_numbers_5541f58a944b85ce6d00006a"
version.workspace = true
edition.workspace = true

[dependencies]
itertools = "0.12"
```

然后在代码中使用：

```rust
use itertools::Itertools;
```

---

## 工作流总结

**日常刷题**:
1. `./add_kata.sh <kata_id>` - 添加题目
2. 编辑 `lib.rs` - 实现代码
3. `cargo test -p <package>` - 测试
4. 提交到 Codewars

**复习时**:
1. `./add_kata.sh <kata_id> --review` - 创建复习
2. 编辑 `review_N.rs` - 重新实现
3. `cargo test -p <package> review_N` - 测试
4. 对比两种实现，总结经验

**管理题目**:
1. `./manage_kata.sh list` - 浏览题目
2. `./manage_kata.sh search <keyword>` - 搜索
3. `./manage_kata.sh stats` - 查看统计
4. `./manage_kata.sh info <kata_id>` - 查看详情
