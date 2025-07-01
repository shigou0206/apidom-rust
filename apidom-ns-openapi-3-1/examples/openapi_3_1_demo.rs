use apidom_ns_openapi_3_1::{
    elements::*,
    field_registry::*,
    schema_loader::*,
    builder_dispatch::*,
    field_extractor::*,
    context::BuildContext,
};
use apidom_ast::{StringElement, ObjectElement, Element, MemberElement, NumberElement, BooleanElement, ArrayElement};

fn main() {
    println!("=== ApiDOM OpenAPI 3.1 命名空间演示 ===\n");
    
    // 1. 演示字段注册系统
    demonstrate_field_registry();
    
    // 2. 演示 schema 加载
    demonstrate_schema_loader();
    
    // 3. 演示构建器分发
    demonstrate_builder_dispatch();
    
    // 4. 演示字段提取器
    demonstrate_field_extractor();
    
    // 5. 演示 OpenAPI 3.1 元素
    demonstrate_openapi_elements();
}

fn demonstrate_field_registry() {
    println!("1. 字段注册系统演示");
    println!("-------------------");
    
    // 创建字段处理器映射
    let mut handlers = FieldHandlerMap::<InfoElement>::new();
    
    // 添加固定字段处理器
    handlers.register_fixed("title", |value, target, _| {
        if let Element::String(s) = value {
            target.set_title(s.clone());
            Some(())
        } else {
            None
        }
    });
    
    // 创建测试 Info 元素
    let mut info = InfoElement::new();
    let title_value = Element::String(StringElement::new("My API"));
    
    // 分发处理
    if handlers.dispatch("title", &title_value, &mut info, None) {
        println!("✓ 成功处理 title 字段");
        if let Some(title) = info.title() {
            println!("  标题: {}", title.content);
        }
    }
    
    println!();
}

fn demonstrate_schema_loader() {
    println!("2. Schema 加载器演示");
    println!("------------------");
    
    let loader = SchemaLoader::new();
    
    // 测试 JSON 解析
    let json = r#"
    {
        "type": "object",
        "properties": {
            "title": {"type": "string"},
            "version": {"type": "string"}
        },
        "required": ["title", "version"]
    }
    "#;
    
    match loader.parse_json_to_element(json) {
        Ok(element) => {
            println!("✓ 成功解析 JSON schema");
            if let Element::Object(obj) = element {
                println!("  包含 {} 个字段", obj.content.len());
                for member in &obj.content {
                    if let Element::String(key) = member.key.as_ref() {
                        println!("    - {}", key.content);
                    }
                }
            }
        }
        Err(e) => {
            println!("✗ 解析失败: {:?}", e);
        }
    }
    
    println!();
}

fn demonstrate_builder_dispatch() {
    println!("3. 构建器分发演示");
    println!("---------------");
    
    let dispatch = BuilderDispatch::new();
    
    // 创建测试 info 对象
    let mut info_obj = ObjectElement::new();
    info_obj.content.push(MemberElement::new(
        Element::String(StringElement::new("title")),
        Element::String(StringElement::new("示例 API"))
    ));
    info_obj.content.push(MemberElement::new(
        Element::String(StringElement::new("version")),
        Element::String(StringElement::new("1.0.0"))
    ));
    info_obj.content.push(MemberElement::new(
        Element::String(StringElement::new("description")),
        Element::String(StringElement::new("这是一个示例 API"))
    ));
    
    let info_element = Element::Object(info_obj);
    
    // 构建 InfoElement
    if let Ok(info) = dispatch.build_info(&info_element, &BuildContext::default()) {
        println!("✓ 成功构建 Info 元素");
        if let Some(title) = info.title() {
            println!("  标题: {}", title.content);
        }
        if let Some(version) = info.version() {
            println!("  版本: {}", version.content);
        }
        if let Some(description) = info.description() {
            println!("  描述: {}", description.content);
        }
    } else {
        println!("✗ 构建 Info 元素失败");
    }
    
    println!();
}

fn demonstrate_field_extractor() {
    println!("4. 字段提取器演示");
    println!("---------------");
    
    // 创建测试对象
    let mut api_spec = ObjectElement::new();
    
    // 添加基本信息
    api_spec.set("title", Element::String(StringElement::new("用户管理 API")));
    api_spec.set("version", Element::String(StringElement::new("2.1.0")));
    api_spec.set("port", Element::Number(NumberElement::new(3000.0)));
    api_spec.set("enabled", Element::Boolean(BooleanElement::new(true)));
    
    // 添加标签数组
    let tags = ArrayElement::from_strings(&["users", "management", "rest"]);
    api_spec.set("tags", Element::Array(tags));
    
    // 添加扩展字段
    api_spec.set("x-api-id", Element::String(StringElement::new("user-mgmt-001")));
    api_spec.set("x-rate-limit", Element::Number(NumberElement::new(1000.0)));
    
    let element = Element::Object(api_spec);
    
    // 演示基本字段提取
    println!("✓ 基本字段提取:");
    if let Some(title) = FieldExtractor::extract_string(&element, "title") {
        println!("  标题: {}", title);
    }
    
    if let Some(version) = FieldExtractor::extract_version(&element, "version") {
        println!("  版本: {}", version);
    }
    
    if let Some(port) = FieldExtractor::extract_integer(&element, "port") {
        println!("  端口: {}", port);
    }
    
    if let Some(enabled) = FieldExtractor::extract_boolean(&element, "enabled") {
        println!("  启用状态: {}", enabled);
    }
    
    // 演示数组提取
    println!("\n✓ 数组字段提取:");
    if let Some(tags) = FieldExtractor::extract_string_array(&element, "tags") {
        println!("  标签: {:?}", tags);
    }
    
    // 演示必填字段验证
    println!("\n✓ 必填字段验证:");
    match FieldExtractor::validate_required_fields(&element, &["title", "version"]) {
        Ok(_) => println!("  ✅ 所有必填字段都存在"),
        Err(missing) => println!("  ❌ 缺少字段: {:?}", missing),
    }
    
    match FieldExtractor::validate_required_fields(&element, &["title", "nonexistent"]) {
        Ok(_) => println!("  ✅ 所有必填字段都存在"),
        Err(missing) => println!("  ❌ 缺少字段: {:?}", missing),
    }
    
    // 演示扩展字段提取
    println!("\n✓ 扩展字段提取:");
    let extensions = FieldExtractor::extract_extension_fields(&element);
    for (key, value) in extensions {
        match value {
            Element::String(s) => println!("  {}: \"{}\"", key, s.content),
            Element::Number(n) => println!("  {}: {}", key, n.content),
            _ => println!("  {}: <其他类型>", key),
        }
    }
    
    // 演示类型检查
    println!("\n✓ 类型检查:");
    println!("  title 是字符串: {}", FieldExtractor::is_field_of_type(&element, "title", "string"));
    println!("  port 是数字: {}", FieldExtractor::is_field_of_type(&element, "port", "number"));
    println!("  enabled 是布尔: {}", FieldExtractor::is_field_of_type(&element, "enabled", "boolean"));
    println!("  tags 是数组: {}", FieldExtractor::is_field_of_type(&element, "tags", "array"));
    
    // 演示默认值提取
    println!("\n✓ 默认值提取:");
    let description = FieldExtractor::extract_with_default(
        &element,
        "description",
        FieldExtractor::extract_string,
        "默认描述".to_string()
    );
    println!("  描述: {}", description);
    
    // 演示批量字段提取
    println!("\n✓ 批量字段提取:");
    let fields = FieldExtractor::extract_string_fields(&element, &["title", "version", "nonexistent"]);
    for (key, value) in fields {
        println!("  {}: {}", key, value);
    }
    
    println!();
}

fn demonstrate_openapi_elements() {
    println!("5. OpenAPI 3.1 元素演示");
    println!("--------------------");
    
    // 创建 Info 元素
    let mut info = InfoElement::new();
    info.set_title(StringElement::new("REST API"));
    info.set_version(StringElement::new("2.0.0"));
    info.set_description(StringElement::new("一个强大的 REST API"));
    
    println!("✓ Info 元素:");
    if let Some(title) = info.title() {
        println!("  标题: {}", title.content);
    }
    if let Some(version) = info.version() {
        println!("  版本: {}", version.content);
    }
    if let Some(description) = info.description() {
        println!("  描述: {}", description.content);
    }
    
    // 创建 Server 元素
    let mut server = ServerElement::new();
    server.set_url(StringElement::new("https://api.example.com/v2"));
    server.set_description(StringElement::new("生产环境服务器"));
    
    println!("\n✓ Server 元素:");
    if let Some(url) = server.url() {
        println!("  URL: {}", url.content);
    }
    if let Some(description) = server.description() {
        println!("  描述: {}", description.content);
    }
    
    // 创建 PathItem 元素
    let mut path_item = PathItemElement::new();
    path_item.set_summary(StringElement::new("用户操作"));
    path_item.set_description(StringElement::new("管理用户的相关操作"));
    
    println!("\n✓ PathItem 元素:");
    if let Some(summary) = path_item.summary() {
        println!("  摘要: {}", summary.content);
    }
    if let Some(description) = path_item.description() {
        println!("  描述: {}", description.content);
    }
    
    println!("\n=== 演示完成 ===");
} 