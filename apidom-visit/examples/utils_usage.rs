use apidom_visit::utils::{Map, MoveMap, ExtendedMoveMap, safe};
use std::rc::Rc;
use std::sync::Arc;
use std::collections::{VecDeque, LinkedList};

fn main() {
    println!("=== ApiDOM Utils 优化功能演示 ===\n");

    // 1. Box<T> Map 优化 - 使用 ManuallyDrop 防止内存泄漏
    println!("1. Box<T> Map 优化 (ManuallyDrop):");
    let boxed_number = Box::new(42);
    let doubled = boxed_number.map(|x| x * 2);
    println!("   Box::new(42).map(|x| x * 2) = {}", *doubled);

    let boxed_string = Box::new("Hello".to_string());
    let greeting = boxed_string.map(|s| s + ", World!");
    println!("   Box::new(\"Hello\").map(|s| s + \", World!\") = {}", *greeting);

    // 2. Rc<T> 和 Arc<T> 支持
    println!("\n2. Rc<T> 和 Arc<T> 支持:");
    let rc_value = Rc::new(100);
    let rc_result = rc_value.map(|x| x / 2);
    println!("   Rc::new(100).map(|x| x / 2) = {}", *rc_result);

    let arc_value = Arc::new(200);
    let arc_result = arc_value.map(|x| x + 50);
    println!("   Arc::new(200).map(|x| x + 50) = {}", *arc_result);

    // 3. Vec 优化的 move_map
    println!("\n3. Vec 优化的 move_map (VecTransformer):");
    let numbers = vec![1, 2, 3, 4, 5];
    let squares = numbers.move_map(|x| x * x);
    println!("   vec![1,2,3,4,5].move_map(|x| x * x) = {:?}", squares);

    let words = vec!["rust".to_string(), "is".to_string(), "awesome".to_string()];
    let uppercase = words.move_map(|s| s.to_uppercase());
    println!("   words.move_map(|s| s.to_uppercase()) = {:?}", uppercase);

    // 4. Vec 优化的 move_flat_map - 预分配空间，避免 insert
    println!("\n4. Vec 优化的 move_flat_map (预分配 + extend):");
    let numbers = vec![1, 2, 3];
    let expanded = numbers.move_flat_map(|x| vec![x, x * 10]);
    println!("   vec![1,2,3].move_flat_map(|x| vec![x, x*10]) = {:?}", expanded);

    let words = vec!["ab".to_string(), "cd".to_string()];
    let chars: Vec<String> = words.move_flat_map(|s| {
        s.chars().map(|c| c.to_string()).collect::<Vec<_>>()
    });
    println!("   words.move_flat_map(chars) = {:?}", chars);

    // 5. 其他容器类型支持
    println!("\n5. 其他容器类型支持 (ExtendedMoveMap):");
    let mut deque = VecDeque::new();
    deque.extend(vec![1, 2, 3]);
    let deque_result = deque.extended_move_flat_map(|x| vec![x, x + 10]);
    println!("   VecDeque[1,2,3].extended_move_flat_map(|x| vec![x, x+10]) = {:?}", deque_result);

    let mut list = LinkedList::new();
    list.extend(vec!["a".to_string(), "b".to_string()]);
    let list_result = list.extended_move_flat_map(|s| vec![s.clone(), s + "!"]);
    println!("   LinkedList[\"a\",\"b\"].extended_move_flat_map(duplicate) = {:?}", list_result);

    // 6. 安全版本 - 恐慌处理
    println!("\n6. 安全版本 (恐慌处理):");
    let safe_box = Box::new(10);
    let safe_result = safe::safe_box_map(safe_box, |x| x * 5);
    match safe_result {
        Ok(result) => println!("   safe_box_map 成功: {}", *result),
        Err(_) => println!("   safe_box_map 失败"),
    }

    let safe_vec = vec![1, 2, 3];
    let safe_vec_result = safe::safe_move_map(safe_vec, |x| x + 100);
    match safe_vec_result {
        Ok(result) => println!("   safe_move_map 成功: {:?}", result),
        Err(_) => println!("   safe_move_map 失败"),
    }

    // 演示恐慌处理
    println!("\n7. 恐慌处理演示:");
    let panic_vec = vec![1, 2, 3];
    let panic_result = safe::safe_move_map(panic_vec, |x| {
        if x == 2 {
            panic!("故意触发恐慌进行测试");
        }
        x * 2
    });
    match panic_result {
        Ok(result) => println!("   意外成功: {:?}", result),
        Err(_) => println!("   ✓ 成功捕获恐慌并返回错误"),
    }

    // 8. 性能对比演示
    println!("\n8. 性能对比演示:");
    use std::time::Instant;
    
    let large_vec: Vec<i32> = (0..10000).collect();
    
    // 优化版本
    let start = Instant::now();
    let result1 = large_vec.clone().move_flat_map(|x| {
        if x % 2 == 0 { vec![x] } else { vec![] }
    });
    let duration1 = start.elapsed();
    
    // 标准版本
    let start = Instant::now();
    let result2: Vec<i32> = large_vec.into_iter()
        .filter(|&x| x % 2 == 0)
        .collect();
    let duration2 = start.elapsed();
    
    println!("   优化版本耗时: {:?}", duration1);
    println!("   标准版本耗时: {:?}", duration2);
    println!("   结果一致性: {}", result1 == result2);
    println!("   结果长度: {}", result1.len());

    // 9. 复杂嵌套示例
    println!("\n9. 复杂嵌套示例:");
    let nested_data = vec![
        vec!["hello".to_string(), "world".to_string()],
        vec!["rust".to_string(), "lang".to_string()],
    ];
    
    let flattened: Vec<String> = nested_data.into_iter()
        .flatten()
        .collect::<Vec<_>>()
        .move_map(|s| s.to_uppercase());
    
    println!("   嵌套数据扁平化并转大写: {:?}", flattened);

    // 10. 内存安全演示
    println!("\n10. 内存安全演示:");
    let large_data = Box::new(vec![0; 1000]);
    let processed = large_data.map(|mut v| {
        v.iter_mut().enumerate().for_each(|(i, val)| *val = i);
        v
    });
    println!("   大数据处理完成，前5个元素: {:?}", &processed[0..5]);
    println!("   ✓ 无内存泄漏，使用 ManuallyDrop 保证安全");

    println!("\n=== 演示完成 ===");
} 