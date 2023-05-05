/*
场景:现在在做一款游戏，需要将多个对象渲染在屏幕上，这些对象属于不同的类型，存储在列表中，渲染的时候，需要循环该列表并顺序渲染每个对象，在 Rust 中该怎么实现？
本质是存储不同类型的元素，可以通过枚举实现，但是枚举有个问题，就是需要事先知道所有的穷尽类型，后续如果需要新增或者改动的话，那还需要重构这个枚举。局限性较大。再考虑到范型T的使用，也存在问题，一旦确定了T就不可再变，所有类型就都成了T的某一个具体类型。这时候就引入了特征对象的概念。
1.通过特征对象实现
可以通过 & 引用或者 Box<T> 智能指针的方式来创建特征对象。
dyn 关键字只用在特征对象的类型声明上，在创建时无需使用 dyn。标志着动态分发，直到运行时，才能确定需要调用什么方法。当使用特征对象时，Rust 必须使用动态分发。编译器无法知晓所有可能用于特征对象代码的类型，所以它也不知道应该调用哪个类型的哪个方法实现。为此，Rust 在运行时使用特征对象中的指针来知晓需要调用哪个方法。
Box<dyn Draw> 形式的特征对象，该特征对象是通过 Box::new(x) 的方式创建的
&dyn Draw 形式的特征对象，该特征对象是通过 &x 的方式创建的
可以使用特征对象来代表泛型或具体的类型。
> 不是所有特征都能拥有特征对象，只有对象安全的特征才行(特征对象不关注具体类型，对象安全意味着类型确定，两者有同一个目标):
方法的返回类型不能是 Self  ---  因为一旦有了特征对象就代表我们不需要知道实现了该特征的具体类型是什么，而一旦返回了Self就意味着返回了某一类具体类型，与特征对象相冲突。
方法没有任何泛型参数  ---  对于泛型类型参数来说，当使用特征时其会放入具体的类型参数：此具体类型变成了实现该特征的类型的一部分
 */
pub struct Button {
    pub width: u8,
    pub height: u8,
    pub label: String,
}
pub struct SelectBox {
    pub width: u8,
    pub height: u8,
    pub options: Vec<String>,
}
impl Draw for Button {
    fn draw(&self) {
        println!("绘制了Button")
    }
}
impl Draw for SelectBox {
    fn draw(&self) {
        println!("绘制了SelectBox")
    }
}
pub struct Screen {
    pub components: Vec<Box<dyn Draw>>,
    // pub components: Vec<&dyn Draw>,  // 这样用提示需要声明周期参数？暂时没学到 先注释掉
}
impl Screen {
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw()
        }
    }
}
pub trait Draw {
    fn draw(&self);
}

/*
2.通过范型实现
Screen列表中存储了类型为T的元素，并且通过特征约束让T实现Draw特征，因此可以调用draw方法。
但是这种写法限制了T一旦首先确定为Button或者是SelectBox中的某一个，Vec中只能是这一种具体类型了。
因此这种写法适合需要同质集合，更加清晰，性能也更好(特征对象，需要在运行时从 vtable 动态查找需要调用的方法)。
 */
pub struct Screen1<T: Draw> {
    pub components: Vec<T>,
}
impl<T> Screen1<T>
where
    T: Draw,
{
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}

fn main() {
    let screen = Screen {
        components: vec![
            Box::new(Button {
                width: 50,
                height: 20,
                label: "ok".to_string(),
            }),
            Box::new(SelectBox {
                width: 80,
                height: 20,
                options: vec!["first".to_string(), "second".to_string()],
            }),
        ],
    };
    // let screen = Screen {
    //     components: vec![
    //         &Button {
    //             width: 50,
    //             height: 20,
    //             label: "ok".to_string(),
    //         },
    //         &SelectBox {
    //             width: 80,
    //             height: 20,
    //             options: vec!["first".to_string(), "second".to_string()],
    //         },
    //     ],
    // };
    let screen1 = Screen1 {
        components: vec![
            Button {
                width: 50,
                height: 20,
                label: "ok".to_string(),
            },
            Button {
                width: 50,
                height: 20,
                label: "ok".to_string(),
            },
        ],
    };
    screen.run();
    screen1.run();
}
