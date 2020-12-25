use std::io::{self, BufRead};

const INITIAL_SUBJECT_NUMBER: u128 = 7;
const MODULO_NUMBER: u128 = 20201227;

fn main() {
    let stdin = io::stdin();
    let mut iter = stdin.lock().lines().filter_map(Result::ok);
    let door_pub_key: u128 = iter.next().unwrap().parse().unwrap();
    let card_pub_key: u128 = iter.next().unwrap().parse().unwrap();

    let mut x: u128 = 1;
    let mut card_loop_size = 0;
    while x != card_pub_key {
        card_loop_size += 1;
        x = (x * INITIAL_SUBJECT_NUMBER) % MODULO_NUMBER;
    }
    let card_loop_size = card_loop_size;

    let mut x: u128 = 1;
    let mut door_loop_size = 0;
    while x != door_pub_key {
        door_loop_size += 1;
        x = (x * INITIAL_SUBJECT_NUMBER) % MODULO_NUMBER;
    }
    let door_loop_size = door_loop_size;

    let decryption_key = perform_transformation(door_pub_key, card_loop_size);
    assert_eq!(
        decryption_key,
        perform_transformation(card_pub_key, door_loop_size)
    );

    println!("Part 1: {}", decryption_key);
}

fn perform_transformation(subject_number: u128, loop_size: usize) -> u128 {
    let mut x: u128 = 1;

    for _ in 0..loop_size {
        x = (x * subject_number) % MODULO_NUMBER;
    }

    x
}
