fn main() {
    about_unused_variable();
    about_bind_variability_shadowing_scope();
    about_deconstruction();
}

/**
 * 未使用变量 & 格式化占位符
 * 注意函数名命名格式 a_b_c
 * 未使用变量通过_variable声明，避免warning
 * 关于 {} (显示格式化占位符) 和 {:?} (调试打印格式化占位符) 的区别：
 * {} 输出方式取决于类型实现的 std::fmt::Display trait
 * {:?}基于 Debug trait 实现的，用于将一个值格式化为可供调试输出的形式。对于 Rust 内建类型，一般会以比较友好的方式输出；
 * 可以通过 #[derive(Debug)] 来手动实现Debug trait，继而可以使用{:?}
 */
fn about_unused_variable() {
    let x: i32 = 2;
    let _y: i32 = 4;
    println!("x is {}", x);
    println!("x is {:?}", x);
    let s = String::from("hello");
    println!("Value of s is {}", s); // hello 可以看出，使用 {} 占位符输出时，字符串 s 的引号被去掉了。它以更加自然的形式进行输出
    println!("Value of s is {:?}", s); // "hello"
    let v = vec![1, 2, 3];
    println!("Value of v is {:?}", v);
    // println!("Value of v is {}", v); // error:因为Vec类型没有实现`std::fmt::Display`

    #[derive(Debug)] // 手动实现Debug trait

    struct Person {
        name: String,
        age: i32,
    }
    let nick = Person {
        name: "nick".to_string(),
        age: 12,
    };
    // println!("nick is {}", Nick); // error:因为Person没有实现`std::fmt::Display`
    println!("nick is {:?}", nick); // 如果没有 #[derive(Debug)] 则 error:因为Person没有实现`Debug`
                                    // 以下结构体.xx的输出直接可以使用,因为它们所代表的类型都默认实现了Display和Debug
    println!("nick name is {}", nick.name);
    println!("nick age is {}", nick.age);
    println!("nick name is {:?}", nick.name);
    println!("nick age is {:?}", nick.age);
}

/**
 * 变量绑定 & 变量可见性 & 变量遮蔽 & 变量作用域
 */
fn about_bind_variability_shadowing_scope() {
    let x: i32 = 1; // 将i32类型的1绑定到变量x上
                    // x = 3; error:因为x不是可变的
    let mut y: i32 = 1;
    // y = "e"; // error:需要注意 mut的可变 意思是值的可变而不是类型的可变，一旦确定了类型，只能在这个类型范围内可变
    y = 3;
    y += 1;
    println!("y is {}", y); // 4
    let mut z: i32 = 1; // warning：这个变量z未被使用 因为被下面的同名变量z遮蔽了
    let mut z = "xx";
    // {} 为作用域 作用域内的变量只在当前作用域内生效，不会影响到作用域外
    {
        let mut z = 100;
        println!("scope z is {}", z); // 100
    }
    println!("z is {}", z); // xx
}

/**
 * 变量解构 & 解构赋值
 */
fn about_deconstruction() {
    let (mut x, y) = (1, 2);
    x = 4;
    println!("x is {}", x); // 4
    println!("y is {}", y); // 2
    let (a, b): (i32, i32);
    // (a, _) = (1, 2); // 也可以 但是2没有意义
    // (a, _) = (1, _); // error:_只能用在左边
    (a, _) = (1, ..);
    println!("a is {}", a); // 1
    [_, b] = [1, 3];
    println!("b is {}", b); // 3
    let (x, y, z): (i32, i32, i32);
    (x, ..) = (1, ..);
    println!("x is {}", x); // 1
}
