use crate::specification::{OpenApiSpecification, apply_fixed_fields_visitor};
use apidom_ast::Element;
use std::collections::HashMap;

/// FoldPass represents a single transformation pass over the document
pub trait FoldPass: Send + Sync {
    /// Apply the pass to an element
    fn apply(&self, element: &Element) -> Option<Element>;
    
    /// Get the name of this pass for debugging
    fn name(&self) -> &str;
    
    /// Check if this pass should continue running
    fn should_continue(&self, previous: &Element, current: &Element) -> bool {
        // Default: continue if elements are different
        !elements_equal(previous, current)
    }
}

/// Pipeline for running multiple fold passes
pub struct FoldPipeline {
    passes: Vec<Box<dyn FoldPass>>,
    max_iterations: usize,
    debug: bool,
}

impl FoldPipeline {
    /// Create a new fold pipeline
    pub fn new() -> Self {
        Self {
            passes: Vec::new(),
            max_iterations: 10,
            debug: false,
        }
    }
    
    /// Add a pass to the pipeline
    pub fn add_pass(mut self, pass: Box<dyn FoldPass>) -> Self {
        self.passes.push(pass);
        self
    }
    
    /// Set maximum iterations for run_until_fixed
    pub fn max_iterations(mut self, max: usize) -> Self {
        self.max_iterations = max;
        self
    }
    
    /// Enable debug output
    pub fn debug(mut self, enabled: bool) -> Self {
        self.debug = enabled;
        self
    }
    
    /// Get the number of passes in this pipeline
    pub fn pass_count(&self) -> usize {
        self.passes.len()
    }
    
    /// Run all passes once
    pub fn run_once(&self, element: &Element) -> Option<Element> {
        let mut current = element.clone();
        
        for pass in &self.passes {
            if self.debug {
                println!("Running pass: {}", pass.name());
            }
            
            if let Some(transformed) = pass.apply(&current) {
                current = transformed;
            }
        }
        
        Some(current)
    }
    
    /// Run passes until no more changes occur (fixed point)
    pub fn run_until_fixed(&self, element: &Element) -> Option<Element> {
        let mut current = element.clone();
        let mut iteration = 0;
        
        loop {
            if iteration >= self.max_iterations {
                if self.debug {
                    println!("Reached maximum iterations ({}), stopping", self.max_iterations);
                }
                break;
            }
            
            let _previous = current.clone();
            let mut changed = false;
            
            for pass in &self.passes {
                if self.debug {
                    println!("Running pass: {} (iteration {})", pass.name(), iteration);
                }
                
                if let Some(transformed) = pass.apply(&current) {
                    if pass.should_continue(&current, &transformed) {
                        current = transformed;
                        changed = true;
                    }
                }
            }
            
            if !changed {
                if self.debug {
                    println!("No changes in iteration {}, stopping", iteration);
                }
                break;
            }
            
            iteration += 1;
        }
        
        Some(current)
    }
}

impl Default for FoldPipeline {
    fn default() -> Self {
        Self::new()
    }
}

/// OpenAPI specification-aware fold pass
pub struct OpenApiSpecPass {
    spec: OpenApiSpecification,
    name: String,
}

impl OpenApiSpecPass {
    pub fn new(spec: OpenApiSpecification, name: String) -> Self {
        Self { spec, name }
    }
}

impl FoldPass for OpenApiSpecPass {
    fn apply(&self, element: &Element) -> Option<Element> {
        // Determine element type and apply appropriate visitor
        let element_type = determine_element_type(element);
        apply_fixed_fields_visitor(&self.spec, element, &element_type)
    }
    
    fn name(&self) -> &str {
        &self.name
    }
}

/// Reference resolution pass
pub struct ReferenceResolutionPass {
    name: String,
    reference_cache: HashMap<String, Element>,
}

impl ReferenceResolutionPass {
    pub fn new() -> Self {
        Self {
            name: "ReferenceResolution".to_string(),
            reference_cache: HashMap::new(),
        }
    }
    
    pub fn with_cache(mut self, cache: HashMap<String, Element>) -> Self {
        self.reference_cache = cache;
        self
    }
}

impl FoldPass for ReferenceResolutionPass {
    fn apply(&self, element: &Element) -> Option<Element> {
        resolve_references_in_element(element, &self.reference_cache)
    }
    
    fn name(&self) -> &str {
        &self.name
    }
}

/// Semantic enhancement pass
pub struct SemanticEnhancementPass {
    name: String,
}

impl SemanticEnhancementPass {
    pub fn new() -> Self {
        Self {
            name: "SemanticEnhancement".to_string(),
        }
    }
}

impl FoldPass for SemanticEnhancementPass {
    fn apply(&self, element: &Element) -> Option<Element> {
        enhance_element_semantics(element)
    }
    
    fn name(&self) -> &str {
        &self.name
    }
}

/// Validation pass
pub struct ValidationPass {
    name: String,
    strict: bool,
}

impl ValidationPass {
    pub fn new(strict: bool) -> Self {
        Self {
            name: "Validation".to_string(),
            strict,
        }
    }
}

impl FoldPass for ValidationPass {
    fn apply(&self, element: &Element) -> Option<Element> {
        validate_element(element, self.strict)
    }
    
    fn name(&self) -> &str {
        &self.name
    }
}

/// Create a standard OpenAPI 3.0 processing pipeline
pub fn create_openapi_pipeline(spec: OpenApiSpecification) -> FoldPipeline {
    FoldPipeline::new()
        .add_pass(Box::new(OpenApiSpecPass::new(spec, "OpenAPISpec".to_string())))
        .add_pass(Box::new(ReferenceResolutionPass::new()))
        .add_pass(Box::new(SemanticEnhancementPass::new()))
        .add_pass(Box::new(ValidationPass::new(false)))
        .max_iterations(5)
}

/// Create a strict OpenAPI 3.0 processing pipeline
pub fn create_strict_openapi_pipeline(spec: OpenApiSpecification) -> FoldPipeline {
    FoldPipeline::new()
        .add_pass(Box::new(OpenApiSpecPass::new(spec, "OpenAPISpec".to_string())))
        .add_pass(Box::new(ReferenceResolutionPass::new()))
        .add_pass(Box::new(SemanticEnhancementPass::new()))
        .add_pass(Box::new(ValidationPass::new(true)))
        .max_iterations(10)
        .debug(true)
}

// Helper functions

/// Determine the type of an OpenAPI element
fn determine_element_type(element: &Element) -> String {
    match element {
        Element::Object(obj) => {
            // Check for specific OpenAPI patterns
            let keys: Vec<String> = obj.content.iter()
                .filter_map(|member| {
                    if let Element::String(key) = &*member.key {
                        Some(key.content.clone())
                    } else {
                        None
                    }
                })
                .collect();
            
            // Determine type based on field patterns
            if keys.contains(&"openapi".to_string()) {
                "openApi3_0".to_string()
            } else if keys.contains(&"title".to_string()) && keys.contains(&"version".to_string()) {
                "info".to_string()
            } else if keys.contains(&"$ref".to_string()) {
                "reference".to_string()
            } else if keys.contains(&"type".to_string()) || keys.contains(&"properties".to_string()) {
                "schema".to_string()
            } else if keys.contains(&"description".to_string()) && keys.contains(&"content".to_string()) {
                "response".to_string()
            } else if keys.contains(&"name".to_string()) && keys.contains(&"in".to_string()) {
                "parameter".to_string()
            } else if keys.contains(&"url".to_string()) && keys.contains(&"variables".to_string()) {
                "server".to_string()
            } else if keys.contains(&"get".to_string()) || keys.contains(&"post".to_string()) || 
                     keys.contains(&"put".to_string()) || keys.contains(&"delete".to_string()) {
                "pathItem".to_string()
            } else {
                "object".to_string()
            }
        }
        Element::Array(_) => "array".to_string(),
        Element::String(_) => "string".to_string(),
        Element::Number(_) => "number".to_string(),
        Element::Boolean(_) => "boolean".to_string(),
        Element::Null(_) => "null".to_string(),
        _ => "unknown".to_string(),
    }
}

/// Check if two elements are equal
fn elements_equal(a: &Element, b: &Element) -> bool {
    // This is a simplified equality check
    // In a full implementation, you would do deep comparison
    std::ptr::eq(a, b) || format!("{:?}", a) == format!("{:?}", b)
}

/// Resolve references in an element
fn resolve_references_in_element(element: &Element, _cache: &HashMap<String, Element>) -> Option<Element> {
    // Simplified reference resolution
    // In a full implementation, you would:
    // 1. Find all $ref fields
    // 2. Resolve them using the cache or by loading from external sources
    // 3. Replace the reference with the resolved content
    
    if let Element::Object(obj) = element {
        let has_ref = obj.content.iter().any(|member| {
            if let Element::String(key) = &*member.key {
                key.content == "$ref"
            } else {
                false
            }
        });
        
        if has_ref {
            // For now, just add metadata indicating this is a reference
            let new_obj = obj.clone();
            // Add reference metadata here
            return Some(Element::Object(new_obj));
        }
    }
    
    Some(element.clone())
}

/// Enhance element with semantic information
fn enhance_element_semantics(element: &Element) -> Option<Element> {
    // Add semantic classes, metadata, etc.
    // This is where we would add the semantic tree enhancements
    
    match element {
        Element::Object(obj) => {
            let mut enhanced_obj = obj.clone();
            
            // Add semantic classes based on element type
            let element_type = determine_element_type(element);
            match element_type.as_str() {
                "openApi3_0" => {
                    // Add OpenAPI root classes
                    enhanced_obj.add_class("openapi");
                    enhanced_obj.add_class("openapi-3-0");
                }
                "info" => {
                    enhanced_obj.add_class("info");
                    enhanced_obj.add_class("openapi-info");
                }
                "schema" => {
                    enhanced_obj.add_class("schema");
                    enhanced_obj.add_class("json-schema");
                }
                "reference" => {
                    enhanced_obj.add_class("reference");
                    enhanced_obj.add_class("json-reference");
                }
                _ => {}
            }
            
            Some(Element::Object(enhanced_obj))
        }
        _ => Some(element.clone()),
    }
}

/// Validate an element
fn validate_element(element: &Element, _strict: bool) -> Option<Element> {
    // Perform validation and potentially add validation metadata
    // For now, just pass through
    Some(element.clone())
}

pub fn enhance_element_with_metadata(element: &mut Element) -> Result<(), String> {
    match element {
        Element::Object(obj) => {
            // Add semantic class based on element type
            match obj.element.as_str() {
                "openapi" => obj.add_class("openapi"),
                "info" => obj.add_class("info"),
                "contact" => obj.add_class("contact"),
                "license" => obj.add_class("license"),
                "paths" => obj.add_class("paths"),
                "pathItem" => obj.add_class("pathItem"),
                "operation" => obj.add_class("operation"),
                "components" => obj.add_class("components"),
                "schema" => obj.add_class("schema"),
                "response" => obj.add_class("response"),
                "parameter" => obj.add_class("parameter"),
                "example" => obj.add_class("example"),
                "requestBody" => obj.add_class("requestBody"),
                "header" => obj.add_class("header"),
                "securityScheme" => obj.add_class("securityScheme"),
                "oauthFlow" => obj.add_class("oauthFlow"),
                "encoding" => obj.add_class("encoding"),
                "mediaType" => obj.add_class("mediaType"),
                "link" => obj.add_class("link"),
                "callback" => obj.add_class("callback"),
                "reference" => obj.add_class("reference"),
                "discriminator" => obj.add_class("discriminator"),
                "xml" => obj.add_class("xml"),
                "externalDocs" => obj.add_class("externalDocs"),
                "tag" => obj.add_class("tag"),
                "server" => obj.add_class("server"),
                "serverVariable" => obj.add_class("serverVariable"),
                "securityRequirement" => obj.add_class("securityRequirement"),
                _ => {}
            }

            // Add reference detection
            if obj.has_key("$ref") {
                obj.add_class("reference");
            }

            // Add specification extension detection
            for member in &obj.content {
                if let Element::String(key_str) = member.key.as_ref() {
                    if key_str.content.starts_with("x-") {
                        obj.add_class("specification-extension");
                        break;
                    }
                }
            }
        }
        Element::String(str_elem) => {
            // Add string-specific metadata if needed
            if str_elem.content.starts_with("http") {
                str_elem.add_class("url");
            }
        }
        _ => {}
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::specification::create_openapi_specification;
    
    #[test]
    fn test_fold_pipeline_creation() {
        let spec = create_openapi_specification();
        let pipeline = create_openapi_pipeline(spec);
        assert_eq!(pipeline.passes.len(), 4);
    }
    
    #[test]
    fn test_element_type_determination_basic() {
        use apidom_ast::{ObjectElement, StringElement};
        
        // Test basic element type determination
        let element = Element::String(StringElement::new("test"));
        let element_type = determine_element_type(&element);
        assert_eq!(element_type, "string");
        
        let element = Element::Object(ObjectElement::new());
        let element_type = determine_element_type(&element);
        assert_eq!(element_type, "object");
    }
} 