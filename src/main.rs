
use regex_recognizer::find_patterns;

use std::io; // бибилотека для вывода результата в командную строку


fn main() {
    
    println!("Введите строку: ");

    // чтение данных от пользователя
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Error of reading!"); 

    // запуск программы и вывод результатов
    let result: Vec<(usize, &str)> = find_patterns(&input);
    for i in 0..result.len(){
        println!("{} : {}", result[i].0,result[i].1);
    }
}