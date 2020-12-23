use std::{
    cmp::Reverse,
    collections::{HashSet, VecDeque},
    io::{self, BufRead},
};

fn main() {
    let mut decks: Vec<VecDeque<u16>> = Vec::new();

    let mut deck = VecDeque::new();

    for line in io::stdin().lock().lines().filter_map(Result::ok) {
        if line.is_empty() {
            continue;
        }

        if line.starts_with("Player") {
            if !deck.is_empty() {
                decks.push(deck);
                deck = VecDeque::new();
            }
        } else {
            let card: u16 = line.parse().unwrap();
            deck.push_back(card);
        }
    }

    decks.push(deck);

    {
        let mut combat_decks = decks.clone();

        while combat_decks.iter().all(|deck| !deck.is_empty()) {
            let mut played_cards: Vec<_> = combat_decks
                .iter_mut()
                .enumerate()
                .map(|(i, deck)| (i, deck.pop_front().unwrap()))
                .collect();

            played_cards.sort_by_key(|&(_, card)| Reverse(card));

            let player = played_cards[0].0;
            for (_, card) in played_cards.into_iter() {
                combat_decks[player].push_back(card);
            }
        }

        let (_winner, winning_deck) = combat_decks
            .iter()
            .enumerate()
            .find(|(_, deck)| !deck.is_empty())
            .unwrap();
        let mut points = 0;
        for (i, card) in winning_deck.iter().rev().enumerate() {
            points += *card as usize * (i + 1);
        }

        println!("Part 1: {}", points);
    }

    {
        let winner = play_recursive_combat(&mut decks);
        let mut points = 0;
        for (i, card) in decks[winner].iter().rev().enumerate() {
            points += *card as usize * (i + 1);
        }

        println!("Part 2: {}", points);
    }
}

fn play_recursive_combat(decks: &mut Vec<VecDeque<u16>>) -> usize {
    let mut states: HashSet<Vec<VecDeque<u16>>> = HashSet::new();

    while decks.iter().all(|deck| !deck.is_empty()) {
        if states.contains(decks) {
            return 0;
        }
        states.insert(decks.clone());

        let mut played_cards: Vec<_> = decks
            .iter_mut()
            .enumerate()
            .map(|(i, deck)| (i, deck.pop_front().unwrap()))
            .collect();

        let winner = if played_cards
            .iter()
            .all(|(i, card)| decks[*i].len() >= *card as usize)
        {
            let mut subdecks = decks
                .iter()
                .enumerate()
                .map(|(i, deck)| {
                    deck.iter()
                        .copied()
                        .take(played_cards[i].1 as usize)
                        .collect()
                })
                .collect();
            play_recursive_combat(&mut subdecks)
        } else {
            played_cards
                .iter()
                .fold(&(0, 0), |acc, x| if x.1 > acc.1 { x } else { acc })
                .0
        };

        decks[winner].push_back(played_cards.remove(winner).1);
        decks[winner].push_back(played_cards.pop().unwrap().1);
    }

    let (winner, _winning_deck) = decks
        .iter()
        .enumerate()
        .find(|(_, deck)| !deck.is_empty())
        .unwrap();

    winner
}
