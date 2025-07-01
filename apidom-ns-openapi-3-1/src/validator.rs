use crate::error::{Result, ValidationError};

/// 通用验证器 trait，对目标类型执行语义校验
pub trait Validator<T> {
    /// 返回 Ok(()) 表示验证通过，否则返回 ValidationError
    fn validate(&self, target: &T) -> Result<()>;
}

use crate::elements::InfoElement;

/// InfoElement 的示例验证器
/// 规则：title 与 version 必须存在
pub struct InfoValidator;

impl Validator<InfoElement> for InfoValidator {
    fn validate(&self, target: &InfoElement) -> Result<()> {
        if target.title().is_none() {
            return Err(ValidationError::new("Info.title is required"));
        }
        if target.version().is_none() {
            return Err(ValidationError::new("Info.version is required"));
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::elements::InfoElement;
    use apidom_ast::StringElement;

    #[test]
    fn test_info_validator_pass() {
        let mut info = InfoElement::new();
        info.set_title(StringElement::new("API"));
        info.set_version(StringElement::new("1.0"));

        let validator = InfoValidator;
        assert!(validator.validate(&info).is_ok());
    }

    #[test]
    fn test_info_validator_fail() {
        let info = InfoElement::new(); // missing fields
        let validator = InfoValidator;
        assert!(validator.validate(&info).is_err());
    }
} 