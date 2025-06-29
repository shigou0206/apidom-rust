use std::collections::HashMap;
use std::fmt;
use std::sync::Arc;
use serde_json::Value;
use apidom_ast::minim_model::*;
use apidom_ast::Fold;
use crate::specification::{VisitorSpec, VisitorRef};
use crate::fold_pass::FoldPass;
use crate::reference_resolver::ReferenceResolver;
use crate::patterned_fields::PatternedFieldsProcessor;

/// Simple visitor that just returns the element unchanged
fn simple_visitor(element: &Element, _folder: Option<&mut dyn Fold>) -> Option<Element> {
    Some(element.clone())
}

/// Core extensible framework for handling multiple API specifications
pub struct ExtensibleFramework {
    /// Registered specification handlers
    specifications: HashMap<SpecificationType, Arc<dyn SpecificationHandler>>,
    /// Global reference resolver
    reference_resolver: ReferenceResolver,
    /// Global pattern processor
    pattern_processor: PatternedFieldsProcessor,
    /// Framework configuration
    config: FrameworkConfig,
}

/// Framework configuration
pub struct FrameworkConfig {
    /// Enable reference resolution
    pub enable_reference_resolution: bool,
    /// Enable pattern field processing
    pub enable_pattern_processing: bool,
    /// Enable semantic enhancement
    pub enable_semantic_enhancement: bool,
    /// Enable validation
    pub enable_validation: bool,
    /// Maximum processing iterations
    pub max_iterations: usize,
    /// Custom processing hooks
    pub custom_hooks: HashMap<String, Box<dyn ProcessingHook>>,
}

impl fmt::Debug for FrameworkConfig {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("FrameworkConfig")
            .field("enable_reference_resolution", &self.enable_reference_resolution)
            .field("enable_pattern_processing", &self.enable_pattern_processing)
            .field("enable_semantic_enhancement", &self.enable_semantic_enhancement)
            .field("enable_validation", &self.enable_validation)
            .field("max_iterations", &self.max_iterations)
            .field("custom_hooks", &format!("{} hooks", self.custom_hooks.len()))
            .finish()
    }
}

impl Clone for FrameworkConfig {
    fn clone(&self) -> Self {
        Self {
            enable_reference_resolution: self.enable_reference_resolution,
            enable_pattern_processing: self.enable_pattern_processing,
            enable_semantic_enhancement: self.enable_semantic_enhancement,
            enable_validation: self.enable_validation,
            max_iterations: self.max_iterations,
            custom_hooks: HashMap::new(), // Don't clone trait objects
        }
    }
}

/// Supported specification types
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub enum SpecificationType {
    /// OpenAPI 3.0 specification
    OpenApi30,
    /// OpenAPI 3.1 specification
    OpenApi31,
    /// AsyncAPI 2.0 specification
    AsyncApi20,
    /// AsyncAPI 2.1 specification
    AsyncApi21,
    /// AsyncAPI 2.2 specification
    AsyncApi22,
    /// AsyncAPI 2.3 specification
    AsyncApi23,
    /// AsyncAPI 2.4 specification
    AsyncApi24,
    /// AsyncAPI 2.5 specification
    AsyncApi25,
    /// AsyncAPI 2.6 specification
    AsyncApi26,
    /// AsyncAPI 3.0 specification
    AsyncApi30,
    /// JSON Schema Draft 4
    JsonSchemaDraft4,
    /// JSON Schema Draft 6
    JsonSchemaDraft6,
    /// JSON Schema Draft 7
    JsonSchemaDraft7,
    /// JSON Schema 2019-09
    JsonSchema201909,
    /// JSON Schema 2020-12
    JsonSchema202012,
    /// Custom specification type
    Custom(String),
}

/// Generic specification handler trait
pub trait SpecificationHandler: Send + Sync + fmt::Debug {
    /// Get the specification type
    fn specification_type(&self) -> SpecificationType;
    
    /// Get the visitor specifications for this specification
    fn get_visitor_specs(&self) -> HashMap<String, VisitorSpec>;
    
    /// Get the fold passes for this specification
    fn get_fold_passes(&self) -> Vec<Box<dyn FoldPass>>;
    
    /// Detect if an element belongs to this specification
    fn can_handle_element(&self, element: &Element) -> bool;
    
    /// Get the root element name for this specification
    fn get_root_element_name(&self) -> &str;
    
    /// Validate an element according to this specification
    fn validate_element(&self, element: &Element) -> Result<ValidationResult, SpecificationError>;
    
    /// Transform an element for this specification
    fn transform_element(&self, element: Element, context: &TransformContext) -> Result<Element, SpecificationError>;
    
    /// Get specification-specific metadata
    fn get_metadata(&self) -> HashMap<String, Value>;
}

/// Processing hook trait for custom processing
pub trait ProcessingHook: Send + Sync {
    /// Called before processing starts
    fn before_processing(&self, element: &Element, context: &ProcessingContext) -> Result<(), ProcessingError>;
    
    /// Called after each pass
    fn after_pass(&self, element: &Element, pass_name: &str, context: &ProcessingContext) -> Result<(), ProcessingError>;
    
    /// Called after processing completes
    fn after_processing(&self, element: &Element, context: &ProcessingContext) -> Result<(), ProcessingError>;
}

/// Processing context for hooks and transforms
#[derive(Debug, Clone)]
pub struct ProcessingContext {
    /// Current specification type
    pub specification_type: SpecificationType,
    /// Current processing iteration
    pub iteration: usize,
    /// Processing metadata
    pub metadata: HashMap<String, Value>,
    /// Reference to the framework
    pub framework_config: FrameworkConfig,
}

/// Transform context for element transformation
#[derive(Debug, Clone)]
pub struct TransformContext {
    /// Current specification type
    pub specification_type: SpecificationType,
    /// Available transformations
    pub transformations: HashMap<String, Value>,
    /// Processing metadata
    pub metadata: HashMap<String, Value>,
}

/// Validation result
#[derive(Debug, Clone)]
pub struct ValidationResult {
    /// Whether the element is valid
    pub is_valid: bool,
    /// Validation errors
    pub errors: Vec<ValidationError>,
    /// Validation warnings
    pub warnings: Vec<ValidationWarning>,
    /// Validation metadata
    pub metadata: HashMap<String, Value>,
}

/// Validation error
#[derive(Debug, Clone)]
pub struct ValidationError {
    /// Error message
    pub message: String,
    /// Error path in the document
    pub path: Vec<String>,
    /// Error code
    pub code: String,
    /// Error severity
    pub severity: ErrorSeverity,
}

/// Validation warning
#[derive(Debug, Clone)]
pub struct ValidationWarning {
    /// Warning message
    pub message: String,
    /// Warning path in the document
    pub path: Vec<String>,
    /// Warning code
    pub code: String,
}

/// Error severity levels
#[derive(Debug, Clone, PartialEq)]
pub enum ErrorSeverity {
    /// Critical error that prevents processing
    Critical,
    /// Error that should be fixed
    Error,
    /// Warning that should be addressed
    Warning,
    /// Information message
    Info,
}

/// Specification processing errors
#[derive(Debug, thiserror::Error)]
pub enum SpecificationError {
    #[error("Unsupported specification type: {0:?}")]
    UnsupportedSpecification(SpecificationType),
    #[error("Validation failed: {0}")]
    ValidationFailed(String),
    #[error("Transformation failed: {0}")]
    TransformationFailed(String),
    #[error("Processing failed: {0}")]
    ProcessingFailed(String),
    #[error("Configuration error: {0}")]
    ConfigurationError(String),
}

/// Processing errors
#[derive(Debug, thiserror::Error)]
pub enum ProcessingError {
    #[error("Hook execution failed: {0}")]
    HookFailed(String),
    #[error("Processing interrupted: {0}")]
    ProcessingInterrupted(String),
    #[error("Context error: {0}")]
    ContextError(String),
}

impl ExtensibleFramework {
    /// Create a new extensible framework
    pub fn new() -> Self {
        Self {
            specifications: HashMap::new(),
            reference_resolver: ReferenceResolver::new(),
            pattern_processor: PatternedFieldsProcessor::new(),
            config: FrameworkConfig::default(),
        }
    }

    /// Register a specification handler
    pub fn register_specification(&mut self, handler: Arc<dyn SpecificationHandler>) {
        let spec_type = handler.specification_type();
        self.specifications.insert(spec_type, handler);
    }

    /// Process an element with automatic specification detection
    pub fn process_element(&self, element: Element) -> Result<Element, SpecificationError> {
        // Detect specification type
        let spec_type = self.detect_specification_type(&element)?;
        
        // Get specification handler
        let handler = self.specifications.get(&spec_type)
            .ok_or_else(|| SpecificationError::UnsupportedSpecification(spec_type.clone()))?;
        
        // Create processing context
        let context = ProcessingContext {
            specification_type: spec_type,
            iteration: 0,
            metadata: HashMap::new(),
            framework_config: self.config.clone(),
        };
        
        // Execute processing hooks
        for hook in self.config.custom_hooks.values() {
            hook.before_processing(&element, &context)
                .map_err(|e| SpecificationError::ProcessingFailed(e.to_string()))?;
        }
        
        // Process the element
        let processed = self.process_with_specification(element, handler.as_ref(), &context)?;
        
        // Execute after processing hooks
        for hook in self.config.custom_hooks.values() {
            hook.after_processing(&processed, &context)
                .map_err(|e| SpecificationError::ProcessingFailed(e.to_string()))?;
        }
        
        Ok(processed)
    }

    /// Process an element with a specific specification
    pub fn process_with_specification_type(&self, element: Element, spec_type: SpecificationType) -> Result<Element, SpecificationError> {
        let handler = self.specifications.get(&spec_type)
            .ok_or_else(|| SpecificationError::UnsupportedSpecification(spec_type.clone()))?;
        
        let context = ProcessingContext {
            specification_type: spec_type,
            iteration: 0,
            metadata: HashMap::new(),
            framework_config: self.config.clone(),
        };
        
        self.process_with_specification(element, handler.as_ref(), &context)
    }

    /// Internal processing with a specific handler
    fn process_with_specification(&self, mut element: Element, handler: &dyn SpecificationHandler, context: &ProcessingContext) -> Result<Element, SpecificationError> {
        // Get fold passes from the handler
        let passes = handler.get_fold_passes();
        
        // Execute before processing hooks
        for hook in &self.config.custom_hooks {
            hook.1.before_processing(&element, context)
                .map_err(|e| SpecificationError::ProcessingFailed(e.to_string()))?;
        }
        
        // Execute fold passes
        for (_iteration, pass) in passes.iter().enumerate() {
            // Execute before pass hooks
            for hook in &self.config.custom_hooks {
                hook.1.after_pass(&element, pass.name(), context)
                    .map_err(|e| SpecificationError::ProcessingFailed(e.to_string()))?;
            }
            
            let previous_element = element.clone();
            
            // Execute pass
            let pass_result = pass.apply(&element);
            
            if let Some(result) = pass_result {
                element = result;
            }
            
            // Execute after pass hooks
            for hook in &self.config.custom_hooks {
                hook.1.after_pass(&element, pass.name(), context)
                    .map_err(|e| SpecificationError::ProcessingFailed(e.to_string()))?;
            }
            
            // Check if we should continue
            if !pass.should_continue(&previous_element, &element) {
                break;
            }
        }
        
        // Execute after processing hooks
        for hook in &self.config.custom_hooks {
            hook.1.after_processing(&element, context)
                .map_err(|e| SpecificationError::ProcessingFailed(e.to_string()))?;
        }
        
        Ok(element)
    }

    /// Detect the specification type of an element
    fn detect_specification_type(&self, element: &Element) -> Result<SpecificationType, SpecificationError> {
        // Try each registered specification
        for (spec_type, handler) in &self.specifications {
            if handler.can_handle_element(element) {
                return Ok(spec_type.clone());
            }
        }
        
        Err(SpecificationError::UnsupportedSpecification(SpecificationType::Custom("unknown".to_string())))
    }

    /// Validate an element
    pub fn validate_element(&self, element: &Element, spec_type: Option<SpecificationType>) -> Result<ValidationResult, SpecificationError> {
        let spec_type = match spec_type {
            Some(st) => st,
            None => self.detect_specification_type(element)?,
        };
        
        let handler = self.specifications.get(&spec_type)
            .ok_or_else(|| SpecificationError::UnsupportedSpecification(spec_type))?;
        
        handler.validate_element(element)
    }

    /// Configure the framework
    pub fn with_config(mut self, config: FrameworkConfig) -> Self {
        self.config = config;
        self
    }

    /// Configure the reference resolver
    pub fn with_reference_resolver(mut self, resolver: ReferenceResolver) -> Self {
        self.reference_resolver = resolver;
        self
    }

    /// Configure the pattern processor
    pub fn with_pattern_processor(mut self, processor: PatternedFieldsProcessor) -> Self {
        self.pattern_processor = processor;
        self
    }

    /// Get supported specification types
    pub fn supported_specifications(&self) -> Vec<SpecificationType> {
        self.specifications.keys().cloned().collect()
    }

    /// Get specification metadata
    pub fn get_specification_metadata(&self, spec_type: &SpecificationType) -> Option<HashMap<String, Value>> {
        self.specifications.get(spec_type).map(|handler| handler.get_metadata())
    }
}

/// AsyncAPI 2.6 specification handler
pub struct AsyncApi26Handler {
    visitor_specs: HashMap<String, VisitorSpec>,
    fold_passes: Vec<Box<dyn FoldPass>>,
}

impl fmt::Debug for AsyncApi26Handler {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("AsyncApi26Handler")
            .field("visitor_specs", &format!("{} specs", self.visitor_specs.len()))
            .field("fold_passes", &format!("{} passes", self.fold_passes.len()))
            .finish()
    }
}

impl AsyncApi26Handler {
    pub fn new() -> Self {
        let mut handler = Self {
            visitor_specs: HashMap::new(),
            fold_passes: Vec::new(),
        };
        
        handler.initialize_visitor_specs();
        handler.initialize_fold_passes();
        handler
    }

    fn initialize_visitor_specs(&mut self) {
        // AsyncAPI root object
        let mut asyncapi_fields = HashMap::new();
        asyncapi_fields.insert("asyncapi".to_string(), VisitorRef::Direct(simple_visitor));
        asyncapi_fields.insert("id".to_string(), VisitorRef::Direct(simple_visitor));
        asyncapi_fields.insert("info".to_string(), VisitorRef::Reference("info".to_string()));
        asyncapi_fields.insert("servers".to_string(), VisitorRef::Reference("servers".to_string()));
        asyncapi_fields.insert("channels".to_string(), VisitorRef::Reference("channels".to_string()));
        
        self.visitor_specs.insert("asyncapi".to_string(), VisitorSpec {
            visitor: Some(simple_visitor),
            fixed_fields: Some(asyncapi_fields),
        });

        // Info object
        let mut info_fields = HashMap::new();
        info_fields.insert("title".to_string(), VisitorRef::Direct(simple_visitor));
        info_fields.insert("version".to_string(), VisitorRef::Direct(simple_visitor));
        info_fields.insert("description".to_string(), VisitorRef::Direct(simple_visitor));
        info_fields.insert("contact".to_string(), VisitorRef::Reference("contact".to_string()));
        info_fields.insert("license".to_string(), VisitorRef::Reference("license".to_string()));
        
        self.visitor_specs.insert("info".to_string(), VisitorSpec {
            visitor: Some(simple_visitor),
            fixed_fields: Some(info_fields),
        });

        // Server object
        let mut server_fields = HashMap::new();
        server_fields.insert("url".to_string(), VisitorRef::Direct(simple_visitor));
        server_fields.insert("protocol".to_string(), VisitorRef::Direct(simple_visitor));
        server_fields.insert("protocolVersion".to_string(), VisitorRef::Direct(simple_visitor));
        server_fields.insert("description".to_string(), VisitorRef::Direct(simple_visitor));
        server_fields.insert("variables".to_string(), VisitorRef::Reference("server_variables".to_string()));
        server_fields.insert("security".to_string(), VisitorRef::Reference("security_requirements".to_string()));
        server_fields.insert("bindings".to_string(), VisitorRef::Reference("server_bindings".to_string()));
        
        self.visitor_specs.insert("server".to_string(), VisitorSpec {
            visitor: Some(simple_visitor),
            fixed_fields: Some(server_fields),
        });

        // Channel object
        let mut channel_fields = HashMap::new();
        channel_fields.insert("description".to_string(), VisitorRef::Direct(simple_visitor));
        channel_fields.insert("subscribe".to_string(), VisitorRef::Reference("operation".to_string()));
        channel_fields.insert("publish".to_string(), VisitorRef::Reference("operation".to_string()));
        channel_fields.insert("parameters".to_string(), VisitorRef::Reference("parameters".to_string()));
        
        self.visitor_specs.insert("channel".to_string(), VisitorSpec {
            visitor: Some(simple_visitor),
            fixed_fields: Some(channel_fields),
        });

        // Message object
        let mut message_fields = HashMap::new();
        message_fields.insert("messageId".to_string(), VisitorRef::Direct(simple_visitor));
        message_fields.insert("headers".to_string(), VisitorRef::Reference("schema".to_string()));
        message_fields.insert("payload".to_string(), VisitorRef::Reference("schema".to_string()));
        message_fields.insert("correlationId".to_string(), VisitorRef::Reference("correlation_id".to_string()));
        message_fields.insert("schemaFormat".to_string(), VisitorRef::Direct(simple_visitor));
        message_fields.insert("contentType".to_string(), VisitorRef::Direct(simple_visitor));
        message_fields.insert("name".to_string(), VisitorRef::Direct(simple_visitor));
        message_fields.insert("title".to_string(), VisitorRef::Direct(simple_visitor));
        message_fields.insert("summary".to_string(), VisitorRef::Direct(simple_visitor));
        message_fields.insert("description".to_string(), VisitorRef::Direct(simple_visitor));
        message_fields.insert("tags".to_string(), VisitorRef::Reference("tags".to_string()));
        message_fields.insert("externalDocs".to_string(), VisitorRef::Reference("external_docs".to_string()));
        message_fields.insert("bindings".to_string(), VisitorRef::Reference("message_bindings".to_string()));
        message_fields.insert("examples".to_string(), VisitorRef::Reference("examples".to_string()));
        message_fields.insert("traits".to_string(), VisitorRef::Reference("message_traits".to_string()));
        
        self.visitor_specs.insert("message".to_string(), VisitorSpec {
            visitor: Some(simple_visitor),
            fixed_fields: Some(message_fields),
        });
    }

    fn initialize_fold_passes(&mut self) {
        // Add AsyncAPI-specific fold passes
        self.fold_passes.push(Box::new(AsyncApiSpecPass::new()));
        self.fold_passes.push(Box::new(AsyncApiReferenceResolutionPass::new()));
        self.fold_passes.push(Box::new(AsyncApiSemanticEnhancementPass::new()));
        self.fold_passes.push(Box::new(AsyncApiValidationPass::new()));
    }
}

impl SpecificationHandler for AsyncApi26Handler {
    fn specification_type(&self) -> SpecificationType {
        SpecificationType::AsyncApi26
    }

    fn get_visitor_specs(&self) -> HashMap<String, VisitorSpec> {
        self.visitor_specs.clone()
    }

    fn get_fold_passes(&self) -> Vec<Box<dyn FoldPass>> {
        // Clone the fold passes (this is a simplified approach)
        vec![
            Box::new(AsyncApiSpecPass::new()),
            Box::new(AsyncApiReferenceResolutionPass::new()),
            Box::new(AsyncApiSemanticEnhancementPass::new()),
            Box::new(AsyncApiValidationPass::new()),
        ]
    }

    fn can_handle_element(&self, element: &Element) -> bool {
        if let Element::Object(obj) = element {
            // Check for AsyncAPI version field
            for member in &obj.content {
                if let Element::String(key) = member.key.as_ref() {
                    if key.content == "asyncapi" {
                        if let Element::String(version) = member.value.as_ref() {
                            return version.content.starts_with("2.6");
                        }
                    }
                }
            }
        }
        false
    }

    fn get_root_element_name(&self) -> &str {
        "asyncapi"
    }

    fn validate_element(&self, element: &Element) -> Result<ValidationResult, SpecificationError> {
        let mut errors = Vec::new();
        let mut warnings = Vec::new();

        // Basic AsyncAPI validation
        if let Element::Object(obj) = element {
            let mut has_asyncapi = false;
            let mut has_info = false;
            let mut has_channels = false;

            for member in &obj.content {
                if let Element::String(key) = member.key.as_ref() {
                    match key.content.as_str() {
                        "asyncapi" => has_asyncapi = true,
                        "info" => has_info = true,
                        "channels" => has_channels = true,
                        _ => {}
                    }
                }
            }

            if !has_asyncapi {
                errors.push(ValidationError {
                    message: "Missing required field: asyncapi".to_string(),
                    path: vec![],
                    code: "MISSING_ASYNCAPI_VERSION".to_string(),
                    severity: ErrorSeverity::Critical,
                });
            }

            if !has_info {
                errors.push(ValidationError {
                    message: "Missing required field: info".to_string(),
                    path: vec![],
                    code: "MISSING_INFO".to_string(),
                    severity: ErrorSeverity::Critical,
                });
            }

            if !has_channels {
                warnings.push(ValidationWarning {
                    message: "No channels defined in AsyncAPI document".to_string(),
                    path: vec![],
                    code: "NO_CHANNELS".to_string(),
                });
            }
        } else {
            errors.push(ValidationError {
                message: "AsyncAPI document must be an object".to_string(),
                path: vec![],
                code: "INVALID_DOCUMENT_TYPE".to_string(),
                severity: ErrorSeverity::Critical,
            });
        }

        Ok(ValidationResult {
            is_valid: errors.is_empty(),
            errors,
            warnings,
            metadata: HashMap::new(),
        })
    }

    fn transform_element(&self, element: Element, _context: &TransformContext) -> Result<Element, SpecificationError> {
        // Basic transformation - add AsyncAPI-specific metadata
        if let Element::Object(mut obj) = element {
            obj.meta.properties.insert("specification".to_string(), Value::String("AsyncAPI 2.6".to_string()));
            obj.meta.properties.insert("specification_type".to_string(), Value::String("asyncapi26".to_string()));
            Ok(Element::Object(obj))
        } else {
            Ok(element)
        }
    }

    fn get_metadata(&self) -> HashMap<String, Value> {
        let mut metadata = HashMap::new();
        metadata.insert("specification".to_string(), Value::String("AsyncAPI".to_string()));
        metadata.insert("version".to_string(), Value::String("2.6.0".to_string()));
        metadata.insert("description".to_string(), Value::String("AsyncAPI 2.6 specification handler".to_string()));
        metadata.insert("supported_features".to_string(), Value::Array(vec![
            Value::String("channels".to_string()),
            Value::String("messages".to_string()),
            Value::String("schemas".to_string()),
            Value::String("servers".to_string()),
            Value::String("operations".to_string()),
            Value::String("bindings".to_string()),
        ]));
        metadata
    }
}

/// JSON Schema 2020-12 specification handler
pub struct JsonSchema202012Handler {
    visitor_specs: HashMap<String, VisitorSpec>,
    fold_passes: Vec<Box<dyn FoldPass>>,
}

impl fmt::Debug for JsonSchema202012Handler {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("JsonSchema202012Handler")
            .field("visitor_specs", &format!("{} specs", self.visitor_specs.len()))
            .field("fold_passes", &format!("{} passes", self.fold_passes.len()))
            .finish()
    }
}

impl JsonSchema202012Handler {
    pub fn new() -> Self {
        let mut handler = Self {
            visitor_specs: HashMap::new(),
            fold_passes: Vec::new(),
        };
        
        handler.initialize_visitor_specs();
        handler.initialize_fold_passes();
        handler
    }

    fn initialize_visitor_specs(&mut self) {
        // JSON Schema root object
        let mut schema_fields = HashMap::new();
        schema_fields.insert("$schema".to_string(), VisitorRef::Direct(simple_visitor));
        schema_fields.insert("$id".to_string(), VisitorRef::Direct(simple_visitor));
        schema_fields.insert("$ref".to_string(), VisitorRef::Direct(simple_visitor));
        schema_fields.insert("$defs".to_string(), VisitorRef::Reference("definitions".to_string()));
        schema_fields.insert("type".to_string(), VisitorRef::Direct(simple_visitor));
        schema_fields.insert("title".to_string(), VisitorRef::Direct(simple_visitor));
        schema_fields.insert("description".to_string(), VisitorRef::Direct(simple_visitor));
        schema_fields.insert("properties".to_string(), VisitorRef::Reference("properties".to_string()));
        schema_fields.insert("items".to_string(), VisitorRef::Reference("schema".to_string()));
        schema_fields.insert("required".to_string(), VisitorRef::Direct(simple_visitor));
        schema_fields.insert("enum".to_string(), VisitorRef::Direct(simple_visitor));
        schema_fields.insert("const".to_string(), VisitorRef::Direct(simple_visitor));
        schema_fields.insert("default".to_string(), VisitorRef::Direct(simple_visitor));
        
        self.visitor_specs.insert("schema".to_string(), VisitorSpec {
            visitor: Some(simple_visitor),
            fixed_fields: Some(schema_fields),
        });
    }

    fn initialize_fold_passes(&mut self) {
        // Add JSON Schema-specific fold passes
        self.fold_passes.push(Box::new(JsonSchemaSpecPass::new()));
        self.fold_passes.push(Box::new(JsonSchemaReferenceResolutionPass::new()));
        self.fold_passes.push(Box::new(JsonSchemaSemanticEnhancementPass::new()));
        self.fold_passes.push(Box::new(JsonSchemaValidationPass::new()));
    }
}

impl SpecificationHandler for JsonSchema202012Handler {
    fn specification_type(&self) -> SpecificationType {
        SpecificationType::JsonSchema202012
    }

    fn get_visitor_specs(&self) -> HashMap<String, VisitorSpec> {
        self.visitor_specs.clone()
    }

    fn get_fold_passes(&self) -> Vec<Box<dyn FoldPass>> {
        vec![
            Box::new(JsonSchemaSpecPass::new()),
            Box::new(JsonSchemaReferenceResolutionPass::new()),
            Box::new(JsonSchemaSemanticEnhancementPass::new()),
            Box::new(JsonSchemaValidationPass::new()),
        ]
    }

    fn can_handle_element(&self, element: &Element) -> bool {
        if let Element::Object(obj) = element {
            // Check for JSON Schema $schema field
            for member in &obj.content {
                if let Element::String(key) = member.key.as_ref() {
                    if key.content == "$schema" {
                        if let Element::String(schema_uri) = member.value.as_ref() {
                            return schema_uri.content.contains("2020-12");
                        }
                    }
                }
            }
            // Also check for common JSON Schema patterns
            let has_type = obj.content.iter().any(|m| {
                if let Element::String(key) = m.key.as_ref() {
                    key.content == "type"
                } else {
                    false
                }
            });
            let has_properties = obj.content.iter().any(|m| {
                if let Element::String(key) = m.key.as_ref() {
                    key.content == "properties"
                } else {
                    false
                }
            });
            return has_type || has_properties;
        }
        false
    }

    fn get_root_element_name(&self) -> &str {
        "schema"
    }

    fn validate_element(&self, element: &Element) -> Result<ValidationResult, SpecificationError> {
        let mut errors = Vec::new();
        let warnings = Vec::new();

        // Basic JSON Schema validation
        if let Element::Object(obj) = element {
            // Check for valid type values
            for member in &obj.content {
                if let Element::String(key) = member.key.as_ref() {
                    if key.content == "type" {
                        if let Element::String(type_val) = member.value.as_ref() {
                            let valid_types = ["null", "boolean", "object", "array", "number", "integer", "string"];
                            if !valid_types.contains(&type_val.content.as_str()) {
                                errors.push(ValidationError {
                                    message: format!("Invalid type value: {}", type_val.content),
                                    path: vec!["type".to_string()],
                                    code: "INVALID_TYPE".to_string(),
                                    severity: ErrorSeverity::Error,
                                });
                            }
                        }
                    }
                }
            }
        }

        Ok(ValidationResult {
            is_valid: errors.is_empty(),
            errors,
            warnings,
            metadata: HashMap::new(),
        })
    }

    fn transform_element(&self, element: Element, _context: &TransformContext) -> Result<Element, SpecificationError> {
        // Basic transformation - add JSON Schema-specific metadata
        if let Element::Object(mut obj) = element {
            obj.meta.properties.insert("specification".to_string(), Value::String("JSON Schema 2020-12".to_string()));
            obj.meta.properties.insert("specification_type".to_string(), Value::String("jsonschema202012".to_string()));
            Ok(Element::Object(obj))
        } else {
            Ok(element)
        }
    }

    fn get_metadata(&self) -> HashMap<String, Value> {
        let mut metadata = HashMap::new();
        metadata.insert("specification".to_string(), Value::String("JSON Schema".to_string()));
        metadata.insert("version".to_string(), Value::String("2020-12".to_string()));
        metadata.insert("description".to_string(), Value::String("JSON Schema 2020-12 specification handler".to_string()));
        metadata.insert("supported_features".to_string(), Value::Array(vec![
            Value::String("type_validation".to_string()),
            Value::String("property_validation".to_string()),
            Value::String("array_validation".to_string()),
            Value::String("conditional_schemas".to_string()),
            Value::String("meta_schemas".to_string()),
        ]));
        metadata
    }
}

// AsyncAPI-specific fold passes
#[derive(Debug)]
pub struct AsyncApiSpecPass;

impl AsyncApiSpecPass {
    pub fn new() -> Self {
        Self
    }
}

impl FoldPass for AsyncApiSpecPass {
    fn apply(&self, element: &Element) -> Option<Element> {
        Some(element.clone())
    }

    fn name(&self) -> &str {
        "AsyncApiSpecPass"
    }
}

#[derive(Debug)]
pub struct AsyncApiReferenceResolutionPass;

impl AsyncApiReferenceResolutionPass {
    pub fn new() -> Self {
        Self
    }
}

impl FoldPass for AsyncApiReferenceResolutionPass {
    fn apply(&self, element: &Element) -> Option<Element> {
        Some(element.clone())
    }

    fn name(&self) -> &str {
        "AsyncApiReferenceResolutionPass"
    }
}

#[derive(Debug)]
pub struct AsyncApiSemanticEnhancementPass;

impl AsyncApiSemanticEnhancementPass {
    pub fn new() -> Self {
        Self
    }
}

impl FoldPass for AsyncApiSemanticEnhancementPass {
    fn apply(&self, element: &Element) -> Option<Element> {
        Some(element.clone())
    }

    fn name(&self) -> &str {
        "AsyncApiSemanticEnhancementPass"
    }
}

#[derive(Debug)]
pub struct AsyncApiValidationPass;

impl AsyncApiValidationPass {
    pub fn new() -> Self {
        Self
    }
}

impl FoldPass for AsyncApiValidationPass {
    fn apply(&self, element: &Element) -> Option<Element> {
        Some(element.clone())
    }

    fn name(&self) -> &str {
        "AsyncApiValidationPass"
    }
}

// JSON Schema-specific fold passes
#[derive(Debug)]
pub struct JsonSchemaSpecPass;

impl JsonSchemaSpecPass {
    pub fn new() -> Self {
        Self
    }
}

impl FoldPass for JsonSchemaSpecPass {
    fn apply(&self, element: &Element) -> Option<Element> {
        Some(element.clone())
    }

    fn name(&self) -> &str {
        "JsonSchemaSpecPass"
    }
}

#[derive(Debug)]
pub struct JsonSchemaReferenceResolutionPass;

impl JsonSchemaReferenceResolutionPass {
    pub fn new() -> Self {
        Self
    }
}

impl FoldPass for JsonSchemaReferenceResolutionPass {
    fn apply(&self, element: &Element) -> Option<Element> {
        Some(element.clone())
    }

    fn name(&self) -> &str {
        "JsonSchemaReferenceResolutionPass"
    }
}

#[derive(Debug)]
pub struct JsonSchemaSemanticEnhancementPass;

impl JsonSchemaSemanticEnhancementPass {
    pub fn new() -> Self {
        Self
    }
}

impl FoldPass for JsonSchemaSemanticEnhancementPass {
    fn apply(&self, element: &Element) -> Option<Element> {
        Some(element.clone())
    }

    fn name(&self) -> &str {
        "JsonSchemaSemanticEnhancementPass"
    }
}

#[derive(Debug)]
pub struct JsonSchemaValidationPass;

impl JsonSchemaValidationPass {
    pub fn new() -> Self {
        Self
    }
}

impl FoldPass for JsonSchemaValidationPass {
    fn apply(&self, element: &Element) -> Option<Element> {
        Some(element.clone())
    }

    fn name(&self) -> &str {
        "JsonSchemaValidationPass"
    }
}

/// Default configuration
impl Default for FrameworkConfig {
    fn default() -> Self {
        Self {
            enable_reference_resolution: true,
            enable_pattern_processing: true,
            enable_semantic_enhancement: true,
            enable_validation: true,
            max_iterations: 10,
            custom_hooks: HashMap::new(),
        }
    }
}

/// Default implementation
impl Default for ExtensibleFramework {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;

    #[test]
    fn test_asyncapi_specification_detection() {
        let handler = AsyncApi26Handler::new();
        
        // Create a mock AsyncAPI document
        let mut asyncapi_doc = ObjectElement::new();
        asyncapi_doc.set("asyncapi", Element::String(StringElement::new("2.6.0")));
        asyncapi_doc.set("info", Element::Object(ObjectElement::new()));
        
        let element = Element::Object(asyncapi_doc);
        assert!(handler.can_handle_element(&element));
    }

    #[test]
    fn test_json_schema_specification_detection() {
        let handler = JsonSchema202012Handler::new();
        
        // Create a mock JSON Schema document
        let mut schema_doc = ObjectElement::new();
        schema_doc.set("$schema", Element::String(StringElement::new("https://json-schema.org/draft/2020-12/schema")));
        schema_doc.set("type", Element::String(StringElement::new("object")));
        
        let element = Element::Object(schema_doc);
        assert!(handler.can_handle_element(&element));
    }

    #[test]
    fn test_framework_specification_registration() {
        let mut framework = ExtensibleFramework::new();
        
        // Register AsyncAPI handler
        let asyncapi_handler = Arc::new(AsyncApi26Handler::new());
        framework.register_specification(asyncapi_handler);
        
        // Register JSON Schema handler
        let json_schema_handler = Arc::new(JsonSchema202012Handler::new());
        framework.register_specification(json_schema_handler);
        
        let supported = framework.supported_specifications();
        assert!(supported.contains(&SpecificationType::AsyncApi26));
        assert!(supported.contains(&SpecificationType::JsonSchema202012));
    }

    #[test]
    fn test_asyncapi_validation() {
        let handler = AsyncApi26Handler::new();
        
        // Test valid AsyncAPI document
        let mut valid_doc = ObjectElement::new();
        valid_doc.set("asyncapi", Element::String(StringElement::new("2.6.0")));
        valid_doc.set("info", Element::Object(ObjectElement::new()));
        
        let result = handler.validate_element(&Element::Object(valid_doc)).unwrap();
        assert!(result.is_valid);
        
        // Test invalid AsyncAPI document (missing required fields)
        let invalid_doc = ObjectElement::new();
        let result = handler.validate_element(&Element::Object(invalid_doc)).unwrap();
        assert!(!result.is_valid);
        assert!(!result.errors.is_empty());
    }

    #[test]
    fn test_json_schema_validation() {
        let handler = JsonSchema202012Handler::new();
        
        // Test valid JSON Schema
        let mut valid_schema = ObjectElement::new();
        valid_schema.set("type", Element::String(StringElement::new("object")));
        
        let result = handler.validate_element(&Element::Object(valid_schema)).unwrap();
        assert!(result.is_valid);
        
        // Test invalid JSON Schema (invalid type)
        let mut invalid_schema = ObjectElement::new();
        invalid_schema.set("type", Element::String(StringElement::new("invalid_type")));
        
        let result = handler.validate_element(&Element::Object(invalid_schema)).unwrap();
        assert!(!result.is_valid);
        assert!(!result.errors.is_empty());
    }
} 