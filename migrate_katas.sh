#!/bin/bash

# 迁移旧题目脚本
# 从 /home/somnus/code/codewars-rust/crates 迁移题目到新结构

set -e

OLD_WORKSPACE="/home/somnus/code/codewars-rust/crates"
NEW_WORKSPACE="/home/somnus/code/rust-codewars/katas"

echo "开始迁移题目..."

# 遍历所有难度目录
for RANK_DIR in "$OLD_WORKSPACE"/*kyu; do
    if [ ! -d "$RANK_DIR" ]; then
        continue
    fi
    
    RANK=$(basename "$RANK_DIR")
    echo "处理 $RANK 题目..."
    
    # 遍历该难度下的所有题目
    for OLD_KATA_DIR in "$RANK_DIR"/*; do
        if [ ! -d "$OLD_KATA_DIR" ]; then
            continue
        fi
        
        OLD_KATA_NAME=$(basename "$OLD_KATA_DIR")
        LIB_RS="$OLD_KATA_DIR/src/lib.rs"
        
        if [ ! -f "$LIB_RS" ]; then
            echo "  警告: 跳过 $OLD_KATA_NAME (没有 lib.rs)"
            continue
        fi
        
        # 从 lib.rs 提取 kata ID
        KATA_ID=$(grep -oP 'https://www\.codewars\.com/kata/\K[a-f0-9]+' "$LIB_RS" | head -1)
        
        if [ -z "$KATA_ID" ]; then
            echo "  警告: 跳过 $OLD_KATA_NAME (未找到 kata ID)"
            continue
        fi
        
        # 检查是否已存在
        if find "$NEW_WORKSPACE/$RANK" -maxdepth 1 -type d -name "*_${KATA_ID}" 2>/dev/null | grep -q .; then
            echo "  跳过 $OLD_KATA_NAME (已存在)"
            continue
        fi
        
        # 提取本地编号和题目名称
        LOCAL_NUM=$(echo "$OLD_KATA_NAME" | grep -oP '^\d+')
        KATA_NAME_PART=$(echo "$OLD_KATA_NAME" | sed "s/^${LOCAL_NUM}_//")
        
        # 创建新目录
        NEW_KATA_DIR="$NEW_WORKSPACE/$RANK/${LOCAL_NUM}_${KATA_NAME_PART}_${KATA_ID}"
        
        echo "  迁移: $OLD_KATA_NAME -> $NEW_KATA_DIR"
        
        # 复制整个目录
        cp -r "$OLD_KATA_DIR" "$NEW_KATA_DIR"
        
        # 更新 Cargo.toml 的 package name
        if [ -f "$NEW_KATA_DIR/Cargo.toml" ]; then
            PACKAGE_NAME="${LOCAL_NUM}_${KATA_NAME_PART}_${KATA_ID}"
            sed -i "s/^name = .*/name = \"$PACKAGE_NAME\"/" "$NEW_KATA_DIR/Cargo.toml"
        fi
        
        # 在 lib.rs 中添加 Kata ID (如果不存在)
        if ! grep -q "Kata ID:" "$NEW_KATA_DIR/src/lib.rs"; then
            # 在 Link 行后添加 Kata ID
            sed -i "/^\/\/! Link:/a \/\/! Kata ID: $KATA_ID" "$NEW_KATA_DIR/src/lib.rs"
        fi
    done
done

echo ""
echo "✓ 迁移完成!"
echo "运行 'cargo build' 来验证所有题目"
