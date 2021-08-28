use scc050_t2::{Direction, TuringMachine};

#[test]
fn test1() {
    // Caso teste 1 passado no enunciado
    let tm = TuringMachine {
        input_symbols: vec!['a', 'b', 'c'].into_iter().collect(),
        tape_symbols: vec!['#', '*', '@', 'B'].into_iter().collect(),
        blank_symbol: 'B',
        states: vec![0, 1, 2, 3, 4, 5].into_iter().collect(),
        initial_state: 0,
        accepting_states: vec![5].into_iter().collect(),
        transitions: vec![
            ((0, 'a'), (1, '#', Direction::Right)),
            ((1, '#'), (1, '#', Direction::Right)),
            ((1, 'a'), (1, 'a', Direction::Right)),
            ((1, '*'), (1, '*', Direction::Right)),
            ((1, 'b'), (2, '*', Direction::Right)),
            ((2, 'b'), (2, 'b', Direction::Right)),
            ((2, '@'), (2, '@', Direction::Right)),
            ((2, 'c'), (3, '@', Direction::Left)),
            ((3, '#'), (3, '#', Direction::Left)),
            ((3, '*'), (3, '*', Direction::Left)),
            ((3, '@'), (3, '@', Direction::Left)),
            ((3, 'b'), (3, 'b', Direction::Left)),
            ((3, 'a'), (1, '#', Direction::Right)),
            ((3, 'B'), (4, 'B', Direction::Right)),
            ((4, '#'), (4, '#', Direction::Right)),
            ((4, '*'), (4, '*', Direction::Right)),
            ((4, '@'), (4, '@', Direction::Right)),
            ((4, 'B'), (5, 'B', Direction::Right)),
        ]
        .into_iter()
        .collect(),
    };

    assert!(!tm.run_tape("abbcca".chars().collect()));
    assert!(tm.run_tape("aabbcc".chars().collect()));
    assert!(!tm.run_tape("bac".chars().collect()));
    assert!(!tm.run_tape("aaabbbcccc".chars().collect()));
    assert!(!tm.run_tape("-".chars().collect()));
    assert!(!tm.run_tape("abcabc".chars().collect()));
    assert!(tm.run_tape("abc".chars().collect()));
    assert!(!tm.run_tape("abcc".chars().collect()));
    assert!(!tm.run_tape("c".chars().collect()));
    assert!(!tm.run_tape("aaabbbbccc".chars().collect()));
}
