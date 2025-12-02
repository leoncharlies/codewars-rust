#!/bin/bash

# Codewars Kata 列表同步脚本
# 扫描所有题目，生成可视化的 KATA_LIST.md 文件

set -e

OUTPUT_FILE="KATA_LIST.md"
KATA_INDEX=".kata_index"

echo "开始同步题目列表..."

# 重建索引
echo "1. 重建题目索引..."
if [ -f "$KATA_INDEX" ]; then
    cp "$KATA_INDEX" "${KATA_INDEX}.bak"
    echo "   已备份旧索引"
fi

# 创建新索引
cat > "$KATA_INDEX" <<EOF
# Codewars 题目索引
# 格式: kata_id|题目名称|难度|路径|完成日期
# 此文件由 sync_kata_list.sh 自动维护

EOF

# 扫描所有题目
TOTAL_COUNT=0
for kata_dir in katas/*/*; do
    if [ -d "$kata_dir" ] && [ -f "$kata_dir/src/lib.rs" ]; then
        KATA_ID=$(grep "^//! Kata ID:" "$kata_dir/src/lib.rs" | sed 's/^\/\/! Kata ID: //' | tr -d ' ')
        TITLE=$(grep "^//! Title:" "$kata_dir/src/lib.rs" | sed 's/^\/\/! Title: //')
        RANK=$(grep "^//! Rank:" "$kata_dir/src/lib.rs" | sed 's/^\/\/! Rank: //')
        COMPLETED=$(grep "^//! Completed:" "$kata_dir/src/lib.rs" | sed 's/^\/\/! Completed: //')
        
        if [ -n "$KATA_ID" ] && [ -n "$TITLE" ] && [ -n "$RANK" ]; then
            echo "$KATA_ID|$TITLE|$RANK|$kata_dir|$COMPLETED" >> "$KATA_INDEX"
            TOTAL_COUNT=$((TOTAL_COUNT + 1))
        fi
    fi
done

echo "   ✓ 索引已更新，共 $TOTAL_COUNT 题"

# 统计信息
echo "2. 收集统计信息..."

declare -A rank_count
TOTAL_REVIEWS=0

while IFS='|' read -r id name rank path date; do
    if [[ "$id" =~ ^# ]] || [ -z "$id" ]; then
        continue
    fi
    
    rank_count[$rank]=$((${rank_count[$rank]:-0} + 1))
    
    # 统计复习次数
    if [ -d "$path/src" ]; then
        reviews=$(find "$path/src" -name "review_*.rs" 2>/dev/null | wc -l)
        TOTAL_REVIEWS=$((TOTAL_REVIEWS + reviews))
    fi
done < "$KATA_INDEX"

# 生成 KATA_LIST.md
echo "3. 生成 KATA_LIST.md..."

cat > "$OUTPUT_FILE" <<EOF
# 📚 我的 Codewars 刷题记录

> 最后更新: $(date '+%Y-%m-%d %H:%M:%S')

## 📊 统计概览

- **总题目数**: $TOTAL_COUNT
- **总复习次数**: $TOTAL_REVIEWS
EOF

if [ $TOTAL_COUNT -gt 0 ]; then
    AVG_REVIEWS=$(echo "scale=2; $TOTAL_REVIEWS / $TOTAL_COUNT" | bc)
    echo "- **平均复习次数**: $AVG_REVIEWS" >> "$OUTPUT_FILE"
fi

cat >> "$OUTPUT_FILE" <<EOF

### 难度分布

| 难度 | 数量 | 占比 | 进度条 |
|------|------|------|--------|
EOF

# 按难度排序
for kyu in 1kyu 2kyu 3kyu 4kyu 5kyu 6kyu 7kyu 8kyu; do
    count=${rank_count[$kyu]:-0}
    if [ $count -gt 0 ]; then
        percentage=$(echo "scale=0; $count * 100 / $TOTAL_COUNT" | bc)
        bars=$(echo "scale=0; $percentage / 10" | bc)
        progress=$(printf '█%.0s' $(seq 1 $bars))
        empty=$(printf '░%.0s' $(seq 1 $((10 - bars))))
        
        echo "| $kyu | $count | ${percentage}% | ${progress}${empty} |" >> "$OUTPUT_FILE"
    fi
done

echo "" >> "$OUTPUT_FILE"
echo "---" >> "$OUTPUT_FILE"
echo "" >> "$OUTPUT_FILE"
echo "## 📝 题目列表" >> "$OUTPUT_FILE"
echo "" >> "$OUTPUT_FILE"

# 按难度生成题目列表
for kyu in 1kyu 2kyu 3kyu 4kyu 5kyu 6kyu 7kyu 8kyu; do
    count=${rank_count[$kyu]:-0}
    if [ $count -eq 0 ]; then
        continue
    fi
    
    echo "### $kyu ($count 题)" >> "$OUTPUT_FILE"
    echo "" >> "$OUTPUT_FILE"
    
    item_num=1
    while IFS='|' read -r id name rank path date; do
        if [[ "$id" =~ ^# ]] || [ -z "$id" ] || [ "$rank" != "$kyu" ]; then
            continue
        fi
        
        # 检查复习次数
        review_count=0
        review_marker=""
        if [ -d "$path/src" ]; then
            review_count=$(find "$path/src" -name "review_*.rs" 2>/dev/null | wc -l)
            if [ $review_count -gt 0 ]; then
                review_marker=" 🔄"
            fi
        fi
        
        echo "#### $item_num. $name$review_marker" >> "$OUTPUT_FILE"
        echo "- **Kata ID**: $id" >> "$OUTPUT_FILE"
        echo "- **链接**: https://www.codewars.com/kata/$id" >> "$OUTPUT_FILE"
        echo "- **完成日期**: $date" >> "$OUTPUT_FILE"
        echo "- **路径**: \`$path\`" >> "$OUTPUT_FILE"
        echo "- **复习次数**: $review_count" >> "$OUTPUT_FILE"
        
        # 显示复习记录
        if [ $review_count -gt 0 ]; then
            echo "- **复习记录**:" >> "$OUTPUT_FILE"
            for review_file in "$path/src"/review_*.rs; do
                if [ -f "$review_file" ]; then
                    review_num=$(basename "$review_file" | sed 's/review_\([0-9]*\)\.rs/\1/')
                    review_date=$(grep "^//! Date:" "$review_file" | sed 's/^\/\/! Date: //')
                    echo "  - Review #$review_num: $review_date" >> "$OUTPUT_FILE"
                fi
            done
        fi
        
        echo "" >> "$OUTPUT_FILE"
        item_num=$((item_num + 1))
    done < "$KATA_INDEX"
    
    echo "---" >> "$OUTPUT_FILE"
    echo "" >> "$OUTPUT_FILE"
done

# 添加学习进度和快速链接
cat >> "$OUTPUT_FILE" <<EOF
## 🎯 学习进度

### 按难度完成情况

EOF

for kyu in 1kyu 2kyu 3kyu 4kyu 5kyu 6kyu 7kyu 8kyu; do
    count=${rank_count[$kyu]:-0}
    if [ $count -gt 0 ]; then
        echo "- ✅ $kyu: $count 题完成" >> "$OUTPUT_FILE"
    fi
done

cat >> "$OUTPUT_FILE" <<EOF

### 复习情况

EOF

# 找出复习次数最多的题目
MAX_REVIEWS=0
MAX_REVIEW_KATA=""
REVIEWED_COUNT=0

while IFS='|' read -r id name rank path date; do
    if [[ "$id" =~ ^# ]] || [ -z "$id" ]; then
        continue
    fi
    
    if [ -d "$path/src" ]; then
        reviews=$(find "$path/src" -name "review_*.rs" 2>/dev/null | wc -l)
        if [ $reviews -gt 0 ]; then
            REVIEWED_COUNT=$((REVIEWED_COUNT + 1))
            if [ $reviews -gt $MAX_REVIEWS ]; then
                MAX_REVIEWS=$reviews
                MAX_REVIEW_KATA="$name"
            fi
        fi
    fi
done < "$KATA_INDEX"

echo "- 📌 已复习题目: $REVIEWED_COUNT 题" >> "$OUTPUT_FILE"
echo "- 📈 总复习次数: $TOTAL_REVIEWS 次" >> "$OUTPUT_FILE"

if [ $MAX_REVIEWS -gt 0 ]; then
    echo "- 🎖️ 最常复习: $MAX_REVIEW_KATA (${MAX_REVIEWS}次)" >> "$OUTPUT_FILE"
fi

cat >> "$OUTPUT_FILE" <<EOF

---

## 🔗 快速链接

### 添加新题目
\`\`\`bash
./add_kata.sh <kata_id>
\`\`\`

### 添加复习版本
\`\`\`bash
./add_kata.sh <kata_id> --review
\`\`\`

### 查看统计
\`\`\`bash
./manage_kata.sh stats
\`\`\`

### 搜索题目
\`\`\`bash
./manage_kata.sh search <keyword>
\`\`\`

### 更新此列表
\`\`\`bash
./sync_kata_list.sh
\`\`\`

---

*此文件由 \`sync_kata_list.sh\` 自动生成和维护*  
*请勿手动编辑，运行 \`./sync_kata_list.sh\` 重新生成*
EOF

echo "   ✓ KATA_LIST.md 已生成"
echo ""
echo "✅ 同步完成！"
echo ""
echo "📊 统计信息:"
echo "   总题目数: $TOTAL_COUNT"
echo "   总复习次数: $TOTAL_REVIEWS"
echo "   已复习题目: $REVIEWED_COUNT"
echo ""
echo "📖 查看: cat KATA_LIST.md"
