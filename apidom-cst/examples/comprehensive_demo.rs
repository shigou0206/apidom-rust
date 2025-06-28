use apidom_cst::{CstParser, SourceType, TreeCursorSyntaxNode};
use std::sync::Arc;
use std::thread;
use std::time::Instant;

fn main() {
    println!("🚀 ApiDOM CST Parser - 综合功能演示");
    println!("=====================================\n");

    // 1. 多格式支持演示
    demo_multi_format_support();
    
    // 2. 智能检测演示  
    demo_smart_detection();
    
    // 3. 内存优化演示
    demo_memory_optimization();
    
    // 4. 并发性能演示
    demo_concurrent_performance();
    
    // 5. 错误处理演示
    demo_error_handling();
    
    // 6. 高级遍历演示
    demo_advanced_traversal();
    
    println!("\n✅ 演示完成！");
}

fn demo_multi_format_support() {
    println!("1️⃣ 多格式支持演示");
    println!("==================");
    
    let json_data = r#"{
  "name": "ApiDOM CST",
  "version": "2.0.0",
  "features": ["json", "yaml", "smart-detection"],
  "performance": {
    "parsing_speed": "高速",
    "memory_usage": "优化"
  }
}"#;

    let yaml_data = r#"---
name: ApiDOM CST
version: 2.0.0
features:
  - json
  - yaml
  - smart-detection
performance:
  parsing_speed: 高速
  memory_usage: 优化
metadata:
  created: 2024-01-01
  author: Rust Team
"#;

    // JSON 解析
    let json_cst = CstParser::parse_as(json_data, SourceType::Json);
    println!("📄 JSON 解析结果:");
    print_cst_summary(&json_cst, "JSON");
    
    // YAML 解析
    let yaml_cst = CstParser::parse_as(yaml_data, SourceType::Yaml);
    println!("📄 YAML 解析结果:");
    print_cst_summary(&yaml_cst, "YAML");
    
    println!();
}

fn demo_smart_detection() {
    println!("2️⃣ 智能格式检测演示");
    println!("====================");
    
    let test_cases = vec![
        (r#"{"auto": "detected", "format": "json"}"#, "JSON 对象"),
        (r#"[1, 2, 3, 4, 5]"#, "JSON 数组"),
        ("name: value\nlist:\n  - item1\n  - item2", "YAML 映射"),
        ("---\ntitle: Document\ncontent: Hello", "YAML 文档"),
        ("- first\n- second\n- third", "YAML 列表"),
    ];
    
    for (source, description) in test_cases {
        let (cst, detected_type) = CstParser::parse_smart(source);
        let success = if cst.has_error() { "❌" } else { "✅" };
        println!("{} {} -> 检测为 {} ({})", 
                success, description, detected_type.display_name(), 
                if cst.has_error() { "有错误" } else { "解析成功" });
    }
    
    println!();
}

fn demo_memory_optimization() {
    println!("3️⃣ 内存优化演示 (Arc<str> 零拷贝)");
    println!("=================================");
    
    let large_json = format!(r#"{{
  "description": "大型数据集测试",
  "items": [{}],
  "stats": {{
    "total": {},
    "memory_optimized": true
  }}
}}"#, 
        (0..100).map(|i| format!(r#"{{"id": {}, "name": "item_{}", "data": "{}"}}"#, 
                                  i, i, "x".repeat(20)))
                 .collect::<Vec<_>>().join(", "),
        100
    );
    
    let start = Instant::now();
    let cst = CstParser::parse(&large_json);
    let parse_time = start.elapsed();
    
    // 验证内存共享
    let source_arc = cst.shared_source();
    let initial_ref_count = Arc::strong_count(source_arc);
    
    let mut node_count = 0;
    let mut shared_count = 0;
    
    for node in cst.iter_preorder() {
        node_count += 1;
        if Arc::ptr_eq(node.shared_source(), source_arc) {
            shared_count += 1;
        }
    }
    
    println!("📊 内存优化统计:");
    println!("  • 解析时间: {:?}", parse_time);
    println!("  • 总节点数: {}", node_count);
    println!("  • 共享源码的节点: {} ({}%)", shared_count, (shared_count * 100) / node_count);
    println!("  • Arc<str> 引用计数: {}", initial_ref_count);
    println!("  • 源码大小: {} bytes", source_arc.len());
    println!("  • 零拷贝文本提取: ✅");
    
    println!();
}

fn demo_concurrent_performance() {
    println!("4️⃣ 并发性能演示");
    println!("================");
    
    let test_data = vec![
        (r#"{"type": "json", "thread": 1}"#, SourceType::Json),
        ("type: yaml\nthread: 2", SourceType::Yaml),
        (r#"{"type": "json", "thread": 3, "nested": {"deep": true}}"#, SourceType::Json),
        ("type: yaml\nthread: 4\nnested:\n  deep: true", SourceType::Yaml),
    ];
    
    let start = Instant::now();
    
    let handles: Vec<_> = test_data.into_iter().enumerate().map(|(i, (source, source_type))| {
        let source = source.to_string();
        thread::spawn(move || {
            let thread_start = Instant::now();
            let cst = CstParser::parse_as(&source, source_type);
            let thread_time = thread_start.elapsed();
            
            (i + 1, source_type, !cst.has_error(), thread_time)
        })
    }).collect();
    
    println!("🧵 并发解析结果:");
    for handle in handles {
        let (thread_id, source_type, success, time) = handle.join().unwrap();
        let status = if success { "✅" } else { "❌" };
        println!("  • 线程 {} ({}): {} ({:?})", 
                thread_id, source_type.display_name(), status, time);
    }
    
    let total_time = start.elapsed();
    println!("  • 总并发时间: {:?}", total_time);
    
    println!();
}

fn demo_error_handling() {
    println!("5️⃣ 错误处理演示");
    println!("================");
    
    let error_cases = vec![
        ("JSON 语法错误", r#"{"invalid": json syntax"#, SourceType::Json),
        ("JSON 缺少括号", r#"{"missing": "bracket""#, SourceType::Json),
        ("YAML 缩进错误", "items:\n  - item1\n    - item2", SourceType::Yaml),
        ("YAML 未闭合引号", r#"key: "unclosed string"#, SourceType::Yaml),
    ];
    
    println!("🔍 错误检测结果:");
    for (description, source, source_type) in error_cases {
        let cst = CstParser::parse_as(source, source_type);
        
        let error_count = count_errors(&cst);
        let status = if error_count > 0 { "🔴" } else { "🟢" };
        
        println!("  {} {}: {} 个错误", status, description, error_count);
        
        if error_count > 0 {
            // 显示第一个错误的位置
            if let Some(error_node) = find_first_error(&cst) {
                println!("    └─ 错误位置: {}:{} ({})", 
                        error_node.start_point.row + 1,
                        error_node.start_point.column + 1,
                        &error_node.kind);
            }
        }
    }
    
    println!();
}

fn demo_advanced_traversal() {
    println!("6️⃣ 高级遍历演示");
    println!("================");
    
    let complex_data = r#"{
  "root": {
    "metadata": {
      "version": "1.0",
      "author": "Rust Team"
    },
    "content": {
      "sections": [
        {"title": "Introduction", "pages": 10},
        {"title": "Advanced Topics", "pages": 25}
      ]
    }
  }
}"#;
    
    let cst = CstParser::parse(complex_data);
    
    println!("🌳 遍历方式对比:");
    
    // 前序遍历
    let preorder_nodes: Vec<_> = cst.iter_preorder().take(8).collect();
    println!("  • 前序遍历 (前8个): {}", 
            preorder_nodes.iter().map(|n| n.kind.as_str()).collect::<Vec<_>>().join(" → "));
    
    // 后序遍历  
    let postorder_nodes: Vec<_> = cst.iter_postorder().take(8).collect();
    println!("  • 后序遍历 (前8个): {}", 
            postorder_nodes.iter().map(|n| n.kind.as_str()).collect::<Vec<_>>().join(" → "));
    
    // 广度优先遍历
    let breadth_first_nodes: Vec<_> = cst.iter_breadth_first().take(8).collect();
    println!("  • 广度优先 (前8个): {}", 
            breadth_first_nodes.iter().map(|n| n.kind.as_str()).collect::<Vec<_>>().join(" → "));
    
    // 按类型查找节点
    let strings = cst.find_nodes_by_kind("string");
    println!("  • 字符串节点数量: {}", strings.len());
    
    let objects = cst.find_nodes_by_kind("object");
    println!("  • 对象节点数量: {}", objects.len());
    
    println!();
}

// 辅助函数
fn print_cst_summary(cst: &TreeCursorSyntaxNode, format: &str) {
    let node_count = cst.iter_preorder().count();
    let has_errors = cst.has_error();
    let status = if has_errors { "❌" } else { "✅" };
    
    println!("  {} {} 解析: {} 个节点, {}", 
            status, format, node_count,
            if has_errors { "有错误" } else { "无错误" });
}

fn count_errors(node: &TreeCursorSyntaxNode) -> usize {
    let mut count = 0;
    if node.has_error() {
        count += 1;
    }
    for child in &node.children {
        count += count_errors(child);
    }
    count
}

fn find_first_error(node: &TreeCursorSyntaxNode) -> Option<&TreeCursorSyntaxNode> {
    if node.has_error() {
        return Some(node);
    }
    for child in &node.children {
        if let Some(error) = find_first_error(child) {
            return Some(error);
        }
    }
    None
} 