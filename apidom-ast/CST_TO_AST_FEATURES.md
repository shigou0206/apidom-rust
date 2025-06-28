# CST 到 AST 转换功能

本文档描述了 `apidom-ast` 包中新实现的 CST (Concrete Syntax Tree) 到 AST (Abstract Syntax Tree) 转换功能。

## 概述

CST 到 AST 转换功能允许将 `apidom-cst` 包生成的具体语法树转换为 `apidom-ast` 的抽象语法树表示。这提供了一个从原始代码到结构化数据模型的完整转换管道。

## 核心组件

### 1. `FoldFromCst` Trait

这是 CST 到 AST 转换的核心 trait，扩展了 `Fold` trait：

```rust
pub trait FoldFromCst: Fold {
    /// 主要入口点：将 CST 节点转换为 AST 元素
    fn fold_from_cst(&mut self, node: &TreeCursorSyntaxNode) -> Element;
    
    /// 基于节点类型分发转换方法
    fn fold_cst_node(&mut self, node: &TreeCursorSyntaxNode) -> Element;
    
    /// 特定类型的转换方法
    fn fold_cst_object(&mut self, node: &TreeCursorSyntaxNode) -> Element;
    fn fold_cst_array(&mut self, node: &TreeCursorSyntaxNode) -> Element;
    fn fold_cst_string(&mut self, node: &TreeCursorSyntaxNode) -> Element;
    fn fold_cst_number(&mut self, node: &TreeCursorSyntaxNode) -> Element;
    fn fold_cst_boolean(&mut self, node: &TreeCursorSyntaxNode) -> Element;
    fn fold_cst_null(&mut self, node: &TreeCursorSyntaxNode) -> Element;
    fn fold_cst_pair(&mut self, node: &TreeCursorSyntaxNode) -> Element;
    fn fold_cst_unknown(&mut self, node: &TreeCursorSyntaxNode) -> Element;
}
```

### 2. `JsonFolder` 结构体

专门用于 JSON CST 到 AST 转换的实现：

```rust
pub struct JsonFolder {
    include_source_info: bool,
    preserve_formatting: bool,
}
```

#### 功能特性：

- **源码位置信息**：可选择包含原始源码的位置信息
- **字符串转义处理**：正确处理 JSON 字符串中的转义序列
- **错误处理**：处理格式错误的 JSON 并在元数据中标记错误
- **类型转换**：将 CST 的文本节点转换为适当的 AST 元素类型

## 支持的转换

### JSON 数据类型

| CST 节点类型 | AST 元素类型 | 说明 |
|-------------|-------------|------|
| `document` | 根据内容 | 文档根节点，转发到实际内容 |
| `object` | `ObjectElement` | JSON 对象 |
| `array` | `ArrayElement` | JSON 数组 |
| `string` | `StringElement` | 字符串（处理转义） |
| `number` | `NumberElement` | 数字（解析为 f64） |
| `true`/`false` | `BooleanElement` | 布尔值 |
| `null` | `NullElement` | 空值 |
| `pair` | `MemberElement` | 对象成员（键值对） |

### 字符串转义序列

支持完整的 JSON 字符串转义：

- `\"` → `"`
- `\\` → `\`
- `\/` → `/`
- `\b` → 退格
- `\f` → 换页
- `\n` → 换行
- `\r` → 回车
- `\t` → 制表符
- `\uXXXX` → Unicode 字符

### 元数据信息

转换过程中会添加以下元数据：

- **源码位置**：包含起始和结束位置（行、列、字节偏移）
- **错误标记**：标识解析过程中遇到的错误
- **字段名称**：CST 节点的字段名（如果有）

## API 使用示例

### 基础转换

```rust
use apidom_ast::fold::{JsonFolder, FoldFromCst};
use apidom_cst::CstParser;

// 解析 JSON 源码为 CST
let cst = CstParser::parse(r#"{"name": "Alice", "age": 30}"#);

// 创建转换器
let mut folder = JsonFolder::new();

// 转换为 AST
let ast = folder.fold_from_cst(&cst);
```

### 便利函数

```rust
use apidom_ast::fold::{json_cst_to_ast, json_source_to_ast};

// 直接从 CST 转换
let ast1 = json_cst_to_ast(&cst);

// 从源码一步转换
let ast2 = json_source_to_ast(r#"{"hello": "world"}"#);
```

### 自定义配置

```rust
// 包含源码位置信息，不保留格式
let mut folder = JsonFolder::with_options(true, false);
let ast = folder.fold_from_cst(&cst);
```

### 与 Fold 机制结合

```rust
use apidom_ast::fold::folders::{StringNormalizer, TypeConverter};

// 转换为 AST
let mut folder = JsonFolder::new();
let mut ast = folder.fold_from_cst(&cst);

// 应用字符串规范化
let mut normalizer = StringNormalizer;
ast = normalizer.fold_element(ast);

// 应用类型转换
let mut converter = TypeConverter::new("number".to_string());
ast = converter.fold_element(ast);
```

## 错误处理

转换器能够处理格式错误的 JSON：

1. **语法错误**：在元数据中标记 `hasError: true`
2. **不完整的结构**：创建部分 AST 包含已解析的内容
3. **未知节点类型**：创建 `CustomElement` 保存原始文本

## 性能特性

- **惰性转换**：只转换请求的节点
- **内存效率**：避免不必要的字符串复制
- **递归安全**：处理深度嵌套的结构

## 扩展性

通过实现 `FoldFromCst` trait，可以：

1. **支持新的语言**：为其他语言实现 CST 到 AST 转换
2. **自定义转换逻辑**：重写特定节点类型的转换方法
3. **添加验证**：在转换过程中添加语义验证

## 测试覆盖

包含全面的测试套件：

- 基本 JSON 类型转换
- 字符串转义处理
- 嵌套结构转换
- 错误情况处理
- 源码位置信息
- 便利函数测试

## 示例程序

查看以下示例程序了解更多用法：

- `examples/cst_to_ast_demo.rs` - 完整的转换演示
- `examples/debug_cst.rs` - CST 结构调试工具

## 与现有 Fold 系统的集成

CST 到 AST 转换完全兼容现有的 Fold 系统：

1. 所有 `FoldFromCst` 实现都自动是 `Fold` 实现
2. 可以在转换后应用任何现有的 Folder
3. 支持 `CompositeFolder` 组合多个转换步骤

这种设计确保了 CST 到 AST 转换能够无缝集成到现有的数据处理管道中。 