use apidom_ast::{Fold, CompositeFolder, folders::*, utils::*};
use apidom_ast::*;

fn main() {
    println!("🚀 API DOM Fold 演示");
    println!("==================");

    // 创建一个复杂的测试数据结构
    let test_data = create_test_data();
    println!("\n📊 原始数据结构:");
    print_element_summary(&test_data);

    // 演示 1: 基础字符串规范化
    println!("\n🔧 演示 1: 字符串规范化");
    let mut normalizer = StringNormalizer;
    let normalized = normalizer.fold_element(test_data.clone());
    print_element_summary(&normalized);

    // 演示 2: 收集特定类型的元素
    println!("\n🔍 演示 2: 收集所有字符串元素");
    let strings = collect_elements(test_data.clone(), |e| matches!(e, Element::String(_)));
    println!("找到 {} 个字符串元素", strings.len());
    for (i, element) in strings.iter().enumerate() {
        if let Element::String(s) = element {
            println!("  {}: '{}'", i + 1, s.content);
        }
    }

    // 演示 3: 类型转换
    println!("\n🔄 演示 3: 类型转换 (字符串转数字)");
    let mut converter = TypeConverter::new("number".to_string());
    let converted = converter.fold_element(test_data.clone());
    print_element_summary(&converted);

    // 演示 4: 空元素移除
    println!("\n🗑️ 演示 4: 移除空数组和对象");
    let data_with_empties = create_data_with_empties();
    println!("移除前:");
    print_element_summary(&data_with_empties);
    
    let mut remover = EmptyRemover;
    let cleaned = remover.fold_element(data_with_empties);
    println!("移除后:");
    print_element_summary(&cleaned);

    // 演示 5: 组合多个 folder
    println!("\n🔗 演示 5: 组合多个转换");
    let mut composite = CompositeFolder::new(vec![
        Box::new(StringNormalizer),
        Box::new(TypeConverter::new("number".to_string())),
        Box::new(EmptyRemover),
    ]);
    
    let final_result = composite.fold_element(test_data.clone());
    println!("经过组合转换后:");
    print_element_summary(&final_result);

    // 演示 6: 自定义 folder
    println!("\n✨ 演示 6: 自定义 folder - 数字翻倍");
    struct NumberDoubler;
    impl Fold for NumberDoubler {
        fn fold_number_element(&mut self, mut element: NumberElement) -> Element {
            element.content *= 2.0;
            Element::Number(element)
        }
    }

    let mut doubler = NumberDoubler;
    let doubled = doubler.fold_element(test_data.clone());
    print_element_summary(&doubled);

    // 演示 7: 使用工具函数
    println!("\n🛠️ 演示 7: 工具函数使用");
    
    // 计算元素数量
    let string_count = count_elements(test_data.clone(), |e| matches!(e, Element::String(_)));
    let number_count = count_elements(test_data.clone(), |e| matches!(e, Element::Number(_)));
    println!("字符串元素数量: {}", string_count);
    println!("数字元素数量: {}", number_count);
    
    // 查找特定元素
    let found_hello = find_element(test_data.clone(), |e| {
        matches!(e, Element::String(s) if s.content.contains("hello"))
    });
    if let Some(Element::String(s)) = found_hello {
        println!("找到包含 'hello' 的字符串: '{}'", s.content);
    }
    
    // 批量转换字符串
    let uppercased = map_strings(test_data.clone(), |s| s.to_uppercase());
    println!("所有字符串转大写后:");
    print_element_summary(&uppercased);

    if let Some(obj) = test_data.as_object() {
        if let Some(has_error) = obj.meta.properties.get("hasError") {
            println!("AST 错误标记: {:?}", has_error);
        }
    }

    println!("\n✅ 演示完成!");
}

fn create_test_data() -> Element {
    Element::Object(ObjectElement {
        element: "object".to_string(),
        meta: MetaElement::default(),
        attributes: AttributesElement::default(),
        classes: ArrayElement {
            element: "array".to_string(),
            meta: MetaElement::default(),
            attributes: AttributesElement::default(),
            content: vec![],
        },
        children: vec![],
        parent: None,
        content: vec![
            MemberElement {
                key: Box::new(Element::String(StringElement::new("name"))),
                value: Box::new(Element::String(StringElement::new("  Hello World  "))),
            },
            MemberElement {
                key: Box::new(Element::String(StringElement::new("age"))),
                value: Box::new(Element::String(StringElement::new("25"))),
            },
            MemberElement {
                key: Box::new(Element::String(StringElement::new("scores"))),
                value: Box::new(Element::Array(ArrayElement {
                    element: "array".to_string(),
                    meta: MetaElement::default(),
                    attributes: AttributesElement::default(),
                    content: vec![
                        Element::Number(NumberElement {
                            element: "number".to_string(),
                            meta: MetaElement::default(),
                            attributes: AttributesElement::default(),
                            content: 85.5,
                        }),
                        Element::String(StringElement::new("92")),
                        Element::Number(NumberElement {
                            element: "number".to_string(),
                            meta: MetaElement::default(),
                            attributes: AttributesElement::default(),
                            content: 78.0,
                        }),
                    ],
                })),
            },
            MemberElement {
                key: Box::new(Element::String(StringElement::new("active"))),
                value: Box::new(Element::String(StringElement::new("true"))),
            },
        ],
    })
}

fn create_data_with_empties() -> Element {
    Element::Object(ObjectElement {
        element: "object".to_string(),
        meta: MetaElement::default(),
        attributes: AttributesElement::default(),
        classes: ArrayElement {
            element: "array".to_string(),
            meta: MetaElement::default(),
            attributes: AttributesElement::default(),
            content: vec![],
        },
        children: vec![],
        parent: None,
        content: vec![
            MemberElement {
                key: Box::new(Element::String(StringElement::new("data"))),
                value: Box::new(Element::String(StringElement::new("content"))),
            },
            MemberElement {
                key: Box::new(Element::String(StringElement::new("empty_array"))),
                value: Box::new(Element::Array(ArrayElement {
                    element: "array".to_string(),
                    meta: MetaElement::default(),
                    attributes: AttributesElement::default(),
                    content: vec![],
                })),
            },
            MemberElement {
                key: Box::new(Element::String(StringElement::new("empty_object"))),
                value: Box::new(Element::Object(ObjectElement {
                    element: "object".to_string(),
                    meta: MetaElement::default(),
                    attributes: AttributesElement::default(),
                    classes: ArrayElement {
                        element: "array".to_string(),
                        meta: MetaElement::default(),
                        attributes: AttributesElement::default(),
                        content: vec![],
                    },
                    children: vec![],
                    parent: None,
                    content: vec![],
                })),
            },
        ],
    })
}

fn print_element_summary(element: &Element) {
    match element {
        Element::Null(_) => println!("  null"),
        Element::Boolean(b) => println!("  boolean: {}", b.content),
        Element::Number(n) => println!("  number: {}", n.content),
        Element::String(s) => println!("  string: '{}'", s.content),
        Element::Array(arr) => {
            println!("  array[{}]:", arr.content.len());
            for (i, item) in arr.content.iter().enumerate() {
                print!("    [{}] ", i);
                print_element_summary(item);
            }
        }
        Element::Object(obj) => {
            println!("  object{{{}}}:", obj.content.len());
            for member in &obj.content {
                if let Element::String(key) = member.key.as_ref() {
                    print!("    '{}': ", key.content);
                    print_element_summary(member.value.as_ref());
                }
            }
        }
        Element::Member(_) => println!("  member"),
        Element::Ref(r) => println!("  ref: {}", r.path),
        Element::Link(l) => println!("  link: {} -> {}", l.relation, l.href),
        Element::Custom(name, _) => println!("  custom({})", name),
    }
} 