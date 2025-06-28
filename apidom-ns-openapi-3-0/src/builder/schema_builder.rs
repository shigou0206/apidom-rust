use apidom_ast::minim_model::*;
use crate::elements::schema::OpenApiSchemaElement;

/// 构建 OpenAPI SchemaElement（适配 Minim AST 到 OpenAPI 特定结构）
/// 支持 `$ref`、object schema、多种 OpenAPI 扩展字段
pub fn build_openapi_schema(element: &Element) -> Option<OpenApiSchemaElement> {
    match element {
        Element::Object(obj) => {
            // 如果包含 $ref，则保留原结构（留给引用系统处理）
            if obj.get("$ref").is_some() {
                return Some(OpenApiSchemaElement::with_content(obj.clone()));
            }

            Some(OpenApiSchemaElement::with_content(obj.clone()))
        }
        // OpenAPI 不支持直接将 bool 用作 schema（如 true/false），可视为 noop
        Element::Boolean(_) => None,
        Element::Ref(_) => Some(OpenApiSchemaElement::with_content(ObjectElement::new())),
        Element::Link(_) => Some(OpenApiSchemaElement::with_content(ObjectElement::new())),
        _ => None,
    }
}