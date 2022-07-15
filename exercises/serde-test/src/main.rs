// Step1: 日常的写法
// use serde::{Deserialize, Serialize};

// &#[derive(Serialize)]
// struct Persion {
//     name: String,
//     age: u8,
//     phones: Vec<String>,
// }

// #######################################
// 上面用宏的方式，与下面这种手动写的方式是一样的。
// 这样写了之后，还是不能用的，因为我们还要实现Serializer

// // Step2: 宏实际上做了什么:
// use serde::ser::{Serialize, SerializeStruct};
// struct Persion2 {
//     name: String,
//     age: u8,
//     phones: Vec<String>,
// }
// impl Serialize for Persion2 {
//     fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
//     where
//         S: serde::Serializer,
//     {
//         let mut s = serializer.serialize_struct("person", 3)?; // 3个字段
//         s.serialize_field("name", &self.name)?;
//         s.serialize_field("age", &self.age)?;
//         s.serialize_field("phones", &self.phones)?;
//         todo!()
//     }
// }

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct Pay {
    amount: i32,
    tax_percent: f32,
}

#[derive(Serialize, Deserialize, Debug)]
struct Person {
    name: String,
    age: u8,
    phone: String,
    pays: Vec<Pay>,
}

fn main() {
    // 看如何使用serde_yaml, serde_json来实现serializer
    let ps = vec![Person {
        name: "zz".to_string(),
        age: 45,
        phone: "12345".to_string(),
        pays: vec![Pay {
            amount: 78,
            tax_percent: 0.3,
        }],
    }];

    // 将它转为string类型(序列化)：
    let json_str = serde_json::to_string_pretty(&ps).unwrap();
    let yaml_str = serde_yaml::to_string(&ps).unwrap();

    println!("json: {}", json_str);
    println!("yaml: {}", yaml_str);

    let ps_json: Vec<Person> = serde_json::from_str(json_str.as_str()).unwrap();
    let ps_yaml: Vec<Person> = serde_yaml::from_str(yaml_str.as_str()).unwrap();

    println!("json: {:?}", ps_json);
    println!("yaml: {:?}", ps_yaml);

    // 下面开始修改json的实战
    let json_data = std::fs::read_to_string("./data.json").unwrap();
    let mut data: serde_json::Value = serde_json::from_str(json_data.as_str()).unwrap();
    println!("before: {}", data);

    // api 给数据添加一辆车
    // 可以认为data就是个Map
    data["car"] = serde_json::Value::String("ffo".to_string());
    println!("After: {}", data);

    // 也可以添加json
    let mut map_value = serde_json::Map::new();
    map_value.insert(
        "color".to_string(),
        serde_json::Value::String("blue".to_string()),
    );
    map_value.insert(
        "born_year".to_string(),
        serde_json::Value::String("1900".to_string()),
    );

    // 添加数组
    map_value.insert(
        "fixed_year".to_string(),
        serde_json::Value::Array(vec![
            serde_json::Value::String("1991".to_string()),
            serde_json::Value::String("1992".to_string()),
        ]),
    );

    data["car"] = serde_json::Value::Object(map_value);
    println!("After: {}", data);
}
