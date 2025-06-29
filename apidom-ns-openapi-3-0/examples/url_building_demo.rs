use std::collections::HashMap;
use apidom_ast::minim_model::{Element, ObjectElement, ArrayElement, StringElement};
use apidom_ns_openapi_3_0::url_builder::{UrlBuilder, UrlTemplate, extract_server_urls, extract_path_templates};

fn main() {
    println!("=== OpenAPI URL 构建演示 ===\n");
    
    // 1. 基本URL构建
    demo_basic_url_building();
    
    // 2. 从OpenAPI文档提取信息并构建URL
    demo_openapi_url_building();
    
    // 3. URL模板处理
    demo_url_template_processing();
    
    // 4. 复杂场景演示
    demo_complex_scenarios();
}

fn demo_basic_url_building() {
    println!("1. 基本URL构建演示:");
    
    let mut builder = UrlBuilder::new("https://api.example.com/v1");
    
    // 简单路径
    let url1 = builder.path("/users").build();
    println!("   简单路径: {}", url1);
    
    // 带路径参数
    builder.reset();
    let url2 = builder
        .path("/users/{userId}")
        .path_param("userId", "123")
        .build();
    println!("   路径参数: {}", url2);
    
    // 带查询参数
    builder.reset();
    let url3 = builder
        .path("/users")
        .query_param("page", "1")
        .query_param("limit", "10")
        .query_param("sort", "name")
        .build();
    println!("   查询参数: {}", url3);
    
    // 复合参数
    builder.reset();
    let url4 = builder
        .path("/users/{userId}/posts/{postId}")
        .path_param("userId", "456")
        .path_param("postId", "789")
        .query_param("include", "comments")
        .query_param("format", "json")
        .build();
    println!("   复合参数: {}", url4);
    
    println!();
}

fn demo_openapi_url_building() {
    println!("2. 从OpenAPI文档构建URL:");
    
    // 创建模拟的OpenAPI文档
    let openapi_doc = create_mock_openapi_document();
    
    // 提取服务器URL
    let server_urls = extract_server_urls(&openapi_doc);
    println!("   发现的服务器:");
    for (i, url) in server_urls.iter().enumerate() {
        println!("     {}. {}", i + 1, url);
    }
    
    // 提取路径模板
    if let Element::Object(openapi_obj) = &openapi_doc {
        if let Some(Element::Object(paths_obj)) = openapi_obj.get("paths") {
            let path_templates = extract_path_templates(&Element::Object(paths_obj.clone()));
            println!("   发现的路径模板:");
            for (i, path) in path_templates.iter().enumerate() {
                println!("     {}. {}", i + 1, path);
            }
            
            // 使用第一个服务器和路径构建URL
            if let (Some(base_url), Some(path_template)) = (server_urls.first(), path_templates.first()) {
                let mut builder = UrlBuilder::new(base_url);
                
                if path_template.contains("{petId}") {
                    let url = builder
                        .path(path_template)
                        .path_param("petId", "123")
                        .build();
                    println!("   构建的URL示例: {}", url);
                }
            }
        }
    }
    
    println!();
}

fn demo_url_template_processing() {
    println!("3. URL模板处理演示:");
    
    let template = UrlTemplate::new("/users/{userId}/posts/{postId}/comments/{commentId}");
    
    // 提取参数
    let params = template.extract_parameters();
    println!("   模板: /users/{{userId}}/posts/{{postId}}/comments/{{commentId}}");
    println!("   提取的参数: {:?}", params);
    
    // 验证参数
    let mut provided_params = HashMap::new();
    provided_params.insert("userId".to_string(), "123".to_string());
    provided_params.insert("postId".to_string(), "456".to_string());
    
    let (is_valid, missing) = template.validate_parameters(&provided_params);
    println!("   参数验证: 有效={}, 缺失={:?}", is_valid, missing);
    
    // 补充缺失参数
    provided_params.insert("commentId".to_string(), "789".to_string());
    let (is_valid, missing) = template.validate_parameters(&provided_params);
    println!("   补充后验证: 有效={}, 缺失={:?}", is_valid, missing);
    
    println!();
}

fn demo_complex_scenarios() {
    println!("4. 复杂场景演示:");
    
    // 场景1: 电商API
    println!("   场景1 - 电商API:");
    let mut ecommerce_builder = UrlBuilder::new("https://api.shop.com/v2");
    
    let product_url = ecommerce_builder
        .path("/categories/{categoryId}/products/{productId}")
        .path_param("categoryId", "electronics")
        .path_param("productId", "laptop-123")
        .query_param("include", "reviews,specs")
        .query_param("currency", "USD")
        .build();
    println!("     产品详情: {}", product_url);
    
    ecommerce_builder.reset();
    let search_url = ecommerce_builder
        .path("/search")
        .query_param("q", "gaming laptop")
        .query_param("category", "electronics")
        .query_param("price_min", "500")
        .query_param("price_max", "2000")
        .query_param("sort", "price_asc")
        .query_param("page", "1")
        .build();
    println!("     商品搜索: {}", search_url);
    
    // 场景2: 社交媒体API
    println!("   场景2 - 社交媒体API:");
    let mut social_builder = UrlBuilder::new("https://api.social.com/v1");
    
    let user_posts_url = social_builder
        .path("/users/{userId}/posts")
        .path_param("userId", "john_doe")
        .query_param("limit", "20")
        .query_param("since", "2024-01-01")
        .query_param("include_replies", "false")
        .build();
    println!("     用户动态: {}", user_posts_url);
    
    social_builder.reset();
    let timeline_url = social_builder
        .path("/timeline")
        .query_param("feed_type", "home")
        .query_param("count", "50")
        .query_param("max_id", "12345")
        .build();
    println!("     时间线: {}", timeline_url);
    
    // 场景3: 地理位置API
    println!("   场景3 - 地理位置API:");
    let mut geo_builder = UrlBuilder::new("https://api.maps.com/v3");
    
    let geocoding_url = geo_builder
        .path("/geocode")
        .query_param("address", "1600 Amphitheatre Parkway, Mountain View, CA")
        .query_param("key", "API_KEY")
        .query_param("language", "zh-CN")
        .build();
    println!("     地理编码: {}", geocoding_url);
    
    geo_builder.reset();
    let directions_url = geo_builder
        .path("/directions")
        .query_param("origin", "37.7749,-122.4194")
        .query_param("destination", "37.3382,-121.8863")
        .query_param("mode", "driving")
        .query_param("avoid", "tolls,highways")
        .build();
    println!("     路线规划: {}", directions_url);
    
    println!();
}

fn create_mock_openapi_document() -> Element {
    let mut openapi = ObjectElement::new();
    
    // 添加版本信息
    openapi.set("openapi", Element::String(StringElement::new("3.0.0")));
    
    // 添加服务器信息
    let mut servers = ArrayElement::new_empty();
    
    let mut prod_server = ObjectElement::new();
    prod_server.set("url", Element::String(StringElement::new("https://petstore.swagger.io/v2")));
    prod_server.set("description", Element::String(StringElement::new("Production server")));
    servers.content.push(Element::Object(prod_server));
    
    let mut staging_server = ObjectElement::new();
    staging_server.set("url", Element::String(StringElement::new("https://staging-petstore.swagger.io/v2")));
    staging_server.set("description", Element::String(StringElement::new("Staging server")));
    servers.content.push(Element::Object(staging_server));
    
    openapi.set("servers", Element::Array(servers));
    
    // 添加路径信息
    let mut paths = ObjectElement::new();
    
    // /pet/{petId} 路径
    let mut pet_path = ObjectElement::new();
    let mut get_operation = ObjectElement::new();
    get_operation.set("summary", Element::String(StringElement::new("Find pet by ID")));
    
    let mut parameters = ArrayElement::new_empty();
    let mut pet_id_param = ObjectElement::new();
    pet_id_param.set("name", Element::String(StringElement::new("petId")));
    pet_id_param.set("in", Element::String(StringElement::new("path")));
    pet_id_param.set("required", Element::Boolean(apidom_ast::minim_model::BooleanElement::new(true)));
    parameters.content.push(Element::Object(pet_id_param));
    
    get_operation.set("parameters", Element::Array(parameters));
    pet_path.set("get", Element::Object(get_operation));
    
    paths.set("/pet/{petId}", Element::Object(pet_path));
    
    // /pet/findByStatus 路径
    let mut find_by_status_path = ObjectElement::new();
    let mut find_operation = ObjectElement::new();
    find_operation.set("summary", Element::String(StringElement::new("Finds Pets by status")));
    
    let mut status_parameters = ArrayElement::new_empty();
    let mut status_param = ObjectElement::new();
    status_param.set("name", Element::String(StringElement::new("status")));
    status_param.set("in", Element::String(StringElement::new("query")));
    status_param.set("required", Element::Boolean(apidom_ast::minim_model::BooleanElement::new(true)));
    status_parameters.content.push(Element::Object(status_param));
    
    find_operation.set("parameters", Element::Array(status_parameters));
    find_by_status_path.set("get", Element::Object(find_operation));
    
    paths.set("/pet/findByStatus", Element::Object(find_by_status_path));
    
    openapi.set("paths", Element::Object(paths));
    
    Element::Object(openapi)
} 