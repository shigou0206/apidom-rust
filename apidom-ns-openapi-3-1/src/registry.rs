use std::any::Any;
use std::collections::HashMap;
use std::sync::RwLock;

use lazy_static::lazy_static;

use crate::error::Result;
use apidom_ast::Element;
use crate::builder::{InfoBuilder, Builder};
use crate::elements::InfoElement;
use crate::context::BuildContext;

/// 构建器函数签名：接收 Element，返回装箱的构建结果
pub type BuilderFunc = fn(&Element) -> Result<Box<dyn Any + Send + Sync>>;

struct BuilderRegistry {
    map: HashMap<&'static str, BuilderFunc>,
}

impl BuilderRegistry {
    fn new() -> Self {
        Self { map: HashMap::new() }
    }

    fn register(&mut self, name: &'static str, func: BuilderFunc) {
        self.map.insert(name, func);
    }

    fn get(&self, name: &str) -> Option<&BuilderFunc> {
        self.map.get(name)
    }
}

lazy_static! {
    static ref BUILDER_REGISTRY: RwLock<BuilderRegistry> = RwLock::new(BuilderRegistry::new());
}

/// 注册构建器
pub fn register_builder(name: &'static str, func: BuilderFunc) {
    let mut reg = BUILDER_REGISTRY.write().unwrap();
    reg.register(name, func);
}

/// 获取构建器
pub fn get_builder(name: &str) -> Option<BuilderFunc> {
    let reg = BUILDER_REGISTRY.read().unwrap();
    reg.get(name).copied()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::builder::{InfoBuilder, Builder};
    use crate::elements::InfoElement;
    use apidom_ast::{ObjectElement, StringElement, MemberElement, Element};

    #[test]
    fn test_registry_workflow() {
        // 注册 InfoBuilder
        register_builder("info", |elem: &Element| {
            let b = InfoBuilder::new();
            b.build(elem, &BuildContext::default()).map(|v| Box::new(v) as Box<dyn Any + Send + Sync>)
        });

        // 构建测试 Element
        let mut obj = ObjectElement::new();
        obj.content.push(MemberElement::new(
            Element::String(StringElement::new("title")),
            Element::String(StringElement::new("API")),
        ));
        obj.content.push(MemberElement::new(
            Element::String(StringElement::new("version")),
            Element::String(StringElement::new("1.0")),
        ));
        let element = Element::Object(obj);

        // 通过注册中心获取 builder 并调用
        let func = get_builder("info").expect("builder not found");
        let boxed = func(&element).expect("build failed");
        let info = boxed.downcast_ref::<InfoElement>().expect("downcast");
        assert_eq!(info.title().unwrap().content, "API");
    }
} 