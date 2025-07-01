use crate::error::Result;
use apidom_ast::SimpleValue;

/// 字段路径，使用 Vec<String> 表示从文档根开始的层级
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct FieldPath(pub Vec<String>);

impl FieldPath {
    pub fn root() -> Self {
        FieldPath(vec!["$".to_string()])
    }

    pub fn push(&mut self, segment: impl Into<String>) {
        self.0.push(segment.into());
    }

    pub fn pop(&mut self) {
        self.0.pop();
    }

    pub fn join(&self, sep: &str) -> String {
        self.0.join(sep)
    }
}

/// 构建上下文，用于在 Builder/Validator 之间传递元数据
#[derive(Debug, Clone)]
pub struct BuildContext {
    pub path: FieldPath,
}

impl Default for BuildContext {
    fn default() -> Self {
        BuildContext { path: FieldPath::root() }
    }
}

impl BuildContext {
    /// 创建子路径上下文 (用于进入对象字段)
    pub fn child(&self, segment: &str) -> Self {
        let mut new_path = self.path.clone();
        new_path.push(segment.to_string());
        BuildContext { path: new_path }
    }

    /// 将路径写入目标元素 meta
    pub fn inject_path_meta(&self, meta: &mut std::collections::HashMap<String, SimpleValue>) {
        meta.insert("path".to_string(), SimpleValue::string(self.path.join(".")));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_field_path() {
        let mut p = FieldPath::root();
        p.push("info");
        p.push("title");
        assert_eq!(p.join("."), "$.info.title");
        p.pop();
        assert_eq!(p.join("/"), "$/info");
    }

    #[test]
    fn test_context_child() {
        let ctx = BuildContext::default();
        let child = ctx.child("info");
        assert_eq!(child.path.join("/"), "$/info");
    }
} 