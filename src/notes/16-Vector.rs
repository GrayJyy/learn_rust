fn main() {
    // 创建动态数组方法1: Vec::new()
    // 显示指定vec元素类型
    let mut vec1: Vec<i32> = Vec::new();
    vec1.push(1);
    // 隐式推导
    let mut vec2 = Vec::new();
    vec2.push('a');
    // 创建动态数组方法2: Vec！
    // 能在创建同时给予初始化值,也无需显式指定类型
    let mut vec3 = vec![1, 2, 3];
    vec3.push(4);
    println!("vec3 is {:?}", vec3);
    println!("last item is {}", vec3[2]); // 3
                                          // 获取vec中的某个元素方法：1.[index]下标直接获取元素的值，会有越界问题。get(index)方法，获取Option枚举，越界也会返回None
                                          // println!("last item is {}", vec3[100]); // error 越界
    println!("last item is {:?}", vec3.get(2)); // Some(3)
    println!("last item is {:?}", vec3.get(100)); // None
                                                  // 需要注意的所有权问题
    let mut vec4 = vec![1, 2, 3];
    let v = &vec4[0]; // 创建不可变引用
                      // vec4.push(3); // 创建可变引用
    println!("v is {v}"); // 注释掉上面的可变引用才不会出问题
                          // 如果事先知道需要存储的元素的个数：
                          // 通过with_capacity创建的vec，如果在容量以内，每次操作不会重新分配内存性能更好，只有超出容量以后才会重新分配
                          // 并不是说with_capacity(n),n是多少创建的初始vec长度就是多少，初始也是0。也并不是说长度就被限制在n了。
    let mut vec: Vec<i32> = Vec::with_capacity(3);
    println!("vec is {:?}", vec.len()); // 0
                                        // extend方法扩容
    vec.extend([1, 2, 3]);
    println!("vec is {:?}", vec.len()); // 3
                                        // 将其他类型转化为Vec类型 vec::from() into()
    let a = String::from("aaa");
    let b = Vec::from(a);
    let x = String::from("aaa");
    let y = x.into_bytes();
    println!("y is {:?}", y); // y is [97, 97, 97]
    println!("b is {:?}", b); // b is [97, 97, 97]
    let arr = [1, 2, 3];
    // into() 必须指定变量类型
    let c: Vec<i32> = arr.into(); // c is [1, 2, 3]
    println!("c is {:?}", c);
}
