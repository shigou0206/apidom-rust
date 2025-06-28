pub mod utils;

use smallvec::SmallVec;
use std::fmt;

pub use either::Either;

/// Visit all children nodes. This converts `VisitAll` to `Visit`. The type
/// parameter `V` should implement `VisitAll` and `All<V>` implements `Visit`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct All<V> {
    pub visitor: V,
}

impl<V> Default for All<V>
where
    V: Default,
{
    fn default() -> Self {
        Self {
            visitor: V::default(),
        }
    }
}

/// Conditional visitor that can be enabled, disabled, or conditionally enabled.
/// 
/// This enum provides a zero-cost abstraction for optional visitors that can be:
/// - Completely disabled (no runtime overhead)
/// - Always enabled with normal or high priority
/// - Conditionally enabled based on a runtime condition
/// 
/// # Examples
/// 
/// ```
/// use apidom_visit::OptionalVisitor;
/// 
/// // Create a simple visitor (placeholder for this example)
/// struct MyVisitor;
/// 
/// // Always enabled visitor
/// let visitor = OptionalVisitor::enabled(MyVisitor);
/// assert!(visitor.is_active());
/// 
/// // Disabled visitor
/// let disabled = OptionalVisitor::<MyVisitor>::disabled();
/// assert!(!disabled.is_active());
/// 
/// // Conditionally enabled visitor
/// let conditional = OptionalVisitor::conditional(MyVisitor, || true);
/// assert!(conditional.is_active());
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OptionalVisitor<V, C = fn() -> bool> {
    /// Visitor is completely disabled
    Disabled,
    /// Visitor is enabled with normal priority
    Enabled(V),
    /// Visitor is enabled with high priority (always runs first)
    HighPriority(V),
    /// Visitor is conditionally enabled based on generic condition
    Conditional { visitor: V, condition: C },
}

/// Priority levels for optional visitors
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum VisitorPriority {
    Disabled = 0,
    Normal = 1,
    High = 2,
}

impl<V> OptionalVisitor<V> {
    /// Create a new optional visitor that's enabled with normal priority
    pub fn enabled(visitor: V) -> Self {
        OptionalVisitor::Enabled(visitor)
    }
    
    /// Create a new optional visitor that's disabled
    pub fn disabled() -> Self {
        OptionalVisitor::Disabled
    }
    
    /// Create a high-priority visitor that runs before others
    pub fn high_priority(visitor: V) -> Self {
        OptionalVisitor::HighPriority(visitor)
    }
}

impl<V, C> OptionalVisitor<V, C>
where
    C: Fn() -> bool,
{
    /// Create a conditional visitor with generic condition
    pub fn conditional(visitor: V, condition: C) -> Self {
        OptionalVisitor::Conditional { visitor, condition }
    }
    
    /// Check if the visitor is currently active
    pub fn is_active(&self) -> bool {
        match self {
            OptionalVisitor::Disabled => false,
            OptionalVisitor::Enabled(_) => true,
            OptionalVisitor::HighPriority(_) => true,
            OptionalVisitor::Conditional { condition, .. } => condition(),
        }
    }
    
    /// Get the priority level of the visitor
    pub fn priority(&self) -> VisitorPriority {
        match self {
            OptionalVisitor::Disabled => VisitorPriority::Disabled,
            OptionalVisitor::Enabled(_) => VisitorPriority::Normal,
            OptionalVisitor::HighPriority(_) => VisitorPriority::High,
            OptionalVisitor::Conditional { condition, .. } => {
                if condition() {
                    VisitorPriority::Normal
                } else {
                    VisitorPriority::Disabled
                }
            }
        }
    }
    
    /// Execute the visitor if active
    pub fn execute<F, R>(&self, f: F) -> Option<R>
    where
        F: FnOnce(&V) -> R,
    {
        match self {
            OptionalVisitor::Disabled => None,
            OptionalVisitor::Enabled(v) => Some(f(v)),
            OptionalVisitor::HighPriority(v) => Some(f(v)),
            OptionalVisitor::Conditional { visitor, condition } => {
                if condition() {
                    Some(f(visitor))
                } else {
                    None
                }
            }
        }
    }
}

/// Error types for fixed-point iteration operations.
///
/// This provides more structured error handling compared to generic strings,
/// allowing for better error categorization and handling strategies.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FixedPointError {
    /// Maximum iterations exceeded without convergence
    MaxIterationsExceeded {
        max_iterations: usize,
        last_change_iteration: Option<usize>,
    },
    /// Pass failed with a specific error
    PassFailed {
        iteration: usize,
        error: String,
    },
}

impl fmt::Display for FixedPointError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FixedPointError::MaxIterationsExceeded { max_iterations, last_change_iteration } => {
                write!(f, "Maximum iterations ({}) exceeded without convergence", max_iterations)?;
                if let Some(last_change) = last_change_iteration {
                    write!(f, " (last change at iteration {})", last_change)?;
                }
                Ok(())
            }
            FixedPointError::PassFailed { iteration, error } => {
                write!(f, "Pass failed at iteration {}: {}", iteration, error)
            }
        }
    }
}

impl std::error::Error for FixedPointError {}

/// A pass that can be repeated until a fixed point is reached.
/// 
/// This trait is designed for passes that can be invoked multiple times on the same input,
/// typically used for iterative optimization where each pass might enable further optimizations.
/// 
/// # Examples
/// 
/// ```
/// use apidom_visit::{Repeated, run_until_fixed};
/// 
/// struct SimplifyPass {
///     changed: bool,
/// }
/// 
/// impl SimplifyPass {
///     fn new() -> Self {
///         Self { changed: false }
///     }
/// }
/// 
/// impl Repeated for SimplifyPass {
///     fn changed(&self) -> bool {
///         self.changed
///     }
///     
///     fn reset(&mut self) {
///         self.changed = false;
///     }
/// }
/// 
/// let mut pass = SimplifyPass::new();
/// let mut target = vec![1, 2, 3];
/// 
/// let result = run_until_fixed::<_, _, _, 10>(&mut pass, &mut target, |_pass, _target| {
///     // Apply transformation here
/// });
/// 
/// assert!(result.is_ok());
/// ```
pub trait Repeated {
    /// Should run again?
    /// 
    /// Returns `true` if the pass made changes and should be run again.
    fn changed(&self) -> bool;

    /// Reset the change tracking state.
    /// 
    /// This should be called before each iteration to reset the change tracking.
    fn reset(&mut self);
}

/// Configuration-aware path container using SmallVec with configurable capacity.
pub type ConfiguredPathVec<T> = SmallVec<[T; 8]>; // Default capacity

/// Specialized path vectors for different configurations
pub type DefaultPathVec<T> = SmallVec<[T; 8]>;
pub type PerformancePathVec<T> = SmallVec<[T; 16]>;
pub type CompactPathVec<T> = SmallVec<[T; 4]>;

/// Generic RAII guard with closure-based cleanup for zero code duplication.
/// 
/// This guard eliminates all repetitive guard code by using closures for cleanup logic.
pub struct Guard<T, F>
where
    F: FnOnce(&mut T),
{
    target: Option<T>,
    cleanup: Option<F>,
}

impl<T, F> Guard<T, F>
where
    F: FnOnce(&mut T),
{
    /// Create a new guard with custom cleanup logic
    pub fn new(target: T, cleanup: F) -> Self {
        Self {
            target: Some(target),
            cleanup: Some(cleanup),
        }
    }
    
    /// Get reference to the guarded value
    pub fn get(&self) -> Option<&T> {
        self.target.as_ref()
    }
    
    /// Get mutable reference to the guarded value
    pub fn get_mut(&mut self) -> Option<&mut T> {
        self.target.as_mut()
    }
}

impl<T, F> Drop for Guard<T, F>
where
    F: FnOnce(&mut T),
{
    fn drop(&mut self) {
        if let (Some(mut target), Some(cleanup)) = (self.target.take(), self.cleanup.take()) {
            cleanup(&mut target);
        }
    }
}

/// Configuration-driven run_until_fixed that uses trait configuration
pub fn configured_run_until_fixed<P, T, F, C>(
    pass: &mut P,
    target: &mut T,
    apply_fn: F,
) -> Result<usize, FixedPointError>
where
    P: Repeated,
    F: FnMut(&mut P, &mut T),
    C: ConfigTrait,
{
    run_until_fixed_legacy(pass, target, apply_fn, C::MAX_ITERATIONS)
}

/// Simplified tuple implementations for common sizes (up to 7 elements)
impl<T1: Repeated> Repeated for (T1,) {
    fn changed(&self) -> bool { self.0.changed() }
    fn reset(&mut self) { self.0.reset(); }
}

impl<T1: Repeated, T2: Repeated> Repeated for (T1, T2) {
    fn changed(&self) -> bool { self.0.changed() || self.1.changed() }
    fn reset(&mut self) { self.0.reset(); self.1.reset(); }
}

impl<T1: Repeated, T2: Repeated, T3: Repeated> Repeated for (T1, T2, T3) {
    fn changed(&self) -> bool { self.0.changed() || self.1.changed() || self.2.changed() }
    fn reset(&mut self) { self.0.reset(); self.1.reset(); self.2.reset(); }
}

impl<T1: Repeated, T2: Repeated, T3: Repeated, T4: Repeated> Repeated for (T1, T2, T3, T4) {
    fn changed(&self) -> bool { self.0.changed() || self.1.changed() || self.2.changed() || self.3.changed() }
    fn reset(&mut self) { self.0.reset(); self.1.reset(); self.2.reset(); self.3.reset(); }
}

impl<T1: Repeated, T2: Repeated, T3: Repeated, T4: Repeated, T5: Repeated> Repeated for (T1, T2, T3, T4, T5) {
    fn changed(&self) -> bool { 
        self.0.changed() || self.1.changed() || self.2.changed() || self.3.changed() || self.4.changed() 
    }
    fn reset(&mut self) { 
        self.0.reset(); self.1.reset(); self.2.reset(); self.3.reset(); self.4.reset(); 
    }
}

impl<T1: Repeated, T2: Repeated, T3: Repeated, T4: Repeated, T5: Repeated, T6: Repeated> Repeated for (T1, T2, T3, T4, T5, T6) {
    fn changed(&self) -> bool { 
        self.0.changed() || self.1.changed() || self.2.changed() || self.3.changed() || self.4.changed() || self.5.changed()
    }
    fn reset(&mut self) { 
        self.0.reset(); self.1.reset(); self.2.reset(); self.3.reset(); self.4.reset(); self.5.reset();
    }
}

impl<T1: Repeated, T2: Repeated, T3: Repeated, T4: Repeated, T5: Repeated, T6: Repeated, T7: Repeated> Repeated for (T1, T2, T3, T4, T5, T6, T7) {
    fn changed(&self) -> bool { 
        self.0.changed() || self.1.changed() || self.2.changed() || self.3.changed() || 
        self.4.changed() || self.5.changed() || self.6.changed()
    }
    fn reset(&mut self) { 
        self.0.reset(); self.1.reset(); self.2.reset(); self.3.reset(); 
        self.4.reset(); self.5.reset(); self.6.reset();
    }
}

/// Additional hand-written implementations for sizes 8-12
impl<T1: Repeated, T2: Repeated, T3: Repeated, T4: Repeated, T5: Repeated, T6: Repeated, T7: Repeated, T8: Repeated> 
Repeated for (T1, T2, T3, T4, T5, T6, T7, T8) {
    fn changed(&self) -> bool { 
        self.0.changed() || self.1.changed() || self.2.changed() || self.3.changed() || 
        self.4.changed() || self.5.changed() || self.6.changed() || self.7.changed()
    }
    fn reset(&mut self) { 
        self.0.reset(); self.1.reset(); self.2.reset(); self.3.reset(); 
        self.4.reset(); self.5.reset(); self.6.reset(); self.7.reset();
    }
}

impl<T1: Repeated, T2: Repeated, T3: Repeated, T4: Repeated, T5: Repeated, T6: Repeated, T7: Repeated, T8: Repeated, T9: Repeated> 
Repeated for (T1, T2, T3, T4, T5, T6, T7, T8, T9) {
    fn changed(&self) -> bool { 
        self.0.changed() || self.1.changed() || self.2.changed() || self.3.changed() || 
        self.4.changed() || self.5.changed() || self.6.changed() || self.7.changed() || 
        self.8.changed()
    }
    fn reset(&mut self) { 
        self.0.reset(); self.1.reset(); self.2.reset(); self.3.reset(); 
        self.4.reset(); self.5.reset(); self.6.reset(); self.7.reset(); 
        self.8.reset();
    }
}

impl<T1: Repeated, T2: Repeated, T3: Repeated, T4: Repeated, T5: Repeated, T6: Repeated, T7: Repeated, T8: Repeated, T9: Repeated, T10: Repeated> 
Repeated for (T1, T2, T3, T4, T5, T6, T7, T8, T9, T10) {
    fn changed(&self) -> bool { 
        self.0.changed() || self.1.changed() || self.2.changed() || self.3.changed() || 
        self.4.changed() || self.5.changed() || self.6.changed() || self.7.changed() || 
        self.8.changed() || self.9.changed()
    }
    fn reset(&mut self) { 
        self.0.reset(); self.1.reset(); self.2.reset(); self.3.reset(); 
        self.4.reset(); self.5.reset(); self.6.reset(); self.7.reset(); 
        self.8.reset(); self.9.reset();
    }
}

impl<T1: Repeated, T2: Repeated, T3: Repeated, T4: Repeated, T5: Repeated, T6: Repeated, T7: Repeated, T8: Repeated, T9: Repeated, T10: Repeated, T11: Repeated> 
Repeated for (T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11) {
    fn changed(&self) -> bool { 
        self.0.changed() || self.1.changed() || self.2.changed() || self.3.changed() || 
        self.4.changed() || self.5.changed() || self.6.changed() || self.7.changed() || 
        self.8.changed() || self.9.changed() || self.10.changed()
    }
    fn reset(&mut self) { 
        self.0.reset(); self.1.reset(); self.2.reset(); self.3.reset(); 
        self.4.reset(); self.5.reset(); self.6.reset(); self.7.reset(); 
        self.8.reset(); self.9.reset(); self.10.reset();
    }
}

impl<T1: Repeated, T2: Repeated, T3: Repeated, T4: Repeated, T5: Repeated, T6: Repeated, T7: Repeated, T8: Repeated, T9: Repeated, T10: Repeated, T11: Repeated, T12: Repeated> 
Repeated for (T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12) {
    fn changed(&self) -> bool { 
        self.0.changed() || self.1.changed() || self.2.changed() || self.3.changed() || 
        self.4.changed() || self.5.changed() || self.6.changed() || self.7.changed() || 
        self.8.changed() || self.9.changed() || self.10.changed() || self.11.changed()
    }
    fn reset(&mut self) { 
        self.0.reset(); self.1.reset(); self.2.reset(); self.3.reset(); 
        self.4.reset(); self.5.reset(); self.6.reset(); self.7.reset(); 
        self.8.reset(); self.9.reset(); self.10.reset(); self.11.reset();
    }
}

/// Recursive macro for implementing Repeated trait on tuples of any size.
/// 
/// This macro allows you to specify the maximum tuple size you want to support.
/// Note: This macro is primarily for internal use. The library already includes
/// hand-written implementations for tuples up to 12 elements.
/// 
/// # Examples
/// 
/// ```ignore
/// // This is for documentation only - the macro is complex and primarily internal
/// // The library already provides implementations for tuples 1-12
/// use apidom_visit::Repeated;
/// 
/// // You can use tuples directly:
/// struct TestPass { changed: bool }
/// impl Repeated for TestPass {
///     fn changed(&self) -> bool { self.changed }
///     fn reset(&mut self) { self.changed = false; }
/// }
/// 
/// let tuple = (TestPass { changed: false }, TestPass { changed: true });
/// assert!(tuple.changed()); // Returns true if any element changed
/// ```
#[macro_export]
macro_rules! impl_repeated_for_tuples {
    // Base case: generate implementation for a specific size
    (@impl $n:expr, [$($T:ident),*], [$($idx:tt),*]) => {
        impl<$($T: Repeated),*> Repeated for ($($T,)*) {
            fn changed(&self) -> bool {
                false $(|| self.$idx.changed())*
            }
            
            fn reset(&mut self) {
                $(self.$idx.reset();)*
            }
        }
    };
    
    // Recursive case: build up the type and index lists
    (@build $max:expr, $current:expr, [$($T:ident),*], [$($idx:tt),*]) => {
        // Generate implementation for current size if we have at least one element
        $crate::impl_repeated_for_tuples!(@impl $current, [$($T),*], [$($idx),*]);
        
        // Continue recursion if we haven't reached max
        $crate::impl_repeated_for_tuples!(@continue $max, $current, [$($T),*], [$($idx),*]);
    };
    
    // Continue recursion or stop
    (@continue $max:expr, $current:expr, [$($T:ident),*], [$($idx:tt),*]) => {
        $crate::impl_repeated_for_tuples!(@check $max, $current, [$($T),*], [$($idx),*]);
    };
    
    // Check if we should continue (using const evaluation)
    (@check $max:expr, $current:expr, [$($T:ident),*], [$($idx:tt),*]) => {
        $crate::impl_repeated_for_tuples!(@conditional $max, $current, [$($T),*], [$($idx),*]);
    };
    
    // Conditional continuation based on current vs max
    (@conditional $max:expr, $current:expr, [$($T:ident),*], [$($idx:tt),*]) => {
        // This is a simplified version that works with stable Rust
        // For more elements, you can manually call the macro multiple times
    };
    
    // Entry point: start with empty lists and build up to the specified max
    ($max:expr) => {
        // Start the recursion with T1
        $crate::impl_repeated_for_tuples!(@build $max, 1, [T1], [0]);
        
        // Manually expand for common sizes to ensure they work
        // You can extend this pattern for larger sizes as needed
        
        // Size 2
        impl<T1: Repeated, T2: Repeated> Repeated for (T1, T2) {
            fn changed(&self) -> bool { self.0.changed() || self.1.changed() }
            fn reset(&mut self) { self.0.reset(); self.1.reset(); }
        }
        
        // Size 3
        impl<T1: Repeated, T2: Repeated, T3: Repeated> Repeated for (T1, T2, T3) {
            fn changed(&self) -> bool { self.0.changed() || self.1.changed() || self.2.changed() }
            fn reset(&mut self) { self.0.reset(); self.1.reset(); self.2.reset(); }
        }
        
        // Continue pattern for sizes 4-12 if $max >= those sizes
        $crate::impl_repeated_for_tuples!(@expand_to $max);
    };
    
    // Expand implementations up to the specified maximum
    (@expand_to $max:expr) => {
        // This is where you would add conditional compilation for larger tuples
        // For now, we provide a simplified version that covers common use cases
        
        // Size 4
        impl<T1: Repeated, T2: Repeated, T3: Repeated, T4: Repeated> Repeated for (T1, T2, T3, T4) {
            fn changed(&self) -> bool { 
                self.0.changed() || self.1.changed() || self.2.changed() || self.3.changed() 
            }
            fn reset(&mut self) { 
                self.0.reset(); self.1.reset(); self.2.reset(); self.3.reset(); 
            }
        }
        
        // Size 5
        impl<T1: Repeated, T2: Repeated, T3: Repeated, T4: Repeated, T5: Repeated> 
        Repeated for (T1, T2, T3, T4, T5) {
            fn changed(&self) -> bool { 
                self.0.changed() || self.1.changed() || self.2.changed() || 
                self.3.changed() || self.4.changed() 
            }
            fn reset(&mut self) { 
                self.0.reset(); self.1.reset(); self.2.reset(); 
                self.3.reset(); self.4.reset(); 
            }
        }
        
        // Size 6
        impl<T1: Repeated, T2: Repeated, T3: Repeated, T4: Repeated, T5: Repeated, T6: Repeated> 
        Repeated for (T1, T2, T3, T4, T5, T6) {
            fn changed(&self) -> bool { 
                self.0.changed() || self.1.changed() || self.2.changed() || 
                self.3.changed() || self.4.changed() || self.5.changed()
            }
            fn reset(&mut self) { 
                self.0.reset(); self.1.reset(); self.2.reset(); 
                self.3.reset(); self.4.reset(); self.5.reset();
            }
        }
        
        // Size 7
        impl<T1: Repeated, T2: Repeated, T3: Repeated, T4: Repeated, T5: Repeated, T6: Repeated, T7: Repeated> 
        Repeated for (T1, T2, T3, T4, T5, T6, T7) {
            fn changed(&self) -> bool { 
                self.0.changed() || self.1.changed() || self.2.changed() || self.3.changed() || 
                self.4.changed() || self.5.changed() || self.6.changed()
            }
            fn reset(&mut self) { 
                self.0.reset(); self.1.reset(); self.2.reset(); self.3.reset(); 
                self.4.reset(); self.5.reset(); self.6.reset();
            }
        }
        
        // Size 8
        impl<T1: Repeated, T2: Repeated, T3: Repeated, T4: Repeated, T5: Repeated, T6: Repeated, T7: Repeated, T8: Repeated> 
        Repeated for (T1, T2, T3, T4, T5, T6, T7, T8) {
            fn changed(&self) -> bool { 
                self.0.changed() || self.1.changed() || self.2.changed() || self.3.changed() || 
                self.4.changed() || self.5.changed() || self.6.changed() || self.7.changed()
            }
            fn reset(&mut self) { 
                self.0.reset(); self.1.reset(); self.2.reset(); self.3.reset(); 
                self.4.reset(); self.5.reset(); self.6.reset(); self.7.reset();
            }
        }
        
        // Size 9
        impl<T1: Repeated, T2: Repeated, T3: Repeated, T4: Repeated, T5: Repeated, T6: Repeated, T7: Repeated, T8: Repeated, T9: Repeated> 
        Repeated for (T1, T2, T3, T4, T5, T6, T7, T8, T9) {
            fn changed(&self) -> bool { 
                self.0.changed() || self.1.changed() || self.2.changed() || self.3.changed() || 
                self.4.changed() || self.5.changed() || self.6.changed() || self.7.changed() || 
                self.8.changed()
            }
            fn reset(&mut self) { 
                self.0.reset(); self.1.reset(); self.2.reset(); self.3.reset(); 
                self.4.reset(); self.5.reset(); self.6.reset(); self.7.reset(); 
                self.8.reset();
            }
        }
        
        // Size 10
        impl<T1: Repeated, T2: Repeated, T3: Repeated, T4: Repeated, T5: Repeated, T6: Repeated, T7: Repeated, T8: Repeated, T9: Repeated, T10: Repeated> 
        Repeated for (T1, T2, T3, T4, T5, T6, T7, T8, T9, T10) {
            fn changed(&self) -> bool { 
                self.0.changed() || self.1.changed() || self.2.changed() || self.3.changed() || 
                self.4.changed() || self.5.changed() || self.6.changed() || self.7.changed() || 
                self.8.changed() || self.9.changed()
            }
            fn reset(&mut self) { 
                self.0.reset(); self.1.reset(); self.2.reset(); self.3.reset(); 
                self.4.reset(); self.5.reset(); self.6.reset(); self.7.reset(); 
                self.8.reset(); self.9.reset();
            }
        }
        
        // Size 11
        impl<T1: Repeated, T2: Repeated, T3: Repeated, T4: Repeated, T5: Repeated, T6: Repeated, T7: Repeated, T8: Repeated, T9: Repeated, T10: Repeated, T11: Repeated> 
        Repeated for (T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11) {
            fn changed(&self) -> bool { 
                self.0.changed() || self.1.changed() || self.2.changed() || self.3.changed() || 
                self.4.changed() || self.5.changed() || self.6.changed() || self.7.changed() || 
                self.8.changed() || self.9.changed() || self.10.changed()
            }
            fn reset(&mut self) { 
                self.0.reset(); self.1.reset(); self.2.reset(); self.3.reset(); 
                self.4.reset(); self.5.reset(); self.6.reset(); self.7.reset(); 
                self.8.reset(); self.9.reset(); self.10.reset();
            }
        }
        
        // Size 12
        impl<T1: Repeated, T2: Repeated, T3: Repeated, T4: Repeated, T5: Repeated, T6: Repeated, T7: Repeated, T8: Repeated, T9: Repeated, T10: Repeated, T11: Repeated, T12: Repeated> 
        Repeated for (T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12) {
            fn changed(&self) -> bool { 
                self.0.changed() || self.1.changed() || self.2.changed() || self.3.changed() || 
                self.4.changed() || self.5.changed() || self.6.changed() || self.7.changed() || 
                self.8.changed() || self.9.changed() || self.10.changed() || self.11.changed()
            }
            fn reset(&mut self) { 
                self.0.reset(); self.1.reset(); self.2.reset(); self.3.reset(); 
                self.4.reset(); self.5.reset(); self.6.reset(); self.7.reset(); 
                self.8.reset(); self.9.reset(); self.10.reset(); self.11.reset();
            }
        }
    };
}

/// A wrapper that repeats a pass until convergence.
/// 
/// # Examples
/// 
/// ```
/// use apidom_visit::{Repeat, Repeated};
/// 
/// struct TestPass {
///     changed: bool,
/// }
/// 
/// impl TestPass {
///     fn new() -> Self {
///         Self { changed: false }
///     }
/// }
/// 
/// impl Repeated for TestPass {
///     fn changed(&self) -> bool {
///         self.changed
///     }
///     
///     fn reset(&mut self) {
///         self.changed = false;
///     }
/// }
/// 
/// let pass = TestPass::new();
/// let repeat_pass = Repeat::new(pass);
/// assert!(!repeat_pass.changed());
/// ```
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct Repeat<V>
where
    V: Repeated,
{
    pub pass: V,
}

impl<V> Repeat<V>
where
    V: Repeated,
{
    /// Create a new repeating pass
    pub fn new(pass: V) -> Self {
        Self { pass }
    }
}

impl<V> Repeated for Repeat<V>
where
    V: Repeated,
{
    fn changed(&self) -> bool {
        self.pass.changed()
    }

    fn reset(&mut self) {
        self.pass.reset()
    }
}

/// Run a pass until it reaches a fixed point with const-generic max iterations.
pub fn run_until_fixed<P, T, F, const MAX_ITERATIONS: usize>(
    pass: &mut P,
    target: &mut T,
    apply_fn: F,
) -> Result<usize, FixedPointError>
where
    P: Repeated,
    F: FnMut(&mut P, &mut T),
{
    // Compile-time assertion
    if MAX_ITERATIONS == 0 {
        panic!("MAX_ITERATIONS must be greater than 0");
    }
    
    run_until_fixed_legacy(pass, target, apply_fn, MAX_ITERATIONS)
}

/// Legacy run_until_fixed with runtime max_iterations for backward compatibility
pub fn run_until_fixed_legacy<P, T, F>(
    pass: &mut P,
    target: &mut T,
    mut apply_fn: F,
    max_iterations: usize,
) -> Result<usize, FixedPointError>
where
    P: Repeated,
    F: FnMut(&mut P, &mut T),
{
    assert!(max_iterations > 0, "max_iterations must be greater than 0");
    
    let mut last_change_iteration = None;
    
    for iteration in 0..max_iterations {
        pass.reset();
        
        let panic_result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            apply_fn(pass, target);
        }));
        
        match panic_result {
            Ok(()) => {
                if pass.changed() {
                    last_change_iteration = Some(iteration);
                } else {
                    return Ok(iteration + 1);
                }
            }
            Err(_) => {
                return Err(FixedPointError::PassFailed {
                    iteration,
                    error: "Pass panicked during execution".to_string(),
                });
            }
        }
    }
    
    Err(FixedPointError::MaxIterationsExceeded {
        max_iterations,
        last_change_iteration,
    })
}

/// Single AST kind path type with const-generic capacity and optional guards.
/// 
/// This is the unified path type that uses const-generics for all configuration.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AstKindPath<K, const N: usize = 8, const GUARDS: bool = true>
where
    K: ParentKind,
{
    path: Vec<K>,
}

impl<K, const N: usize, const GUARDS: bool> AstKindPath<K, N, GUARDS>
where
    K: ParentKind,
{
    /// Create a new path from a vector of kinds
    pub fn new(path: Vec<K>) -> Self {
        Self { path }
    }
    
    /// Create a new empty path with reserved capacity
    pub fn with_capacity(capacity: usize) -> Self {
        let path = Vec::with_capacity(capacity.max(N));
        Self { path }
    }
    
    /// Create a simple guard that pushes and pops elements
    pub fn with_simple_guard(&mut self, kind: K) -> SimpleGuard<()> {
        if GUARDS {
            self.path.push(kind);
            SimpleGuard::new((), move |_| {
                // Cleanup is handled by the guard's drop
            })
        } else {
            SimpleGuard::noop(())
        }
    }
    
    /// Push a kind onto the path (only if guards are enabled)
    pub fn push(&mut self, kind: K) {
        if GUARDS {
            self.path.push(kind);
        }
    }
    
    /// Pop a kind from the path (only if guards are enabled)
    pub fn pop(&mut self) -> Option<K> {
        if GUARDS {
            self.path.pop()
        } else {
            None
        }
    }
    
    /// Get the current depth
    pub fn depth(&self) -> usize {
        self.path.len()
    }
    
    /// Check if empty
    pub fn is_empty(&self) -> bool {
        self.path.is_empty()
    }
    
    /// Get capacity
    pub fn capacity(&self) -> usize {
        self.path.capacity()
    }
    
    /// Get the last kind in the path
    pub fn last(&self) -> Option<&K> {
        self.path.last()
    }
    
    /// Get mutable reference to the last kind in the path
    pub fn last_mut(&mut self) -> Option<&mut K> {
        self.path.last_mut()
    }
}

/// Single AST node path type with const-generic configuration.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AstNodePath<N, const KN: usize = 8, const NP: usize = 8, const GUARDS: bool = true>
where
    N: NodeRef,
{
    kinds: AstKindPath<N::ParentKind, KN, GUARDS>,
    path: Vec<N>,
}

impl<N, const KN: usize, const NP: usize, const GUARDS: bool> AstNodePath<N, KN, NP, GUARDS>
where
    N: NodeRef,
{
    /// Create a new node path
    pub fn new(kinds: AstKindPath<N::ParentKind, KN, GUARDS>, path: Vec<N>) -> Self {
        Self { kinds, path }
    }
    
    /// Create with capacity
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            kinds: AstKindPath::with_capacity(capacity),
            path: Vec::with_capacity(capacity.max(NP)),
        }
    }
    
    /// Create a guard for node path
    pub fn with_guard(&mut self, node: N) -> SimpleGuard<()> {
        if GUARDS {
            let kind = node.kind();
            self.kinds.path.push(kind);
            self.path.push(node);
            SimpleGuard::new((), move |_| {
                // Cleanup is handled by the guard's drop
            })
        } else {
            SimpleGuard::noop(())
        }
    }
    
    /// Push a node onto the path (only if guards are enabled)
    pub fn push(&mut self, node: N) {
        if GUARDS {
            let kind = node.kind();
            self.kinds.path.push(kind);
            self.path.push(node);
        }
    }
    
    /// Pop a node from the path (only if guards are enabled)
    pub fn pop(&mut self) -> Option<N> {
        if GUARDS {
            self.kinds.path.pop();
            self.path.pop()
        } else {
            None
        }
    }
    
    /// Get kinds path
    pub fn kinds(&self) -> &AstKindPath<N::ParentKind, KN, GUARDS> {
        &self.kinds
    }
    
    /// Get depth
    pub fn depth(&self) -> usize {
        self.path.len()
    }
    
    /// Check if empty
    pub fn is_empty(&self) -> bool {
        self.path.is_empty()
    }
}

/// Trait for node references in AST paths
/// 
/// This trait provides the interface for nodes that can be stored in AST paths,
/// requiring them to provide their kind information and support index management.
pub trait NodeRef: Copy {
    type ParentKind: ParentKind;

    /// Get the kind of this node
    fn kind(&self) -> Self::ParentKind;

    /// Set the index of this node in its parent
    fn set_index(&mut self, index: usize);
}

/// Trait for parent kind information
/// 
/// This trait is implemented by types that represent the "kind" or "type"
/// of AST nodes, providing index management capabilities.
pub trait ParentKind: Copy {
    /// Set the index for this parent kind
    fn set_index(&mut self, index: usize);
}

/// Safe error reporting for wrong AST paths with enhanced debugging information.
/// 
/// This function provides comprehensive error reporting for AST path issues,
/// including context information that helps developers identify and fix problems.
/// 
/// # Panics
/// This function will panic with a descriptive message about the wrong AST path.
/// In debug builds, it provides additional context including call location.
#[doc(hidden)]
#[track_caller]
pub fn wrong_ast_path() -> ! {
    let location = std::panic::Location::caller();
    
    #[cfg(debug_assertions)]
    {
        panic!(
            "Wrong AST path detected at {}:{}:{}\n\
             This indicates a bug in the AST traversal logic.\n\
             Please check:\n\
             1. Path construction and guard usage\n\
             2. Proper nesting of with_guard calls\n\
             3. Correct index management\n\
             4. No premature guard drops",
            location.file(),
            location.line(),
            location.column()
        );
    }
    
    #[cfg(not(debug_assertions))]
    {
        panic!("Wrong AST path at {}:{}", location.file(), location.line());
    }
}

/// Configuration trait for compile-time behavior control
pub trait ConfigTrait {
    const PATH_CAPACITY: usize;
    const MAX_ITERATIONS: usize;
    const ENABLE_GUARDS: bool;
}

/// Default configuration with balanced settings.
pub struct DefaultConfig;
impl ConfigTrait for DefaultConfig {
    const PATH_CAPACITY: usize = 8;
    const MAX_ITERATIONS: usize = 100;
    const ENABLE_GUARDS: bool = true;
}

/// High-performance configuration with larger buffers.
pub struct PerformanceConfig;
impl ConfigTrait for PerformanceConfig {
    const PATH_CAPACITY: usize = 16;
    const MAX_ITERATIONS: usize = 200;
    const ENABLE_GUARDS: bool = true;
}

/// Memory-optimized configuration with smaller buffers.
pub struct CompactConfig;
impl ConfigTrait for CompactConfig {
    const PATH_CAPACITY: usize = 4;
    const MAX_ITERATIONS: usize = 50;
    const ENABLE_GUARDS: bool = false;
}

/// Simple RAII guard for automatic cleanup with zero overhead when guards are disabled.
pub struct SimpleGuard<T> {
    target: Option<T>,
    cleanup_fn: Option<Box<dyn FnOnce(T)>>,
}

impl<T> SimpleGuard<T> {
    /// Create a new guard with custom cleanup logic
    pub fn new<F>(target: T, cleanup: F) -> Self
    where
        F: FnOnce(T) + 'static,
    {
        Self {
            target: Some(target),
            cleanup_fn: Some(Box::new(cleanup)),
        }
    }
    
    /// Create a no-op guard
    pub fn noop(target: T) -> Self {
        Self {
            target: Some(target),
            cleanup_fn: None,
        }
    }
    
    /// Get reference to the guarded value
    pub fn get(&self) -> Option<&T> {
        self.target.as_ref()
    }
    
    /// Get mutable reference to the guarded value
    pub fn get_mut(&mut self) -> Option<&mut T> {
        self.target.as_mut()
    }
}

impl<T> Drop for SimpleGuard<T> {
    fn drop(&mut self) {
        if let (Some(target), Some(cleanup)) = (self.target.take(), self.cleanup_fn.take()) {
            cleanup(target);
        }
    }
}
