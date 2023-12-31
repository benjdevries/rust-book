// fn largest<T>(list: &[T]) -> &T {
//     let mut largest = &list[0];
//
//     for item in list {
//         if item > largest {
//             largest = item;
//         }
//     }
//
//     largest
// }

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}


fn main() {
    // let number_list = vec![34, 50, 25, 100, 65];
    //
    // let result = largest(&number_list);
    //
    // println!("The largest number is {result}");
    //
    // let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];
    //
    // let result = largest(&number_list);
    // println!("The largest number is {}", result);

    let string1 = String::from("long string is long");
    let result;
    {
        let string2 = String::from("xyz");
        result = longest(string1.as_str(), string2.as_str());
    }
    println!("The longest string is {}", result);
}
