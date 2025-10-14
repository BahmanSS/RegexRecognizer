
#[cfg(test)]
mod tests {
    use regex_recognizer::{find_patterns,state_machine};
 

    #[test]
    fn test_state_machine_valid() {
        assert_eq!(state_machine("**ab*"), true, "Простая валидная последовательность");
        assert_eq!(state_machine("**bb*"), true, "Другая валидная последовательность");
        assert_eq!(state_machine("**babbbbab*"), true, "Другая валидная последовательность");
        assert_eq!(state_machine("***"), true, "Минимальная валидная последовательность");
    }

    #[test]
    fn test_state_machine_invalid() {
        assert_eq!(state_machine("**ab"), false, "Невалидная последовательность без завершающей *");
        assert_eq!(state_machine("ab*"), false, "Невалидная последовательность без начальной **");
        assert_eq!(state_machine("*"), false, "Одна звезда невалидна");
        assert_eq!(state_machine("**a*"), false, "После a должна идти b");
        assert_eq!(state_machine(""), false, "Пустая строка невалидна");
    }

    #[test]
    fn test_find_patterns() {
        let test_cases = vec![
            // Базовые валидные случаи
            ("", Vec::<(usize, &str)>::new()),
            ("***", vec![(0, "***")]),
            ("**b*", vec![(0, "**b*")]),
            ("**ab*", vec![(0, "**ab*")]),
            ("**aab*", Vec::<(usize, &str)>::new()),
            ("**aaab*", Vec::<(usize, &str)>::new()),
            
            // Сложные валидные случаи
            ("**abab*", vec![(0, "**abab*")]),
            ("**ab***ab*", vec![(0, "**ab*"), (4, "***"), (5, "**ab*")]),
            
            // Невалидные случаи (неправильный порядок ab)
            ("**ba*", Vec::<(usize, &str)>::new()),
            ("**aba*", Vec::<(usize, &str)>::new()),
            ("**aabba*", Vec::<(usize, &str)>::new()),
            
            // Невалидные случаи (неправильные символы)
            ("**c*", Vec::<(usize, &str)>::new()),
            ("**a1*", Vec::<(usize, &str)>::new()),
            ("**!@#*", Vec::<(usize, &str)>::new()),
            
            // Граничные случаи
            ("**", Vec::<(usize, &str)>::new()),
            ("**a*", Vec::<(usize, &str)>::new()),
            ("**b*", vec![(0, "**b*")]),
        ];

        for (input, expected) in test_cases.iter() {
            let result = find_patterns(input);
            assert_eq!(
                result,
                expected.clone(),
                "Ошибка при вводе: '{}'. Ожидалось: {:?}, получено: {:?}",
                input,
                expected,
                result
            );
        }
    }



}