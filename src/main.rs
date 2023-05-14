mod parser;
mod ast;

use parser::*;

fn main() {
    let text = "Пудовичок Полушка Целковый";
    println!("Вход: {text}");
    match rus_number(text) {
        Ok(num) => println!("Ответ: {}", num.1),
        Err(e) => println!("Error: {e}"),
    }

    println!("---");

    let text = "Пудовичок да Целковый";
    println!("Вход: {text}");
    match sum(text) {
        Ok(num) => println!("Ответ: {}", num.1),
        Err(e) => println!("Error: {e}"),
    }

    println!("---");

    let text = "Пудовичок да Целковый да Осьмушка";
    println!("Вход: {text}");
    match sum(text) {
        Ok(num) => println!("Ответ: {}", num.1),
        Err(e) => println!("Error: {e}"),
    }

    println!("---");

    let text = "переменная есть Пудовичок да Целковый";
    println!("Вход: {text}");
    match var_init(text) {
        Ok(num) => println!("Ответ: {} = {}", num.1.0, num.1.1),
        Err(e) => println!("Error: {e}"),
    }
}
