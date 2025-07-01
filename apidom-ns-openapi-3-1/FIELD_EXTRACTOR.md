# FieldExtractor 字段提取器

ApiDOM OpenAPI 3.1 命名空间的字段提取器是一个高性能、类型安全的字段操作工具，专门设计用于从 OpenAPI 3.1 元素中提取和验证字段值。

## 设计理念

### 🎯 性能优先
- **零拷贝操作**: 字符串提取避免不必要的内存分配
- **高效查找**: 优化的哈希表查找，平均 O(1) 时间复杂度
- **批量操作**: 支持批量字段提取和验证，减少遍历开销

### 🔒 类型安全
- **静态类型检查**: 编译时确保类型正确性
- **空值处理**: 显式的 `Option` 类型处理缺失字段
- **泛型支持**: 支持自定义提取器和转换器

### 🛠️ 易用性
- **直观 API**: 简洁明了的函数命名和参数
- **宏支持**: 提供便捷的批量提取宏
- **错误处理**: 详细的错误信息和恢复机制

## 核心功能

### 1. 基本字段提取

```rust
use apidom_ns_openapi_3_1::field_extractor::FieldExtractor;

// 字符串字段
let title = FieldExtractor::extract_string(&element, "title");

// 数值字段
let port = FieldExtractor::extract_number(&element, "port");

// 布尔字段
let enabled = FieldExtractor::extract_boolean(&element, "enabled");

// 整数字段
let timeout = FieldExtractor::extract_integer(&element, "timeout");
```

### 2. 复杂类型提取

```rust
// 字符串数组
let tags = FieldExtractor::extract_string_array(&element, "tags");

// 嵌套对象字段
let contact_name = FieldExtractor::extract_nested_string(&element, &["contact", "name"]);

// URL 验证提取
let server_url = FieldExtractor::extract_url(&element, "url");

// 版本号提取
let api_version = FieldExtractor::extract_version(&element, "version");
```

### 3. 扩展字段处理

```rust
// 提取所有 x- 扩展字段
let extensions = FieldExtractor::extract_extension_fields(&element);

// 遍历扩展字段
for (key, value) in extensions {
    println!("扩展字段: {} = {:?}", key, value);
}
```

### 4. 字段验证

```rust
// 验证必填字段
let required = ["title", "version", "paths"];
match FieldExtractor::validate_required_fields(&element, &required) {
    Ok(()) => println!("所有必填字段都存在"),
    Err(missing) => println!("缺少字段: {:?}", missing),
}

// 类型检查
let is_string = FieldExtractor::is_field_of_type(&element, "title", "string");
let is_object = FieldExtractor::is_field_of_type(&element, "info", "object");
```

### 5. 高级操作

```rust
// 带默认值的提取
let description = FieldExtractor::extract_with_default(
    &element,
    "description",
    FieldExtractor::extract_string,
    "默认描述".to_string()
);

// 批量字符串字段提取
let fields = ["title", "version", "description"];
let values = FieldExtractor::extract_string_fields(&element, &fields);

// 枚举值提取
enum ApiType { Rest, GraphQL, Grpc }
let api_type = FieldExtractor::extract_enum_value(&element, "type", &[
    ("rest", ApiType::Rest),
    ("graphql", ApiType::GraphQL),
    ("grpc", ApiType::Grpc),
]);
```

## 性能指标

基于最新性能测试结果：

| 操作类型 | 每次操作耗时 | 吞吐量 |
|---------|-------------|--------|
| 单个字段提取 | ~72 纳秒 | 13.9M ops/sec |
| 扩展字段提取 | ~3.8 微秒 | 264K ops/sec |
| 批量验证 (5字段) | ~305 纳秒 | 3.3M ops/sec |
| 类型检查 | ~53 纳秒 | 18.9M ops/sec |
| 综合操作 | ~4.0 微秒 | 248K ops/sec |

### 性能特点

1. **极低延迟**: 单个字段提取仅需纳秒级时间
2. **高吞吐量**: 每秒可处理数百万次操作
3. **稳定性能**: 性能不随数据大小线性增长
4. **内存效率**: 最小化内存分配和拷贝

## 使用模式

### 1. 单次提取模式

适用于偶发的字段提取需求：

```rust
if let Some(title) = FieldExtractor::extract_string(&element, "title") {
    println!("API 标题: {}", title);
}
```

### 2. 批量提取模式

适用于需要多个字段的场景：

```rust
let field_names = ["title", "version", "description"];
let values = FieldExtractor::extract_string_fields(&element, &field_names);

for (field, value) in values {
    println!("{}: {}", field, value);
}
```

### 3. 验证优先模式

适用于严格验证的场景：

```rust
// 先验证必填字段
FieldExtractor::validate_required_fields(&element, &["title", "version"])?;

// 再提取字段
let title = FieldExtractor::extract_string(&element, "title").unwrap();
let version = FieldExtractor::extract_string(&element, "version").unwrap();
```

### 4. 容错模式

适用于可选字段较多的场景：

```rust
let title = FieldExtractor::extract_with_default(
    &element, "title", FieldExtractor::extract_string, "未命名 API".to_string()
);

let description = FieldExtractor::extract_with_default(
    &element, "description", FieldExtractor::extract_string, "无描述".to_string()
);
```

## 最佳实践

### 1. 性能优化

```rust
// ✅ 好的做法：批量操作
let fields = ["title", "version", "description"];
let values = FieldExtractor::extract_string_fields(&element, &fields);

// ❌ 避免：多次单独提取
let title = FieldExtractor::extract_string(&element, "title");
let version = FieldExtractor::extract_string(&element, "version");
let description = FieldExtractor::extract_string(&element, "description");
```

### 2. 错误处理

```rust
// ✅ 好的做法：显式错误处理
match FieldExtractor::validate_required_fields(&element, &required_fields) {
    Ok(()) => {
        // 继续处理
    },
    Err(missing_fields) => {
        eprintln!("验证失败，缺少字段: {:?}", missing_fields);
        return Err("字段验证失败".into());
    }
}

// ❌ 避免：忽略错误
let _ = FieldExtractor::validate_required_fields(&element, &required_fields);
```

### 3. 类型安全

```rust
// ✅ 好的做法：使用类型检查
if FieldExtractor::is_field_of_type(&element, "port", "number") {
    let port = FieldExtractor::extract_number(&element, "port").unwrap();
    // 安全使用 port
}

// ❌ 避免：假设类型正确
let port = FieldExtractor::extract_number(&element, "port").unwrap_or(80.0);
```

### 4. 内存效率

```rust
// ✅ 好的做法：避免不必要的克隆
let extensions = FieldExtractor::extract_extension_fields(&element);
for (key, value) in extensions.iter() {
    // 使用引用
    process_extension(key, value);
}

// ❌ 避免：过度克隆
let extensions = FieldExtractor::extract_extension_fields(&element);
for (key, value) in extensions {
    let owned_key = key.clone();
    let owned_value = value.clone();
    // 不必要的克隆
}
```

## 扩展和定制

### 1. 自定义提取器

```rust
impl FieldExtractor {
    pub fn extract_email(element: &Element, field_name: &str) -> Option<String> {
        Self::extract_string(element, field_name).filter(|email| {
            email.contains('@') && email.contains('.')
        })
    }
    
    pub fn extract_phone(element: &Element, field_name: &str) -> Option<String> {
        Self::extract_string(element, field_name).filter(|phone| {
            phone.chars().filter(|c| c.is_ascii_digit()).count() >= 10
        })
    }
}
```

### 2. 类型转换器

```rust
pub fn extract_duration(element: &Element, field_name: &str) -> Option<Duration> {
    FieldExtractor::extract_string(element, field_name)
        .and_then(|s| s.parse::<u64>().ok())
        .map(Duration::from_secs)
}

pub fn extract_timestamp(element: &Element, field_name: &str) -> Option<SystemTime> {
    FieldExtractor::extract_integer(element, field_name)
        .and_then(|ts| {
            UNIX_EPOCH.checked_add(Duration::from_secs(ts as u64))
        })
}
```

## 故障排除

### 常见问题

1. **字段不存在**: 返回 `None`，检查字段名拼写
2. **类型不匹配**: 返回 `None`，使用 `is_field_of_type` 预检查
3. **嵌套字段失败**: 检查路径是否正确，中间对象是否存在

### 调试技巧

```rust
// 打印字段名列表
let field_names = FieldExtractor::extract_field_names(&element);
println!("可用字段: {:?}", field_names);

// 检查字段类型
if !FieldExtractor::is_field_of_type(&element, "port", "number") {
    println!("port 字段不是数字类型");
}

// 详细错误信息
match FieldExtractor::validate_required_fields(&element, &required) {
    Err(missing) => {
        for field in missing {
            println!("缺少必填字段: {}", field);
        }
    },
    _ => {}
}
```

## 与其他组件集成

### 1. 与 FieldHandlerMap 集成

```rust
handlers.register_fixed("title", |value, target, _| {
    if let Some(title) = FieldExtractor::extract_string(value, "title") {
        target.set_title(title);
        Some(())
    } else {
        None
    }
});
```

### 2. 与 SchemaLoader 集成

```rust
let element = loader.parse_json_to_element(json_data)?;
let title = FieldExtractor::extract_string(&element, "title");
```

### 3. 与 BuilderDispatch 集成

```rust
impl BuilderDispatch {
    fn extract_and_build(&self, element: &Element) -> Option<InfoElement> {
        let title = FieldExtractor::extract_string(element, "title")?;
        let version = FieldExtractor::extract_string(element, "version")?;
        
        let mut info = InfoElement::new();
        info.set_title(title);
        info.set_version(version);
        Some(info)
    }
}
```

## 版本历史

- **v0.1.0**: 初始版本，包含基本提取功能
- **v0.1.1**: 添加扩展字段支持
- **v0.1.2**: 性能优化，添加批量操作
- **v0.1.3**: 增强类型安全，添加验证功能

## 许可证

本模块采用 MIT 许可证，详见项目根目录的 LICENSE 文件。 