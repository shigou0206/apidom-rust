use apidom_ns_openapi_3_1::{
    field_extractor::FieldExtractor,
    schema_loader::SchemaLoader,
};
use std::time::Instant;

fn main() {
    println!("=== ApiDOM OpenAPI 3.1 性能演示 ===\n");

    // 创建复杂的测试数据
    let json_data = r#"{
        "title": "Complex API",
        "version": "1.0.0",
        "description": "A complex API with many fields",
        "termsOfService": "https://example.com/terms",
        "contact": {
            "name": "API Support",
            "url": "https://example.com/support",
            "email": "support@example.com"
        },
        "license": {
            "name": "MIT",
            "url": "https://opensource.org/licenses/MIT"
        },
        "servers": [
            {
                "url": "https://api.example.com/v1",
                "description": "Production server"
            },
            {
                "url": "https://staging-api.example.com/v1",
                "description": "Staging server"
            }
        ],
        "paths": {
            "/users": {
                "get": {
                    "summary": "List users",
                    "operationId": "listUsers",
                    "tags": ["users"],
                    "responses": {
                        "200": {
                            "description": "Success"
                        }
                    }
                }
            }
        },
        "x-rate-limit": 1000,
        "x-api-version": "2024",
        "x-feature-flags": ["newUI", "betaFeatures", "analytics"],
        "x-metadata": {
            "owner": "api-team",
            "maintained": true
        }
    }"#;

    let loader = SchemaLoader::new();
    let element = loader.parse_json_to_element(json_data).unwrap();

    // 性能测试：单个字段提取
    println!("1. 单个字段提取性能");
    println!("--------------------");
    
    let iterations = 10000;
    let start = Instant::now();
    
    for _ in 0..iterations {
        let _title = FieldExtractor::extract_string(&element, "title");
        let _version = FieldExtractor::extract_string(&element, "version");
        let _description = FieldExtractor::extract_string(&element, "description");
    }
    
    let duration = start.elapsed();
    println!("✓ {} 次单个字段提取: {:?}", iterations * 3, duration);
    println!("  平均每次提取: {:?}", duration / (iterations * 3));

    // 性能测试：扩展字段提取
    println!("\n2. 扩展字段提取性能");
    println!("--------------------");
    
    let start = Instant::now();
    
    for _ in 0..iterations {
        let _extensions = FieldExtractor::extract_extension_fields(&element);
    }
    
    let duration = start.elapsed();
    println!("✓ {} 次扩展字段提取: {:?}", iterations, duration);
    println!("  平均每次提取: {:?}", duration / iterations);

    // 性能测试：批量字段验证
    println!("\n3. 批量字段验证性能");
    println!("--------------------");
    
    let required_fields = ["title", "version", "description", "contact", "license"];
    let start = Instant::now();
    
    for _ in 0..iterations {
        let _result = FieldExtractor::validate_required_fields(&element, &required_fields);
    }
    
    let duration = start.elapsed();
    println!("✓ {} 次批量验证 ({} 字段): {:?}", iterations, required_fields.len(), duration);
    println!("  平均每次验证: {:?}", duration / iterations);

    // 性能测试：类型检查
    println!("\n4. 类型检查性能");
    println!("----------------");
    
    let start = Instant::now();
    
    for _ in 0..iterations {
        let _is_string = FieldExtractor::is_field_of_type(&element, "title", "string");
        let _is_object = FieldExtractor::is_field_of_type(&element, "contact", "object");
    }
    
    let duration = start.elapsed();
    println!("✓ {} 次类型检查: {:?}", iterations * 2, duration);
    println!("  平均每次检查: {:?}", duration / (iterations * 2));

    // 内存使用演示
    println!("\n5. 内存效率演示");
    println!("----------------");
    
    // 提取并展示结果，不进行不必要的克隆
    let title = FieldExtractor::extract_string(&element, "title");
    let extensions = FieldExtractor::extract_extension_fields(&element);
    
    println!("✓ 零拷贝字符串提取: {:?}", title);
    println!("✓ 高效扩展字段提取: {} 个扩展字段", extensions.len());
    
    for (key, _) in extensions.iter().take(3) {
        println!("  - {}", key);
    }

    // 综合性能测试
    println!("\n6. 综合操作性能");
    println!("----------------");
    
    let start = Instant::now();
    
    for _ in 0..1000 {
        // 模拟真实使用场景的操作序列
        let _title = FieldExtractor::extract_string(&element, "title");
        let _version = FieldExtractor::extract_string(&element, "version");
        let _extensions = FieldExtractor::extract_extension_fields(&element);
        let _validation = FieldExtractor::validate_required_fields(&element, &["title", "version"]);
        let _type_check = FieldExtractor::is_field_of_type(&element, "title", "string");
    }
    
    let duration = start.elapsed();
    println!("✓ 1000 次综合操作: {:?}", duration);
    println!("  平均每次综合操作: {:?}", duration / 1000);

    println!("\n=== 性能演示完成 ===");
    println!("\n性能特点:");
    println!("• 零拷贝字符串操作");
    println!("• 高效的内存管理");
    println!("• 快速类型检查");
    println!("• 批量操作优化");
    println!("• 扩展字段专门优化");
} 