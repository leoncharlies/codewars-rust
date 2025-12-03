#!/bin/bash

# Codewars Kata 自动化添加脚本
# 用法: ./add_kata.sh <kata_id> [--review]

set -e

KATA_ID="$1"
IS_REVIEW=false
IS_INCOMPLETE=false
KATA_INDEX=".kata_index"

# 解析参数
for arg in "$@"; do
    if [ "$arg" == "--review" ]; then
        IS_REVIEW=true
    elif [ "$arg" == "--incomplete" ]; then
        IS_INCOMPLETE=true
    fi
done

if [ -z "$KATA_ID" ]; then
    echo "用法: ./add_kata.sh <kata_id> [--review] [--incomplete]"
    echo "示例: ./add_kata.sh 5541f58a944b85ce6d00006a"
    echo "添加复习: ./add_kata.sh 5541f58a944b85ce6d00006a --review"
    echo "添加未完成题目: ./add_kata.sh 5541f58a944b85ce6d00006a --incomplete"
    exit 1
fi

# 创建索引文件（如果不存在）
if [ ! -f "$KATA_INDEX" ]; then
    cat > "$KATA_INDEX" <<EOF
# Codewars 题目索引
# 格式: kata_id|题目名称|难度|路径|完成日期|状态(completed/incomplete)
# 此文件由 add_kata.sh 自动维护

EOF
fi

echo "正在获取题目信息..."
API_URL="https://www.codewars.com/api/v1/code-challenges/$KATA_ID"
RESPONSE=$(curl -s "$API_URL")

if [ -z "$RESPONSE" ] || echo "$RESPONSE" | grep -q "\"success\":false"; then
    echo "错误: 无法获取题目信息，请检查 kata ID 是否正确"
    exit 1
fi

# 提取题目信息
KATA_NAME=$(echo "$RESPONSE" | grep -oP '"name":\s*"\K[^"]+' | head -1)
RANK_ID=$(echo "$RESPONSE" | grep -oP '"rank":\s*{\s*"id":\s*\K-?\d+' | head -1)
KATA_URL=$(echo "$RESPONSE" | grep -oP '"url":\s*"\K[^"]+' | head -1)

# 从 API 返回中提取 kyu 数字
RANK_NAME=$(echo "$RESPONSE" | grep -oP '"name":\s*"\K[^"]+' | grep "kyu" | head -1)
RANK_NUM=$(echo "$RANK_NAME" | grep -oP '^\d+')

if [ -n "$RANK_NUM" ]; then
    RANK="${RANK_NUM}kyu"
else
    echo "错误: 无法解析难度信息"
    exit 1
fi

# 转换题目名称为文件名格式
KATA_NAME_SNAKE=$(echo "$KATA_NAME" | tr '[:upper:]' '[:lower:]' | sed 's/[^a-z0-9]/_/g' | sed 's/__*/_/g' | sed 's/^_//; s/_$//')

# 检查索引文件中是否已存在该题目
INDEX_ENTRY=$(grep "^${KATA_ID}|" "$KATA_INDEX" 2>/dev/null || true)

# 同时检查文件系统（双重保险）
RANK_DIR="katas/$RANK"
EXISTING_KATA=$(find katas -maxdepth 2 -type d -name "*_${KATA_ID}" 2>/dev/null | head -1)

# 如果在索引或文件系统中找到了题目
if [ -n "$INDEX_ENTRY" ] || [ -n "$EXISTING_KATA" ]; then
    if [ "$IS_REVIEW" == "false" ]; then
        echo ""
        echo "⚠️  警告: 你已经做过这道题目了！"
        echo ""
        if [ -n "$INDEX_ENTRY" ]; then
            echo "📋 索引记录:"
            echo "   $INDEX_ENTRY" | sed 's/|/\n   /g'
        fi
        if [ -n "$EXISTING_KATA" ]; then
            echo ""
            echo "📁 文件位置: $EXISTING_KATA"
            
            # 显示复习记录
            REVIEW_COUNT=$(find "$EXISTING_KATA/src" -name "review_*.rs" 2>/dev/null | wc -l)
            if [ "$REVIEW_COUNT" -gt 0 ]; then
                echo "🔄 已复习: $REVIEW_COUNT 次"
            fi
        fi
        echo ""
        echo "如果要添加复习版本，请使用:"
        echo "  ./add_kata.sh $KATA_ID --review"
        echo ""
        exit 1
    fi
fi

if [ "$IS_REVIEW" == "true" ]; then
    if [ -z "$EXISTING_KATA" ]; then
        echo "错误: 未找到原始题目，无法创建复习版本"
        exit 1
    fi
    
    # 找到下一个可用的 review 编号
    REVIEW_NUM=1
    while [ -f "$EXISTING_KATA/src/review_$REVIEW_NUM.rs" ]; do
        REVIEW_NUM=$((REVIEW_NUM + 1))
    done
    
    REVIEW_FILE="$EXISTING_KATA/src/review_$REVIEW_NUM.rs"
    
    echo "创建复习版本: review_$REVIEW_NUM.rs"
    
    # 创建 review 文件
    cat > "$REVIEW_FILE" <<EOF
//! Review #$REVIEW_NUM
//! Date: $(date +%Y-%m-%d)
#![allow(dead_code)]

pub fn solution() {
    unimplemented!()
}

#[cfg(test)]
mod tests {
    use super::*;

    // 在这里粘贴测试代码
}
EOF
    
    # 在 lib.rs 中添加模块引用（如果还没有）
    LIB_RS="$EXISTING_KATA/src/lib.rs"
    if ! grep -q "^#\[cfg(test)\]" "$LIB_RS" || ! grep -q "mod review_$REVIEW_NUM;" "$LIB_RS"; then
        # 在文件末尾添加模块声明
        cat >> "$LIB_RS" <<EOF

#[cfg(test)]
mod review_$REVIEW_NUM;
EOF
    fi
    
    echo "✓ 复习版本创建成功: $REVIEW_FILE"
    echo "运行测试: cargo test -p $(basename "$EXISTING_KATA") review_$REVIEW_NUM"
    
    # 自动同步题目列表
    if [ -x "./sync_kata_list.sh" ]; then
        echo ""
        echo "同步题目列表..."
        ./sync_kata_list.sh > /dev/null 2>&1 && echo "✓ KATA_LIST.md 已更新" || echo "⚠ 同步失败，请手动运行 ./sync_kata_list.sh"
    fi
    
else
    # 获取下一个编号
    NEXT_NUM=0
    for dir in "$RANK_DIR"/*; do
        if [ -d "$dir" ]; then
            NUM=$(basename "$dir" | grep -oP '^\d+' || echo "0")
            if [ "$NUM" -ge "$NEXT_NUM" ]; then
                NEXT_NUM=$((NUM + 1))
            fi
        fi
    done
    
    NEXT_NUM_PADDED=$(printf "%03d" $NEXT_NUM)
    KATA_DIR="$RANK_DIR/${NEXT_NUM_PADDED}_${KATA_NAME_SNAKE}_${KATA_ID}"
    
    echo "创建新题目: $KATA_DIR"
    
    # 创建目录结构
    mkdir -p "$KATA_DIR/src"
    
    # 创建 Cargo.toml
    PACKAGE_NAME="kata_${NEXT_NUM_PADDED}_${KATA_NAME_SNAKE}_${KATA_ID}"
    cat > "$KATA_DIR/Cargo.toml" <<EOF
[package]
name = "$PACKAGE_NAME"
version.workspace = true
edition.workspace = true

[dependencies]
EOF
    
    # 创建 lib.rs
    cat > "$KATA_DIR/src/lib.rs" <<EOF
//! Title: $KATA_NAME
//! Link: $KATA_URL
//! Kata ID: $KATA_ID
//! Rank: $RANK
//! Completed: $(date +%Y-%m-%d)
#![allow(dead_code)]

pub fn solution() {
    unimplemented!()
}

#[cfg(test)]
mod tests {
    use super::*;

    // 在这里粘贴从 Codewars 复制的测试代码
}
EOF
    
    echo "✓ 题目创建成功!"
    echo "  路径: $KATA_DIR"
    echo "  题目: $KATA_NAME"
    echo "  难度: $RANK"
    
    # 更新索引文件
    COMPLETED_DATE=$(date +%Y-%m-%d)
    STATUS="completed"
    if [ "$IS_INCOMPLETE" == "true" ]; then
        STATUS="incomplete"
    fi
    echo "$KATA_ID|$KATA_NAME|$RANK|$KATA_DIR|$COMPLETED_DATE|$STATUS" >> "$KATA_INDEX"
    echo "  ✓ 已添加到题目索引 (状态: $STATUS)"
    
    # 自动同步题目列表
    if [ -x "./sync_kata_list.sh" ]; then
        echo ""
        echo "同步题目列表..."
        ./sync_kata_list.sh > /dev/null 2>&1 && echo "  ✓ KATA_LIST.md 已更新" || echo "  ⚠ 同步失败，请手动运行 ./sync_kata_list.sh"
    fi
    
    echo ""
    echo "下一步:"
    echo "  1. 编辑 $KATA_DIR/src/lib.rs"
    echo "  2. 粘贴测试用例到 tests 模块"
    echo "  3. 实现 solution 函数"
    echo "  4. 运行测试: cargo test -p $PACKAGE_NAME"
    echo "  5. 查看列表: cat KATA_LIST.md"
fi
