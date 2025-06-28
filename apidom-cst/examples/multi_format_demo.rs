use apidom_cst::{CstParser, SourceType, demonstrate_cst_features_multi_format};

fn main() {
    println!("🚀 ApiDOM CST 多格式解析演示\n");
    
    // 1. JSON 解析演示
    println!("{}", "=".repeat(60));
    println!("📄 JSON 解析演示");
    println!("{}", "=".repeat(60));
    
    let json_example = r#"{
  "name": "ApiDOM CST",
  "version": "0.1.0",
  "features": ["json", "yaml", "concurrent"],
  "metadata": {
    "author": "CST Team",
    "license": "MIT",
    "performance": {
      "memory_optimized": true,
      "thread_safe": true,
      "zero_copy": true
    }
  },
  "numbers": [1, 2, 3.14, -42, 1e10]
}"#;
    
    demonstrate_cst_features_multi_format(json_example, Some(SourceType::Json));
    
    // 2. YAML 解析演示
    println!("\n\n");
    println!("{}", "=".repeat(60));
    println!("📄 YAML 解析演示");
    println!("{}", "=".repeat(60));
    
    let yaml_example = r#"
name: ApiDOM CST
version: 0.1.0
features:
  - json
  - yaml
  - concurrent
metadata:
  author: CST Team
  license: MIT
  performance:
    memory_optimized: true
    thread_safe: true
    zero_copy: true
numbers:
  - 1
  - 2
  - 3.14
  - -42
  - 1e10
"#;
    
    demonstrate_cst_features_multi_format(yaml_example, Some(SourceType::Yaml));
    
    // 3. 智能检测演示
    println!("\n\n");
    println!("{}", "=".repeat(60));
    println!("🧠 智能格式检测演示");
    println!("{}", "=".repeat(60));
    
    let test_cases = vec![
        (r#"{"auto": "json"}"#, "明显的 JSON 格式"),
        (r#"[1, 2, 3]"#, "JSON 数组"),
        ("key: value", "简单的 YAML"),
        ("list:\n  - item1\n  - item2", "YAML 列表"),
        ("---\ndoc: yaml", "YAML 文档标记"),
    ];
    
    for (i, (source, description)) in test_cases.iter().enumerate() {
        println!("\n测试案例 {}: {}", i + 1, description);
        println!("源码: {}", source);
        
        let (cst, detected_type) = CstParser::parse_smart(source);
        println!("检测结果: {}", detected_type.display_name());
        println!("解析成功: {}", !cst.has_error());
        println!("节点数量: {}", cst.preorder().count());
    }
    
    // 4. 性能对比演示
    println!("\n\n");
    println!("{}", "=".repeat(60));
    println!("⚡ 性能特性演示");
    println!("{}", "=".repeat(60));
    
    // 创建一个较大的 JSON 来演示性能
    let large_json = format!(r#"{{
  "data": [{}],
  "metadata": {{
    "count": {},
    "generated": true
  }}
}}"#, 
        (0..100).map(|i| format!(r#"{{"id": {}, "name": "item{}", "value": {}}}"#, i, i, i * 10))
                .collect::<Vec<_>>().join(", "),
        100
    );
    
    println!("解析大型 JSON (约 {} 字节)...", large_json.len());
    let start = std::time::Instant::now();
    let large_cst = CstParser::parse(&large_json);
    let parse_time = start.elapsed();
    
    println!("✅ 解析完成!");
    println!("   解析时间: {:?}", parse_time);
    println!("   总节点数: {}", large_cst.preorder().count());
    println!("   内存共享: Arc 引用计数 = {}", std::sync::Arc::strong_count(large_cst.shared_source()));
    println!("   平均节点大小: {:.1} 字节", large_json.len() as f64 / large_cst.preorder().count() as f64);
    
    // 演示零拷贝迭代
    let start = std::time::Instant::now();
    let string_count = large_cst
        .preorder()
        .filter(|node| node.kind == "string")
        .count();
    let iteration_time = start.elapsed();
    
    println!("   字符串节点: {} 个", string_count);
    println!("   迭代时间: {:?}", iteration_time);
    
    // 5. 并发解析演示
    println!("\n\n");
    println!("{}", "=".repeat(60));
    println!("🔄 并发解析演示");
    println!("{}", "=".repeat(60));
    
    use std::thread;
    use std::sync::Arc;
    use std::time::Instant;
    
    let sources = Arc::new(vec![
        (json_example, SourceType::Json),
        (yaml_example, SourceType::Yaml),
        (r#"{"concurrent": true}"#, SourceType::Json),
        ("concurrent: true", SourceType::Yaml),
    ]);
    
    println!("启动 4 个并发解析任务...");
    let start = Instant::now();
    
    let handles: Vec<_> = (0..4).map(|i| {
        let sources = sources.clone();
        thread::spawn(move || {
            let (source, source_type) = &sources[i];
            let thread_id = thread::current().id();
            println!("  线程 {:?}: 解析 {} 格式", thread_id, source_type.display_name());
            
            let cst = CstParser::parse_as(source, *source_type);
            let node_count = cst.preorder().count();
            
            println!("  线程 {:?}: 完成! ({} 个节点)", thread_id, node_count);
            (thread_id, node_count, !cst.has_error())
        })
    }).collect();
    
    let results: Vec<_> = handles.into_iter().map(|h| h.join().unwrap()).collect();
    let total_time = start.elapsed();
    
    println!("✅ 并发解析完成!");
    println!("   总耗时: {:?}", total_time);
    println!("   成功率: {}/{}", results.iter().filter(|(_, _, success)| *success).count(), results.len());
    println!("   总节点数: {}", results.iter().map(|(_, count, _)| count).sum::<usize>());
    
    println!("\n🎉 演示完成! ApiDOM CST 支持高性能的多格式并发解析。");
} 