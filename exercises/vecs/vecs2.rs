// vecs2.rs
// A Vec of even numbers is given. Your task is to complete the loop
// so that each number in the Vec is multiplied by 2.
//
// Make me pass the test!
//
// Execute `rustlings hint vecs2` or use the `hint` watch subcommand for a hint.

fn vec_loop(mut v: Vec<i32>) -> Vec<i32> {
    // 迭代器遍历Vector
    for i in v.iter_mut() {
        // TODO: Fill this up so that each element in the Vec `v` is
        // multiplied by 2.
        *i *= 2;
    }
    // At this point, `v` should be equal to [4, 8, 12, 16, 20].
    v
}

fn vec_map(v: &Vec<i32>) -> Vec<i32> {
    /*
     * .map()方法被用于对集合类型进行转换操作
     * 调用这个方法时，可以传入一个闭包 (Closure) 作为参数，用于对集合中的每个元素进行转换
     * .map() 方法返回一个新的迭代器，其中包含了对每个元素进行转换后的值
     * 这个新的迭代器可以继续被链式调用，或者使用 .collect() 方法转换成一个集合
     */
    v.iter().map(|num| {
        // TODO: Do the same thing as above - but instead of mutating the
        // Vec, you can just return the new number!
        num * 2
    }).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vec_loop() {
        let v: Vec<i32> = (1..).filter(|x| x % 2 == 0).take(5).collect();
        let ans = vec_loop(v.clone());

        assert_eq!(ans, v.iter().map(|x| x * 2).collect::<Vec<i32>>());
    }

    #[test]
    fn test_vec_map() {
        let v: Vec<i32> = (1..).filter(|x| x % 2 == 0).take(5).collect();
        let ans = vec_map(&v);

        assert_eq!(ans, v.iter().map(|x| x * 2).collect::<Vec<i32>>());
    }
}
