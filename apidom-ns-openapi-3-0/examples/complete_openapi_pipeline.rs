use apidom_ns_openapi_3_0::fold_pass::{create_openapi_pipeline, create_strict_openapi_pipeline, enhance_element_with_metadata};
use apidom_ns_openapi_3_0::specification::create_openapi_specification;
use apidom_ast::{Element, ObjectElement, StringElement, ArrayElement};

fn main() {
    println!("=== Complete OpenAPI Pipeline Demo ===\n");
    
    // 1. Create OpenAPI specification with fixed fields support
    let spec = create_openapi_specification();
    println!("✓ Created OpenAPI specification with {} visitors", spec.visitor_count());
    
    // 2. Create pipeline with fold passes
    let pipeline = create_openapi_pipeline(spec.clone());
    println!("✓ Created standard pipeline with {} passes", pipeline.pass_count());
    
    let strict_pipeline = create_strict_openapi_pipeline(spec);
    println!("✓ Created strict pipeline with {} passes", strict_pipeline.pass_count());
    
    // 3. Create sample OpenAPI document
    let sample_doc = create_sample_openapi_document();
    println!("✓ Created sample OpenAPI document");
    
    // 4. Process through pipeline
    println!("\n=== Processing through Standard Pipeline ===");
    if let Some(processed) = pipeline.run_until_fixed(&sample_doc) {
        println!("✓ Pipeline processing completed successfully");
        analyze_processed_element(&processed);
    } else {
        println!("✗ Pipeline processing failed");
    }
    
    // 5. Process through strict pipeline
    println!("\n=== Processing through Strict Pipeline ===");
    if let Some(strict_processed) = strict_pipeline.run_until_fixed(&sample_doc) {
        println!("✓ Strict pipeline processing completed successfully");
        analyze_processed_element(&strict_processed);
    } else {
        println!("✗ Strict pipeline processing failed");
    }
    
    // 6. Demonstrate metadata enhancement
    println!("\n=== Metadata Enhancement Demo ===");
    demonstrate_metadata_enhancement();
    
    // 7. Demonstrate reference resolution
    println!("\n=== Reference Resolution Demo ===");
    demonstrate_reference_resolution();
    
    println!("\n=== Complete Pipeline Demo Finished ===");
}

fn create_sample_openapi_document() -> Element {
    let mut root = ObjectElement::new();
    root.set_element_type("openApi3_0");
    
    // Add openapi version
    root.set("openapi", Element::String(StringElement::new("3.0.3")));
    
    // Add info object
    let mut info = ObjectElement::new();
    info.set_element_type("info");
    info.set("title", Element::String(StringElement::new("Sample API")));
    info.set("version", Element::String(StringElement::new("1.0.0")));
    info.set("description", Element::String(StringElement::new("A sample API for demonstration")));
    root.set("info", Element::Object(info));
    
    // Add paths
    let mut paths = ObjectElement::new();
    paths.set_element_type("paths");
    
    // Add a path item with reference
    let mut path_item = ObjectElement::new();
    path_item.set_element_type("pathItem");
    path_item.set("$ref", Element::String(StringElement::new("#/components/pathItems/UserPath")));
    paths.set("/users/{id}", Element::Object(path_item));
    
    // Add a direct path item
    let mut direct_path = ObjectElement::new();
    direct_path.set_element_type("pathItem");
    
    let mut get_operation = ObjectElement::new();
    get_operation.set_element_type("operation");
    get_operation.set("summary", Element::String(StringElement::new("Get user")));
    get_operation.set("operationId", Element::String(StringElement::new("getUser")));
    direct_path.set("get", Element::Object(get_operation));
    
    paths.set("/users", Element::Object(direct_path));
    root.set("paths", Element::Object(paths));
    
    // Add components
    let mut components = ObjectElement::new();
    components.set_element_type("components");
    
    // Add schemas
    let mut schemas = ObjectElement::new();
    let mut user_schema = ObjectElement::new();
    user_schema.set_element_type("schema");
    user_schema.set("type", Element::String(StringElement::new("object")));
    
    let mut properties = ObjectElement::new();
    let mut id_property = ObjectElement::new();
    id_property.set_element_type("schema");
    id_property.set("type", Element::String(StringElement::new("integer")));
    properties.set("id", Element::Object(id_property));
    
    let mut name_property = ObjectElement::new();
    name_property.set_element_type("schema");
    name_property.set("type", Element::String(StringElement::new("string")));
    properties.set("name", Element::Object(name_property));
    
    user_schema.set("properties", Element::Object(properties));
    schemas.set("User", Element::Object(user_schema));
    
    // Add schema reference
    let mut user_ref = ObjectElement::new();
    user_ref.set_element_type("reference");
    user_ref.set("$ref", Element::String(StringElement::new("#/components/schemas/User")));
    schemas.set("UserRef", Element::Object(user_ref));
    
    components.set("schemas", Element::Object(schemas));
    
    // Add specification extension
    components.set("x-custom-extension", Element::String(StringElement::new("custom-value")));
    
    root.set("components", Element::Object(components));
    
    Element::Object(root)
}

fn analyze_processed_element(element: &Element) {
    match element {
        Element::Object(obj) => {
            println!("  📊 Element Analysis:");
            println!("    - Type: {}", obj.element);
            println!("    - Classes: {:?}", obj.classes.content.len());
            
            // Show classes
            if !obj.classes.content.is_empty() {
                print!("    - Class names: ");
                for (i, class) in obj.classes.content.iter().enumerate() {
                    if let Element::String(s) = class {
                        if i > 0 { print!(", "); }
                        print!("{}", s.content);
                    }
                }
                println!();
            }
            
            println!("    - Content members: {}", obj.content.len());
            
            // Analyze key members
            for member in &obj.content {
                if let Element::String(key) = member.key.as_ref() {
                    match member.value.as_ref() {
                        Element::Object(child_obj) => {
                            println!("    - {}: {} ({})", key.content, child_obj.element, child_obj.classes.content.len());
                        }
                        Element::String(s) => {
                            println!("    - {}: \"{}\"", key.content, s.content);
                        }
                        _ => {
                            println!("    - {}: {:?}", key.content, member.value);
                        }
                    }
                }
            }
        }
        _ => {
            println!("  📊 Non-object element: {:?}", element);
        }
    }
}

fn demonstrate_metadata_enhancement() {
    // Create various OpenAPI elements and enhance them
    let mut elements = vec![
        create_openapi_element(),
        create_info_element(),
        create_schema_element(),
        create_reference_element(),
        create_component_with_extension(),
    ];
    
    for (i, element) in elements.iter_mut().enumerate() {
        println!("  Element {}: Before enhancement", i + 1);
        if let Element::Object(obj) = element {
            println!("    - Type: {}", obj.element);
            println!("    - Classes: {}", obj.classes.content.len());
        }
        
        // Enhance with metadata
        if let Err(e) = enhance_element_with_metadata(element) {
            println!("    ✗ Enhancement failed: {}", e);
        } else {
            println!("    ✓ Enhancement successful");
            if let Element::Object(obj) = element {
                println!("    - Classes after: {}", obj.classes.content.len());
                for class in &obj.classes.content {
                    if let Element::String(s) = class {
                        println!("      - {}", s.content);
                    }
                }
            }
        }
        println!();
    }
}

fn demonstrate_reference_resolution() {
    // Create elements with references
    let mut ref_element = ObjectElement::new();
    ref_element.set_element_type("reference");
    ref_element.set("$ref", Element::String(StringElement::new("#/components/schemas/User")));
    
    let mut schema_with_ref = ObjectElement::new();
    schema_with_ref.set_element_type("schema");
    schema_with_ref.set("allOf", {
        let mut array = ArrayElement::new_empty();
        array.content.push(Element::Object(ref_element));
        Element::Array(array)
    });
    
    println!("  📎 Reference Resolution:");
    println!("    - Created schema with reference");
    println!("    - Reference target: #/components/schemas/User");
    println!("    - In a full implementation, this would resolve to the actual schema");
    
    // Enhance with metadata to show reference detection
    let mut element = Element::Object(schema_with_ref);
    if enhance_element_with_metadata(&mut element).is_ok() {
        if let Element::Object(obj) = element {
            println!("    - Classes added: {}", obj.classes.content.len());
            for class in &obj.classes.content {
                if let Element::String(s) = class {
                    println!("      - {}", s.content);
                }
            }
        }
    }
}

fn create_openapi_element() -> Element {
    let mut obj = ObjectElement::new();
    obj.set_element_type("openapi");
    obj.set("openapi", Element::String(StringElement::new("3.0.3")));
    Element::Object(obj)
}

fn create_info_element() -> Element {
    let mut obj = ObjectElement::new();
    obj.set_element_type("info");
    obj.set("title", Element::String(StringElement::new("Test API")));
    obj.set("version", Element::String(StringElement::new("1.0.0")));
    Element::Object(obj)
}

fn create_schema_element() -> Element {
    let mut obj = ObjectElement::new();
    obj.set_element_type("schema");
    obj.set("type", Element::String(StringElement::new("object")));
    Element::Object(obj)
}

fn create_reference_element() -> Element {
    let mut obj = ObjectElement::new();
    obj.set_element_type("reference");
    obj.set("$ref", Element::String(StringElement::new("#/components/schemas/User")));
    Element::Object(obj)
}

fn create_component_with_extension() -> Element {
    let mut obj = ObjectElement::new();
    obj.set_element_type("components");
    obj.set("x-custom-property", Element::String(StringElement::new("custom-value")));
    Element::Object(obj)
} 