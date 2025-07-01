use apidom_ast::{FoldFromCst, JsonFolder, json_source_to_ast, json_cst_to_ast, Fold};
use apidom_ast::*;
use apidom_cst::CstParser;

fn main() {
    println!("🌳 CST to AST 转换演示");
    println!("====================");

    // 演示 1: 基础 JSON 对象转换
    println!("\n📦 演示 1: JSON 对象转换");
    let json_object = r#"{"name": "Alice", "age": 30, "active": true}"#;
    
    // 先解析为 CST
    let cst = CstParser::parse(json_object);
    println!("CST 根节点类型: {}", cst.kind);
    println!("CST 子节点数量: {}", cst.children.len());
    
    // 转换为 AST
    let mut folder = JsonFolder::new();
    let ast = folder.fold_from_cst(&cst);
    
    match &ast {
        Element::Object(obj) => {
            println!("✅ 成功转换为 ObjectElement");
            println!("   成员数量: {}", obj.content.len());
            for member in &obj.content {
                if let Element::String(key) = member.key.as_ref() {
                    println!("   - 键: '{}'", key.content);
                }
            }
        }
        _ => println!("❌ 转换失败"),
    }

    // 演示 2: JSON 数组转换
    println!("\n🔢 演示 2: JSON 数组转换");
    let json_array = r#"[1, "hello", true, null, {"nested": "object"}]"#;
    let ast_array = json_source_to_ast(json_array);
    
    match &ast_array {
        Element::Array(arr) => {
            println!("✅ 成功转换为 ArrayElement");
            println!("   元素数量: {}", arr.content.len());
            for (i, element) in arr.content.iter().enumerate() {
                let type_name = match element {
                    Element::Number(_) => "Number",
                    Element::String(_) => "String", 
                    Element::Boolean(_) => "Boolean",
                    Element::Null(_) => "Null",
                    Element::Object(_) => "Object",
                    _ => "Other",
                };
                println!("   [{}]: {}", i, type_name);
            }
        }
        _ => println!("❌ 转换失败"),
    }

    // 演示 3: 字符串转义处理
    println!("\n🔤 演示 3: 字符串转义处理");
    let escaped_string = r#""Hello\nWorld\t\"Quote\"""#;
    let cst_string = CstParser::parse(escaped_string);
    let ast_string = json_cst_to_ast(&cst_string);
    
    match &ast_string {
        Element::String(s) => {
            println!("✅ 原始: {}", escaped_string);
            println!("   转换: '{}'", s.content);
            println!("   长度: {} 字符", s.content.len());
        }
        _ => println!("❌ 转换失败"),
    }

    // 演示 4: 数字类型转换
    println!("\n🔢 演示 4: 数字类型转换");
    let numbers = vec!["42", "-17", "3.14159", "1.23e-4", "0"];
    
    for num_str in numbers {
        let ast_num = json_source_to_ast(num_str);
        match &ast_num {
            Element::Number(n) => {
                println!("   '{}' → {}", num_str, n.content);
            }
            _ => println!("   '{}' → 转换失败", num_str),
        }
    }

    // 演示 5: 源码位置信息
    println!("\n📍 演示 5: 源码位置信息");
    let source_with_location = r#"{"key": "value"}"#;
    let cst_with_location = CstParser::parse(source_with_location);
    let mut folder_with_location = JsonFolder::with_options(true, false);
    let ast_with_location = folder_with_location.fold_from_cst(&cst_with_location);
    
    match &ast_with_location {
        Element::Object(obj) => {
            if let Some(location) = obj.meta.properties.get("sourceLocation") {
                println!("✅ 包含源码位置信息:");
                println!("   {}", serde_json::to_string_pretty(location).unwrap());
            } else {
                println!("❌ 未找到源码位置信息");
            }
        }
        _ => println!("❌ 转换失败"),
    }

    // 演示 6: 嵌套结构转换
    println!("\n🏗️ 演示 6: 嵌套结构转换");
    let nested_json = r#"{
        "user": {
            "name": "Bob",
            "preferences": {
                "theme": "dark",
                "language": "en"
            },
            "scores": [95, 87, 92]
        },
        "metadata": {
            "version": "1.0",
            "created": "2024-01-01"
        }
    }"#;
    
    let nested_ast = json_source_to_ast(nested_json);
    print_ast_structure(&nested_ast, 0);

    // 演示 7: 错误处理
    println!("\n⚠️ 演示 7: 错误处理");
    let malformed_json = r#"{"key": "value", "incomplete": }"#;
    let cst_with_error = CstParser::parse(malformed_json);
    
    println!("CST 是否有错误: {}", cst_with_error.has_error());
    
    let ast_with_error = json_cst_to_ast(&cst_with_error);
    match &ast_with_error {
        Element::Object(obj) => {
            if let Some(has_error) = obj.meta.properties.get("hasError") {
                println!("AST 错误标记: {:?}", has_error);
            }
            println!("成功解析的成员数量: {}", obj.content.len());
        }
        _ => println!("转换失败"),
    }

    // 演示 8: 与 Fold 机制结合
    println!("\n🔄 演示 8: 与 Fold 机制结合");
    let source_for_fold = r#"{"name": "  JOHN DOE  ", "age": "25", "active": "true"}"#;
    let mut cst_folder = JsonFolder::new();
    let mut ast_from_cst = cst_folder.fold_from_cst(&CstParser::parse(source_for_fold));
    
    // 应用字符串规范化
    use apidom_ast::folders::StringNormalizer;
    let mut normalizer = StringNormalizer;
    ast_from_cst = normalizer.fold_element(ast_from_cst);
    
    // 应用类型转换
    use apidom_ast::folders::TypeConverter;
    let mut converter = TypeConverter::new("number".to_string());
    ast_from_cst = converter.fold_element(ast_from_cst);
    
    match &ast_from_cst {
        Element::Object(obj) => {
            println!("✅ 经过 fold 处理后:");
            for member in &obj.content {
                if let (Element::String(key), value) = (member.key.as_ref(), member.value.as_ref()) {
                    match value {
                        Element::String(s) => println!("   {}: '{}' (String)", key.content, s.content),
                        Element::Number(n) => println!("   {}: {} (Number)", key.content, n.content),
                        Element::Boolean(b) => println!("   {}: {} (Boolean)", key.content, b.content),
                        _ => println!("   {}: Other", key.content),
                    }
                }
            }
        }
        _ => println!("❌ 处理失败"),
    }

    println!("\n✅ 演示完成!");
}

fn print_ast_structure(element: &Element, depth: usize) {
    let indent = "  ".repeat(depth);
    
    match element {
        Element::Object(obj) => {
            println!("{}Object ({} members):", indent, obj.content.len());
            for member in &obj.content {
                if let Element::String(key) = member.key.as_ref() {
                    println!("{}  '{}': ", indent, key.content);
                    print_ast_structure(member.value.as_ref(), depth + 2);
                }
            }
        }
        Element::Array(arr) => {
            println!("{}Array ({} elements):", indent, arr.content.len());
            for (i, element) in arr.content.iter().enumerate() {
                println!("{}  [{}]: ", indent, i);
                print_ast_structure(element, depth + 2);
            }
        }
        Element::String(s) => {
            println!("{}String: '{}'", indent, s.content);
        }
        Element::Number(n) => {
            println!("{}Number: {}", indent, n.content);
        }
        Element::Boolean(b) => {
            println!("{}Boolean: {}", indent, b.content);
        }
        Element::Null(_) => {
            println!("{}Null", indent);
        }
        _ => {
            println!("{}Other", indent);
        }
    }
} 