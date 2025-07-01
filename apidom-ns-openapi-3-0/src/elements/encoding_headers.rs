use apidom_ast::*;

/// OpenAPI Encoding Headers Element
/// Specialized element for encoding headers which is a Map type rather than generic Object
#[derive(Debug, Clone)]
pub struct EncodingHeadersElement {
    pub object: ObjectElement,
}

impl EncodingHeadersElement {
    pub fn new() -> Self {
        let mut obj = ObjectElement::new();
        obj.set_element_type("encodingHeaders");
        obj.add_class("encoding-headers");
        Self { object: obj }
    }

    pub fn with_content(content: ObjectElement) -> Self {
        let mut content = content;
        content.set_element_type("encodingHeaders");
        content.add_class("encoding-headers");
        Self { object: content }
    }

    /// Get a header by name
    pub fn get_header(&self, name: &str) -> Option<&Element> {
        self.object.get(name)
    }

    /// Set a header
    pub fn set_header(&mut self, name: &str, header: Element) {
        self.object.set(name, header);
    }

    /// Get all header names
    pub fn header_names(&self) -> Vec<String> {
        self.object.content.iter()
            .filter_map(|member| {
                if let Element::String(key_str) = &*member.key {
                    Some(key_str.content.clone())
                } else {
                    None
                }
            })
            .collect()
    }

    /// Check if headers has a specific name
    pub fn has_header(&self, name: &str) -> bool {
        self.object.has_key(name)
    }

    /// Get the number of headers
    pub fn header_count(&self) -> usize {
        self.object.content.len()
    }

    /// Iterate over all headers
    pub fn headers(&self) -> impl Iterator<Item = (&str, &Element)> {
        self.object.content.iter().filter_map(|member| {
            if let Element::String(key) = &*member.key {
                Some((key.content.as_str(), &*member.value))
            } else {
                None
            }
        })
    }
} 