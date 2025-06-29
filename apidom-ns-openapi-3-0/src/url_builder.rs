use std::collections::HashMap;
use apidom_ast::minim_model::Element;

/// URL构建器，用于从OpenAPI组件构建完整的URL
#[derive(Debug, Clone)]
pub struct UrlBuilder {
    /// 基础URL（来自server.url）
    pub base_url: String,
    /// 路径模板（来自paths）
    pub path_template: String,
    /// 路径参数值
    pub path_parameters: HashMap<String, String>,
    /// 查询参数
    pub query_parameters: HashMap<String, String>,
}

impl UrlBuilder {
    /// 创建新的URL构建器
    /// 
    /// # Arguments
    /// * `base_url` - 基础URL，通常来自OpenAPI的server.url
    /// 
    /// # Example
    /// ```
    /// use apidom_ns_openapi_3_0::url_builder::UrlBuilder;
    /// 
    /// let builder = UrlBuilder::new("https://api.example.com/v1");
    /// ```
    pub fn new(base_url: &str) -> Self {
        Self {
            base_url: base_url.trim_end_matches('/').to_string(),
            path_template: String::new(),
            path_parameters: HashMap::new(),
            query_parameters: HashMap::new(),
        }
    }

    /// 从OpenAPI服务器对象创建URL构建器
    /// 
    /// # Arguments
    /// * `server_element` - OpenAPI服务器元素
    /// 
    /// # Returns
    /// 如果服务器元素包含有效的URL，则返回URL构建器
    pub fn from_server_element(server_element: &Element) -> Option<Self> {
        if let Element::Object(server_obj) = server_element {
            if let Some(Element::String(url_str)) = server_obj.get("url") {
                return Some(Self::new(&url_str.content));
            }
        }
        None
    }

    /// 设置路径模板
    /// 
    /// # Arguments
    /// * `path` - 路径模板，如 "/users/{userId}/posts/{postId}"
    /// 
    /// # Example
    /// ```
    /// use apidom_ns_openapi_3_0::url_builder::UrlBuilder;
    /// 
    /// let mut builder = UrlBuilder::new("https://api.example.com");
    /// builder.path("/users/{userId}");
    /// ```
    pub fn path(&mut self, path: &str) -> &mut Self {
        self.path_template = path.to_string();
        self
    }

    /// 设置路径参数
    /// 
    /// # Arguments
    /// * `name` - 参数名
    /// * `value` - 参数值
    /// 
    /// # Example
    /// ```
    /// use apidom_ns_openapi_3_0::url_builder::UrlBuilder;
    /// 
    /// let mut builder = UrlBuilder::new("https://api.example.com");
    /// builder.path("/users/{userId}")
    ///        .path_param("userId", "123");
    /// ```
    pub fn path_param(&mut self, name: &str, value: &str) -> &mut Self {
        self.path_parameters.insert(name.to_string(), value.to_string());
        self
    }

    /// 批量设置路径参数
    /// 
    /// # Arguments
    /// * `params` - 参数映射
    pub fn path_params(&mut self, params: HashMap<String, String>) -> &mut Self {
        self.path_parameters.extend(params);
        self
    }

    /// 设置查询参数
    /// 
    /// # Arguments
    /// * `name` - 参数名
    /// * `value` - 参数值
    /// 
    /// # Example
    /// ```
    /// use apidom_ns_openapi_3_0::url_builder::UrlBuilder;
    /// 
    /// let mut builder = UrlBuilder::new("https://api.example.com");
    /// builder.path("/users")
    ///        .query_param("page", "1")
    ///        .query_param("limit", "10");
    /// ```
    pub fn query_param(&mut self, name: &str, value: &str) -> &mut Self {
        self.query_parameters.insert(name.to_string(), value.to_string());
        self
    }

    /// 批量设置查询参数
    /// 
    /// # Arguments
    /// * `params` - 参数映射
    pub fn query_params(&mut self, params: HashMap<String, String>) -> &mut Self {
        self.query_parameters.extend(params);
        self
    }

    /// 构建完整的URL
    /// 
    /// # Returns
    /// 构建的完整URL字符串
    /// 
    /// # Example
    /// ```
    /// use apidom_ns_openapi_3_0::url_builder::UrlBuilder;
    /// use std::collections::HashMap;
    /// 
    /// let mut builder = UrlBuilder::new("https://api.example.com/v1");
    /// let url = builder.path("/users/{userId}/posts")
    ///                  .path_param("userId", "123")
    ///                  .query_param("page", "1")
    ///                  .build();
    /// 
    /// assert_eq!(url, "https://api.example.com/v1/users/123/posts?page=1");
    /// ```
    pub fn build(&self) -> String {
        let mut url = self.base_url.clone();
        
        // 处理路径模板和路径参数
        let mut resolved_path = self.path_template.clone();
        for (param_name, param_value) in &self.path_parameters {
            let placeholder = format!("{{{}}}", param_name);
            resolved_path = resolved_path.replace(&placeholder, param_value);
        }
        
        // 确保路径以 / 开头
        if !resolved_path.is_empty() && !resolved_path.starts_with('/') {
            resolved_path = format!("/{}", resolved_path);
        }
        
        url.push_str(&resolved_path);
        
        // 添加查询参数
        if !self.query_parameters.is_empty() {
            url.push('?');
            let query_parts: Vec<String> = self.query_parameters
                .iter()
                .map(|(key, value)| format!("{}={}", urlencoding::encode(key), urlencoding::encode(value)))
                .collect();
            url.push_str(&query_parts.join("&"));
        }
        
        url
    }

    /// 重置所有参数
    pub fn reset(&mut self) -> &mut Self {
        self.path_template.clear();
        self.path_parameters.clear();
        self.query_parameters.clear();
        self
    }

    /// 克隆构建器但保留基础URL
    pub fn clone_base(&self) -> Self {
        Self::new(&self.base_url)
    }
}

/// 从OpenAPI文档中提取服务器URL的辅助函数
/// 
/// # Arguments
/// * `openapi_element` - OpenAPI文档元素
/// 
/// # Returns
/// 服务器URL列表
pub fn extract_server_urls(openapi_element: &Element) -> Vec<String> {
    let mut urls = Vec::new();
    
    if let Element::Object(openapi_obj) = openapi_element {
        if let Some(Element::Array(servers_arr)) = openapi_obj.get("servers") {
            for server in &servers_arr.content {
                if let Element::Object(server_obj) = server {
                    if let Some(Element::String(url_str)) = server_obj.get("url") {
                        urls.push(url_str.content.clone());
                    }
                }
            }
        }
    }
    
    urls
}

/// 从路径项中提取路径模板的辅助函数
/// 
/// # Arguments
/// * `paths_element` - OpenAPI paths元素
/// 
/// # Returns
/// 路径模板列表
pub fn extract_path_templates(paths_element: &Element) -> Vec<String> {
    let mut paths = Vec::new();
    
    if let Element::Object(paths_obj) = paths_element {
        for member in &paths_obj.content {
            if let Element::String(path_str) = member.key.as_ref() {
                paths.push(path_str.content.clone());
            }
        }
    }
    
    paths
}

/// 从操作参数中提取参数信息的辅助函数
/// 
/// # Arguments
/// * `operation_element` - OpenAPI操作元素
/// 
/// # Returns
/// (路径参数列表, 查询参数列表)
pub fn extract_operation_parameters(operation_element: &Element) -> (Vec<String>, Vec<String>) {
    let mut path_params = Vec::new();
    let mut query_params = Vec::new();
    
    if let Element::Object(operation_obj) = operation_element {
        if let Some(Element::Array(parameters_arr)) = operation_obj.get("parameters") {
            for param in &parameters_arr.content {
                if let Element::Object(param_obj) = param {
                    if let (Some(Element::String(name)), Some(Element::String(in_type))) = 
                        (param_obj.get("name"), param_obj.get("in")) {
                        match in_type.content.as_str() {
                            "path" => path_params.push(name.content.clone()),
                            "query" => query_params.push(name.content.clone()),
                            _ => {}
                        }
                    }
                }
            }
        }
    }
    
    (path_params, query_params)
}

/// URL模板处理工具
pub struct UrlTemplate {
    template: String,
}

impl UrlTemplate {
    /// 创建新的URL模板
    pub fn new(template: &str) -> Self {
        Self {
            template: template.to_string(),
        }
    }

    /// 提取模板中的所有参数名
    /// 
    /// # Returns
    /// 参数名列表
    /// 
    /// # Example
    /// ```
    /// use apidom_ns_openapi_3_0::url_builder::UrlTemplate;
    /// 
    /// let template = UrlTemplate::new("/users/{userId}/posts/{postId}");
    /// let params = template.extract_parameters();
    /// assert_eq!(params, vec!["userId", "postId"]);
    /// ```
    pub fn extract_parameters(&self) -> Vec<String> {
        let mut params = Vec::new();
        let mut chars = self.template.chars().peekable();
        
        while let Some(ch) = chars.next() {
            if ch == '{' {
                let mut param_name = String::new();
                while let Some(&next_ch) = chars.peek() {
                    if next_ch == '}' {
                        chars.next(); // 消费 '}'
                        break;
                    }
                    param_name.push(chars.next().unwrap());
                }
                if !param_name.is_empty() {
                    params.push(param_name);
                }
            }
        }
        
        params
    }

    /// 验证所有必需的参数是否都已提供
    /// 
    /// # Arguments
    /// * `provided_params` - 提供的参数映射
    /// 
    /// # Returns
    /// (是否所有参数都已提供, 缺失的参数列表)
    pub fn validate_parameters(&self, provided_params: &HashMap<String, String>) -> (bool, Vec<String>) {
        let required_params = self.extract_parameters();
        let mut missing_params = Vec::new();
        
        for param in &required_params {
            if !provided_params.contains_key(param) {
                missing_params.push(param.clone());
            }
        }
        
        (missing_params.is_empty(), missing_params)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use apidom_ast::minim_model::{ObjectElement, ArrayElement, StringElement};

    #[test]
    fn test_url_builder_basic() {
        let mut builder = UrlBuilder::new("https://api.example.com");
        let url = builder.path("/users").build();
        assert_eq!(url, "https://api.example.com/users");
    }

    #[test]
    fn test_url_builder_with_path_params() {
        let mut builder = UrlBuilder::new("https://api.example.com/v1");
        let url = builder
            .path("/users/{userId}/posts/{postId}")
            .path_param("userId", "123")
            .path_param("postId", "456")
            .build();
        
        assert_eq!(url, "https://api.example.com/v1/users/123/posts/456");
    }

    #[test]
    fn test_url_builder_with_query_params() {
        let mut builder = UrlBuilder::new("https://api.example.com");
        let url = builder
            .path("/users")
            .query_param("page", "1")
            .query_param("limit", "10")
            .query_param("sort", "name")
            .build();
        
        assert!(url.starts_with("https://api.example.com/users?"));
        assert!(url.contains("page=1"));
        assert!(url.contains("limit=10"));
        assert!(url.contains("sort=name"));
    }

    #[test]
    fn test_url_builder_mixed_params() {
        let mut builder = UrlBuilder::new("https://api.example.com");
        let url = builder
            .path("/users/{userId}")
            .path_param("userId", "123")
            .query_param("include", "profile")
            .build();
        
        assert_eq!(url, "https://api.example.com/users/123?include=profile");
    }

    #[test]
    fn test_url_builder_base_url_normalization() {
        let mut builder1 = UrlBuilder::new("https://api.example.com/");
        let mut builder2 = UrlBuilder::new("https://api.example.com");
        
        let url1 = builder1.path("/users").build();
        let url2 = builder2.path("/users").build();
        
        assert_eq!(url1, url2);
        assert_eq!(url1, "https://api.example.com/users");
    }

    #[test]
    fn test_url_builder_reset() {
        let mut builder = UrlBuilder::new("https://api.example.com");
        builder
            .path("/users/{userId}")
            .path_param("userId", "123")
            .query_param("page", "1");
        
        builder.reset();
        let url = builder.path("/posts").build();
        
        assert_eq!(url, "https://api.example.com/posts");
    }

    #[test]
    fn test_url_builder_clone_base() {
        let original = UrlBuilder::new("https://api.example.com");
        let mut cloned = original.clone_base();
        
        let url = cloned.path("/users").build();
        assert_eq!(url, "https://api.example.com/users");
    }

    #[test]
    fn test_url_template_extract_parameters() {
        let template = UrlTemplate::new("/users/{userId}/posts/{postId}");
        let params = template.extract_parameters();
        
        assert_eq!(params.len(), 2);
        assert!(params.contains(&"userId".to_string()));
        assert!(params.contains(&"postId".to_string()));
    }

    #[test]
    fn test_url_template_validate_parameters() {
        let template = UrlTemplate::new("/users/{userId}/posts/{postId}");
        let mut provided_params = HashMap::new();
        provided_params.insert("userId".to_string(), "123".to_string());
        
        let (is_valid, missing) = template.validate_parameters(&provided_params);
        assert!(!is_valid);
        assert_eq!(missing, vec!["postId"]);
        
        provided_params.insert("postId".to_string(), "456".to_string());
        let (is_valid, missing) = template.validate_parameters(&provided_params);
        assert!(is_valid);
        assert!(missing.is_empty());
    }

    #[test]
    fn test_url_encoding() {
        let mut builder = UrlBuilder::new("https://api.example.com");
        let url = builder
            .path("/search")
            .query_param("q", "hello world")
            .query_param("category", "tech & science")
            .build();
        
        assert!(url.contains("q=hello%20world"));
        assert!(url.contains("category=tech%20%26%20science"));
    }

    #[test]
    fn test_extract_server_urls() {
        // 创建一个模拟的OpenAPI文档元素
        let mut openapi = ObjectElement::new();
        let mut servers = ArrayElement::new_empty();
        
        let mut server1 = ObjectElement::new();
        server1.set("url", Element::String(StringElement::new("https://api.example.com/v1")));
        servers.content.push(Element::Object(server1));
        
        let mut server2 = ObjectElement::new();
        server2.set("url", Element::String(StringElement::new("https://staging-api.example.com/v1")));
        servers.content.push(Element::Object(server2));
        
        openapi.set("servers", Element::Array(servers));
        
        let urls = extract_server_urls(&Element::Object(openapi));
        
        assert_eq!(urls.len(), 2);
        assert!(urls.contains(&"https://api.example.com/v1".to_string()));
        assert!(urls.contains(&"https://staging-api.example.com/v1".to_string()));
    }

    #[test]
    fn test_from_server_element() {
        let mut server = ObjectElement::new();
        server.set("url", Element::String(StringElement::new("https://api.example.com/v1")));
        server.set("description", Element::String(StringElement::new("Production server")));
        
        let builder = UrlBuilder::from_server_element(&Element::Object(server));
        
        assert!(builder.is_some());
        let builder = builder.unwrap();
        assert_eq!(builder.base_url, "https://api.example.com/v1");
    }

    #[test]
    fn test_realistic_openapi_scenario() {
        // 模拟真实的OpenAPI使用场景
        let mut builder = UrlBuilder::new("https://petstore.swagger.io/v2");
        
        // GET /pet/{petId}
        let url = builder
            .path("/pet/{petId}")
            .path_param("petId", "123")
            .build();
        
        assert_eq!(url, "https://petstore.swagger.io/v2/pet/123");
        
        // 重置并构建另一个URL
        builder.reset();
        let url = builder
            .path("/pet/findByStatus")
            .query_param("status", "available")
            .query_param("status", "pending")
            .build();
        
        assert!(url.starts_with("https://petstore.swagger.io/v2/pet/findByStatus?"));
        assert!(url.contains("status="));
    }
} 