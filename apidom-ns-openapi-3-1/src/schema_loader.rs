use std::fs;
use std::path::Path;
use std::collections::HashMap;
use apidom_ast::{Element, MemberElement};
use apidom_cst::{CstParser, SourceType, TreeCursorSyntaxNode};

/// Schema 加载错误
#[derive(Debug)]
pub enum SchemaLoadError {
    IoError(std::io::Error),
    ParseError(String),
    DerefError(String),
    ConversionError(String),
}

impl From<std::io::Error> for SchemaLoadError {
    fn from(err: std::io::Error) -> Self {
        SchemaLoadError::IoError(err)
    }
}

/// OpenAPI 3.1 Schema 加载器
pub struct SchemaLoader {
    /// 缓存已加载的 schema
    cache: HashMap<String, Element>,
}

impl Default for SchemaLoader {
    fn default() -> Self {
        Self::new()
    }
}

impl SchemaLoader {
    pub fn new() -> Self {
        Self {
            cache: HashMap::new(),
        }
    }

    /// 从文件加载 OpenAPI 3.1 schema
    pub fn load_from_file<P: AsRef<Path>>(&mut self, path: P) -> Result<Element, SchemaLoadError> {
        let path_str = path.as_ref().to_string_lossy().to_string();
        
        // 检查缓存
        if let Some(cached) = self.cache.get(&path_str) {
            return Ok(cached.clone());
        }

        // 读取文件
        let content = fs::read_to_string(&path)?;
        
        // 使用 CST 解析 JSON 为 Element
        let element = self.parse_json_to_element(&content)?;
        
        // 缓存结果
        self.cache.insert(path_str, element.clone());
        
        Ok(element)
    }

    /// 将 JSON 字符串解析为 Element
    pub fn parse_json_to_element(&self, json_str: &str) -> Result<Element, SchemaLoadError> {
        // 使用 apidom-cst 解析 JSON
        let cst = CstParser::parse_as(json_str, SourceType::Json);
        
        // 检查解析错误
        if cst.has_error() {
            return Err(SchemaLoadError::ParseError("CST parsing failed".to_string()));
        }
        
        // 将 CST 转换为 Element
        self.cst_to_element(&cst)
    }

    /// 将 CST 节点转换为 Element
    fn cst_to_element(&self, cst_node: &TreeCursorSyntaxNode) -> Result<Element, SchemaLoadError> {
        use apidom_ast::{ObjectElement, ArrayElement, StringElement, NumberElement, BooleanElement, NullElement};
        
        match cst_node.kind.as_str() {
            "document" => {
                // JSON 文档的根节点，递归处理第一个子节点
                if let Some(first_child) = cst_node.children.first() {
                    self.cst_to_element(first_child)
                } else {
                    Ok(Element::Null(NullElement::default()))
                }
            }
            "object" => {
                let mut object_element = ObjectElement::new();
                
                // 查找所有 pair 节点
                for child in &cst_node.children {
                    if child.kind == "pair" {
                        let member = self.convert_pair_to_member(child)?;
                        object_element.content.push(member);
                    }
                }
                
                Ok(Element::Object(object_element))
            }
            "array" => {
                let mut array_element = ArrayElement::new_empty();
                
                // 处理数组中的每个值
                for child in &cst_node.children {
                    match child.kind.as_str() {
                        "string" | "number" | "true" | "false" | "null" | "object" | "array" => {
                            let element = self.cst_to_element(child)?;
                            array_element.content.push(element);
                        }
                        _ => {} // 跳过非值节点（如逗号、括号等）
                    }
                }
                
                Ok(Element::Array(array_element))
            }
            "string" => {
                // 提取字符串内容（去掉引号）
                let text = cst_node.text();
                let content = if text.len() >= 2 && text.starts_with('"') && text.ends_with('"') {
                    &text[1..text.len()-1]
                } else {
                    &text
                };
                Ok(Element::String(StringElement::new(content)))
            }
            "number" => {
                let text = cst_node.text();
                match text.parse::<f64>() {
                    Ok(num) => Ok(Element::Number(NumberElement::new(num))),
                    Err(_) => Err(SchemaLoadError::ConversionError(format!("Invalid number: {}", text)))
                }
            }
            "true" => Ok(Element::Boolean(BooleanElement::new(true))),
            "false" => Ok(Element::Boolean(BooleanElement::new(false))),
            "null" => Ok(Element::Null(NullElement::default())),
            _ => {
                // 对于其他节点类型，尝试递归处理第一个子节点
                if let Some(first_child) = cst_node.children.first() {
                    self.cst_to_element(first_child)
                } else {
                    Err(SchemaLoadError::ConversionError(format!("Unknown node type: {}", cst_node.kind)))
                }
            }
        }
    }

    /// 将 pair 节点转换为 MemberElement
    fn convert_pair_to_member(&self, pair_node: &TreeCursorSyntaxNode) -> Result<MemberElement, SchemaLoadError> {
        let mut key_element = None;
        let mut value_element = None;
        
        // 查找 key 和 value
        for child in &pair_node.children {
            // 跳过语法标记
            match child.kind.as_str() {
                ":" | "," => continue, // 跳过冒号和逗号
                _ => {}
            }
            
            if let Some(field_name) = child.field_name() {
                match field_name {
                    "key" => {
                        key_element = Some(self.cst_to_element(child)?);
                    }
                    "value" => {
                        value_element = Some(self.cst_to_element(child)?);
                    }
                    _ => {}
                }
            } else {
                // 如果没有字段名，按照位置判断
                match child.kind.as_str() {
                    "string" | "number" | "true" | "false" | "null" | "object" | "array" => {
                        if key_element.is_none() {
                            key_element = Some(self.cst_to_element(child)?);
                        } else if value_element.is_none() {
                            value_element = Some(self.cst_to_element(child)?);
                        }
                    }
                    _ => {} // 跳过其他类型
                }
            }
        }
        
        let key = key_element.ok_or_else(|| SchemaLoadError::ConversionError("Missing key in pair".to_string()))?;
        let value = value_element.ok_or_else(|| SchemaLoadError::ConversionError("Missing value in pair".to_string()))?;
        
        Ok(MemberElement::new(key, value))
    }

    /// 解引用 schema 中的 $ref
    pub fn dereference(&self, mut schema: Element) -> Result<Element, SchemaLoadError> {
        self.dereference_recursive(&mut schema)?;
        Ok(schema)
    }

    /// 递归解引用
    fn dereference_recursive(&self, element: &mut Element) -> Result<(), SchemaLoadError> {
        match element {
            Element::Object(obj) => {
                // 检查是否有 $ref
                if let Some(ref_element) = obj.get("$ref") {
                    if let Element::String(ref_str) = ref_element {
                        // 解析 $ref 路径
                        let ref_path = &ref_str.content;
                        if let Some(resolved) = self.resolve_ref(ref_path)? {
                            *element = resolved;
                            return Ok(());
                        }
                    }
                }
                
                // 递归处理对象中的其他字段
                for member in &mut obj.content {
                    self.dereference_recursive(&mut *member.value)?;
                }
            }
            Element::Array(arr) => {
                // 递归处理数组中的元素
                for item in &mut arr.content {
                    self.dereference_recursive(item)?;
                }
            }
            _ => {}
        }
        Ok(())
    }

    /// 解析 $ref 路径
    fn resolve_ref(&self, ref_path: &str) -> Result<Option<Element>, SchemaLoadError> {
        // 简化版本：只处理内部 JSON Pointer 引用（如 #/$defs/info）
        // 实际实现应该支持外部文件引用
        
        if ref_path.starts_with("#/") {
            // 内部引用，暂时返回 None（需要上下文 schema）
            Ok(None)
        } else {
            // 外部引用，暂时不支持
            Err(SchemaLoadError::DerefError(format!("External references not supported: {}", ref_path)))
        }
    }

    /// 从 schema 中获取特定定义
    pub fn get_definition(&self, schema: &Element, def_path: &str) -> Option<Element> {
        // 解析如 "#/$defs/info" 这样的路径
        if let Some(stripped) = def_path.strip_prefix("#/") {
            let parts: Vec<&str> = stripped.split('/').collect();
            self.navigate_to_path(schema, &parts)
        } else {
            None
        }
    }

    /// 导航到指定路径
    fn navigate_to_path(&self, element: &Element, path: &[&str]) -> Option<Element> {
        if path.is_empty() {
            return Some(element.clone());
        }

        match element {
            Element::Object(obj) => {
                if let Some(next_element) = obj.get(path[0]) {
                    self.navigate_to_path(next_element, &path[1..])
                } else {
                    None
                }
            }
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::NamedTempFile;

    #[test]
    fn test_load_simple_schema() {
        let mut loader = SchemaLoader::new();
        
        // 创建临时 JSON 文件
        let mut temp_file = NamedTempFile::new().unwrap();
        let json_content = r#"
        {
            "type": "object",
            "properties": {
                "title": {"type": "string"},
                "version": {"type": "string"}
            },
            "required": ["title", "version"]
        }
        "#;
        
        temp_file.write_all(json_content.as_bytes()).unwrap();
        
        let schema = loader.load_from_file(temp_file.path()).unwrap();
        
        // 验证解析结果
        if let Element::Object(obj) = schema {
            assert!(obj.get("type").is_some());
            assert!(obj.get("properties").is_some());
            assert!(obj.get("required").is_some());
        } else {
            panic!("Expected object element");
        }
    }

    #[test]
    fn test_cst_conversion() {
        let loader = SchemaLoader::new();
        
        // 测试简单对象转换
        let json = r#"{"type": "string", "minLength": 1}"#;
        let element = loader.parse_json_to_element(json).unwrap();
        
        if let Element::Object(obj) = element {
            // 验证 type 字段
            if let Some(Element::String(type_str)) = obj.get("type") {
                assert_eq!(type_str.content, "string");
            } else {
                panic!("Expected string type");
            }
            
            // 验证 minLength 字段
            if let Some(Element::Number(min_len)) = obj.get("minLength") {
                assert_eq!(min_len.content, 1.0);
            } else {
                panic!("Expected number minLength");
            }
        } else {
            panic!("Expected object element");
        }
    }

    #[test]
    fn test_array_conversion() {
        let loader = SchemaLoader::new();
        
        // 测试数组转换
        let json = r#"["string", "number", "boolean"]"#;
        let element = loader.parse_json_to_element(json).unwrap();
        
        if let Element::Array(arr) = element {
            assert_eq!(arr.content.len(), 3);
            
            for (i, expected) in ["string", "number", "boolean"].iter().enumerate() {
                if let Element::String(s) = &arr.content[i] {
                    assert_eq!(s.content, *expected);
                } else {
                    panic!("Expected string element at index {}", i);
                }
            }
        } else {
            panic!("Expected array element");
        }
    }

    #[test]
    fn test_nested_structure() {
        let loader = SchemaLoader::new();
        
        // 测试嵌套结构
        let json = r#"
        {
            "type": "object",
            "properties": {
                "name": {"type": "string"},
                "items": {
                    "type": "array",
                    "items": {"type": "number"}
                }
            }
        }
        "#;
        
        let element = loader.parse_json_to_element(json).unwrap();
        
        if let Element::Object(obj) = element {
            // 验证 properties 字段
            if let Some(Element::Object(props)) = obj.get("properties") {
                // 验证 name 属性
                if let Some(Element::Object(name_schema)) = props.get("name") {
                    if let Some(Element::String(type_str)) = name_schema.get("type") {
                        assert_eq!(type_str.content, "string");
                    }
                }
                
                // 验证 items 属性
                if let Some(Element::Object(items_schema)) = props.get("items") {
                    if let Some(Element::String(type_str)) = items_schema.get("type") {
                        assert_eq!(type_str.content, "array");
                    }
                }
            } else {
                panic!("Expected properties object");
            }
        } else {
            panic!("Expected object element");
        }
    }

    #[test]
    fn test_get_definition() {
        let loader = SchemaLoader::new();
        
        // 构建测试 schema
        use apidom_ast::{ObjectElement, StringElement};
        let mut schema = ObjectElement::new();
        let mut defs = ObjectElement::new();
        let mut info = ObjectElement::new();
        
        info.set("type", Element::String(StringElement::new("object")));
        defs.set("info", Element::Object(info));
        schema.set("$defs", Element::Object(defs));
        
        let schema_element = Element::Object(schema);
        
        // 测试获取定义
        let info_def = loader.get_definition(&schema_element, "#/$defs/info");
        assert!(info_def.is_some());
        
        if let Some(Element::Object(info_obj)) = info_def {
            assert!(info_obj.get("type").is_some());
        }
    }
} 