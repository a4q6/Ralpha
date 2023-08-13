use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct Book {
    id: i32,
    title: String,
}

fn main() {
    // シリアライズ (構造体 → JSON 文字列)
    // 整形したいときは to_string の代わりに to_string_pretty を使えば OK
    let book = Book {
        id: 1,
        title: String::from("Title-1"),
    };
    let json = serde_json::to_string(&book).unwrap();
    println!("{}", json); //=> {"id":1,"title":"Title-1"}

    // デシリアライズ (JSON 文字列 → 構造体)
    let json = r#"{"id":2, "title":"Title-2"}"#;
    let book: Book = serde_json::from_str(json).unwrap();
    println!("{}, {}", book.id, book.title); //=> 2, Title-2
}
