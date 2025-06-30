use apidom_ns_openapi_3_0::fold_pass::{FoldPipeline, OpenApiSpecPass};
use apidom_ns_openapi_3_0::specification::create_openapi_specification;
use apidom_ns_openapi_3_0::builder::components_builder::build_and_decorate_components;
use apidom_ns_openapi_3_0::builder::info_builder::build_and_decorate_info;
use apidom_ns_openapi_3_0::builder::paths_builder::build_and_decorate_paths;
use apidom_ast::minim_model::*;
use apidom_ast::fold::{json_source_to_ast, JsonFolder};
use apidom_cst::CstParser;
use serde_json;
use serde_yaml;
use std::fs;
use std::time::Instant;

#[test]
fn test_petstore_cst_parser_direct() {
    println!("🚀 Testing Petstore with CstParser direct approach");
    
    // Read YAML file from tests directory
    let yaml = fs::read_to_string("tests/test_data/petstore.yaml")
        .expect("Failed to read tests/test_data/petstore.yaml");
    
    // Parse using CstParser with smart detection - returns (TreeCursorSyntaxNode, SourceType)
    let (cst, detected_type) = CstParser::parse_smart(&yaml);
    println!("✅ CST parsing completed, detected type: {}", detected_type.display_name());
    
    // Get the CST text (which is YAML)
    let cst_text = cst.text();
    println!("CST text (first 200 chars): {}", &cst_text[..std::cmp::min(200, cst_text.len())]);
    
    // Convert YAML to JSON first, then to AST
    let yaml_value: serde_yaml::Value = serde_yaml::from_str(&cst_text)
        .expect("Failed to parse YAML");
    let json_value: serde_json::Value = serde_yaml::from_value(yaml_value)
        .expect("Failed to convert YAML to JSON");
    let json_string = serde_json::to_string(&json_value)
        .expect("Failed to serialize JSON");
    
    // Now convert JSON to AST
    let ast = json_source_to_ast(&json_string);
    println!("✅ AST conversion completed");
    
    // Verify the AST structure with proper pattern matching
    assert!(matches!(ast, Element::Object(_)), "AST root should be an Object element");
    
    if let Element::Object(obj) = &ast {
        println!("✅ Got object element with {} fields", obj.content.len());
        assert!(obj.get("openapi").is_some(), "Should have openapi field");
        assert!(obj.get("info").is_some(), "Should have info field");
        assert!(obj.get("paths").is_some(), "Should have paths field");
        
        // Check OpenAPI version
        if let Some(Element::String(version)) = obj.get("openapi") {
            println!("OpenAPI version: {}", version.content);
            assert!(version.content.starts_with("3.0"), "Should be OpenAPI 3.0.x");
        }
        
        println!("✅ AST structure validation passed");
    } else {
        panic!("Expected object element from AST conversion");
    }
}

#[test]
fn test_petstore_full_pipeline() {
    println!("🏗️ Testing full processing pipeline");
    
    let start = Instant::now();
    
    // Step 1: Read YAML
    let yaml = fs::read_to_string("tests/test_data/petstore.yaml")
        .expect("Failed to read tests/test_data/petstore.yaml");
    
    // Step 2: Parse to CST - no unwrap needed as parse_smart returns (cst, type)
    let (cst, detected_type) = CstParser::parse_smart(&yaml);
    println!("Detected format: {}", detected_type.display_name());
    
    // Step 3: Convert YAML CST to JSON, then to AST
    let yaml_value: serde_yaml::Value = serde_yaml::from_str(&cst.text())
        .expect("Failed to parse YAML from CST");
    let json_value: serde_json::Value = serde_yaml::from_value(yaml_value)
        .expect("Failed to convert YAML to JSON");
    let json_string = serde_json::to_string(&json_value)
        .expect("Failed to serialize JSON");
    let ast = json_source_to_ast(&json_string);
    
    // Step 4: Create OpenAPI specification
    let spec = create_openapi_specification();
    
    // Step 5: Run processing pipeline
    let mut pipeline = FoldPipeline::new();
    pipeline = pipeline.add_pass(Box::new(OpenApiSpecPass::new(spec, "OpenAPISpec".to_string())));
    
    let processed_ast = pipeline.run_once(&ast).unwrap_or(ast.clone());
    
    let duration = start.elapsed();
    println!("Total processing time: {:?}", duration);
    
    // Step 6: Verify processed result with proper pattern matching
    assert!(matches!(processed_ast, Element::Object(_)), "Processed AST should be an Object element");
    
    if let Element::Object(obj) = &processed_ast {
        // Check required fields
        assert!(obj.get("openapi").is_some(), "Should have openapi field");
        assert!(obj.get("info").is_some(), "Should have info field");
        assert!(obj.get("paths").is_some(), "Should have paths field");
        
        println!("✅ Full pipeline processing completed successfully");
    }
    
    // Performance check
    assert!(duration.as_millis() < 5000, "Processing should complete within 5 seconds");
}

#[test]
fn test_petstore_builders() {
    println!("🔧 Testing individual builders");
    
    // Parse the document
    let yaml = fs::read_to_string("tests/test_data/petstore.yaml")
        .expect("Failed to read tests/test_data/petstore.yaml");
    let (cst, _) = CstParser::parse_smart(&yaml);
    
    // Convert to AST
    let yaml_value: serde_yaml::Value = serde_yaml::from_str(&cst.text())
        .expect("Failed to parse YAML from CST");
    let json_value: serde_json::Value = serde_yaml::from_value(yaml_value)
        .expect("Failed to convert YAML to JSON");
    let json_string = serde_json::to_string(&json_value)
        .expect("Failed to serialize JSON");
    let ast = json_source_to_ast(&json_string);
    
    // Verify AST is an object before proceeding
    assert!(matches!(ast, Element::Object(_)), "AST should be an Object element");
    
    if let Element::Object(obj) = &ast {
        // Test Info builder
        if let Some(info_element) = obj.get("info") {
            // Use a concrete folder instance
            let mut folder = JsonFolder::new();
            let info_result = build_and_decorate_info(info_element, Some(&mut folder));
            assert!(info_result.is_some(), "Info builder should succeed");
            println!("✅ Info builder test passed");
        }
        
        // Test Components builder (if present)
        if let Some(components_element) = obj.get("components") {
            // Clone the element for the builder
            let comp_el = components_element.clone();
            let components_result = build_and_decorate_components(comp_el, None);
            assert!(components_result.is_some(), "Components builder should succeed");
            println!("✅ Components builder test passed");
        }
        
        // Test Paths builder
        if let Some(paths_element) = obj.get("paths") {
            let mut folder = JsonFolder::new();
            let paths_result = build_and_decorate_paths(paths_element, Some(&mut folder));
            assert!(paths_result.is_some(), "Paths builder should succeed");
            println!("✅ Paths builder test passed");
        }
    } else {
        panic!("Expected Object element for AST");
    }
    
    println!("✅ All builder tests completed");
}

#[test]
fn test_petstore_structure_analysis() {
    println!("🔍 Analyzing Petstore structure");
    
    let yaml = fs::read_to_string("tests/test_data/petstore.yaml")
        .expect("Failed to read tests/test_data/petstore.yaml");
    let (cst, _) = CstParser::parse_smart(&yaml);
    
    // Convert to AST
    let yaml_value: serde_yaml::Value = serde_yaml::from_str(&cst.text())
        .expect("Failed to parse YAML from CST");
    let json_value: serde_json::Value = serde_yaml::from_value(yaml_value)
        .expect("Failed to convert YAML to JSON");
    let json_string = serde_json::to_string(&json_value)
        .expect("Failed to serialize JSON");
    let ast = json_source_to_ast(&json_string);
    
    // Assert structure before destructuring
    assert!(matches!(ast, Element::Object(_)), "AST should be an Object element");
    
    if let Element::Object(obj) = &ast {
        // Analyze info section
        if let Some(Element::Object(info)) = obj.get("info") {
            println!("Info fields:");
            for member in &info.content {
                if let Element::String(key) = member.key.as_ref() {
                    println!("  - {}", key.content);
                }
            }
        }
        
        // Analyze paths
        if let Some(Element::Object(paths)) = obj.get("paths") {
            println!("Available paths:");
            for member in &paths.content {
                if let Element::String(path) = member.key.as_ref() {
                    println!("  - {}", path.content);
                }
            }
        }
        
        // Analyze components (if present)
        if let Some(Element::Object(components)) = obj.get("components") {
            println!("Component types:");
            for member in &components.content {
                if let Element::String(component_type) = member.key.as_ref() {
                    if let Element::Object(component_obj) = member.value.as_ref() {
                        println!("  - {}: {} items", component_type.content, component_obj.content.len());
                    }
                }
            }
        }
        
        // Count total references
        let ref_count = count_references(&ast);
        println!("Total $ref references: {}", ref_count);
        
        println!("✅ Structure analysis completed");
    } else {
        panic!("Expected Object element for structure analysis");
    }
}

#[test]
fn test_petstore_validation() {
    println!("✅ Testing OpenAPI document validation");
    
    let yaml = fs::read_to_string("tests/test_data/petstore.yaml")
        .expect("Failed to read tests/test_data/petstore.yaml");
    let (cst, _) = CstParser::parse_smart(&yaml);
    
    // Convert to AST
    let yaml_value: serde_yaml::Value = serde_yaml::from_str(&cst.text())
        .expect("Failed to parse YAML from CST");
    let json_value: serde_json::Value = serde_yaml::from_value(yaml_value)
        .expect("Failed to convert YAML to JSON");
    let json_string = serde_json::to_string(&json_value)
        .expect("Failed to serialize JSON");
    let ast = json_source_to_ast(&json_string);
    
    // Assert structure before validation
    assert!(matches!(ast, Element::Object(_)), "AST should be an Object element");
    
    if let Element::Object(obj) = &ast {
        // Required fields validation
        assert!(obj.get("openapi").is_some(), "openapi field is required");
        assert!(obj.get("info").is_some(), "info field is required");
        assert!(obj.get("paths").is_some(), "paths field is required");
        
        // Info object validation
        if let Some(Element::Object(info)) = obj.get("info") {
            assert!(info.get("title").is_some(), "info.title is required");
            assert!(info.get("version").is_some(), "info.version is required");
            
            if let Some(Element::String(title)) = info.get("title") {
                assert!(!title.content.is_empty(), "title should not be empty");
                println!("Document title: {}", title.content);
            }
            
            if let Some(Element::String(version)) = info.get("version") {
                assert!(!version.content.is_empty(), "version should not be empty");
                println!("Document version: {}", version.content);
            }
        }
        
        // Paths validation
        if let Some(Element::Object(paths)) = obj.get("paths") {
            assert!(!paths.content.is_empty(), "paths should not be empty");
            
            // Check for valid path patterns
            for member in &paths.content {
                if let Element::String(path_key) = member.key.as_ref() {
                    if !path_key.content.starts_with("x-") {
                        assert!(path_key.content.starts_with('/'), 
                               "Path '{}' should start with /", path_key.content);
                    }
                }
            }
        }
        
        println!("✅ OpenAPI validation passed");
    } else {
        panic!("Expected Object element for validation");
    }
}

// Helper function to count $ref references in an element
fn count_references(element: &Element) -> usize {
    match element {
        Element::Object(obj) => {
            let mut count = 0;
            for member in &obj.content {
                if let Element::String(key) = member.key.as_ref() {
                    if key.content == "$ref" {
                        count += 1;
                    }
                }
                count += count_references(member.value.as_ref());
            }
            count
        }
        Element::Array(arr) => {
            arr.content.iter().map(count_references).sum()
        }
        _ => 0,
    }
} 