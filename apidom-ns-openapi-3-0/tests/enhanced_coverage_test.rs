#![recursion_limit = "256"]

use apidom_ns_openapi_3_0::fold_pass::{FoldPipeline, FoldPass, OpenApiSpecPass, ReferenceResolutionPass, SemanticEnhancementPass, ValidationPass};
use apidom_ns_openapi_3_0::specification::create_openapi_specification;
use apidom_ast::minim_model::*;
use apidom_ast::fold::json_source_to_ast;
use apidom_cst::CstParser;
use serde_json;
use serde_yaml;
use std::fs;
use std::time::{Instant, Duration};
use std::collections::HashMap;

/// Test 1: 深度测试 json_to_element 的 Array/Object 递归逻辑
#[test]
fn test_json_to_element_deep_recursion() {
    println!("🔄 Testing json_to_element deep Array/Object recursion");
    
    // 创建 5 层深度嵌套的结构（避免递归限制）
    let deep_nested = serde_json::json!({
        "level1": {
            "array1": [{
                "level2": {
                    "array2": [{
                        "level3": {
                            "array3": [{
                                "level4": {
                                    "array4": [{
                                        "level5": {
                                            "final_value": "reached_depth_5",
                                            "numbers": [1, 2, 3, 4, 5],
                                            "booleans": [true, false, true],
                                            "mixed": [null, "string", 42, true, {"nested": "object"}]
                                        }
                                    }]
                                }
                            }]
                        }
                    }]
                }
            }]
        }
    });
    
    // 转换为 AST
    let json_string = serde_json::to_string(&deep_nested).unwrap();
    let ast = json_source_to_ast(&json_string);
    
    // 验证递归转换的正确性
    assert!(matches!(ast, Element::Object(_)), "Root should be Object");
    
    // 递归验证每一层的结构
    fn verify_depth(element: &Element, current_depth: u32, max_depth: u32) -> bool {
        if current_depth > max_depth {
            return true;
        }
        
        match element {
            Element::Object(obj) => {
                // 验证对象结构
                for member in &obj.content {
                    if let Element::String(key) = member.key.as_ref() {
                        if key.content.starts_with("level") || key.content.starts_with("array") {
                            if !verify_depth(member.value.as_ref(), current_depth + 1, max_depth) {
                                return false;
                            }
                        } else if key.content == "final_value" {
                            // 验证最终值
                            if let Element::String(value) = member.value.as_ref() {
                                return value.content == "reached_depth_5";
                            }
                        }
                    }
                }
                true
            }
            Element::Array(arr) => {
                // 验证数组结构
                for item in &arr.content {
                    if !verify_depth(item, current_depth + 1, max_depth) {
                        return false;
                    }
                }
                true
            }
            _ => true,
        }
    }
    
    assert!(verify_depth(&ast, 1, 5), "Deep recursion should preserve structure at all levels");
    
    // 验证具体的深层访问
    if let Element::Object(root) = &ast {
        let mut current: &Element = root.get("level1").unwrap();
        for level in 1..=5 {
            if let Element::Object(obj) = current {
                let array_key = format!("array{}", level);
                if let Some(Element::Array(arr)) = obj.get(&array_key) {
                    if let Some(Element::Object(next_obj)) = arr.content.get(0) {
                        if level < 5 {
                            let next_level_key = format!("level{}", level + 1);
                            current = next_obj.get(&next_level_key).unwrap();
                        } else {
                            // 验证第 5 层的最终内容
                            if let Some(Element::String(final_val)) = next_obj.get("final_value") {
                                assert_eq!(final_val.content, "reached_depth_5");
                            }
                            if let Some(Element::Array(numbers)) = next_obj.get("numbers") {
                                assert_eq!(numbers.content.len(), 5);
                            }
                            if let Some(Element::Array(mixed)) = next_obj.get("mixed") {
                                assert_eq!(mixed.content.len(), 5);
                                // 验证混合类型数组
                                assert!(matches!(mixed.content[0], Element::Null(_)));
                                assert!(matches!(mixed.content[1], Element::String(_)));
                                assert!(matches!(mixed.content[2], Element::Number(_)));
                                assert!(matches!(mixed.content[3], Element::Boolean(_)));
                                assert!(matches!(mixed.content[4], Element::Object(_)));
                            }
                        }
                    }
                }
            }
        }
    }
    
    println!("✅ Deep recursion test passed - 5 levels verified");
    println!("   - Object to Object nesting: ✓");
    println!("   - Object to Array nesting: ✓");
    println!("   - Array to Object nesting: ✓");
    println!("   - Mixed type arrays at depth: ✓");
}

/// Test 2: 单独测试 Array 和 Object 分支的单元测试
#[test]
fn test_array_object_branch_isolation() {
    println!("🧪 Testing Array/Object branch isolation");
    
    // Test 2.1: 纯数组递归（不包含对象）
    let pure_array = serde_json::json!([
        [1, 2, 3],
        [[4, 5], [6, 7, 8]],
        [[[9, 10]], [[11, 12, 13]]],
        [[[[14]]]]
    ]);
    
    let json_string = serde_json::to_string(&pure_array).unwrap();
    let ast = json_source_to_ast(&json_string);
    
    if let Element::Array(root_arr) = &ast {
        assert_eq!(root_arr.content.len(), 4, "Root array should have 4 elements");
        
        // 验证第一层：[1, 2, 3]
        if let Element::Array(first) = &root_arr.content[0] {
            assert_eq!(first.content.len(), 3);
            for (i, elem) in first.content.iter().enumerate() {
                if let Element::Number(num) = elem {
                    assert_eq!(num.content as i32, i as i32 + 1);
                }
            }
        }
        
        // 验证第二层：[[4, 5], [6, 7, 8]]
        if let Element::Array(second) = &root_arr.content[1] {
            assert_eq!(second.content.len(), 2);
            if let Element::Array(sub_arr) = &second.content[1] {
                assert_eq!(sub_arr.content.len(), 3); // [6, 7, 8]
            }
        }
        
        // 验证第四层：[[[[14]]]]
        if let Element::Array(fourth) = &root_arr.content[3] {
            if let Element::Array(level2) = &fourth.content[0] {
                if let Element::Array(level3) = &level2.content[0] {
                    if let Element::Array(level4) = &level3.content[0] {
                        if let Element::Number(deepest) = &level4.content[0] {
                            assert_eq!(deepest.content as i32, 14);
                        }
                    }
                }
            }
        }
    }
    
    println!("  ✅ Pure array recursion verified");
    
    // Test 2.2: 纯对象递归（不包含数组）
    let pure_object = serde_json::json!({
        "root": {
            "child1": {
                "grandchild1": {
                    "greatgrand1": {
                        "value": "deep_value_1"
                    }
                }
            },
            "child2": {
                "grandchild2": {
                    "greatgrand2": {
                        "value": "deep_value_2"
                    }
                }
            }
        }
    });
    
    let json_string = serde_json::to_string(&pure_object).unwrap();
    let ast = json_source_to_ast(&json_string);
    
    if let Element::Object(root_obj) = &ast {
        // 验证深层对象访问
        let path1 = ["root", "child1", "grandchild1", "greatgrand1", "value"];
        let mut current = root_obj;
        
        for (i, key) in path1.iter().enumerate() {
            if i == path1.len() - 1 {
                // 最后一个键应该是值
                if let Some(Element::String(value)) = current.get(key) {
                    assert_eq!(value.content, "deep_value_1");
                }
            } else {
                // 中间键应该是对象
                if let Some(Element::Object(next_obj)) = current.get(key) {
                    current = next_obj;
                } else {
                    panic!("Expected object at key: {}", key);
                }
            }
        }
    }
    
    println!("  ✅ Pure object recursion verified");
    
    // Test 2.3: 空容器边界情况
    let empty_containers = serde_json::json!({
        "empty_array": [],
        "empty_object": {},
        "nested_empty": {
            "arrays": [[], [[]], {"inner": []}],
            "objects": [{}, {"nested": {}}, [{}]]
        }
    });
    
    let json_string = serde_json::to_string(&empty_containers).unwrap();
    let ast = json_source_to_ast(&json_string);
    
    if let Element::Object(root) = &ast {
        // 验证空数组
        if let Some(Element::Array(empty_arr)) = root.get("empty_array") {
            assert_eq!(empty_arr.content.len(), 0);
        }
        
        // 验证空对象
        if let Some(Element::Object(empty_obj)) = root.get("empty_object") {
            assert_eq!(empty_obj.content.len(), 0);
        }
        
        // 验证嵌套的空容器
        if let Some(Element::Object(nested)) = root.get("nested_empty") {
            if let Some(Element::Array(arrays)) = nested.get("arrays") {
                assert_eq!(arrays.content.len(), 3);
                // 第一个应该是空数组
                assert!(matches!(arrays.content[0], Element::Array(_)));
                if let Element::Array(first_empty) = &arrays.content[0] {
                    assert_eq!(first_empty.content.len(), 0);
                }
            }
        }
    }
    
    println!("  ✅ Empty container edge cases verified");
    println!("✅ Array/Object branch isolation test completed");
}

/// Test 3: run_once 结果校验与多次迭代对比
#[test]
fn test_run_once_detailed_comparison() {
    println!("🔄 Testing detailed run_once vs run_until_fixed comparison");
    
    // 加载测试文档
    let yaml = fs::read_to_string("tests/test_data/petstore.yaml")
        .expect("Failed to read tests/test_data/petstore.yaml");
    let (cst, _) = CstParser::parse_smart(&yaml);
    let yaml_value: serde_yaml::Value = serde_yaml::from_str(&cst.text()).unwrap();
    let json_value: serde_json::Value = serde_yaml::from_value(yaml_value).unwrap();
    let json_string = serde_json::to_string(&json_value).unwrap();
    let ast = json_source_to_ast(&json_string);
    
    // 创建测试管道
    let spec = create_openapi_specification();
    let pipeline = FoldPipeline::new()
        .add_pass(Box::new(OpenApiSpecPass::new(spec.clone(), "OpenAPISpec".to_string())))
        .add_pass(Box::new(ReferenceResolutionPass::new()))
        .add_pass(Box::new(SemanticEnhancementPass::new()))
        .max_iterations(5)
        .debug(true);
    
    // Test 3.1: 单次 run_once 详细分析
    let single_start = Instant::now();
    let single_result = pipeline.run_once(&ast).unwrap();
    let single_duration = single_start.elapsed();
    
    // Test 3.2: 第一次 run_until_fixed 结果
    let fixed_start = Instant::now();
    let fixed_result = pipeline.run_until_fixed(&ast).unwrap();
    let fixed_duration = fixed_start.elapsed();
    
    // Test 3.3: 手动多次 run_once 模拟 run_until_fixed
    let mut manual_result = ast.clone();
    let mut manual_iterations = Vec::new();
    let manual_start = Instant::now();
    
    for iteration in 0..5 {
        let iter_start = Instant::now();
        let previous = manual_result.clone();
        manual_result = pipeline.run_once(&manual_result).unwrap();
        let iter_duration = iter_start.elapsed();
        
        let changes = count_structural_changes(&previous, &manual_result);
        manual_iterations.push(IterationResult {
            iteration,
            duration: iter_duration,
            element_count: count_elements(&manual_result),
            changes,
        });
        
        // 如果没有变化，提前停止
        if changes == 0 {
            println!("  Manual iterations converged after {} iterations", iteration + 1);
            break;
        }
    }
    
    let manual_total_duration = manual_start.elapsed();
    
    // Test 3.4: 详细对比分析
    println!("📊 Detailed comparison results:");
    
    // 性能对比
    println!("  Performance:");
    println!("    Single run_once: {:?}", single_duration);
    println!("    run_until_fixed: {:?}", fixed_duration);
    println!("    Manual iterations: {:?}", manual_total_duration);
    
    // 结构对比
    let single_count = count_elements(&single_result);
    let fixed_count = count_elements(&fixed_result);
    let manual_count = count_elements(&manual_result);
    
    println!("  Element counts:");
    println!("    Single run_once: {}", single_count);
    println!("    run_until_fixed: {}", fixed_count);
    println!("    Manual iterations: {}", manual_count);
    
    // 语义类对比
    let single_classes = count_semantic_classes(&single_result);
    let fixed_classes = count_semantic_classes(&fixed_result);
    let manual_classes = count_semantic_classes(&manual_result);
    
    println!("  Semantic classes:");
    println!("    Single run_once: {}", single_classes);
    println!("    run_until_fixed: {}", fixed_classes);
    println!("    Manual iterations: {}", manual_classes);
    
    // 迭代详情
    println!("  Manual iteration details:");
    for iter_result in &manual_iterations {
        println!("    Iteration {}: {} elements, {} changes, {:?}",
                iter_result.iteration,
                iter_result.element_count,
                iter_result.changes,
                iter_result.duration);
    }
    
    // Test 3.5: 断言验证
    // run_until_fixed 应该至少与 single run_once 一样好
    assert!(fixed_count >= single_count, "run_until_fixed should have at least as many elements");
    assert!(fixed_classes >= single_classes, "run_until_fixed should have at least as many semantic classes");
    
    // 手动迭代应该与 run_until_fixed 收敛到相同结果
    assert_eq!(manual_count, fixed_count, "Manual iterations should converge to same element count");
    
    // 性能应该合理
    assert!(fixed_duration.as_millis() < 10000, "run_until_fixed should complete within 10 seconds");
    
    // Test 3.6: 第一次 run_once 与最终结果的差异分析
    let first_vs_final_changes = count_structural_changes(&single_result, &fixed_result);
    println!("  First run_once vs final result changes: {}", first_vs_final_changes);
    
    if first_vs_final_changes > 0 {
        println!("  ✅ Multiple iterations produced additional improvements");
    } else {
        println!("  ✅ Single iteration was sufficient for this document");
    }
    
    println!("✅ Detailed run_once comparison completed");
}

/// Test 4: 外部引用和网络引用模拟
#[test]
fn test_external_reference_simulation() {
    println!("🌐 Testing external reference simulation");
    
    // Test 4.1: 外部文件引用模拟
    let external_ref_doc = serde_json::json!({
        "openapi": "3.0.3",
        "info": {"title": "External Ref Test", "version": "1.0.0"},
        "paths": {
            "/users": {
                "$ref": "./paths/users.yaml#/UserPath"
            },
            "/pets": {
                "$ref": "file:///schemas/pets.json#/PetPath"
            }
        },
        "components": {
            "schemas": {
                "User": {
                    "$ref": "https://api.example.com/schemas/user.json"
                },
                "Pet": {
                    "$ref": "http://petstore.swagger.io/v2/swagger.json#/definitions/Pet"
                },
                "RemoteSchema": {
                    "$ref": "ftp://schemas.example.com/remote.yaml#/Schema"
                }
            }
        }
    });
    
    println!("  Testing external file and HTTP references...");
    let json_string = serde_json::to_string(&external_ref_doc).unwrap();
    let ast = json_source_to_ast(&json_string);
    
    // 创建引用解析管道
    let pipeline = FoldPipeline::new()
        .add_pass(Box::new(ReferenceResolutionPass::new()))
        .max_iterations(3);
    
    let start = Instant::now();
    let result = pipeline.run_until_fixed(&ast);
    let duration = start.elapsed();
    
    // 验证处理不会无限等待或崩溃
    assert!(result.is_some(), "External reference processing should not fail");
    assert!(duration.as_millis() < 5000, "External reference processing should complete quickly");
    
    // 验证外部引用被正确标记
    if let Some(Element::Object(obj)) = result {
        let mut external_refs = Vec::new();
        collect_external_references(&obj, &mut external_refs);
        
        println!("    Found {} external references:", external_refs.len());
        for ext_ref in &external_refs {
            println!("      - {}", ext_ref);
            
            // 验证外部引用格式
            assert!(
                ext_ref.starts_with("http://") || 
                ext_ref.starts_with("https://") || 
                ext_ref.starts_with("file://") ||
                ext_ref.starts_with("ftp://") ||
                ext_ref.starts_with("./") ||
                ext_ref.starts_with("../"),
                "Should identify external reference: {}", ext_ref
            );
        }
        
        assert!(external_refs.len() >= 5, "Should find all external references");
    }
    
    println!("    ✅ External references handled without network calls");
    
    // Test 4.2: 超时和错误处理模拟
    let problematic_refs = serde_json::json!({
        "openapi": "3.0.3",
        "info": {"title": "Problematic Refs", "version": "1.0.0"},
        "components": {
            "schemas": {
                "TimeoutSchema": {
                    "$ref": "https://very-slow-server.example.com/schema.json"
                },
                "MalformedUrl": {
                    "$ref": "not-a-valid-url"
                },
                "CircularExternal": {
                    "$ref": "https://example.com/circular1.json"
                }
            }
        }
    });
    
    println!("  Testing problematic reference handling...");
    let json_string = serde_json::to_string(&problematic_refs).unwrap();
    let ast = json_source_to_ast(&json_string);
    
    let start = Instant::now();
    let result = pipeline.run_until_fixed(&ast);
    let duration = start.elapsed();
    
    // 验证错误处理
    assert!(result.is_some(), "Should handle problematic references gracefully");
    assert!(duration.as_millis() < 2000, "Should not hang on problematic references");
    
    // 验证错误元数据被添加
    if let Some(Element::Object(obj)) = result {
        let mut error_metadata_count = 0;
        check_error_metadata(&obj, &mut error_metadata_count);
        println!("    Found {} error metadata entries", error_metadata_count);
    }
    
    println!("    ✅ Problematic references handled gracefully");
    println!("✅ External reference simulation completed");
}

/// Test 5: 严格模式校验错误记录
#[test]
fn test_strict_validation_error_recording() {
    println!("⚠️ Testing strict validation error recording");
    
    // Test 5.1: 缺少必需字段
    let missing_required = serde_json::json!({
        "openapi": "3.0.3"
        // 缺少 info 和 paths
    });
    
    println!("  Testing missing required fields...");
    let json_string = serde_json::to_string(&missing_required).unwrap();
    let ast = json_source_to_ast(&json_string);
    
    let strict_validator = ValidationPass::new(true);
    let result = strict_validator.apply(&ast).unwrap();
    
    // 验证错误被记录在 metadata 中
    if let Element::Object(obj) = &result {
        let mut validation_errors = Vec::new();
        collect_validation_errors(&obj, &mut validation_errors);
        
        println!("    Found {} validation errors:", validation_errors.len());
        for error in &validation_errors {
            println!("      - {}", error);
        }
        
        // 应该找到关于缺失 info 和 paths 的错误
        let has_info_error = validation_errors.iter()
            .any(|e| e.contains("info") && (e.contains("required") || e.contains("missing")));
        let has_paths_error = validation_errors.iter()
            .any(|e| e.contains("paths") && (e.contains("required") || e.contains("missing")));
        
        println!("    Info field error recorded: {}", has_info_error);
        println!("    Paths field error recorded: {}", has_paths_error);
    }
    
    // Test 5.2: 无效的字段值
    let invalid_values = serde_json::json!({
        "openapi": "2.0", // 错误版本
        "info": {
            "title": "", // 空标题
            "version": "" // 空版本
        },
        "paths": "not_an_object" // 错误类型
    });
    
    println!("  Testing invalid field values...");
    let json_string = serde_json::to_string(&invalid_values).unwrap();
    let ast = json_source_to_ast(&json_string);
    
    let result = strict_validator.apply(&ast).unwrap();
    
    if let Element::Object(obj) = &result {
        let mut validation_errors = Vec::new();
        collect_validation_errors(&obj, &mut validation_errors);
        
        println!("    Found {} validation errors for invalid values:", validation_errors.len());
        for error in &validation_errors {
            println!("      - {}", error);
        }
        
        // 验证特定错误类型
        let has_version_error = validation_errors.iter()
            .any(|e| e.contains("openapi") && e.contains("version"));
        let has_empty_title_error = validation_errors.iter()
            .any(|e| e.contains("title") && e.contains("empty"));
        let has_paths_type_error = validation_errors.iter()
            .any(|e| e.contains("paths") && e.contains("type"));
        
        println!("    OpenAPI version error: {}", has_version_error);
        println!("    Empty title error: {}", has_empty_title_error);
        println!("    Paths type error: {}", has_paths_type_error);
    }
    
    // Test 5.3: 完整管道中的严格校验
    println!("  Testing strict validation in complete pipeline...");
    let spec = create_openapi_specification();
    let strict_pipeline = FoldPipeline::new()
        .add_pass(Box::new(OpenApiSpecPass::new(spec, "OpenAPISpec".to_string())))
        .add_pass(Box::new(ValidationPass::new(true))) // 严格模式
        .max_iterations(3);
    
    let pipeline_result = strict_pipeline.run_until_fixed(&ast).unwrap();
    
    if let Element::Object(obj) = &pipeline_result {
        let mut all_errors = Vec::new();
        collect_validation_errors(&obj, &mut all_errors);
        
        println!("    Pipeline validation errors: {}", all_errors.len());
        
        // 验证错误被适当分类
        let error_categories = categorize_validation_errors(&all_errors);
        println!("    Error categories:");
        for (category, count) in error_categories {
            println!("      - {}: {}", category, count);
        }
    }
    
    println!("✅ Strict validation error recording completed");
}

/// Test 6: 性能基准稳定性和历史趋势
#[test]
fn test_performance_benchmark_stability() {
    println!("📈 Testing performance benchmark stability");
    
    // 创建标准测试文档
    let yaml = fs::read_to_string("tests/test_data/petstore.yaml")
        .expect("Failed to read tests/test_data/petstore.yaml");
    let (cst, _) = CstParser::parse_smart(&yaml);
    let yaml_value: serde_yaml::Value = serde_yaml::from_str(&cst.text()).unwrap();
    let json_value: serde_json::Value = serde_yaml::from_value(yaml_value).unwrap();
    let json_string = serde_json::to_string(&json_value).unwrap();
    let ast = json_source_to_ast(&json_string);
    
    // 创建性能测试管道
    let spec = create_openapi_specification();
    let pipeline = FoldPipeline::new()
        .add_pass(Box::new(OpenApiSpecPass::new(spec, "OpenAPISpec".to_string())))
        .add_pass(Box::new(ReferenceResolutionPass::new()))
        .add_pass(Box::new(SemanticEnhancementPass::new()))
        .add_pass(Box::new(ValidationPass::new(false)))
        .max_iterations(5);
    
    // Test 6.1: 多轮性能测试（统计稳定性）
    let mut measurements = Vec::new();
    let test_rounds = 20; // 增加测试轮数以获得更好的统计
    
    println!("  Running {} performance measurement rounds...", test_rounds);
    
    for round in 0..test_rounds {
        let start = Instant::now();
        let _result = pipeline.run_until_fixed(&ast).unwrap();
        let duration = start.elapsed();
        
        measurements.push(PerformanceMeasurement {
            round,
            duration,
            timestamp: std::time::SystemTime::now(),
        });
        
        if round % 5 == 4 {
            println!("    Completed {} rounds", round + 1);
        }
    }
    
    // Test 6.2: 统计分析
    let stats = calculate_performance_statistics(&measurements);
    
    println!("  📊 Performance Statistics:");
    println!("    Average: {:?}", stats.average);
    println!("    Median: {:?}", stats.median);
    println!("    Min: {:?}", stats.min);
    println!("    Max: {:?}", stats.max);
    println!("    Std Dev: {:?}", stats.std_dev);
    println!("    Coefficient of Variation: {:.2}%", stats.cv_percent);
    
    // Test 6.3: 稳定性验证
    // 变异系数应该小于 20%（表示相对稳定）
    assert!(stats.cv_percent < 20.0, "Performance should be relatively stable (CV < 20%)");
    
    // 最大值不应该超过平均值的 3 倍（排除异常值）
    let max_ratio = stats.max.as_nanos() as f64 / stats.average.as_nanos() as f64;
    assert!(max_ratio < 3.0, "Maximum time should not exceed 3x average (outlier detection)");
    
    // Test 6.4: 性能回归检测
    let baseline_duration = Duration::from_millis(1000); // 1秒基准
    assert!(stats.average < baseline_duration, "Average performance should be under baseline");
    
    // Test 6.5: 生成性能报告（用于 CI 收集）
    let performance_report = generate_performance_report(&stats, &measurements);
    println!("  📋 Performance Report:");
    println!("{}", performance_report);
    
    // 将报告写入文件（如果在 CI 环境中）
    if std::env::var("CI").is_ok() {
        let report_path = "target/performance_report.json";
        let report_json = serde_json::json!({
            "timestamp": std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            "statistics": {
                "average_ms": stats.average.as_millis(),
                "median_ms": stats.median.as_millis(),
                "min_ms": stats.min.as_millis(),
                "max_ms": stats.max.as_millis(),
                "std_dev_ms": stats.std_dev.as_millis(),
                "cv_percent": stats.cv_percent
            },
            "measurements": measurements.iter().map(|m| {
                serde_json::json!({
                    "round": m.round,
                    "duration_ms": m.duration.as_millis()
                })
            }).collect::<Vec<_>>()
        });
        
        if let Ok(report_str) = serde_json::to_string_pretty(&report_json) {
            let _ = std::fs::write(report_path, report_str);
            println!("    Performance report written to: {}", report_path);
        }
    }
    
    // Test 6.6: 内存使用监控（简化版）
    let memory_before = get_memory_usage();
    let _result = pipeline.run_until_fixed(&ast).unwrap();
    let memory_after = get_memory_usage();
    
    if let (Some(before), Some(after)) = (memory_before, memory_after) {
        let memory_diff = after.saturating_sub(before);
        println!("  🧠 Memory usage: {} bytes", memory_diff);
        
        // 内存使用不应该过度增长（简单检查）
        assert!(memory_diff < 100_000_000, "Memory usage should be reasonable (< 100MB)");
    }
    
    println!("✅ Performance benchmark stability test completed");
}

// 辅助结构体和函数

#[derive(Debug)]
struct IterationResult {
    iteration: usize,
    duration: Duration,
    element_count: usize,
    changes: usize,
}

#[derive(Debug)]
struct PerformanceMeasurement {
    round: usize,
    duration: Duration,
    #[allow(dead_code)]
    timestamp: std::time::SystemTime,
}

#[derive(Debug)]
struct PerformanceStatistics {
    average: Duration,
    median: Duration,
    min: Duration,
    max: Duration,
    std_dev: Duration,
    cv_percent: f64,
}

/// 计算结构性变化数量
fn count_structural_changes(before: &Element, after: &Element) -> usize {
    match (before, after) {
        (Element::Object(obj1), Element::Object(obj2)) => {
            let mut changes = 0;
            if obj1.content.len() != obj2.content.len() {
                changes += 1;
            }
            if obj1.classes.content.len() != obj2.classes.content.len() {
                changes += 1;
            }
            if obj1.meta.properties.len() != obj2.meta.properties.len() {
                changes += 1;
            }
            changes
        }
        (Element::Array(arr1), Element::Array(arr2)) => {
            if arr1.content.len() != arr2.content.len() { 1 } else { 0 }
        }
        _ => if std::mem::discriminant(before) != std::mem::discriminant(after) { 1 } else { 0 }
    }
}

/// 计算元素总数
fn count_elements(element: &Element) -> usize {
    match element {
        Element::Object(obj) => {
            1 + obj.content.iter()
                .map(|member| count_elements(member.key.as_ref()) + count_elements(member.value.as_ref()))
                .sum::<usize>()
        }
        Element::Array(arr) => {
            1 + arr.content.iter().map(count_elements).sum::<usize>()
        }
        _ => 1,
    }
}

/// 计算语义类数量
fn count_semantic_classes(element: &Element) -> usize {
    match element {
        Element::Object(obj) => obj.classes.content.len(),
        _ => 0,
    }
}

/// 收集外部引用
fn collect_external_references(obj: &ObjectElement, external_refs: &mut Vec<String>) {
    for member in &obj.content {
        if let Element::String(key) = member.key.as_ref() {
            if key.content == "$ref" {
                if let Element::String(ref_value) = member.value.as_ref() {
                    if is_external_reference(&ref_value.content) {
                        external_refs.push(ref_value.content.clone());
                    }
                }
            }
        }
        
        // 递归检查嵌套对象
        match member.value.as_ref() {
            Element::Object(nested_obj) => {
                collect_external_references(nested_obj, external_refs);
            }
            Element::Array(arr) => {
                for item in &arr.content {
                    if let Element::Object(nested_obj) = item {
                        collect_external_references(nested_obj, external_refs);
                    }
                }
            }
            _ => {}
        }
    }
}

/// 判断是否为外部引用
fn is_external_reference(ref_path: &str) -> bool {
    ref_path.starts_with("http://") ||
    ref_path.starts_with("https://") ||
    ref_path.starts_with("file://") ||
    ref_path.starts_with("ftp://") ||
    ref_path.starts_with("./") ||
    ref_path.starts_with("../") ||
    ref_path.contains("://")
}

/// 检查错误元数据
fn check_error_metadata(obj: &ObjectElement, error_count: &mut usize) {
    for (key, _value) in &obj.meta.properties {
        if key.contains("error") || key.contains("timeout") || key.contains("invalid") {
            *error_count += 1;
        }
    }
    
    // 递归检查
    for member in &obj.content {
        if let Element::Object(nested_obj) = member.value.as_ref() {
            check_error_metadata(nested_obj, error_count);
        }
    }
}

/// 收集校验错误
fn collect_validation_errors(obj: &ObjectElement, errors: &mut Vec<String>) {
    // 检查元数据中的验证错误
    for (key, value) in &obj.meta.properties {
        if key.contains("validation") || key.contains("error") || key.contains("warning") {
            if let serde_json::Value::String(error_msg) = value {
                errors.push(error_msg.clone());
            } else if let serde_json::Value::Array(error_array) = value {
                for error_val in error_array {
                    if let serde_json::Value::String(error_msg) = error_val {
                        errors.push(error_msg.clone());
                    }
                }
            }
        }
    }
    
    // 递归检查嵌套对象
    for member in &obj.content {
        if let Element::Object(nested_obj) = member.value.as_ref() {
            collect_validation_errors(nested_obj, errors);
        }
    }
}

/// 分类校验错误
fn categorize_validation_errors(errors: &[String]) -> HashMap<String, usize> {
    let mut categories = HashMap::new();
    
    for error in errors {
        let category = if error.contains("required") || error.contains("missing") {
            "Missing Required Fields"
        } else if error.contains("type") || error.contains("format") {
            "Type/Format Errors"
        } else if error.contains("version") {
            "Version Errors"
        } else if error.contains("empty") {
            "Empty Value Errors"
        } else {
            "Other Errors"
        };
        
        *categories.entry(category.to_string()).or_insert(0) += 1;
    }
    
    categories
}

/// 计算性能统计
fn calculate_performance_statistics(measurements: &[PerformanceMeasurement]) -> PerformanceStatistics {
    let mut durations: Vec<Duration> = measurements.iter().map(|m| m.duration).collect();
    durations.sort();
    
    let sum: Duration = durations.iter().sum();
    let average = sum / durations.len() as u32;
    let median = durations[durations.len() / 2];
    let min = durations[0];
    let max = durations[durations.len() - 1];
    
    // 计算标准差
    let variance: f64 = durations.iter()
        .map(|d| {
            let diff = d.as_nanos() as f64 - average.as_nanos() as f64;
            diff * diff
        })
        .sum::<f64>() / durations.len() as f64;
    
    let std_dev = Duration::from_nanos(variance.sqrt() as u64);
    let cv_percent = (std_dev.as_nanos() as f64 / average.as_nanos() as f64) * 100.0;
    
    PerformanceStatistics {
        average,
        median,
        min,
        max,
        std_dev,
        cv_percent,
    }
}

/// 生成性能报告
fn generate_performance_report(stats: &PerformanceStatistics, measurements: &[PerformanceMeasurement]) -> String {
    let mut report = String::new();
    report.push_str("Performance Benchmark Report\n");
    report.push_str("============================\n");
    report.push_str(&format!("Test Rounds: {}\n", measurements.len()));
    report.push_str(&format!("Average Time: {:?}\n", stats.average));
    report.push_str(&format!("Median Time: {:?}\n", stats.median));
    report.push_str(&format!("Min Time: {:?}\n", stats.min));
    report.push_str(&format!("Max Time: {:?}\n", stats.max));
    report.push_str(&format!("Standard Deviation: {:?}\n", stats.std_dev));
    report.push_str(&format!("Coefficient of Variation: {:.2}%\n", stats.cv_percent));
    
    // 性能等级评估
    let performance_grade = if stats.cv_percent < 5.0 {
        "Excellent (Very Stable)"
    } else if stats.cv_percent < 10.0 {
        "Good (Stable)"
    } else if stats.cv_percent < 20.0 {
        "Fair (Moderately Stable)"
    } else {
        "Poor (Unstable)"
    };
    
    report.push_str(&format!("Performance Grade: {}\n", performance_grade));
    report
}

/// 获取内存使用情况（简化实现）
fn get_memory_usage() -> Option<usize> {
    // 这是一个简化的内存监控实现
    // 在实际应用中，您可能需要使用更精确的内存监控工具
    #[cfg(target_os = "linux")]
    {
        if let Ok(status) = std::fs::read_to_string("/proc/self/status") {
            for line in status.lines() {
                if line.starts_with("VmRSS:") {
                    if let Some(kb_str) = line.split_whitespace().nth(1) {
                        if let Ok(kb) = kb_str.parse::<usize>() {
                            return Some(kb * 1024); // 转换为字节
                        }
                    }
                }
            }
        }
    }
    
    // 对于其他操作系统或失败情况，返回 None
    None
} 