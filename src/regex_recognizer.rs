// Подпрограмма - распознающий ДКА
pub fn state_machine(s: &str) -> bool {
    // Состояния ДКА
     #[derive(PartialEq)]
    enum State {
        Q0, Q1, Q2, Q3, Qf
    }

    // Текущее состояние(для прохода по символам строки)
    let mut current_state = State::Q0;

    // делаем посимвольный обход строки 
    for ch in s.chars() {
        // обрабатываем переходы между состояниями
        match (&current_state, ch) {
            (State::Q0, '*') => current_state = State::Q1,
            (State::Q1, '*') => current_state = State::Q2,
            (State::Q2, 'a') => current_state = State::Q3,
            (State::Q2, 'b') => continue,
            (State::Q2, '*') => current_state = State::Qf, // переход в финальное состояние
            (State::Q3, 'b') => current_state = State::Q2,
            _ => return false // не подходящая строка
        }
    }
    current_state == State::Qf
}

// Программа ( поиск подходящих подстрок и их начальных индексов )
pub fn find_patterns(input: &str) -> Vec<(usize, &str )> {
    let mut result: Vec<(usize, &str)> = Vec::new(); // вектор с индексом и строкой, допущенной КА
    let chars: Vec<char> = input.chars().collect(); // символы вводной строки
    let len = chars.len(); // длина вводной строки

    if len < 3 {
        return result; // Возвращаем пустой результат для коротких строк
    }
    
    // проверка всевозможных подстрок КА
    // с небольшой оптимизацией с помощью просмотра только строк начинающихся на ** и заканчивающихся на *
    let mut i = 0;
    while i < len - 2 {
        if chars[i] == '*' && chars[i+1] == '*' {
            for j in i+2..len {
                if chars[j] == '*' {
                    if state_machine(&input[i..=j]) {
                        result.push((i,&input[i..=j]));
                    }
                    break;
                }
            }
        }
        i += 1
    }
    result
}