use apidom_visit::{
    All, OptionalVisitor, Repeated, Repeat, run_until_fixed, run_until_fixed_legacy, FixedPointError,
    AstKindPath, NodeRef, ParentKind
};

// Example implementations for demonstration
#[derive(Debug, Clone, Copy, PartialEq)]
struct DemoKind {
    name: &'static str,
    index: usize,
}

impl ParentKind for DemoKind {
    fn set_index(&mut self, index: usize) {
        self.index = index;
    }
}

#[derive(Debug, Clone, Copy)]
struct DemoNode {
    kind: DemoKind,
    #[allow(dead_code)]
    data: i32,
}

impl NodeRef for DemoNode {
    type ParentKind = DemoKind;
    
    fn kind(&self) -> Self::ParentKind {
        self.kind
    }
    
    fn set_index(&mut self, index: usize) {
        self.kind.set_index(index);
    }
}

// Simple pass that implements Repeated
#[derive(Debug)]
struct SimplifyPass {
    changed: bool,
}

impl SimplifyPass {
    fn new() -> Self {
        Self { changed: false }
    }
}

impl Repeated for SimplifyPass {
    fn changed(&self) -> bool {
        self.changed
    }
    
    fn reset(&mut self) {
        self.changed = false;
    }
}

fn main() {
    println!("🚀 ApiDOM Rust Optimizations Demo");
    println!("==================================\n");

    // 1. Zero-overhead Configuration with const-generics
    println!("1. Zero-overhead Const-Generic Configuration:");
    
    // Different path configurations using const-generics
    let _default_path: AstKindPath<DemoKind, 8, true> = AstKindPath::with_capacity(4);
    let _perf_path: AstKindPath<DemoKind, 16, true> = AstKindPath::with_capacity(4);
    let _compact_path: AstKindPath<DemoKind, 4, false> = AstKindPath::with_capacity(4);
    
    println!("   ✓ Default path (capacity: 8, guards: enabled)");
    println!("   ✓ Performance path (capacity: 16, guards: enabled)");
    println!("   ✓ Compact path (capacity: 4, guards: disabled)");
    println!("   ✓ All configurations compiled with zero runtime overhead");

    // 2. Simplified OptionalVisitor API with generic conditions
    println!("\n2. Simplified OptionalVisitor API:");
    let enabled_visitor = OptionalVisitor::enabled("enabled_visitor");
    let disabled_visitor: OptionalVisitor<&str> = OptionalVisitor::disabled();
    let high_priority_visitor = OptionalVisitor::high_priority("priority_visitor");
    
    // Conditional visitor with custom closure
    let conditional_visitor = OptionalVisitor::conditional("conditional_visitor", || true);
    
    println!("   Enabled visitor active: {}", enabled_visitor.is_active());
    println!("   Disabled visitor active: {}", disabled_visitor.is_active());
    println!("   High priority visitor priority: {:?}", high_priority_visitor.priority());
    println!("   Conditional visitor active: {}", conditional_visitor.is_active());

    // 3. Enhanced Tuple Support
    println!("\n3. Enhanced Tuple Support:");
    
    // Test tuples of different sizes (now supporting up to 12 elements)
    let tuple_1 = (SimplifyPass::new(),);
    let tuple_2 = (SimplifyPass::new(), SimplifyPass::new());
    let tuple_3 = (SimplifyPass::new(), SimplifyPass::new(), SimplifyPass::new());
    let tuple_4 = (SimplifyPass::new(), SimplifyPass::new(), SimplifyPass::new(), SimplifyPass::new());
    let tuple_5 = (SimplifyPass::new(), SimplifyPass::new(), SimplifyPass::new(), SimplifyPass::new(), SimplifyPass::new());
    let tuple_6 = (SimplifyPass::new(), SimplifyPass::new(), SimplifyPass::new(), SimplifyPass::new(), SimplifyPass::new(), SimplifyPass::new());
    let tuple_7 = (SimplifyPass::new(), SimplifyPass::new(), SimplifyPass::new(), SimplifyPass::new(), SimplifyPass::new(), SimplifyPass::new(), SimplifyPass::new());
    let tuple_8 = (SimplifyPass::new(), SimplifyPass::new(), SimplifyPass::new(), SimplifyPass::new(), SimplifyPass::new(), SimplifyPass::new(), SimplifyPass::new(), SimplifyPass::new());
    let tuple_9 = (SimplifyPass::new(), SimplifyPass::new(), SimplifyPass::new(), SimplifyPass::new(), SimplifyPass::new(), SimplifyPass::new(), SimplifyPass::new(), SimplifyPass::new(), SimplifyPass::new());
    let tuple_10 = (SimplifyPass::new(), SimplifyPass::new(), SimplifyPass::new(), SimplifyPass::new(), SimplifyPass::new(), SimplifyPass::new(), SimplifyPass::new(), SimplifyPass::new(), SimplifyPass::new(), SimplifyPass::new());
    let tuple_11 = (SimplifyPass::new(), SimplifyPass::new(), SimplifyPass::new(), SimplifyPass::new(), SimplifyPass::new(), SimplifyPass::new(), SimplifyPass::new(), SimplifyPass::new(), SimplifyPass::new(), SimplifyPass::new(), SimplifyPass::new());
    let tuple_12 = (SimplifyPass::new(), SimplifyPass::new(), SimplifyPass::new(), SimplifyPass::new(), SimplifyPass::new(), SimplifyPass::new(), SimplifyPass::new(), SimplifyPass::new(), SimplifyPass::new(), SimplifyPass::new(), SimplifyPass::new(), SimplifyPass::new());
    
    println!("   Tuple (1): changed = {}", tuple_1.changed());
    println!("   Tuple (2): changed = {}", tuple_2.changed());
    println!("   Tuple (3): changed = {}", tuple_3.changed());
    println!("   Tuple (4): changed = {}", tuple_4.changed());
    println!("   Tuple (5): changed = {}", tuple_5.changed());
    println!("   Tuple (6): changed = {}", tuple_6.changed());
    println!("   Tuple (7): changed = {}", tuple_7.changed());
    println!("   Tuple (8): changed = {}", tuple_8.changed());
    println!("   Tuple (9): changed = {}", tuple_9.changed());
    println!("   Tuple (10): changed = {}", tuple_10.changed());
    println!("   Tuple (11): changed = {}", tuple_11.changed());
    println!("   Tuple (12): changed = {}", tuple_12.changed());
    println!("   Note: Hand-written implementations support up to 12 elements");

    // 4. Enhanced run_until_fixed with const-generic and legacy versions
    println!("\n4. Enhanced run_until_fixed:");
    
    let mut pass = SimplifyPass::new();
    let mut target = DemoNode {
        kind: DemoKind { name: "root", index: 0 },
        data: 42,
    };
    
    // Const-generic version with compile-time max iterations
    match run_until_fixed::<_, _, _, 10>(&mut pass, &mut target, |p, _t| { p.changed = false; }) {
        Ok(iterations) => println!("   ✅ Const-generic version converged in {} iterations", iterations),
        Err(e) => println!("   ❌ Error: {:?}", e),
    }
    
    // Legacy version with runtime max iterations
    match run_until_fixed_legacy(&mut pass, &mut target, |p, _t| { p.changed = true; }, 3) {
        Ok(iterations) => println!("   ✅ Legacy version converged in {} iterations", iterations),
        Err(FixedPointError::MaxIterationsExceeded { max_iterations, last_change_iteration }) => {
            println!("   ⚠️  Max iterations ({}) exceeded, last change at iteration {}", 
                     max_iterations, last_change_iteration.unwrap_or(0));
        }
        Err(FixedPointError::PassFailed { iteration, error }) => {
            println!("   ❌ Pass failed at iteration {}: {}", iteration, error);
        }
    }

    // 5. Optimized Path Usage with const-generics
    println!("\n5. Optimized Path Usage:");
    let mut path: AstKindPath<DemoKind, 8, true> = AstKindPath::new(vec![]);
    println!("   Path initialized with capacity: {}", path.capacity());
    
    // Test push/pop with guards enabled
    let demo_kind = DemoKind { name: "test", index: 0 };
    path.push(demo_kind);
    println!("   After push, depth: {}", path.depth());
    
    let popped = path.pop();
    println!("   After pop, depth: {}, popped: {:?}", path.depth(), popped.is_some());

    // 6. All and Repeat Wrappers
    println!("\n6. All and Repeat Wrappers:");
    let _all_visitor = All { visitor: enabled_visitor };
    let _repeat_visitor = Repeat::new(SimplifyPass::new());
    
    println!("   All wrapper created successfully");
    println!("   Repeat wrapper created successfully");

    println!("\n🎯 Summary of Key Improvements:");
    println!("   • True zero-overhead with const-generics");
    println!("   • Generic conditional visitors with compile-time optimization");
    println!("   • Compile-time capacity configuration for paths");
    println!("   • Simplified tuple implementations (1-12 elements)");
    println!("   • Enhanced error handling with detailed context");
    println!("   • Extensible design for future optimizations");
} 