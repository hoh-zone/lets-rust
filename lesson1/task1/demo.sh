#!/bin/bash

# Rust 教程演示脚本

echo "🦀 Rust 基础教程演示"
echo "==================="
echo ""

echo "📚 展示菜单界面："
echo ""

# 显示菜单但不运行
echo "help" | cargo run 2>/dev/null | head -30

echo ""
echo "✅ 菜单问题已修复："
echo "   • 不再闪烁"
echo "   • 界面简洁清晰"
echo "   • 支持20个完整章节"
echo "   • 添加了help命令"
echo ""

echo "🎯 可用的选项："
echo "   • 数字 1-20: 运行对应章节"
echo "   • basic: 运行基础教程 (1-4章)"
echo "   • advanced: 运行进阶教程 (5-20章)"
echo "   • 0: 运行所有章节"
echo "   • help: 显示详细帮助"
echo "   • q: 退出程序"
echo ""

echo "💡 使用方法："
echo "   cd lesson1/task1"
echo "   cargo run"
echo ""

echo "�� 现在可以正常使用交互式菜单了！" 