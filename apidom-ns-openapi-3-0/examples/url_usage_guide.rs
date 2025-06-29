/// # OpenAPI URL 构建使用指南
/// 
/// 这个示例展示了如何使用 `url_builder` 模块基于 OpenAPI 文档构建 URL。
/// 
/// ## 主要功能
/// 1. **UrlBuilder** - 核心 URL 构建器
/// 2. **UrlTemplate** - URL 模板处理工具
/// 3. **辅助函数** - 从 OpenAPI 文档提取信息的工具函数

use std::collections::HashMap;
use apidom_ast::minim_model::{Element, ObjectElement, ArrayElement, StringElement};
use apidom_ns_openapi_3_0::url_builder::{UrlBuilder, UrlTemplate, extract_server_urls};

fn main() {
    println!("=== OpenAPI URL 构建使用指南 ===\n");
    
    // 1. 基本使用方法
    basic_usage_examples();
    
    // 2. 从 OpenAPI 文档构建 URL
    openapi_document_examples();
    
    // 3. URL 模板处理
    url_template_examples();
    
    // 4. 实际应用场景
    real_world_examples();
}

fn basic_usage_examples() {
    println!("## 1. 基本使用方法\n");
    
    println!("### 创建 URL 构建器");
    let mut builder = UrlBuilder::new("https://api.example.com/v1");
    println!("```rust");
    println!("let mut builder = UrlBuilder::new(\"https://api.example.com/v1\");");
    println!("```\n");
    
    println!("### 构建简单 URL");
    let simple_url = builder.path("/users").build();
    println!("```rust");
    println!("let url = builder.path(\"/users\").build();");
    println!("// 结果: {}", simple_url);
    println!("```\n");
    
    println!("### 使用路径参数");
    builder.reset();
    let path_param_url = builder
        .path("/users/{userId}")
        .path_param("userId", "123")
        .build();
    println!("```rust");
    println!("let url = builder");
    println!("    .path(\"/users/{{userId}}\")");
    println!("    .path_param(\"userId\", \"123\")");
    println!("    .build();");
    println!("// 结果: {}", path_param_url);
    println!("```\n");
    
    println!("### 使用查询参数");
    builder.reset();
    let query_param_url = builder
        .path("/users")
        .query_param("page", "1")
        .query_param("limit", "10")
        .build();
    println!("```rust");
    println!("let url = builder");
    println!("    .path(\"/users\")");
    println!("    .query_param(\"page\", \"1\")");
    println!("    .query_param(\"limit\", \"10\")");
    println!("    .build();");
    println!("// 结果: {}", query_param_url);
    println!("```\n");
    
    println!("### 批量设置参数");
    builder.reset();
    let mut path_params = HashMap::new();
    path_params.insert("userId".to_string(), "123".to_string());
    path_params.insert("postId".to_string(), "456".to_string());
    
    let mut query_params = HashMap::new();
    query_params.insert("include".to_string(), "comments".to_string());
    query_params.insert("format".to_string(), "json".to_string());
    
    let batch_url = builder
        .path("/users/{userId}/posts/{postId}")
        .path_params(path_params)
        .query_params(query_params)
        .build();
    println!("```rust");
    println!("let mut path_params = HashMap::new();");
    println!("path_params.insert(\"userId\".to_string(), \"123\".to_string());");
    println!("path_params.insert(\"postId\".to_string(), \"456\".to_string());");
    println!("");
    println!("let url = builder");
    println!("    .path(\"/users/{{userId}}/posts/{{postId}}\")");
    println!("    .path_params(path_params)");
    println!("    .query_params(query_params)");
    println!("    .build();");
    println!("// 结果: {}", batch_url);
    println!("```\n");
}

fn openapi_document_examples() {
    println!("## 2. 从 OpenAPI 文档构建 URL\n");
    
    // 创建模拟的 OpenAPI 文档
    let openapi_doc = create_sample_openapi();
    
    println!("### 提取服务器 URL");
    let server_urls = extract_server_urls(&openapi_doc);
    println!("```rust");
    println!("let server_urls = extract_server_urls(&openapi_doc);");
    println!("// 发现的服务器:");
    for (i, url) in server_urls.iter().enumerate() {
        println!("// {}. {}", i + 1, url);
    }
    println!("```\n");
    
    println!("### 使用服务器 URL 构建请求");
    if let Some(base_url) = server_urls.first() {
        let mut api_builder = UrlBuilder::new(base_url);
        
        // 获取用户信息
        let get_user_url = api_builder
            .path("/users/{userId}")
            .path_param("userId", "123")
            .build();
        
        println!("```rust");
        println!("let mut api_builder = UrlBuilder::new(\"{}\");", base_url);
        println!("let get_user_url = api_builder");
        println!("    .path(\"/users/{{userId}}\")");
        println!("    .path_param(\"userId\", \"123\")");
        println!("    .build();");
        println!("// GET 用户信息: {}", get_user_url);
        println!("```\n");
        
        // 搜索用户
        api_builder.reset();
        let search_users_url = api_builder
            .path("/users")
            .query_param("q", "john")
            .query_param("limit", "20")
            .build();
        
        println!("```rust");
        println!("let search_users_url = api_builder");
        println!("    .path(\"/users\")");
        println!("    .query_param(\"q\", \"john\")");
        println!("    .query_param(\"limit\", \"20\")");
        println!("    .build();");
        println!("// 搜索用户: {}", search_users_url);
        println!("```\n");
    }
}

fn url_template_examples() {
    println!("## 3. URL 模板处理\n");
    
    println!("### 创建和分析模板");
    let template = UrlTemplate::new("/api/v1/users/{userId}/posts/{postId}/comments/{commentId}");
    let parameters = template.extract_parameters();
    
    println!("```rust");
    println!("let template = UrlTemplate::new(\"/api/v1/users/{{userId}}/posts/{{postId}}/comments/{{commentId}}\");");
    println!("let parameters = template.extract_parameters();");
    println!("// 提取的参数: {:?}", parameters);
    println!("```\n");
    
    println!("### 验证模板参数");
    let mut provided_params = HashMap::new();
    provided_params.insert("userId".to_string(), "123".to_string());
    provided_params.insert("postId".to_string(), "456".to_string());
    
    let (is_valid, missing) = template.validate_parameters(&provided_params);
    println!("```rust");
    println!("let mut provided_params = HashMap::new();");
    println!("provided_params.insert(\"userId\".to_string(), \"123\".to_string());");
    println!("provided_params.insert(\"postId\".to_string(), \"456\".to_string());");
    println!("");
    println!("let (is_valid, missing) = template.validate_parameters(&provided_params);");
    println!("// 验证结果: 有效={}, 缺失参数={:?}", is_valid, missing);
    println!("```\n");
    
    // 补充缺失参数
    provided_params.insert("commentId".to_string(), "789".to_string());
    let (is_valid_after, missing_after) = template.validate_parameters(&provided_params);
    println!("```rust");
    println!("// 补充缺失参数后");
    println!("provided_params.insert(\"commentId\".to_string(), \"789\".to_string());");
    println!("let (is_valid, missing) = template.validate_parameters(&provided_params);");
    println!("// 验证结果: 有效={}, 缺失参数={:?}", is_valid_after, missing_after);
    println!("```\n");
}

fn real_world_examples() {
    println!("## 4. 实际应用场景\n");
    
    println!("### RESTful API 客户端");
    let mut rest_client = UrlBuilder::new("https://jsonplaceholder.typicode.com");
    
    // CRUD 操作示例
    let create_post_url = rest_client.path("/posts").build();
    println!("```rust");
    println!("// CREATE - 创建新文章");
    println!("let create_url = rest_client.path(\"/posts\").build();");
    println!("// POST {}", create_post_url);
    println!("```\n");
    
    rest_client.reset();
    let read_post_url = rest_client
        .path("/posts/{id}")
        .path_param("id", "1")
        .build();
    println!("```rust");
    println!("// READ - 读取文章");
    println!("let read_url = rest_client");
    println!("    .path(\"/posts/{{id}}\")");
    println!("    .path_param(\"id\", \"1\")");
    println!("    .build();");
    println!("// GET {}", read_post_url);
    println!("```\n");
    
    rest_client.reset();
    let update_post_url = rest_client
        .path("/posts/{id}")
        .path_param("id", "1")
        .build();
    println!("```rust");
    println!("// UPDATE - 更新文章");
    println!("let update_url = rest_client");
    println!("    .path(\"/posts/{{id}}\")");
    println!("    .path_param(\"id\", \"1\")");
    println!("    .build();");
    println!("// PUT {}", update_post_url);
    println!("```\n");
    
    rest_client.reset();
    let delete_post_url = rest_client
        .path("/posts/{id}")
        .path_param("id", "1")
        .build();
    println!("```rust");
    println!("// DELETE - 删除文章");
    println!("let delete_url = rest_client");
    println!("    .path(\"/posts/{{id}}\")");
    println!("    .path_param(\"id\", \"1\")");
    println!("    .build();");
    println!("// DELETE {}", delete_post_url);
    println!("```\n");
    
    println!("### 分页和过滤");
    rest_client.reset();
    let paginated_url = rest_client
        .path("/posts")
        .query_param("_page", "2")
        .query_param("_limit", "10")
        .query_param("userId", "1")
        .build();
    println!("```rust");
    println!("// 分页查询用户文章");
    println!("let paginated_url = rest_client");
    println!("    .path(\"/posts\")");
    println!("    .query_param(\"_page\", \"2\")");
    println!("    .query_param(\"_limit\", \"10\")");
    println!("    .query_param(\"userId\", \"1\")");
    println!("    .build();");
    println!("// GET {}", paginated_url);
    println!("```\n");
    
    println!("### 多环境支持");
    let environments = vec![
        ("开发环境", "http://localhost:3000/api/v1"),
        ("测试环境", "https://staging-api.example.com/v1"),
        ("生产环境", "https://api.example.com/v1"),
    ];
    
    println!("```rust");
    println!("let environments = vec![");
    println!("    (\"开发环境\", \"http://localhost:3000/api/v1\"),");
    println!("    (\"测试环境\", \"https://staging-api.example.com/v1\"),");
    println!("    (\"生产环境\", \"https://api.example.com/v1\"),");
    println!("];");
    println!("");
    for (env_name, base_url) in &environments {
        let mut env_builder = UrlBuilder::new(base_url);
        let env_url = env_builder
            .path("/users/{userId}")
            .path_param("userId", "123")
            .build();
        println!("// {}: {}", env_name, env_url);
    }
    println!("```\n");
    
    println!("### URL 编码处理");
    let mut encoding_builder = UrlBuilder::new("https://api.example.com");
    let encoded_url = encoding_builder
        .path("/search")
        .query_param("q", "hello world & special chars")
        .query_param("category", "tech/programming")
        .build();
    println!("```rust");
    println!("// 自动处理 URL 编码");
    println!("let encoded_url = encoding_builder");
    println!("    .path(\"/search\")");
    println!("    .query_param(\"q\", \"hello world & special chars\")");
    println!("    .query_param(\"category\", \"tech/programming\")");
    println!("    .build();");
    println!("// 结果: {}", encoded_url);
    println!("```\n");
    
    println!("## 总结\n");
    println!("URL 构建器提供了以下主要优势:");
    println!("- 🔧 **类型安全**: 编译时检查参数类型");
    println!("- 🎯 **模板支持**: 支持路径参数模板");
    println!("- 🔍 **参数验证**: 验证必需参数是否提供");
    println!("- 🌐 **自动编码**: 自动处理 URL 编码");
    println!("- 🔄 **可重用**: 支持重置和克隆构建器");
    println!("- 📋 **批量操作**: 支持批量设置参数");
    println!("- 🏗️ **OpenAPI 集成**: 与 OpenAPI 文档无缝集成");
}

fn create_sample_openapi() -> Element {
    let mut openapi = ObjectElement::new();
    
    // 基本信息
    openapi.set("openapi", Element::String(StringElement::new("3.0.0")));
    
    // 服务器信息
    let mut servers = ArrayElement::new_empty();
    
    let mut prod_server = ObjectElement::new();
    prod_server.set("url", Element::String(StringElement::new("https://api.example.com/v1")));
    prod_server.set("description", Element::String(StringElement::new("生产环境")));
    servers.content.push(Element::Object(prod_server));
    
    let mut staging_server = ObjectElement::new();
    staging_server.set("url", Element::String(StringElement::new("https://staging-api.example.com/v1")));
    staging_server.set("description", Element::String(StringElement::new("测试环境")));
    servers.content.push(Element::Object(staging_server));
    
    let mut dev_server = ObjectElement::new();
    dev_server.set("url", Element::String(StringElement::new("http://localhost:3000/v1")));
    dev_server.set("description", Element::String(StringElement::new("开发环境")));
    servers.content.push(Element::Object(dev_server));
    
    openapi.set("servers", Element::Array(servers));
    
    Element::Object(openapi)
} 