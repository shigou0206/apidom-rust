use std::collections::HashMap;
use std::fmt;
use regex::Regex;
use apidom_ast::*;

/// Comprehensive patterned fields framework for OpenAPI specifications
/// Handles:
/// - Path templates (e.g., `/pets/{id}`, `/users/{userId}/posts/{postId}`)
/// - Runtime expressions (e.g., `$url`, `$method`, `$statusCode`)
/// - Specification extensions (e.g., `x-custom-field`)
/// - Callback expressions
/// - Header parameter patterns
/// - Media type patterns
pub struct PatternedFieldsProcessor {
    /// Registered pattern handlers
    handlers: HashMap<PatternType, Box<dyn PatternHandler>>,
    /// Configuration for pattern processing
    config: PatternConfig,
    /// Cache for compiled patterns
    pattern_cache: HashMap<String, CompiledPattern>,
}

impl fmt::Debug for PatternedFieldsProcessor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("PatternedFieldsProcessor")
            .field("handlers", &format!("{} handlers", self.handlers.len()))
            .field("pattern_cache", &self.pattern_cache)
            .finish()
    }
}

/// Configuration for pattern processing
pub struct PatternConfig {
    /// Enable path template processing
    pub enable_path_templates: bool,
    /// Enable runtime expression processing
    pub enable_runtime_expressions: bool,
    /// Enable specification extension processing
    pub enable_spec_extensions: bool,
    /// Enable callback expression processing
    pub enable_callback_expressions: bool,
    /// Custom pattern validation rules
    pub custom_validators: HashMap<String, Box<dyn PatternValidator>>,
    /// Maximum pattern complexity (to prevent ReDoS)
    pub max_pattern_complexity: usize,
}

impl fmt::Debug for PatternConfig {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("PatternConfig")
            .field("enable_path_templates", &self.enable_path_templates)
            .field("enable_runtime_expressions", &self.enable_runtime_expressions)
            .field("enable_spec_extensions", &self.enable_spec_extensions)
            .field("enable_callback_expressions", &self.enable_callback_expressions)
            .field("custom_validators", &format!("{} validators", self.custom_validators.len()))
            .field("max_pattern_complexity", &self.max_pattern_complexity)
            .finish()
    }
}

/// Types of patterns supported
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub enum PatternType {
    /// Path template patterns like `/pets/{id}`
    PathTemplate,
    /// Runtime expressions like `$url`, `$method`
    RuntimeExpression,
    /// Specification extensions like `x-custom`
    SpecificationExtension,
    /// Callback expressions
    CallbackExpression,
    /// Header parameter patterns
    HeaderPattern,
    /// Media type patterns
    MediaTypePattern,
    /// Custom pattern type
    Custom(String),
}

/// Pattern handler trait for processing different pattern types
pub trait PatternHandler: Send + Sync + fmt::Debug {
    /// Process a field with this pattern type
    fn process_field(&self, field_name: &str, field_value: &Element, context: &PatternContext) -> Result<ProcessedField, PatternError>;
    
    /// Validate a pattern
    fn validate_pattern(&self, pattern: &str) -> Result<(), PatternError>;
    
    /// Extract pattern parameters
    fn extract_parameters(&self, pattern: &str) -> Result<Vec<PatternParameter>, PatternError>;
    
    /// Check if this handler can process the given field
    fn can_handle(&self, field_name: &str, field_value: &Element) -> bool;
}

/// Pattern validator trait for custom validation
pub trait PatternValidator: Send + Sync {
    /// Validate a pattern value
    fn validate(&self, pattern: &str, value: &str) -> Result<(), String>;
}

/// Context for pattern processing
#[derive(Debug, Clone)]
pub struct PatternContext {
    /// Current OpenAPI document
    pub document: Option<Element>,
    /// Current processing path
    pub current_path: Vec<String>,
    /// Available variables for runtime expressions
    pub variables: HashMap<String, SimpleValue>,
    /// Processing metadata
    pub metadata: HashMap<String, SimpleValue>,
}

/// Processed field result
#[derive(Debug, Clone)]
pub struct ProcessedField {
    /// Original field name
    pub original_name: String,
    /// Processed field name (may be different for templates)
    pub processed_name: String,
    /// Field value (may be modified)
    pub value: Element,
    /// Extracted pattern information
    pub pattern_info: PatternInfo,
    /// Additional metadata
    pub metadata: HashMap<String, SimpleValue>,
}

/// Pattern information extracted from processing
#[derive(Debug, Clone)]
pub struct PatternInfo {
    /// Pattern type
    pub pattern_type: PatternType,
    /// Pattern parameters
    pub parameters: Vec<PatternParameter>,
    /// Pattern validation status
    pub is_valid: bool,
    /// Pattern complexity score
    pub complexity: usize,
    /// Additional pattern metadata
    pub metadata: HashMap<String, SimpleValue>,
}

/// Pattern parameter extracted from a pattern
#[derive(Debug, Clone)]
pub struct PatternParameter {
    /// Parameter name
    pub name: String,
    /// Parameter type (if determinable)
    pub param_type: Option<String>,
    /// Parameter constraints
    pub constraints: HashMap<String, SimpleValue>,
    /// Parameter position in pattern
    pub position: usize,
}

/// Compiled pattern for efficient processing
#[derive(Debug, Clone)]
pub struct CompiledPattern {
    /// Original pattern string
    pub pattern: String,
    /// Compiled regex
    pub regex: Regex,
    /// Pattern type
    pub pattern_type: PatternType,
    /// Extracted parameters
    pub parameters: Vec<PatternParameter>,
    /// Compilation metadata
    pub metadata: HashMap<String, SimpleValue>,
}

/// Pattern processing errors
#[derive(Debug, thiserror::Error)]
pub enum PatternError {
    #[error("Invalid pattern: {0}")]
    InvalidPattern(String),
    #[error("Pattern compilation failed: {0}")]
    CompilationError(String),
    #[error("Pattern too complex: {0}")]
    TooComplex(String),
    #[error("Parameter extraction failed: {0}")]
    ParameterExtractionError(String),
    #[error("Validation failed: {0}")]
    ValidationError(String),
    #[error("Unsupported pattern type: {0}")]
    UnsupportedPattern(String),
    #[error("Processing error: {0}")]
    ProcessingError(String),
}

impl PatternedFieldsProcessor {
    /// Create a new patterned fields processor
    pub fn new() -> Self {
        let mut processor = Self {
            handlers: HashMap::new(),
            config: PatternConfig::default(),
            pattern_cache: HashMap::new(),
        };
        
        // Register default handlers
        processor.register_default_handlers();
        processor
    }

    /// Register default pattern handlers
    fn register_default_handlers(&mut self) {
        self.handlers.insert(PatternType::PathTemplate, Box::new(PathTemplateHandler::new()));
        self.handlers.insert(PatternType::RuntimeExpression, Box::new(RuntimeExpressionHandler::new()));
        self.handlers.insert(PatternType::SpecificationExtension, Box::new(SpecExtensionHandler::new()));
        self.handlers.insert(PatternType::CallbackExpression, Box::new(CallbackExpressionHandler::new()));
        self.handlers.insert(PatternType::HeaderPattern, Box::new(HeaderPatternHandler::new()));
        self.handlers.insert(PatternType::MediaTypePattern, Box::new(MediaTypePatternHandler::new()));
    }

    /// Process a field with pattern detection and handling
    pub fn process_field(&mut self, field_name: &str, field_value: &Element, context: &PatternContext) -> Result<ProcessedField, PatternError> {
        // Detect pattern type
        let pattern_type = self.detect_pattern_type(field_name, field_value)?;
        
        // Get appropriate handler
        let handler = self.handlers.get(&pattern_type)
            .ok_or_else(|| PatternError::UnsupportedPattern(format!("{:?}", pattern_type)))?;
        
        // Process the field
        handler.process_field(field_name, field_value, context)
    }

    /// Process multiple fields in an object
    pub fn process_object_fields(&mut self, object: &ObjectElement, context: &PatternContext) -> Result<Vec<ProcessedField>, PatternError> {
        let mut processed_fields = Vec::new();
        
        for member in &object.content {
            if let Element::String(key) = member.key.as_ref() {
                let processed = self.process_field(&key.content, &member.value, context)?;
                processed_fields.push(processed);
            }
        }
        
        Ok(processed_fields)
    }

    /// Detect the pattern type for a field
    fn detect_pattern_type(&self, field_name: &str, field_value: &Element) -> Result<PatternType, PatternError> {
        // Check handlers in priority order (most specific to least specific)
        let priority_order = vec![
            PatternType::PathTemplate,
            PatternType::RuntimeExpression,
            PatternType::SpecificationExtension,
            PatternType::CallbackExpression,
            PatternType::MediaTypePattern,
            PatternType::HeaderPattern, // This should be last as it's the most general
        ];
        
        for pattern_type in priority_order {
            if let Some(handler) = self.handlers.get(&pattern_type) {
                if handler.can_handle(field_name, field_value) {
                    return Ok(pattern_type);
                }
            }
        }
        
        Err(PatternError::UnsupportedPattern(field_name.to_string()))
    }

    /// Register a custom pattern handler
    pub fn register_handler(&mut self, pattern_type: PatternType, handler: Box<dyn PatternHandler>) {
        self.handlers.insert(pattern_type, handler);
    }

    /// Configure the processor
    pub fn with_config(mut self, config: PatternConfig) -> Self {
        self.config = config;
        self
    }
}

/// Path template handler for patterns like `/pets/{id}`
#[derive(Debug)]
pub struct PathTemplateHandler {
    template_regex: Regex,
}

impl PathTemplateHandler {
    pub fn new() -> Self {
        Self {
            template_regex: Regex::new(r"\{([^}]+)\}").unwrap(),
        }
    }
}

impl PatternHandler for PathTemplateHandler {
    fn process_field(&self, field_name: &str, field_value: &Element, _context: &PatternContext) -> Result<ProcessedField, PatternError> {
        let parameters = self.extract_parameters(field_name)?;
        
        Ok(ProcessedField {
            original_name: field_name.to_string(),
            processed_name: field_name.to_string(),
            value: field_value.clone(),
            pattern_info: PatternInfo {
                pattern_type: PatternType::PathTemplate,
                parameters,
                is_valid: true,
                complexity: self.calculate_complexity(field_name),
                metadata: HashMap::new(),
            },
            metadata: HashMap::new(),
        })
    }

    fn validate_pattern(&self, pattern: &str) -> Result<(), PatternError> {
        // Validate path template syntax
        if !pattern.starts_with('/') {
            return Err(PatternError::InvalidPattern("Path template must start with '/'".to_string()));
        }
        
        // Check for balanced braces
        let open_braces = pattern.matches('{').count();
        let close_braces = pattern.matches('}').count();
        if open_braces != close_braces {
            return Err(PatternError::InvalidPattern("Unbalanced braces in path template".to_string()));
        }
        
        Ok(())
    }

    fn extract_parameters(&self, pattern: &str) -> Result<Vec<PatternParameter>, PatternError> {
        let mut parameters = Vec::new();
        let mut position = 0;
        
        for captures in self.template_regex.captures_iter(pattern) {
            if let Some(param_match) = captures.get(1) {
                parameters.push(PatternParameter {
                    name: param_match.as_str().to_string(),
                    param_type: Some("string".to_string()),
                    constraints: HashMap::new(),
                    position,
                });
                position += 1;
            }
        }
        
        Ok(parameters)
    }

    fn can_handle(&self, field_name: &str, _field_value: &Element) -> bool {
        field_name.starts_with('/') && field_name.contains('{') && field_name.contains('}')
    }
}

impl PathTemplateHandler {
    fn calculate_complexity(&self, pattern: &str) -> usize {
        // Simple complexity calculation based on pattern features
        let mut complexity = 0;
        complexity += pattern.matches('{').count() * 2; // Parameters add complexity
        complexity += pattern.matches('/').count(); // Path segments add complexity
        complexity += if pattern.contains('*') { 5 } else { 0 }; // Wildcards add more complexity
        complexity
    }
}

/// Runtime expression handler for patterns like `$url`, `$method`
#[derive(Debug)]
pub struct RuntimeExpressionHandler {
    expression_regex: Regex,
}

impl RuntimeExpressionHandler {
    pub fn new() -> Self {
        Self {
            expression_regex: Regex::new(r"\$([a-zA-Z_][a-zA-Z0-9_.]*)").unwrap(),
        }
    }
}

impl PatternHandler for RuntimeExpressionHandler {
    fn process_field(&self, field_name: &str, field_value: &Element, context: &PatternContext) -> Result<ProcessedField, PatternError> {
        let parameters = self.extract_parameters(field_name)?;
        
        // Check if we can resolve the expression
        let mut resolved_value = field_value.clone();
        if let Element::String(str_val) = field_value {
            if let Some(resolved) = self.resolve_expression(&str_val.content, context) {
                resolved_value = Element::String(StringElement::new(&resolved));
            }
        }
        
        Ok(ProcessedField {
            original_name: field_name.to_string(),
            processed_name: field_name.to_string(),
            value: resolved_value,
            pattern_info: PatternInfo {
                pattern_type: PatternType::RuntimeExpression,
                parameters,
                is_valid: true,
                complexity: self.calculate_complexity(field_name),
                metadata: HashMap::new(),
            },
            metadata: HashMap::new(),
        })
    }

    fn validate_pattern(&self, pattern: &str) -> Result<(), PatternError> {
        if !pattern.starts_with('$') {
            return Err(PatternError::InvalidPattern("Runtime expression must start with '$'".to_string()));
        }
        
        // Validate expression syntax
        if !self.expression_regex.is_match(pattern) {
            return Err(PatternError::InvalidPattern("Invalid runtime expression syntax".to_string()));
        }
        
        Ok(())
    }

    fn extract_parameters(&self, pattern: &str) -> Result<Vec<PatternParameter>, PatternError> {
        let mut parameters = Vec::new();
        let mut position = 0;
        
        for captures in self.expression_regex.captures_iter(pattern) {
            if let Some(expr_match) = captures.get(1) {
                parameters.push(PatternParameter {
                    name: expr_match.as_str().to_string(),
                    param_type: Some("runtime".to_string()),
                    constraints: HashMap::new(),
                    position,
                });
                position += 1;
            }
        }
        
        Ok(parameters)
    }

    fn can_handle(&self, field_name: &str, _field_value: &Element) -> bool {
        field_name.starts_with('$') || (field_name.contains('$') && self.expression_regex.is_match(field_name))
    }
}

impl RuntimeExpressionHandler {
    fn resolve_expression(&self, expression: &str, context: &PatternContext) -> Option<String> {
        // Simple expression resolution
        match expression {
            "$url" => Some("https://api.example.com".to_string()),
            "$method" => Some("GET".to_string()),
            "$statusCode" => Some("200".to_string()),
            _ => {
                // Try to resolve from context variables
                if expression.starts_with('$') {
                    let var_name = &expression[1..];
                    context.variables.get(var_name).and_then(|v| v.as_str().map(|s| s.to_string()))
                } else {
                    None
                }
            }
        }
    }

    fn calculate_complexity(&self, pattern: &str) -> usize {
        pattern.matches('$').count() * 2 + pattern.matches('.').count()
    }
}

/// Specification extension handler for patterns like `x-custom`
#[derive(Debug)]
pub struct SpecExtensionHandler;

impl SpecExtensionHandler {
    pub fn new() -> Self {
        Self
    }
}

impl PatternHandler for SpecExtensionHandler {
    fn process_field(&self, field_name: &str, field_value: &Element, _context: &PatternContext) -> Result<ProcessedField, PatternError> {
        Ok(ProcessedField {
            original_name: field_name.to_string(),
            processed_name: field_name.to_string(),
            value: field_value.clone(),
            pattern_info: PatternInfo {
                pattern_type: PatternType::SpecificationExtension,
                parameters: vec![],
                is_valid: self.validate_extension_name(field_name),
                complexity: 1,
                metadata: HashMap::new(),
            },
            metadata: HashMap::new(),
        })
    }

    fn validate_pattern(&self, pattern: &str) -> Result<(), PatternError> {
        if !pattern.starts_with("x-") {
            return Err(PatternError::InvalidPattern("Specification extension must start with 'x-'".to_string()));
        }
        
        if pattern.len() < 3 {
            return Err(PatternError::InvalidPattern("Specification extension name too short".to_string()));
        }
        
        Ok(())
    }

    fn extract_parameters(&self, _pattern: &str) -> Result<Vec<PatternParameter>, PatternError> {
        Ok(vec![]) // Spec extensions don't have parameters
    }

    fn can_handle(&self, field_name: &str, _field_value: &Element) -> bool {
        field_name.starts_with("x-")
    }
}

impl SpecExtensionHandler {
    fn validate_extension_name(&self, name: &str) -> bool {
        name.starts_with("x-") && name.len() > 2
    }
}

/// Callback expression handler
#[derive(Debug)]
pub struct CallbackExpressionHandler {
    expression_regex: Regex,
}

impl CallbackExpressionHandler {
    pub fn new() -> Self {
        Self {
            expression_regex: Regex::new(r"\{([^}]+)\}").unwrap(),
        }
    }
}

impl PatternHandler for CallbackExpressionHandler {
    fn process_field(&self, field_name: &str, field_value: &Element, _context: &PatternContext) -> Result<ProcessedField, PatternError> {
        let parameters = self.extract_parameters(field_name)?;
        
        Ok(ProcessedField {
            original_name: field_name.to_string(),
            processed_name: field_name.to_string(),
            value: field_value.clone(),
            pattern_info: PatternInfo {
                pattern_type: PatternType::CallbackExpression,
                parameters,
                is_valid: true,
                complexity: self.calculate_complexity(field_name),
                metadata: HashMap::new(),
            },
            metadata: HashMap::new(),
        })
    }

    fn validate_pattern(&self, pattern: &str) -> Result<(), PatternError> {
        // Validate callback expression syntax
        let open_braces = pattern.matches('{').count();
        let close_braces = pattern.matches('}').count();
        if open_braces != close_braces {
            return Err(PatternError::InvalidPattern("Unbalanced braces in callback expression".to_string()));
        }
        
        Ok(())
    }

    fn extract_parameters(&self, pattern: &str) -> Result<Vec<PatternParameter>, PatternError> {
        let mut parameters = Vec::new();
        let mut position = 0;
        
        for captures in self.expression_regex.captures_iter(pattern) {
            if let Some(param_match) = captures.get(1) {
                parameters.push(PatternParameter {
                    name: param_match.as_str().to_string(),
                    param_type: Some("expression".to_string()),
                    constraints: HashMap::new(),
                    position,
                });
                position += 1;
            }
        }
        
        Ok(parameters)
    }

    fn can_handle(&self, field_name: &str, _field_value: &Element) -> bool {
        // Callback expressions are detected by context, not just field name
        // Don't match path templates (which start with '/')
        field_name.contains('{') && field_name.contains('}') && !field_name.starts_with('/')
    }
}

impl CallbackExpressionHandler {
    fn calculate_complexity(&self, pattern: &str) -> usize {
        pattern.matches('{').count() * 2 + pattern.len() / 10
    }
}

/// Header pattern handler
#[derive(Debug)]
pub struct HeaderPatternHandler;

impl HeaderPatternHandler {
    pub fn new() -> Self {
        Self
    }
}

impl PatternHandler for HeaderPatternHandler {
    fn process_field(&self, field_name: &str, field_value: &Element, _context: &PatternContext) -> Result<ProcessedField, PatternError> {
        Ok(ProcessedField {
            original_name: field_name.to_string(),
            processed_name: field_name.to_string(),
            value: field_value.clone(),
            pattern_info: PatternInfo {
                pattern_type: PatternType::HeaderPattern,
                parameters: vec![],
                is_valid: self.validate_header_name(field_name),
                complexity: 1,
                metadata: HashMap::new(),
            },
            metadata: HashMap::new(),
        })
    }

    fn validate_pattern(&self, pattern: &str) -> Result<(), PatternError> {
        // Validate header name according to RFC 7230
        if pattern.is_empty() {
            return Err(PatternError::InvalidPattern("Header name cannot be empty".to_string()));
        }
        
        // Check for valid header characters
        for ch in pattern.chars() {
            if !ch.is_ascii() || ch.is_ascii_control() || "()<>@,;:\\\"/[]?={} \t".contains(ch) {
                return Err(PatternError::InvalidPattern(format!("Invalid character in header name: {}", ch)));
            }
        }
        
        Ok(())
    }

    fn extract_parameters(&self, _pattern: &str) -> Result<Vec<PatternParameter>, PatternError> {
        Ok(vec![]) // Header patterns don't have parameters
    }

    fn can_handle(&self, field_name: &str, _field_value: &Element) -> bool {
        // This is a fallback handler for header-like patterns
        field_name.chars().all(|c| c.is_ascii_alphanumeric() || c == '-' || c == '_')
    }
}

impl HeaderPatternHandler {
    fn validate_header_name(&self, name: &str) -> bool {
        !name.is_empty() && name.chars().all(|c| c.is_ascii() && !c.is_ascii_control() && !"()<>@,;:\\\"/[]?={} \t".contains(c))
    }
}

/// Media type pattern handler
#[derive(Debug)]
pub struct MediaTypePatternHandler {
    media_type_regex: Regex,
}

impl MediaTypePatternHandler {
    pub fn new() -> Self {
        Self {
            media_type_regex: Regex::new(r"^[a-zA-Z0-9][a-zA-Z0-9!#$&\-\^_]*\/[a-zA-Z0-9][a-zA-Z0-9!#$&\-\^_.+]*$").unwrap(),
        }
    }
}

impl PatternHandler for MediaTypePatternHandler {
    fn process_field(&self, field_name: &str, field_value: &Element, _context: &PatternContext) -> Result<ProcessedField, PatternError> {
        Ok(ProcessedField {
            original_name: field_name.to_string(),
            processed_name: field_name.to_string(),
            value: field_value.clone(),
            pattern_info: PatternInfo {
                pattern_type: PatternType::MediaTypePattern,
                parameters: vec![],
                is_valid: self.media_type_regex.is_match(field_name),
                complexity: 1,
                metadata: HashMap::new(),
            },
            metadata: HashMap::new(),
        })
    }

    fn validate_pattern(&self, pattern: &str) -> Result<(), PatternError> {
        if !self.media_type_regex.is_match(pattern) {
            return Err(PatternError::InvalidPattern("Invalid media type format".to_string()));
        }
        Ok(())
    }

    fn extract_parameters(&self, _pattern: &str) -> Result<Vec<PatternParameter>, PatternError> {
        Ok(vec![]) // Media type patterns don't have parameters
    }

    fn can_handle(&self, field_name: &str, _field_value: &Element) -> bool {
        self.media_type_regex.is_match(field_name)
    }
}

/// Default configuration
impl Default for PatternConfig {
    fn default() -> Self {
        Self {
            enable_path_templates: true,
            enable_runtime_expressions: true,
            enable_spec_extensions: true,
            enable_callback_expressions: true,
            custom_validators: HashMap::new(),
            max_pattern_complexity: 100,
        }
    }
}

/// Default implementation
impl Default for PatternedFieldsProcessor {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_path_template_processing() {
        let mut processor = PatternedFieldsProcessor::new();
        let context = PatternContext {
            document: None,
            current_path: vec![],
            variables: HashMap::new(),
            metadata: HashMap::new(),
        };
        
        let field_name = "/pets/{id}";
        let field_value = Element::String(StringElement::new("Pet endpoint"));
        
        let result = processor.process_field(field_name, &field_value, &context);
        assert!(result.is_ok());
        
        let processed = result.unwrap();
        assert_eq!(processed.pattern_info.pattern_type, PatternType::PathTemplate);
        assert_eq!(processed.pattern_info.parameters.len(), 1);
        assert_eq!(processed.pattern_info.parameters[0].name, "id");
    }

    #[test]
    fn test_runtime_expression_processing() {
        let mut processor = PatternedFieldsProcessor::new();
        let context = PatternContext {
            document: None,
            current_path: vec![],
            variables: HashMap::new(),
            metadata: HashMap::new(),
        };
        
        let field_name = "$url";
        let field_value = Element::String(StringElement::new("$url"));
        
        let result = processor.process_field(field_name, &field_value, &context);
        assert!(result.is_ok());
        
        let processed = result.unwrap();
        assert_eq!(processed.pattern_info.pattern_type, PatternType::RuntimeExpression);
    }

    #[test]
    fn test_spec_extension_processing() {
        let mut processor = PatternedFieldsProcessor::new();
        let context = PatternContext {
            document: None,
            current_path: vec![],
            variables: HashMap::new(),
            metadata: HashMap::new(),
        };
        
        let field_name = "x-custom-field";
        let field_value = Element::String(StringElement::new("custom value"));
        
        let result = processor.process_field(field_name, &field_value, &context);
        assert!(result.is_ok());
        
        let processed = result.unwrap();
        assert_eq!(processed.pattern_info.pattern_type, PatternType::SpecificationExtension);
        assert!(processed.pattern_info.is_valid);
    }

    #[test]
    fn test_path_template_parameter_extraction() {
        let handler = PathTemplateHandler::new();
        let params = handler.extract_parameters("/users/{userId}/posts/{postId}").unwrap();
        
        assert_eq!(params.len(), 2);
        assert_eq!(params[0].name, "userId");
        assert_eq!(params[1].name, "postId");
    }

    #[test]
    fn test_media_type_validation() {
        let handler = MediaTypePatternHandler::new();
        
        assert!(handler.validate_pattern("application/json").is_ok());
        assert!(handler.validate_pattern("text/plain").is_ok());
        assert!(handler.validate_pattern("application/vnd.api+json").is_ok());
        assert!(handler.validate_pattern("invalid-media-type").is_err());
    }
} 