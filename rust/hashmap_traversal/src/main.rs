use std::collections::HashMap;





/// 遍历 hashmap
fn main() {
    let new_map = HashMap::from([
        ("one", "yi"),
        ("two", "er"),
        ("three", "san"),
        ("four", "si"),
        ("five", "wu"),
        ("six", "liu"),
        ("seven", "qi"),
        ("eight", "ba")
    ]);
    let mut result:Vec<String> = Vec::new();
    for v in new_map.values() {
        if v.contains("s") {
            result.push(v.to_string())
        }
    }
    // let result = new_map.drain();
    println!("{:?}", result)
    // new_map.drain()

}





















