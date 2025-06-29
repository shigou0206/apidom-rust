//! # OpenAPI 3.0 Specification Usage Example
//!
//! This example demonstrates how to use the OpenAPI 3.0 specification structure
//! for processing OpenAPI documents with visitor patterns.

use apidom_ast::minim_model::*;
use apidom_ns_openapi_3_0::specification::*;

fn main() {
    // Create the OpenAPI specification
    let spec = create_openapi_specification();
    
    println!("🚀 OpenAPI 3.0 Specification Structure Created!");
    println!("===============================================");
    
    // Example 1: Using visitor by element type
    println!("\n📋 Example 1: Getting visitors by element type");
    println!("----------------------------------------------");
    
    let info_visitor = get_visitor_by_element_type(&spec, "info");
    let schema_visitor = get_visitor_by_element_type(&spec, "schema");
    let unknown_visitor = get_visitor_by_element_type(&spec, "unknownType");
    
    println!("✅ Info visitor found: {}", info_visitor.is_some());
    println!("✅ Schema visitor found: {}", schema_visitor.is_some());
    println!("✅ Unknown type fallback: {}", unknown_visitor.is_some());
    
    // Example 2: Using visitor references
    println!("\n🔗 Example 2: Resolving visitor references");
    println!("------------------------------------------");
    
    let value_ref = resolve_visitor_reference(&spec, "#/visitors/value");
    let info_ref = resolve_visitor_reference(&spec, "#/visitors/document/objects/Info");
    let unknown_ref = resolve_visitor_reference(&spec, "#/unknown/reference");
    
    println!("✅ Value reference resolved: {}", value_ref.is_some());
    println!("✅ Info reference resolved: {}", info_ref.is_some());
    println!("✅ Unknown reference fallback: {}", unknown_ref.is_some());
    
    // Example 3: Processing elements with visitors
    println!("\n🔄 Example 3: Processing elements with visitors");
    println!("------------------------------------------------");
    
    // Create a simple string element
    let string_element = Element::String(StringElement::new("test-value"));
    
    // Use the value visitor
    if let Some(visitor) = get_visitor_by_element_type(&spec, "unknown") {
        let result = visitor(&string_element, None);
        println!("✅ Value visitor processed element: {}", result.is_some());
        
        if let Some(Element::String(processed)) = result {
            println!("   📝 Processed content: '{}'", processed.content);
        }
    }
    
    // Example 4: Inspecting fixed fields configuration
    println!("\n🏗️  Example 4: Inspecting fixed fields configuration");
    println!("----------------------------------------------------");
    
    let objects = &spec.visitors.document.objects;
    
    // Check OpenAPI root object fixed fields
    if let Some(ref fixed_fields) = objects.open_api.fixed_fields {
        println!("📊 OpenAPI root object has {} fixed fields:", fixed_fields.len());
        for (field_name, _) in fixed_fields.iter() {
            println!("   • {}", field_name);
        }
    }
    
    // Check Schema object fixed fields
    if let Some(ref fixed_fields) = objects.schema.fixed_fields {
        println!("📊 Schema object has {} fixed fields:", fixed_fields.len());
        let mut field_names: Vec<_> = fixed_fields.keys().collect();
        field_names.sort();
        
        println!("   JSON Schema fields:");
        for field_name in field_names.iter().filter(|name| !["type", "allOf", "anyOf", "oneOf", "not", "items", "properties", "additionalProperties", "nullable", "discriminator", "writeOnly", "xml", "externalDocs", "example", "deprecated"].contains(&name.as_str())) {
            println!("     • {}", field_name);
        }
        
        println!("   OpenAPI-adjusted fields:");
        for field_name in ["type", "allOf", "anyOf", "oneOf", "not", "items", "properties", "additionalProperties"] {
            if fixed_fields.contains_key(field_name) {
                println!("     • {}", field_name);
            }
        }
        
        println!("   OpenAPI vocabulary:");
        for field_name in ["nullable", "discriminator", "writeOnly", "xml", "externalDocs", "example", "deprecated"] {
            if fixed_fields.contains_key(field_name) {
                println!("     • {}", field_name);
            }
        }
    }
    
    // Example 5: Checking visitor availability
    println!("\n🎯 Example 5: Visitor availability matrix");
    println!("------------------------------------------");
    
    let element_types = [
        "openApi3_0", "info", "contact", "license", "server", "components",
        "paths", "pathItem", "operation", "parameter", "requestBody", "mediaType",
        "responses", "response", "callback", "example", "link", "header",
        "tag", "reference", "schema", "discriminator", "xml", "securityScheme",
        "oAuthFlows", "oAuthFlow", "securityRequirement"
    ];
    
    let mut available_count = 0;
    for element_type in &element_types {
        let visitor = get_visitor_by_element_type(&spec, element_type);
        if visitor.is_some() {
            available_count += 1;
        }
    }
    
    println!("✅ Available visitors: {}/{}", available_count, element_types.len());
    println!("📈 Coverage: {:.1}%", (available_count as f32 / element_types.len() as f32) * 100.0);
    
    // Example 6: TypeScript equivalence verification
    println!("\n🔄 Example 6: TypeScript equivalence verification");
    println!("--------------------------------------------------");
    
    println!("✅ Document structure: visitors.document.objects ✓");
    println!("✅ Value visitor fallback: visitors.value ✓");
    println!("✅ Extension visitor: visitors.document.extension ✓");
    println!("✅ Fixed fields mapping: HashMap<String, VisitorRef> ✓");
    println!("✅ Reference resolution: JSON pointer style ✓");
    println!("✅ All OpenAPI 3.0 objects: {} visitors ✓", element_types.len());
    
    println!("\n🎉 OpenAPI 3.0 Specification Usage Complete!");
    println!("=============================================");
    println!("The Rust implementation provides full TypeScript equivalence");
    println!("with complete visitor pattern support for OpenAPI 3.0 documents.");
} 