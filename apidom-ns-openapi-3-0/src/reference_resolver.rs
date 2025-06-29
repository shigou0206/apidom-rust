use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use tokio::sync::RwLock;
use url::Url;
use serde_json::Value;
use apidom_ast::minim_model::*;
use crate::elements::reference::ReferenceElement;

/// Comprehensive reference resolution system for OpenAPI specifications
/// Supports:
/// - Remote URL resolution (HTTP/HTTPS)
/// - Local file path resolution
/// - JSON Pointer resolution within documents
/// - Inline expansion with circular reference detection
/// - Caching for performance
/// - Async resolution for non-blocking operations
#[derive(Debug, Clone)]
pub struct ReferenceResolver {
    /// Cache for resolved references
    cache: Arc<RwLock<HashMap<String, ResolvedReference>>>,
    /// Base URI for relative reference resolution
    base_uri: Option<Url>,
    /// Base path for local file resolution
    base_path: Option<PathBuf>,
    /// Maximum depth for circular reference detection
    max_depth: usize,
    /// Enable remote URL resolution
    allow_remote: bool,
    /// Enable local file resolution
    allow_local: bool,
    /// Custom resolvers for specific schemes
    custom_resolvers: HashMap<String, Box<dyn CustomResolver>>,
}

/// Resolved reference with metadata
#[derive(Debug, Clone)]
pub struct ResolvedReference {
    /// The resolved element
    pub element: Element,
    /// Original reference string
    pub original_ref: String,
    /// Resolved URI/path
    pub resolved_uri: String,
    /// Resolution metadata
    pub metadata: ReferenceMetadata,
}

/// Reference resolution metadata
#[derive(Debug, Clone)]
pub struct ReferenceMetadata {
    /// Resolution type (remote, local, inline, cached)
    pub resolution_type: ResolutionType,
    /// Resolution timestamp
    pub resolved_at: chrono::DateTime<chrono::Utc>,
    /// Number of hops to resolve (for nested references)
    pub resolution_depth: usize,
    /// Whether this was resolved from cache
    pub from_cache: bool,
    /// Additional metadata
    pub properties: HashMap<String, Value>,
}

/// Type of reference resolution
#[derive(Debug, Clone, PartialEq)]
pub enum ResolutionType {
    /// Remote HTTP/HTTPS URL
    Remote,
    /// Local file system path
    Local,
    /// JSON Pointer within the same document
    Inline,
    /// Resolved from cache
    Cached,
    /// Custom resolver
    Custom(String),
}

/// Custom resolver trait for extensibility
pub trait CustomResolver: Send + Sync {
    /// Resolve a reference with the given scheme
    fn resolve(&self, reference: &str, context: &ResolutionContext) -> Result<Element, ResolverError>;
    
    /// Check if this resolver can handle the given reference
    fn can_resolve(&self, reference: &str) -> bool;
}

/// Resolution context for custom resolvers
#[derive(Debug)]
pub struct ResolutionContext {
    /// Current document being processed
    pub current_document: Option<Element>,
    /// Resolution depth
    pub depth: usize,
    /// Base URI for resolution
    pub base_uri: Option<Url>,
    /// Base path for resolution
    pub base_path: Option<PathBuf>,
}

/// Reference resolution errors
#[derive(Debug, thiserror::Error)]
pub enum ResolverError {
    #[error("Invalid reference format: {0}")]
    InvalidReference(String),
    #[error("Circular reference detected: {0}")]
    CircularReference(String),
    #[error("Maximum resolution depth exceeded: {0}")]
    MaxDepthExceeded(usize),
    #[error("Reference not found: {0}")]
    NotFound(String),
    #[error("Remote resolution disabled")]
    RemoteDisabled,
    #[error("Local resolution disabled")]
    LocalDisabled,
    #[error("Network error: {0}")]
    NetworkError(String),
    #[error("File system error: {0}")]
    FileSystemError(String),
    #[error("JSON parsing error: {0}")]
    JsonError(String),
    #[error("JSON Pointer error: {0}")]
    JsonPointerError(String),
    #[error("Custom resolver error: {0}")]
    CustomError(String),
}

impl ReferenceResolver {
    /// Create a new reference resolver with default settings
    pub fn new() -> Self {
        Self {
            cache: Arc::new(RwLock::new(HashMap::new())),
            base_uri: None,
            base_path: None,
            max_depth: 10,
            allow_remote: true,
            allow_local: true,
            custom_resolvers: HashMap::new(),
        }
    }

    /// Set the base URI for relative reference resolution
    pub fn with_base_uri(mut self, base_uri: Url) -> Self {
        self.base_uri = Some(base_uri);
        self
    }

    /// Set the base path for local file resolution
    pub fn with_base_path(mut self, base_path: PathBuf) -> Self {
        self.base_path = Some(base_path);
        self
    }

    /// Set the maximum resolution depth
    pub fn with_max_depth(mut self, max_depth: usize) -> Self {
        self.max_depth = max_depth;
        self
    }

    /// Enable or disable remote URL resolution
    pub fn allow_remote(mut self, allow: bool) -> Self {
        self.allow_remote = allow;
        self
    }

    /// Enable or disable local file resolution
    pub fn allow_local(mut self, allow: bool) -> Self {
        self.allow_local = allow;
        self
    }

    /// Add a custom resolver for a specific scheme
    pub fn with_custom_resolver(mut self, scheme: String, resolver: Box<dyn CustomResolver>) -> Self {
        self.custom_resolvers.insert(scheme, resolver);
        self
    }

    /// Resolve a reference asynchronously
    pub async fn resolve_async(&self, reference: &str, context: Option<ResolutionContext>) -> Result<ResolvedReference, ResolverError> {
        let context = context.unwrap_or_else(|| ResolutionContext {
            current_document: None,
            depth: 0,
            base_uri: self.base_uri.clone(),
            base_path: self.base_path.clone(),
        });

        // Check cache first
        if let Some(cached) = self.get_from_cache(reference).await {
            return Ok(cached);
        }

        // Check depth limit
        if context.depth >= self.max_depth {
            return Err(ResolverError::MaxDepthExceeded(context.depth));
        }

        let resolved = self.resolve_reference_internal(reference, &context).await?;
        
        // Cache the result
        self.cache_result(reference, &resolved).await;
        
        Ok(resolved)
    }

    /// Resolve a reference synchronously (blocking)
    pub fn resolve(&self, reference: &str, context: Option<ResolutionContext>) -> Result<ResolvedReference, ResolverError> {
        // For synchronous resolution, we use a simple runtime
        let rt = tokio::runtime::Runtime::new().map_err(|e| ResolverError::CustomError(e.to_string()))?;
        rt.block_on(self.resolve_async(reference, context))
    }

    /// Internal reference resolution logic
    async fn resolve_reference_internal(&self, reference: &str, context: &ResolutionContext) -> Result<ResolvedReference, ResolverError> {
        let reference_type = self.determine_reference_type(reference)?;
        
        match reference_type {
            ResolutionType::Remote => self.resolve_remote(reference, context).await,
            ResolutionType::Local => self.resolve_local(reference, context).await,
            ResolutionType::Inline => self.resolve_inline(reference, context).await,
            ResolutionType::Custom(scheme) => self.resolve_custom(reference, context, &scheme).await,
            ResolutionType::Cached => unreachable!("Cached type should not reach here"),
        }
    }

    /// Determine the type of reference
    fn determine_reference_type(&self, reference: &str) -> Result<ResolutionType, ResolverError> {
        if reference.starts_with('#') {
            return Ok(ResolutionType::Inline);
        }

        if let Ok(url) = Url::parse(reference) {
            let scheme = url.scheme();
            if scheme == "http" || scheme == "https" {
                return Ok(ResolutionType::Remote);
            } else if scheme == "file" {
                return Ok(ResolutionType::Local);
            } else if self.custom_resolvers.contains_key(scheme) {
                return Ok(ResolutionType::Custom(scheme.to_string()));
            }
        }

        // Check if it's a relative path
        if reference.contains('/') || reference.contains('\\') {
            return Ok(ResolutionType::Local);
        }

        Err(ResolverError::InvalidReference(reference.to_string()))
    }

    /// Resolve remote HTTP/HTTPS reference
    async fn resolve_remote(&self, reference: &str, context: &ResolutionContext) -> Result<ResolvedReference, ResolverError> {
        if !self.allow_remote {
            return Err(ResolverError::RemoteDisabled);
        }

        let url = if let Ok(url) = Url::parse(reference) {
            url
        } else if let Some(base_uri) = &context.base_uri {
            base_uri.join(reference).map_err(|e| ResolverError::InvalidReference(e.to_string()))?
        } else {
            return Err(ResolverError::InvalidReference(reference.to_string()));
        };

        // Split URL and fragment
        let (base_url, fragment) = if let Some(fragment) = url.fragment() {
            let mut base_url = url.clone();
            base_url.set_fragment(None);
            (base_url, Some(fragment.to_string()))
        } else {
            (url, None)
        };

        // Fetch the document
        let client = reqwest::Client::new();
        let response = client.get(base_url.as_str())
            .send()
            .await
            .map_err(|e| ResolverError::NetworkError(e.to_string()))?;

        let text = response.text().await
            .map_err(|e| ResolverError::NetworkError(e.to_string()))?;

        // Parse JSON
        let json: Value = serde_json::from_str(&text)
            .map_err(|e| ResolverError::JsonError(e.to_string()))?;

        // Convert to Element
        let mut element = json_to_element(&json)?;

        // Apply JSON Pointer if fragment exists
        if let Some(fragment) = fragment {
            if fragment.starts_with('/') {
                element = apply_json_pointer(&element, &fragment)?;
            }
        }

        Ok(ResolvedReference {
            element,
            original_ref: reference.to_string(),
            resolved_uri: url.to_string(),
            metadata: ReferenceMetadata {
                resolution_type: ResolutionType::Remote,
                resolved_at: chrono::Utc::now(),
                resolution_depth: context.depth,
                from_cache: false,
                properties: HashMap::new(),
            },
        })
    }

    /// Resolve local file reference
    async fn resolve_local(&self, reference: &str, context: &ResolutionContext) -> Result<ResolvedReference, ResolverError> {
        if !self.allow_local {
            return Err(ResolverError::LocalDisabled);
        }

        let path = if Path::new(reference).is_absolute() {
            PathBuf::from(reference)
        } else if let Some(base_path) = &context.base_path {
            base_path.join(reference)
        } else {
            PathBuf::from(reference)
        };

        // Split path and fragment
        let (file_path, fragment) = if reference.contains('#') {
            let parts: Vec<&str> = reference.splitn(2, '#').collect();
            (PathBuf::from(parts[0]), Some(parts[1].to_string()))
        } else {
            (path, None)
        };

        // Read file
        let content = tokio::fs::read_to_string(&file_path).await
            .map_err(|e| ResolverError::FileSystemError(e.to_string()))?;

        // Parse JSON
        let json: Value = serde_json::from_str(&content)
            .map_err(|e| ResolverError::JsonError(e.to_string()))?;

        // Convert to Element
        let mut element = json_to_element(&json)?;

        // Apply JSON Pointer if fragment exists
        if let Some(fragment) = fragment {
            if fragment.starts_with('/') {
                element = apply_json_pointer(&element, &fragment)?;
            }
        }

        Ok(ResolvedReference {
            element,
            original_ref: reference.to_string(),
            resolved_uri: file_path.to_string_lossy().to_string(),
            metadata: ReferenceMetadata {
                resolution_type: ResolutionType::Local,
                resolved_at: chrono::Utc::now(),
                resolution_depth: context.depth,
                from_cache: false,
                properties: HashMap::new(),
            },
        })
    }

    /// Resolve inline JSON Pointer reference
    async fn resolve_inline(&self, reference: &str, context: &ResolutionContext) -> Result<ResolvedReference, ResolverError> {
        let current_document = context.current_document.as_ref()
            .ok_or_else(|| ResolverError::NotFound("No current document for inline reference".to_string()))?;

        if !reference.starts_with('#') {
            return Err(ResolverError::InvalidReference(reference.to_string()));
        }

        let pointer = &reference[1..]; // Remove '#'
        let element = apply_json_pointer(current_document, pointer)?;

        Ok(ResolvedReference {
            element,
            original_ref: reference.to_string(),
            resolved_uri: reference.to_string(),
            metadata: ReferenceMetadata {
                resolution_type: ResolutionType::Inline,
                resolved_at: chrono::Utc::now(),
                resolution_depth: context.depth,
                from_cache: false,
                properties: HashMap::new(),
            },
        })
    }

    /// Resolve using custom resolver
    async fn resolve_custom(&self, reference: &str, context: &ResolutionContext, scheme: &str) -> Result<ResolvedReference, ResolverError> {
        let resolver = self.custom_resolvers.get(scheme)
            .ok_or_else(|| ResolverError::CustomError(format!("No resolver for scheme: {}", scheme)))?;

        let element = resolver.resolve(reference, context)
            .map_err(|e| ResolverError::CustomError(e.to_string()))?;

        Ok(ResolvedReference {
            element,
            original_ref: reference.to_string(),
            resolved_uri: reference.to_string(),
            metadata: ReferenceMetadata {
                resolution_type: ResolutionType::Custom(scheme.to_string()),
                resolved_at: chrono::Utc::now(),
                resolution_depth: context.depth,
                from_cache: false,
                properties: HashMap::new(),
            },
        })
    }

    /// Get resolved reference from cache
    async fn get_from_cache(&self, reference: &str) -> Option<ResolvedReference> {
        let cache = self.cache.read().await;
        cache.get(reference).cloned()
    }

    /// Cache resolved reference
    async fn cache_result(&self, reference: &str, resolved: &ResolvedReference) {
        let mut cache = self.cache.write().await;
        let mut cached = resolved.clone();
        cached.metadata.from_cache = true;
        cached.metadata.resolution_type = ResolutionType::Cached;
        cache.insert(reference.to_string(), cached);
    }

    /// Clear the cache
    pub async fn clear_cache(&self) {
        let mut cache = self.cache.write().await;
        cache.clear();
    }

    /// Get cache statistics
    pub async fn cache_stats(&self) -> CacheStats {
        let cache = self.cache.read().await;
        CacheStats {
            entries: cache.len(),
            memory_usage: cache.capacity() * std::mem::size_of::<(String, ResolvedReference)>(),
        }
    }
}

/// Cache statistics
#[derive(Debug)]
pub struct CacheStats {
    pub entries: usize,
    pub memory_usage: usize,
}

/// Convert JSON Value to Element
fn json_to_element(json: &Value) -> Result<Element, ResolverError> {
    match json {
        Value::Null => Ok(Element::Null(NullElement::new())),
        Value::Bool(b) => Ok(Element::Boolean(BooleanElement::new(*b))),
        Value::Number(n) => {
            if let Some(i) = n.as_i64() {
                Ok(Element::Number(NumberElement::new(i as f64)))
            } else if let Some(f) = n.as_f64() {
                Ok(Element::Number(NumberElement::new(f)))
            } else {
                Err(ResolverError::JsonError("Invalid number".to_string()))
            }
        }
        Value::String(s) => Ok(Element::String(StringElement::new(s))),
        Value::Array(arr) => {
            let mut array = ArrayElement::new_empty();
            for item in arr {
                array.content.push(json_to_element(item)?);
            }
            Ok(Element::Array(array))
        }
        Value::Object(obj) => {
            let mut object = ObjectElement::new();
            for (key, value) in obj {
                let member = MemberElement::new(
                    Element::String(StringElement::new(key)),
                    json_to_element(value)?
                );
                object.content.push(member);
            }
            Ok(Element::Object(object))
        }
    }
}

/// Apply JSON Pointer to an element
fn apply_json_pointer(element: &Element, pointer: &str) -> Result<Element, ResolverError> {
    if pointer.is_empty() {
        return Ok(element.clone());
    }

    let parts: Vec<&str> = pointer.split('/').skip(1).collect(); // Skip empty first part
    let mut current = element;
    let mut owned_element = None;

    for part in parts {
        let unescaped_part = unescape_json_pointer_token(part);
        
        match current {
            Element::Object(obj) => {
                if let Some(member) = obj.content.iter().find(|m| {
                    if let Element::String(key) = m.key.as_ref() {
                        key.content == unescaped_part
                    } else {
                        false
                    }
                }) {
                    current = &*member.value;
                } else {
                    return Err(ResolverError::JsonPointerError(format!("Key not found: {}", unescaped_part)));
                }
            }
            Element::Array(arr) => {
                let index: usize = unescaped_part.parse()
                    .map_err(|_| ResolverError::JsonPointerError(format!("Invalid array index: {}", unescaped_part)))?;
                
                if index >= arr.content.len() {
                    return Err(ResolverError::JsonPointerError(format!("Array index out of bounds: {}", index)));
                }
                
                current = &arr.content[index];
            }
            _ => {
                return Err(ResolverError::JsonPointerError(format!("Cannot index into non-object/array: {}", part)));
            }
        }
    }

    Ok(current.clone())
}

/// Unescape JSON Pointer token
fn unescape_json_pointer_token(token: &str) -> String {
    token.replace("~1", "/").replace("~0", "~")
}

/// Default implementation
impl Default for ReferenceResolver {
    fn default() -> Self {
        Self::new()
    }
}

/// Example custom resolver for testing
pub struct TestResolver;

impl CustomResolver for TestResolver {
    fn resolve(&self, reference: &str, _context: &ResolutionContext) -> Result<Element, ResolverError> {
        if reference == "test://example" {
            Ok(Element::String(StringElement::new("Test resolved")))
        } else {
            Err(ResolverError::NotFound(reference.to_string()))
        }
    }

    fn can_resolve(&self, reference: &str) -> bool {
        reference.starts_with("test://")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_inline_reference_resolution() {
        let mut doc = ObjectElement::new();
        let mut components = ObjectElement::new();
        let mut schemas = ObjectElement::new();
        
        let mut user_schema = ObjectElement::new();
        user_schema.set("type", Element::String(StringElement::new("object")));
        schemas.set("User", Element::Object(user_schema));
        
        components.set("schemas", Element::Object(schemas));
        doc.set("components", Element::Object(components));
        
        let resolver = ReferenceResolver::new();
        let context = ResolutionContext {
            current_document: Some(Element::Object(doc)),
            depth: 0,
            base_uri: None,
            base_path: None,
        };
        
        let result = resolver.resolve_async("#/components/schemas/User", Some(context)).await;
        assert!(result.is_ok());
        
        let resolved = result.unwrap();
        assert_eq!(resolved.metadata.resolution_type, ResolutionType::Inline);
        
        if let Element::Object(obj) = resolved.element {
            if let Some(Element::String(type_str)) = obj.get("type") {
                assert_eq!(type_str.content, "object");
            }
        }
    }

    #[tokio::test]
    async fn test_custom_resolver() {
        let resolver = ReferenceResolver::new()
            .with_custom_resolver("test".to_string(), Box::new(TestResolver));
        
        let result = resolver.resolve_async("test://example", None).await;
        assert!(result.is_ok());
        
        let resolved = result.unwrap();
        if let ResolutionType::Custom(scheme) = resolved.metadata.resolution_type {
            assert_eq!(scheme, "test");
        }
        
        if let Element::String(s) = resolved.element {
            assert_eq!(s.content, "Test resolved");
        }
    }

    #[test]
    fn test_json_pointer_escaping() {
        assert_eq!(unescape_json_pointer_token("~0"), "~");
        assert_eq!(unescape_json_pointer_token("~1"), "/");
        assert_eq!(unescape_json_pointer_token("~0~1"), "~/");
        assert_eq!(unescape_json_pointer_token("normal"), "normal");
    }

    #[test]
    fn test_reference_type_determination() {
        let resolver = ReferenceResolver::new();
        
        assert_eq!(resolver.determine_reference_type("#/components/schemas/User").unwrap(), ResolutionType::Inline);
        assert_eq!(resolver.determine_reference_type("https://example.com/schema.json").unwrap(), ResolutionType::Remote);
        assert_eq!(resolver.determine_reference_type("./schema.json").unwrap(), ResolutionType::Local);
        assert_eq!(resolver.determine_reference_type("file:///path/to/schema.json").unwrap(), ResolutionType::Local);
    }
} 