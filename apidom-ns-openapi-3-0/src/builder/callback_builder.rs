use apidom_ast::*;
use crate::elements::callback::CallbackElement;

/// 构建 OpenAPI CallbackElement（适配 Minim ObjectElement → CallbackElement）
/// 支持运行时表达式检测、$ref 处理、递归折叠等高级功能
/// 
/// 例如：
/// {
///   "{$request.body#/callbackUrl}": {
///     "post": { ... }
///   }
/// }
pub fn build_callback(element: &Element) -> Option<CallbackElement> {
    let object = element.as_object()?;
    let mut callback = CallbackElement::new();

    for member in &object.content {
        if let Element::String(key_str) = &*member.key {
            let key = &key_str.content;
            let value = (*member.value).clone();
            
            // 检测运行时表达式模式 {expression}
            if is_runtime_expression(key) {
                // 为匹配运行时表达式的 PathItem 元素添加元数据
                let mut decorated_value = value.clone();
                if let Element::Object(ref mut path_item_obj) = decorated_value {
                    // 检查是否为 PathItem 类型
                    if path_item_obj.element == "pathItem" || contains_path_item_operations(&value) {
                        // 添加运行时表达式元数据
                        path_item_obj.meta.properties.insert(
                            "runtime-expression".to_string(),
                            SimpleValue::string(key.clone())
                        );
                    }
                }
                callback.set(key, decorated_value);
            } else {
                // 检测 $ref 并添加元数据
                let mut decorated_value = value.clone();
                if let Element::Object(ref mut obj) = decorated_value {
                    if obj.has_key("$ref") {
                        obj.meta.properties.insert(
                            "referenced-element".to_string(),
                            SimpleValue::string("callback".to_string())
                        );
                    }
                }
                callback.set(key, decorated_value);
            }
        }
    }

    Some(callback)
}

/// 构建支持递归折叠的 CallbackElement
/// 这是更高级的版本，支持对每个成员进行递归处理
pub fn build_callback_with_folder(element: &Element, folder: &mut dyn Fold) -> Option<CallbackElement> {
    let object = element.as_object()?;
    let mut callback = CallbackElement::new();

    for member in &object.content {
        if let Element::String(key_str) = &*member.key {
            let key = &key_str.content;
            
            // 对值进行递归折叠处理
            let folded_value = folder.fold_element((*member.value).clone());
            
            // 检测运行时表达式模式
            if is_runtime_expression(key) {
                // 为匹配运行时表达式的元素添加元数据
                let mut decorated_value = folded_value.clone();
                if let Element::Object(ref mut obj) = decorated_value {
                    // 为 PathItem 或类似的操作对象添加运行时表达式元数据
                    if obj.element == "pathItem" || contains_path_item_operations(&folded_value) {
                        obj.meta.properties.insert(
                            "runtime-expression".to_string(),
                            SimpleValue::string(key.clone())
                        );
                    }
                }
                callback.set(key, decorated_value);
            } else {
                // 检测和处理 $ref
                let mut decorated_value = folded_value.clone();
                if let Element::Object(ref mut obj) = decorated_value {
                    if obj.has_key("$ref") {
                        obj.meta.properties.insert(
                            "referenced-element".to_string(),
                            SimpleValue::string("callback".to_string())
                        );
                        
                        // 添加引用路径到元数据
                        if let Some(Element::String(ref_str)) = obj.get("$ref") {
                            obj.meta.properties.insert(
                                "reference-path".to_string(),
                                SimpleValue::string(ref_str.content.clone())
                            );
                        }
                    }
                }
                callback.set(key, decorated_value);
            }
        }
    }

    Some(callback)
}

/// 检测字符串是否为运行时表达式格式 {expression}
/// 支持最大 2083 字符长度的表达式（Chrome URL 最大长度）
fn is_runtime_expression(key: &str) -> bool {
    // 简单的运行时表达式检测：以 { 开头，以 } 结尾，内容长度在 1-2083 之间
    key.starts_with('{') && key.ends_with('}') && key.len() > 2 && key.len() <= 2085
}

/// 检测元素是否包含 PathItem 操作（GET, POST, PUT, DELETE 等）
fn contains_path_item_operations(element: &Element) -> bool {
    if let Element::Object(obj) = element {
        let operations = ["get", "post", "put", "delete", "options", "head", "patch", "trace"];
        operations.iter().any(|op| obj.has_key(op))
    } else {
        false
    }
}

/// 为 CallbackElement 中的所有 PathItem 元素添加表达式元数据
/// 这是一个后处理函数，用于确保所有相关元素都有正确的元数据
pub fn decorate_callback_with_expressions(callback: &mut CallbackElement) {
    let mut keys_to_update = Vec::new();
    
    // 收集需要更新的键
    for member in &callback.object.content {
        if let Element::String(key_str) = &*member.key {
            let key = &key_str.content;
            if is_runtime_expression(key) {
                keys_to_update.push((key.clone(), (*member.value).clone()));
            }
        }
    }
    
    // 更新收集到的元素
    for (key, value) in keys_to_update {
        let mut decorated_value = value.clone();
        if let Element::Object(ref mut obj) = decorated_value {
            if obj.element == "pathItem" || contains_path_item_operations(&value) {
                obj.meta.properties.insert(
                    "runtime-expression".to_string(),
                    SimpleValue::string(key.clone())
                );
                callback.set(&key, decorated_value);
            }
        }
    }
}

/// 通用的折叠函数，用于统一处理 callback 构建逻辑
/// 支持自动检测是否需要递归折叠
pub fn with_callback_builder<F>(element: &Element, folder_opt: Option<&mut dyn Fold>, post_processor: F) -> Option<CallbackElement>
where
    F: FnOnce(&mut CallbackElement),
{
    let callback = if let Some(folder) = folder_opt {
        build_callback_with_folder(element, folder)?
    } else {
        build_callback(element)?
    };
    
    let mut callback = callback;
    post_processor(&mut callback);
    Some(callback)
}

/// 便利函数：构建并装饰回调元素
pub fn build_and_decorate_callback(element: &Element, folder: Option<&mut dyn Fold>) -> Option<CallbackElement> {
    with_callback_builder(element, folder, |callback| {
        callback.decorate_path_items_with_expressions();
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use apidom_ast::{Element, ObjectElement, StringElement};
    use apidom_ast::DefaultFolder;

    #[test]
    fn test_is_runtime_expression() {
        assert!(is_runtime_expression("{$request.body#/callbackUrl}"));
        assert!(is_runtime_expression("{$request.query.id}"));
        assert!(is_runtime_expression("{$response.header.location}"));
        assert!(!is_runtime_expression("regular_key"));
        assert!(!is_runtime_expression("callback"));
        assert!(!is_runtime_expression("{}"));
    }

    #[test]
    fn test_contains_path_item_operations() {
        let mut obj = ObjectElement::new();
        obj.set("post", Element::Object(ObjectElement::new()));
        assert!(contains_path_item_operations(&Element::Object(obj)));

        let mut obj2 = ObjectElement::new();
        obj2.set("summary", Element::String(StringElement::new("test")));
        assert!(!contains_path_item_operations(&Element::Object(obj2)));
    }

    #[test]
    fn test_build_callback_with_runtime_expression() {
        let mut callback_obj = ObjectElement::new();
        let mut path_item = ObjectElement::new();
        path_item.set_element_type("pathItem");
        path_item.set("post", Element::Object(ObjectElement::new()));
        
        callback_obj.set("{$request.body#/callbackUrl}", Element::Object(path_item));
        
        let result = build_callback(&Element::Object(callback_obj));
        assert!(result.is_some());
        
        let callback = result.unwrap();
        if let Some(Element::Object(path_item_obj)) = callback.get("{$request.body#/callbackUrl}") {
            assert!(path_item_obj.meta.properties.contains_key("runtime-expression"));
            if let Some(SimpleValue::String(expr)) = path_item_obj.meta.properties.get("runtime-expression") {
                assert_eq!(expr, "{$request.body#/callbackUrl}");
            }
        }
    }

    #[test]
    fn test_build_callback_with_ref() {
        let mut callback_obj = ObjectElement::new();
        let mut ref_obj = ObjectElement::new();
        ref_obj.set("$ref", Element::String(StringElement::new("#/components/pathItems/webhook")));
        
        callback_obj.set("webhookCallback", Element::Object(ref_obj));
        
        let result = build_callback(&Element::Object(callback_obj));
        assert!(result.is_some());
        
        let callback = result.unwrap();
        if let Some(Element::Object(ref_obj)) = callback.get("webhookCallback") {
            assert!(ref_obj.meta.properties.contains_key("referenced-element"));
            if let Some(SimpleValue::String(ref_type)) = ref_obj.meta.properties.get("referenced-element") {
                assert_eq!(ref_type, "callback");
            }
        }
    }

    #[test]
    fn test_build_callback_with_folder() {
        let mut callback_obj = ObjectElement::new();
        let mut path_item = ObjectElement::new();
        path_item.set_element_type("pathItem");
        path_item.set("post", Element::Object(ObjectElement::new()));
        
        callback_obj.set("{$request.body#/url}", Element::Object(path_item));
        
        let mut folder = DefaultFolder;
        let result = build_callback_with_folder(&Element::Object(callback_obj), &mut folder);
        assert!(result.is_some());
        
        let callback = result.unwrap();
        if let Some(Element::Object(path_item_obj)) = callback.get("{$request.body#/url}") {
            assert!(path_item_obj.meta.properties.contains_key("runtime-expression"));
        }
    }

    #[test]
    fn test_decorate_callback_with_expressions() {
        let mut callback = CallbackElement::new();
        let mut path_item = ObjectElement::new();
        path_item.set_element_type("pathItem");
        path_item.set("get", Element::Object(ObjectElement::new()));
        
        callback.set("{$request.header.callback}", Element::Object(path_item));
        
        decorate_callback_with_expressions(&mut callback);
        
        if let Some(Element::Object(decorated_obj)) = callback.get("{$request.header.callback}") {
            assert!(decorated_obj.meta.properties.contains_key("runtime-expression"));
        }
    }

    #[test]
    fn test_complex_callback_scenario() {
        let mut callback_obj = ObjectElement::new();
        
        // 添加多个回调表达式
        let expressions = [
            "{$request.body#/callbackUrl}",
            "{$request.query.webhook}",
            "{$response.header.location}"
        ];
        
        for expr in &expressions {
            let mut path_item = ObjectElement::new();
            path_item.set_element_type("pathItem");
            path_item.set("post", Element::Object(ObjectElement::new()));
            callback_obj.set(expr, Element::Object(path_item));
        }
        
        // 添加普通回调
        let mut normal_callback = ObjectElement::new();
        normal_callback.set("post", Element::Object(ObjectElement::new()));
        callback_obj.set("normalCallback", Element::Object(normal_callback));
        
        let result = build_callback(&Element::Object(callback_obj));
        assert!(result.is_some());
        
        let callback = result.unwrap();
        
        // 验证运行时表达式都有元数据
        for expr in &expressions {
            if let Some(Element::Object(obj)) = callback.get(expr) {
                assert!(obj.meta.properties.contains_key("runtime-expression"));
            }
        }
        
        // 验证普通回调没有运行时表达式元数据
        if let Some(Element::Object(obj)) = callback.get("normalCallback") {
            assert!(!obj.meta.properties.contains_key("runtime-expression"));
        }
    }

    #[test]
    fn test_with_callback_builder() {
        let mut callback_obj = ObjectElement::new();
        let mut path_item = ObjectElement::new();
        path_item.set_element_type("pathItem");
        path_item.set("post", Element::Object(ObjectElement::new()));
        callback_obj.set("{$request.body#/callback}", Element::Object(path_item));
        
        // 不使用 folder
        let callback1 = with_callback_builder(&Element::Object(callback_obj.clone()), None, |_| {});
        assert!(callback1.is_some());
        
        // 使用 folder
        let mut folder = DefaultFolder;
        let callback2 = with_callback_builder(&Element::Object(callback_obj), Some(&mut folder), |callback| {
            callback.decorate_path_items_with_expressions();
        });
        assert!(callback2.is_some());
        let callback2 = callback2.unwrap();
        let expr = callback2.get_path_item_expression("{$request.body#/callback}");
        assert!(expr.is_some());
    }

    #[test]
    fn test_build_and_decorate_callback() {
        let mut callback_obj = ObjectElement::new();
        let mut path_item = ObjectElement::new();
        path_item.set_element_type("pathItem");
        path_item.set("get", Element::Object(ObjectElement::new()));
        callback_obj.set("{$response.header.location}", Element::Object(path_item));
        
        let callback = build_and_decorate_callback(&Element::Object(callback_obj), None);
        assert!(callback.is_some());
        
        let callback = callback.unwrap();
        let expr = callback.get_path_item_expression("{$response.header.location}");
        assert!(expr.is_some());
        assert_eq!(expr.unwrap(), "{$response.header.location}");
    }
}