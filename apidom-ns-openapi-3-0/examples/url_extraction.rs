use apidom_ast::minim_model::{Element, ObjectElement, ArrayElement, StringElement};
use apidom_ns_openapi_3_0::url_builder::extract_server_urls;

fn main() {
    // 创建一个模拟的OpenAPI文档
    let mut openapi = ObjectElement::new();
    let mut servers = ArrayElement::new_empty();
    
    // 添加生产服务器
    let mut prod_server = ObjectElement::new();
    prod_server.set("url", Element::String(StringElement::new("https://api.example.com/v1")));
    prod_server.set("description", Element::String(StringElement::new("Production server")));
    servers.content.push(Element::Object(prod_server));
    
    // 添加测试服务器
    let mut test_server = ObjectElement::new();
    test_server.set("url", Element::String(StringElement::new("https://staging-api.example.com/v1")));
    test_server.set("description", Element::String(StringElement::new("Staging server")));
    servers.content.push(Element::Object(test_server));
    
    // 添加本地开发服务器
    let mut dev_server = ObjectElement::new();
    dev_server.set("url", Element::String(StringElement::new("http://localhost:3000")));
    dev_server.set("description", Element::String(StringElement::new("Development server")));
    servers.content.push(Element::Object(dev_server));
    
    openapi.set("servers", Element::Array(servers));
    
    // 提取服务器URL
    let urls = extract_server_urls(&Element::Object(openapi));
    
    println!("发现的服务器URL:");
    for (i, url) in urls.iter().enumerate() {
        println!("  {}. {}", i + 1, url);
    }
    
    println!("\n总共找到 {} 个服务器URL", urls.len());
} 