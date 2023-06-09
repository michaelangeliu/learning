fn main() {
    // Create
    let _v_declared: Vec<i32> = Vec::new();

    let _v_intantiated = vec![1, 2, 3]; // Vec<i32> since i32 is the default integer type and the vector type is inferred

    // Push
    let mut v = Vec::new();

    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);

    for i in &v {
        println!("{i}");
    }

    // Pop
    v.pop();
    for i in &v {
        println!("{i}");
    }

    // Read
    let v = vec![1, 2, 3, 4, 5];
    let third: &i32 = &v[2];
    println!("The third element is {third}");

    let third: Option<&i32> = v.get(2);
    match third {
        Some(third) => println!("The third element is {third}"),
        None => println!("There is no third element."),
    }

    // Iterate
    let v = vec![100, 32, 57];
    for i in &v {
        println!("{i}");
    }

    let mut v = vec![100, 32, 57];
    for i in &mut v {
        *i += 50; // `*` is the dereference operator to get the value in `i`
        println!("{i}");
    }

    // Enum
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];

    // Drop
    {
        let v_dropped = vec![1, 2, 3, 4];
    }

    // for i in &v_dropped { // v goes out of scope and the memory is freed
    //     println!("{i}");
    // }

    // Create new String
    let mut s = String::new();

    let data = "initial contents";

    let s = data.to_string();

    // the method also works on a literal directly:
    let s = "initial contents".to_string();
    let s = String::from("initial contents");

    // +
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // note s1 has been moved here and can no longer be used. Since `+` uses `add`, it takes owners ship of `self`. It doesn't copy both strings, it just takes ownership of s1 and appends the copy of the contents of s2

    // HashMaps
    use std::collections::HashMap;

    // Create
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    // Read
    let team_name = String::from("Blue");
    let score = scores.get(&team_name).copied().unwrap_or(0); // get returns an Option<&V>. copied handles the Options to get an Option<i32> rather than an Option<&i32>. unwrap-_or sets the score to zero if the scores don't have an entry for that key

    for (key, value) in &scores {
        println!("{key}: {value}");
    }

    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value); // field_name and field_value are invalid after this point

    scores.insert(String::from("Blue"), 10);
    
    // Insert only if not exist
    scores.entry(String::from("Green")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);

    println!("{:?}", scores);

    // Updating a value based on the old value
    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map);
}
