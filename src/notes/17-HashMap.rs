use std::collections::HashMap;

/*
HashMap key的限制:任何实现了 Eq 和 Hash 特征的类型都可以用于 HashMap 的 key
包括：bool (虽然很少用到，因为它只能表达两种 key)
int, uint 以及它们的变体，例如 u8、i32 等
String 和 &str (提示: HashMap 的 key 是 String 类型时，你其实可以使用 &str 配合 get 方法进行查询
    需要注意的是，f32 和 f64 并没有实现 Hash，因为浮点数精度问题导致无法进行相等比较
    如果一个集合类型的所有字段都实现了 Eq 和 Hash,那该集合类型会自动实现 Eq 和 Hash。例如 Vect<T> 要实现 Hash，那么首先需要 T 实现 Hash。
 */
fn main() {
    let mut team: HashMap<String, i32> = HashMap::new();
    team.insert("中国".to_string(), 10);
    team.insert(String::from("美国"), 5);
    println!("team is {:#?}", team);
    // 场景：将数据转化为hashmap类型 通过collect()，需要事先指定类型
    // into_iter 方法将列表转为迭代器，接着通过 collect 进行收集，不过需要注意的是，collect 方法在内部实际上支持生成多种类型的目标集合，因此我们需要通过类型标注 HashMap<_,_> 来告诉编译器类型
    let data = vec![
        ("中国".to_string(), 100),
        ("美国".to_string(), 10),
        ("德国".to_string(), 20),
    ];
    let mut newTeam: HashMap<String, i32> = data.into_iter().collect();
    println!("new team is {:#?}", newTeam);
    // 查询
    let score = newTeam.get(&"中国".to_string());
    println!("score is {:?}", score); // score is Some(100)
    let scoreValue = newTeam.get(&"中国".to_string()).copied().unwrap_or(0);
    println!("value is {scoreValue}"); // value is 100
                                       // 更新hashmap的值
                                       // 覆盖已有的值
    newTeam.insert("美国".to_string(), 12);
    println!("new team is {:#?}", newTeam);
    // 如果有不插入，如果没有就插入
    newTeam.entry("中国".to_string()).or_insert(200);
    println!("new team is {:#?}", newTeam); // 不变 因为中国有了
    let current = newTeam.entry("法国".to_string()).or_insert(200); // current 是&mut的 200
    *current += 20;
    println!("new team is {:#?}", newTeam); // 多了个法国:220
}
