use std::{
    fmt::{format, Debug, Display},
    iter::Sum,
};

/*
特征 Trait ： 定义特征是把一些方法组合在一起，目的是定义一个实现某些目标所必需的行为的集合。
特征只定义行为看起来是什么样的，而不定义行为具体是怎么样的。有点像solidity里的接口语法。
定义某个特征的语法示例：
trait Summary {
    fn summarize(&self) -> String;
}
接下来，每一个实现这个特征的类型都需要具体实现该特征的相应方法。当然也可以定义默认实现的方法，其他类型可以选择使用默认或者重载。
孤儿规则：如果你想要为类型 A 实现特征 T，那么 A 或者 T 至少有一个是在当前作用域中定义的。可以确保其它人编写的代码不会破坏你的代码，也确保了你不会莫名其妙就破坏了风马牛不相及的代码。
用处1:（我觉得用处不大的地方） 为不同的结构体实现相同的行为特征。方法同样能做到 确实没什么卵用。
用处2:特征可以作为函数的参数和返回值。
 */
struct Post {
    title: String,
    author: String,
    content: String,
}
struct Weibo {
    user: String,
    content: String,
}
struct Unknown {
    x: u8,
}

trait Summary {
    fn summarize(&self) -> String;
    // 这是特征的默认实现，具有这个特征的类型默认可以使用这个函数而不需要在impl for中实现，当然也可以对这个方法进行重载。注意默认实现即便没用到self也需要传入这个参数。
    fn defaultImpl(&self) {
        println!("默认实现")
    }
}
impl Summary for Post {
    fn summarize(&self) -> String {
        format!("文章标题{},作者{}", self.title, self.author)
    }
}
impl Summary for Weibo {
    fn summarize(&self) -> String {
        format!("微博用户{},内容{}", self.user, self.content)
    }
}
fn main() {
    let post = Post {
        title: "测试文章".to_string(),
        author: "测试作者".to_string(),
        content: "测试内容".to_string(),
    };
    let weibo = Weibo {
        user: "测试用户".to_string(),
        content: "测试内容weibo".to_string(),
    };
    // 用处1
    let sum1 = post.summarize();
    let sum2 = weibo.summarize();
    println!("sum1 is {},sum2 is {}", sum1, sum2); // sum1 is 文章标题测试文章,作者测试作者,sum2 is 微博用户测试用户,内容测试内容weibo
    let _unknown = Unknown { x: 1 };
    post.defaultImpl(); // 默认实现
    notify(&weibo); // 特征作为函数参数 微博用户测试用户,内容测试内容weibo
                    // notify(&_unknown) // error : 因为Unknown结构体没有实现Summary特征
    notify6();
    let _struct = returns_summarizable();
}
/*
用处2:特征可以作为函数的参数和返回值。
涉及到特征约束的概念：特征约束，可以让我们在指定类型 + 指定特征的条件下去实现方法。如果没有特征约束，只能指定类型实现方法。
item: impl Summary 可以理解为 实现了(impl) Summary 特征 的 item 作为参数。你可以使用任何实现了 Summary 特征的类型作为该函数的参数，同时在函数体内，还可以调用该特征的方法。实际上这种写法是语法糖，完整写法是：
fn notify<T:Summary>(item: &T)   --- 形如 T: Summary 被称为特征约束。
impl 特征xx这种语法糖在简单场景下使用是没问题的，但是考虑一个场景，多个参数都需要是实现了某一特征的同一类型(因为实现某一特征的类型可以是不同的)，那么此时语法糖就无法起到有效的约束效果，这时候就只能使用特征约束的写法。
eg: pub fn notify(item1: &impl Summary, item2: &impl Summary) {}  ---  这种写法item1和item2可以是不同的类型
pub fn notify<T: Summary>(item1: &T, item2: &T) {}  ---  这种写法item1和item2必须是同一类型T
 */
fn notify(item: &impl Summary) {
    println!("特征作为函数参数 {}", item.summarize())
}

fn notify2<T: Summary>(item1: &T, item2: &T) {} // 特征约束
fn notify3<T: Summary + Display>(item: &T) {} // 多重约束 完整写法
fn notify4(item: &(impl Summary + Display)) {} // 多重约束 语法糖

// 再考虑一个问题，多重约束比较多的时候，这种完整写法显得冗余。这时候可以使用where约束
fn notify5<T, U, W, V>(t: &T, u: &U, w: &W, v: &V)
where
    T: Display,
    U: Summary + Copy,
    W: Clone,
    V: Debug,
{
}
struct Pair<T> {
    x: T,
    y: T,
}
impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}
// 只有T实现了Summary特征的Pair类型可以调用test方法
impl<T: Summary> Pair<T> {
    fn test(&self) {
        println!("T实现了Summary的Pair")
    }
}

fn notify6() {
    let pair1 = Pair { x: 1, y: 2 };
    let pair2 = Pair {
        x: Weibo {
            user: "gray".to_string(),
            content: "xxxxx".to_string(),
        },
        y: Weibo {
            user: "mike".to_string(),
            content: "asasa".to_string(),
        },
    };
    // pair1.test(); // error pair1的T没有实现Summary
    pair2.test() // "T实现了Summary的Pair"
}

/*
特征作为函数返回值。这种形式只代表这个函数的返回值实现了Summary特征，并不能具体约束返回的类型，因为实现Summary特征的类型有可能不同，比如在这里就有Weibo和Post两种类型。
使用场景在于当我们不知道返回值的具体类型，也许可能是因为这个返回值的类型实在是太复杂，只有编译器能搞明白，比如闭包和迭代器的类型。好在你可以用 impl Iterator 来告诉调用者，返回了一个迭代器，因为所有迭代器都会实现 Iterator 特征。
但是这种返回值方式有一个很大的限制：只能有一个具体的类型。一旦首先匹配到了一个类型了，不能再更改。比如如果你想在返回值里使用if else来分别返回了实现这个特征的不同类型是不允许的。
 */
fn returns_summarizable() -> impl Summary {
    Weibo {
        user: String::from("gray"),
        content: String::from("m2 pro太厉害了,电脑再也不会卡"),
    }
}
