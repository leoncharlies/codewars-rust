#!/bin/bash

# Codewars Kata 管理工具
# 用于查看、搜索和管理题目

set -e

show_usage() {
    echo "Codewars Kata 管理工具"
    echo ""
    echo "用法:"
    echo "  ./manage_kata.sh list [难度]        - 列出所有题目或指定难度的题目"
    echo "  ./manage_kata.sh search <关键词>    - 搜索题目（按名称或ID）"
    echo "  ./manage_kata.sh stats              - 显示统计信息"
    echo "  ./manage_kata.sh info <kata_id>     - 显示题目详细信息"
    echo "  ./manage_kata.sh index              - 显示题目索引"
    echo "  ./manage_kata.sh check <kata_id>    - 快速检查是否做过某题"
    echo "  ./manage_kata.sh complete <kata_id> - 标记题目为已完成"
    echo "  ./manage_kata.sh incomplete <kata_id> - 标记题目为未完成"
    echo "  ./manage_kata.sh pending            - 列出所有未完成的题目"
    echo ""
    echo "示例:"
    echo "  ./manage_kata.sh list 5kyu"
    echo "  ./manage_kata.sh search fibonacci"
    echo "  ./manage_kata.sh stats"
    echo "  ./manage_kata.sh info 5541f58a944b85ce6d00006a"
    echo "  ./manage_kata.sh check 5541f58a944b85ce6d00006a"
    echo "  ./manage_kata.sh complete 5541f58a944b85ce6d00006a"
    echo "  ./manage_kata.sh incomplete 5541f58a944b85ce6d00006a"
    echo "  ./manage_kata.sh pending"
}

list_katas() {
    local rank="$1"
    
    if [ -n "$rank" ]; then
        echo "=== ${rank} 题目列表 ==="
        for kata_dir in katas/${rank}/*; do
            if [ -d "$kata_dir" ] && [ -f "$kata_dir/src/lib.rs" ]; then
                local basename=$(basename "$kata_dir")
                local title=$(grep "^//! Title:" "$kata_dir/src/lib.rs" | sed 's/^\/\/! Title: //')
                local kata_id=$(grep "^//! Kata ID:" "$kata_dir/src/lib.rs" | sed 's/^\/\/! Kata ID: //')
                
                # 检查是否有复习版本
                local review_count=$(find "$kata_dir/src" -name "review_*.rs" 2>/dev/null | wc -l)
                local review_marker=""
                if [ "$review_count" -gt 0 ]; then
                    review_marker=" [复习×$review_count]"
                fi
                
                echo "  [$kata_id] $title$review_marker"
            fi
        done
    else
        echo "=== 所有题目 ==="
        for rank_dir in katas/*kyu; do
            if [ -d "$rank_dir" ]; then
                local rank=$(basename "$rank_dir")
                local count=$(find "$rank_dir" -maxdepth 1 -type d -name "*_*" | wc -l)
                if [ "$count" -gt 0 ]; then
                    echo ""
                    echo "--- $rank ($count 题) ---"
                    list_katas "$rank"
                fi
            fi
        done
    fi
}

search_katas() {
    local keyword="$1"
    
    if [ -z "$keyword" ]; then
        echo "错误: 请提供搜索关键词"
        exit 1
    fi
    
    echo "搜索包含 '$keyword' 的题目..."
    echo ""
    
    local found=0
    for kata_dir in katas/*/*; do
        if [ -d "$kata_dir" ] && [ -f "$kata_dir/src/lib.rs" ]; then
            local basename=$(basename "$kata_dir")
            local title=$(grep "^//! Title:" "$kata_dir/src/lib.rs" | sed 's/^\/\/! Title: //')
            local kata_id=$(grep "^//! Kata ID:" "$kata_dir/src/lib.rs" | sed 's/^\/\/! Kata ID: //')
            local rank=$(basename $(dirname "$kata_dir"))
            
            # 搜索标题、ID 或目录名
            if echo "$title" | grep -qi "$keyword" || \
               echo "$kata_id" | grep -qi "$keyword" || \
               echo "$basename" | grep -qi "$keyword"; then
                echo "  [$rank] $title"
                echo "      ID: $kata_id"
                echo "      路径: $kata_dir"
                echo ""
                found=$((found + 1))
            fi
        fi
    done
    
    if [ "$found" -eq 0 ]; then
        echo "未找到匹配的题目"
    else
        echo "找到 $found 个匹配的题目"
    fi
}

show_stats() {
    echo "=== Codewars Kata 统计 ==="
    echo ""
    
    local total=0
    local total_reviews=0
    
    for rank_dir in katas/*kyu; do
        if [ -d "$rank_dir" ]; then
            local rank=$(basename "$rank_dir")
            local count=$(find "$rank_dir" -maxdepth 1 -type d -name "*_*" 2>/dev/null | wc -l)
            if [ "$count" -gt 0 ]; then
                total=$((total + count))
                echo "  $rank: $count 题"
                
                # 统计复习次数
                for kata_dir in "$rank_dir"/*; do
                    if [ -d "$kata_dir" ]; then
                        local reviews=$(find "$kata_dir/src" -name "review_*.rs" 2>/dev/null | wc -l)
                        total_reviews=$((total_reviews + reviews))
                    fi
                done
            fi
        fi
    done
    
    echo ""
    echo "总题目数: $total"
    echo "总复习次数: $total_reviews"
    
    if [ "$total" -gt 0 ]; then
        local avg=$(echo "scale=2; $total_reviews / $total" | bc)
        echo "平均复习次数: $avg"
    fi
}

show_info() {
    local kata_id="$1"
    
    if [ -z "$kata_id" ]; then
        echo "错误: 请提供 kata ID"
        exit 1
    fi
    
    # 查找题目
    local kata_dir=$(find katas -maxdepth 2 -type d -name "*_${kata_id}" | head -1)
    
    if [ -z "$kata_dir" ]; then
        echo "错误: 未找到 kata ID: $kata_id"
        exit 1
    fi
    
    local lib_rs="$kata_dir/src/lib.rs"
    
    echo "=== 题目信息 ==="
    echo ""
    grep "^//!" "$lib_rs"
    echo ""
    echo "路径: $kata_dir"
    echo "包名: $(grep "^name = " "$kata_dir/Cargo.toml" | sed 's/name = "\(.*\)"/\1/')"
    echo ""
    
    # 检查复习版本
    local reviews=$(find "$kata_dir/src" -name "review_*.rs" 2>/dev/null)
    if [ -n "$reviews" ]; then
        echo "复习版本:"
        echo "$reviews" | while read review_file; do
            echo "  - $(basename "$review_file")"
            grep "^//! Date:" "$review_file" 2>/dev/null || true
        done
        echo ""
    fi
    
    echo "测试命令:"
    local package_name=$(grep "^name = " "$kata_dir/Cargo.toml" | sed 's/name = "\(.*\)"/\1/')
    echo "  cargo test -p $package_name"
}

show_index() {
    local KATA_INDEX=".kata_index"
    
    if [ ! -f "$KATA_INDEX" ]; then
        echo "索引文件不存在，正在创建..."
        ./add_kata.sh --help > /dev/null 2>&1 || true
        if [ ! -f "$KATA_INDEX" ]; then
            echo "错误: 无法创建索引文件"
            exit 1
        fi
    fi
    
    echo "=== Codewars 题目索引 ==="
    echo ""
    echo "格式: kata_id | 题目名称 | 难度 | 路径 | 完成日期"
    echo ""
    
    grep -v "^#" "$KATA_INDEX" | grep -v "^$" | while IFS='|' read -r id name rank path date; do
        printf "%-24s | %-40s | %-4s | %s\n" "$id" "$name" "$rank" "$date"
    done
}

check_kata() {
    local kata_id="$1"
    local KATA_INDEX=".kata_index"
    
    if [ -z "$kata_id" ]; then
        echo "错误: 请提供 kata ID"
        exit 1
    fi
    
    if [ ! -f "$KATA_INDEX" ]; then
        echo "❌ 索引文件不存在"
        exit 1
    fi
    
    local entry=$(grep "^${kata_id}|" "$KATA_INDEX" 2>/dev/null || true)
    
    if [ -n "$entry" ]; then
        echo "✅ 已完成这道题"
        echo ""
        echo "详细信息:"
        echo "$entry" | awk -F'|' '{
            print "  题目ID:    " $1
            print "  题目名称:  " $2
            print "  难度:      " $3
            print "  路径:      " $4
            print "  完成日期:  " $5
        }'
        
        # 检查复习次数
        local path=$(echo "$entry" | cut -d'|' -f4)
        if [ -d "$path/src" ]; then
            local reviews=$(find "$path/src" -name "review_*.rs" 2>/dev/null | wc -l)
            if [ "$reviews" -gt 0 ]; then
                echo "  复习次数:  $reviews"
            fi
        fi
    else
        echo "❌ 还没做过这道题"
        echo ""
        echo "添加题目:"
        echo "  ./add_kata.sh $kata_id"
    fi
}

mark_status() {
    local kata_id="$1"
    local new_status="$2"
    local KATA_INDEX=".kata_index"
    
    if [ -z "$kata_id" ]; then
        echo "错误: 请提供 kata ID"
        exit 1
    fi
    
    if [ ! -f "$KATA_INDEX" ]; then
        echo "❌ 索引文件不存在"
        exit 1
    fi
    
    # 检查题目是否存在
    local entry=$(grep "^${kata_id}|" "$KATA_INDEX" 2>/dev/null || true)
    
    if [ -z "$entry" ]; then
        echo "❌ 未找到 kata ID: $kata_id"
        exit 1
    fi
    
    # 备份索引
    cp "$KATA_INDEX" "${KATA_INDEX}.bak"
    
    # 更新状态（兼容旧格式和新格式）
    if echo "$entry" | grep -q '|.*|.*|.*|.*|'; then
        # 新格式：包含状态字段
        sed -i "s/^${kata_id}|\(.*\)|\(.*\)|\(.*\)|\(.*\)|.*/${kata_id}|\1|\2|\3|\4|${new_status}/" "$KATA_INDEX"
    else
        # 旧格式：没有状态字段，添加状态
        sed -i "s/^${kata_id}|\(.*\)/${kata_id}|\1|${new_status}/" "$KATA_INDEX"
    fi
    
    # 获取题目名称
    local kata_name=$(echo "$entry" | cut -d'|' -f2)
    
    if [ "$new_status" == "completed" ]; then
        echo "✅ 已标记为完成: $kata_name"
    else
        echo "⏸️  已标记为未完成: $kata_name"
    fi
    
    # 自动同步题目列表
    if [ -x "./sync_kata_list.sh" ]; then
        echo ""
        echo "同步题目列表..."
        ./sync_kata_list.sh > /dev/null 2>&1 && echo "✓ KATA_LIST.md 已更新" || echo "⚠ 同步失败，请手动运行 ./sync_kata_list.sh"
    fi
}

list_pending() {
    local KATA_INDEX=".kata_index"
    
    if [ ! -f "$KATA_INDEX" ]; then
        echo "❌ 索引文件不存在"
        exit 1
    fi
    
    echo "=== 未完成的题目 ==="
    echo ""
    
    local found=0
    while IFS='|' read -r id name rank path date status; do
        if [[ "$id" =~ ^# ]] || [ -z "$id" ]; then
            continue
        fi
        
        # 兼容旧格式（没有状态字段的默认为completed）
        if [ -z "$status" ]; then
            status="completed"
        fi
        
        if [ "$status" == "incomplete" ]; then
            echo "  [$rank] $name"
            echo "      ID: $id"
            echo "      路径: $path"
            echo ""
            found=$((found + 1))
        fi
    done < "$KATA_INDEX"
    
    if [ "$found" -eq 0 ]; then
        echo "🎉 所有题目都已完成！"
    else
        echo "共 $found 个未完成题目"
    fi
}

# 主逻辑
COMMAND="${1:-help}"

case "$COMMAND" in
    list)
        list_katas "$2"
        ;;
    search)
        search_katas "$2"
        ;;
    stats)
        show_stats
        ;;
    info)
        show_info "$2"
        ;;
    index)
        show_index
        ;;
    check)
        check_kata "$2"
        ;;
    complete)
        mark_status "$2" "completed"
        ;;
    incomplete)
        mark_status "$2" "incomplete"
        ;;
    pending)
        list_pending
        ;;
    help|--help|-h)
        show_usage
        ;;
    *)
        echo "错误: 未知命令 '$COMMAND'"
        echo ""
        show_usage
        exit 1
        ;;
esac
