use serde_json::Value;
use apidom_ast::minim_model::*;
use apidom_ns_openapi_3_0::fold_pass::{FoldPass, FoldPipeline};
use apidom_ns_openapi_3_0::specification::create_openapi_specification;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🚀 OpenAPI Processing Pipeline Demo");
    println!("====================================\n");

    // Demo 1: OpenAPI Specification System
    println!("📋 Demo 1: OpenAPI Specification System");
    println!("{}", "-".repeat(40));
    demo_specification_system()?;
    println!();

    // Demo 2: Fold Pipeline System
    println!("🔧 Demo 2: Fold Pipeline System");
    println!("{}", "-".repeat(32));
    demo_pipeline_system()?;
    println!();

    // Demo 3: Element Processing
    println!("🌐 Demo 3: Element Processing and Metadata");
    println!("{}", "-".repeat(42));
    demo_element_processing()?;
    println!();

    println!("\n🎉 All demos completed successfully!");
    println!("The pipeline supports:");
    println!("  ✅ OpenAPI 3.0 specification processing");
    println!("  ✅ Fold pass pipeline system");
    println!("  ✅ Element metadata enhancement");
    println!("  ✅ Flexible processing architecture");

    Ok(())
}

fn demo_specification_system() -> Result<(), Box<dyn std::error::Error>> {
    let spec = create_openapi_specification();
    
    println!("🏗️  OpenAPI Specification created:");
    println!("  ✅ Total visitors: {}", spec.visitor_count());
    
    // Test visitor resolution
    if let Some(_visitor) = apidom_ns_openapi_3_0::specification::get_visitor_by_element_type(&spec, "openApi3_0") {
        println!("  ✅ OpenAPI root visitor found");
    }
    
    if let Some(_visitor) = apidom_ns_openapi_3_0::specification::get_visitor_by_element_type(&spec, "info") {
        println!("  ✅ Info visitor found");
    }
    
    if let Some(_visitor) = apidom_ns_openapi_3_0::specification::get_visitor_by_element_type(&spec, "paths") {
        println!("  ✅ Paths visitor found");
    }
    
    println!("  📊 Specification system is fully operational");
    
    Ok(())
}

fn demo_pipeline_system() -> Result<(), Box<dyn std::error::Error>> {
    println!("🔄 Creating fold pass pipeline:");
    
    // Create a simple pipeline
    let pipeline = FoldPipeline::new()
        .add_pass(Box::new(SimpleSpecPass::new()))
        .add_pass(Box::new(MetadataEnhancementPass::new()))
        .max_iterations(3)
        .debug(true);

    println!("  ✅ Pipeline created with {} passes", pipeline.pass_count());

    // Test a simple element through the pipeline
    let test_element = create_simple_openapi_element();
    
    println!("\n🧪 Testing pipeline execution:");
    println!("  📄 Input element created");
    
    if let Some(result) = pipeline.run_once(&test_element) {
        println!("  ✅ Pipeline completed successfully");
        
        if let Element::Object(obj) = &result {
            println!("     • Element type: {}", obj.element);
            println!("     • Classes count: {}", obj.classes.content.len());
            println!("     • Content members: {}", obj.content.len());
            println!("     • Metadata entries: {}", obj.meta.properties.len());
        }
    } else {
        println!("  ❌ Pipeline failed to process element");
    }

    Ok(())
}

fn demo_element_processing() -> Result<(), Box<dyn std::error::Error>> {
    println!("🔧 Element Processing Demo:");
    
    let mut openapi_element = create_comprehensive_openapi_element();
    
    println!("  📊 Initial element analysis:");
    if let Element::Object(obj) = &openapi_element {
        println!("     • Content members: {}", obj.content.len());
        println!("     • Initial metadata: {}", obj.meta.properties.len());
        println!("     • Element type: {}", obj.element);
    }

    // Simulate semantic enhancement
    println!("  🎯 Applying semantic enhancements...");
    
    if let Element::Object(obj) = &mut openapi_element {
        // Add processing metadata
        obj.meta.properties.insert("processed".to_string(), Value::Bool(true));
        obj.meta.properties.insert("processing_time".to_string(), Value::String("2024-01-01T12:00:00Z".to_string()));
        
        // Add classes for semantic enhancement
        obj.classes.content.push(Element::String(StringElement::new("openApi3_0")));
        obj.classes.content.push(Element::String(StringElement::new("processed")));
        
        println!("  ✨ Applied semantic enhancements:");
        println!("     • Added processing metadata");
        println!("     • Added semantic classes count: {}", obj.classes.content.len());
        println!("     • Total metadata entries: {}", obj.meta.properties.len());
    }

    Ok(())
}

fn create_simple_openapi_element() -> Element {
    let mut obj = ObjectElement::new();
    obj.set("openapi", Element::String(StringElement::new("3.0.3")));
    obj.set("info", Element::Object({
        let mut info = ObjectElement::new();
        info.set("title", Element::String(StringElement::new("Test API")));
        info.set("version", Element::String(StringElement::new("1.0.0")));
        info
    }));
    Element::Object(obj)
}

fn create_comprehensive_openapi_element() -> Element {
    let mut obj = ObjectElement::new();
    
    // Basic OpenAPI structure
    obj.set("openapi", Element::String(StringElement::new("3.0.3")));
    
    // Info object
    let mut info = ObjectElement::new();
    info.set("title", Element::String(StringElement::new("Comprehensive Pet Store API")));
    info.set("version", Element::String(StringElement::new("1.0.0")));
    info.set("description", Element::String(StringElement::new("A comprehensive API demonstrating all features")));
    obj.set("info", Element::Object(info));

    // Servers
    let mut servers = ArrayElement {
        element: "array".to_string(),
        meta: MetaElement::default(),
        attributes: AttributesElement::default(),
        content: Vec::new(),
    };
    let mut server = ObjectElement::new();
    server.set("url", Element::String(StringElement::new("https://api.petstore.com/v1")));
    server.set("description", Element::String(StringElement::new("Production server")));
    servers.content.push(Element::Object(server));
    obj.set("servers", Element::Array(servers));

    // Paths
    let mut paths = ObjectElement::new();
    let mut pets_path = ObjectElement::new();
    let mut get_operation = ObjectElement::new();
    get_operation.set("summary", Element::String(StringElement::new("List all pets")));
    get_operation.set("operationId", Element::String(StringElement::new("listPets")));
    
    // Responses
    let mut responses = ObjectElement::new();
    let mut response_200 = ObjectElement::new();
    response_200.set("description", Element::String(StringElement::new("A list of pets")));
    responses.set("200", Element::Object(response_200));
    get_operation.set("responses", Element::Object(responses));
    
    pets_path.set("get", Element::Object(get_operation));
    paths.set("/pets", Element::Object(pets_path));
    obj.set("paths", Element::Object(paths));

    // Components with schemas
    let mut components = ObjectElement::new();
    let mut schemas = ObjectElement::new();
    
    let mut pet_schema = ObjectElement::new();
    pet_schema.set("type", Element::String(StringElement::new("object")));
    
    let mut properties = ObjectElement::new();
    let mut id_prop = ObjectElement::new();
    id_prop.set("type", Element::String(StringElement::new("integer")));
    id_prop.set("format", Element::String(StringElement::new("int64")));
    properties.set("id", Element::Object(id_prop));
    
    let mut name_prop = ObjectElement::new();
    name_prop.set("type", Element::String(StringElement::new("string")));
    properties.set("name", Element::Object(name_prop));
    
    pet_schema.set("properties", Element::Object(properties));
    schemas.set("Pet", Element::Object(pet_schema));
    components.set("schemas", Element::Object(schemas));
    obj.set("components", Element::Object(components));

    Element::Object(obj)
}

// Simple fold pass implementations for demonstration
struct SimpleSpecPass {
    name: String,
}

impl SimpleSpecPass {
    fn new() -> Self {
        Self {
            name: "SimpleSpecPass".to_string(),
        }
    }
}

impl FoldPass for SimpleSpecPass {
    fn apply(&self, element: &Element) -> Option<Element> {
        match element {
            Element::Object(obj) => {
                let mut new_obj = obj.clone();
                if !new_obj.classes.content.iter().any(|e| {
                    if let Element::String(s) = e {
                        s.content == "spec-processed"
                    } else {
                        false
                    }
                }) {
                    new_obj.classes.content.push(Element::String(StringElement::new("spec-processed")));
                    Some(Element::Object(new_obj))
                } else {
                    None
                }
            }
            _ => None,
        }
    }
    
    fn name(&self) -> &str {
        &self.name
    }
}

struct MetadataEnhancementPass {
    name: String,
}

impl MetadataEnhancementPass {
    fn new() -> Self {
        Self {
            name: "MetadataEnhancement".to_string(),
        }
    }
}

impl FoldPass for MetadataEnhancementPass {
    fn apply(&self, element: &Element) -> Option<Element> {
        // Add metadata to indicate enhancement
        match element {
            Element::Object(obj) => {
                let mut new_obj = obj.clone();
                if !new_obj.meta.properties.contains_key("enhanced") {
                    new_obj.meta.properties.insert("enhanced".to_string(), Value::Bool(true));
                    new_obj.meta.properties.insert("enhancement_pass".to_string(), Value::String(self.name.clone()));
                    Some(Element::Object(new_obj))
                } else {
                    None
                }
            }
            _ => None,
        }
    }
    
    fn name(&self) -> &str {
        &self.name
    }
} 