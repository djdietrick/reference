#[allow(dead_code)]
pub fn main() {
    // Simple enum
    enum Difficulty {
        Easy,
        Medium,
        Hard
    }
    let _diff = Difficulty::Easy;

    // Store values in enums
    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(i32, i32, i32),
    }
    let _mess = Message::Write(String::from("message"));
}
