pub mod minim_model;
pub mod fold;

// Re-export commonly used items for convenience
pub use fold::{Fold, DefaultFolder, CompositeFolder};
pub use minim_model::{Element, ElementRegistry};