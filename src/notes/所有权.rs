fn main() {
    test_one();
    test_two();
}
fn test_one() {
    // 使用尽可能多的方法来通过编译
    let x = String::from("hello, world"); // 变量x进入作用域，String类型的hello world被绑定到变量x上
                                          // let x = "hello,world";
    let y = x; // y进入作用域，x被绑定到y上，x失效
               // 2.  let y = x.clone();
               // 3. let y = x.as_str();
               // println!("{},{}", x, y); // error : 这里x无法被输出，y可以输出
} // y离开作用域被drop x因为已经move到了y，因此什么都不会操作
fn test_two() {
    let s = give_ownership();
    println!("{}", s);
}

// 只能修改下面的代码!
fn give_ownership() -> String {
    let s = String::from("hello, world");
    // convert String to Vec
    // 将 String 转换成 Vec 类型
    // let _s = s.into_bytes(); // error : into_bytes将s的所有权转移到了_s，接下来s失效，应当使用as_bytes
    let _s = s.as_bytes();
    s
}
