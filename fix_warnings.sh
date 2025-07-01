#!/bin/bash

# Rust 警告修复脚本
# 自动为教学示例代码添加警告抑制属性

echo "🔧 开始修复 Rust 警告..."

# 为所有 bin 文件添加警告抑制属性
echo "📁 处理 bin 目录文件..."

for file in lesson/task/src/bin/*.rs; do
    if [ -f "$file" ]; then
        echo "  处理: $file"
        
        # 检查文件是否已经有警告抑制属性
        if ! grep -q "#\!\[allow(dead_code)\]" "$file"; then
            # 在文件开头添加警告抑制属性
            temp_file=$(mktemp)
            {
                echo "// 教学示例 - 允许未使用的代码"
                echo "#![allow(dead_code)]"
                echo "#![allow(unused_variables)]"
                echo "#![allow(unused_imports)]"
                echo ""
                cat "$file"
            } > "$temp_file"
            mv "$temp_file" "$file"
            echo "    ✅ 已添加警告抑制属性"
        else
            echo "    ⏭️  已存在警告抑制属性，跳过"
        fi
    fi
done

# 运行 cargo fix 来自动修复可修复的警告
echo ""
echo "🛠️  运行 cargo fix..."
cd lesson/task
cargo fix --allow-dirty --allow-staged 2>/dev/null || echo "注意: 某些警告需要手动处理"

# 检查编译状态
echo ""
echo "🔍 检查编译状态..."
if cargo check --quiet 2>/dev/null; then
    echo "✅ 编译成功！"
else
    echo "⚠️  仍有一些编译问题，运行 'cargo check' 查看详情"
fi

echo ""
echo "🎉 警告修复完成！"
echo ""
echo "💡 如果仍有警告，可以："
echo "   1. 运行 'cargo check' 查看剩余警告"
echo "   2. 运行 'cargo fix --allow-dirty' 自动修复"
echo "   3. 手动添加 #[allow(dead_code)] 属性" 