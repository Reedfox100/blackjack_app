use deckofcards::{Cards, Deck, Hand};
use rand::{
    rngs::mock::StepRng,
    seq::{self, SliceRandom},
};
use std::{mem, ptr::null, io};

fn deal_hands(player_count: i32, mut hands: [&mut Hand; 5], deck: &mut Deck) {
    //shuffle deck and deal hands to each player, including house (currently only 1).
    //future for statement to iterate through players and deal handsdeal hands.
    if player_count == 1 {
        for _ in 0..2 {
            deck.deal_to_hand(&mut hands[1], 1);
            deck.deal_to_hand(&mut hands[0], 1);
        }
    } else if player_count == 2 {
        for _ in 0..2 {
            deck.deal_to_hand(&mut hands[1], 1);
            deck.deal_to_hand(&mut hands[2], 1);
            deck.deal_to_hand(&mut hands[0], 1);
        }
    } else if player_count == 3 {
        for _ in 0..2 {
            deck.deal_to_hand(&mut hands[1], 1);
            deck.deal_to_hand(&mut hands[2], 1);
            deck.deal_to_hand(&mut hands[3], 1);
            deck.deal_to_hand(&mut hands[0], 1);
        }
    } else if player_count == 4 {
        for _ in 0..2 {
            deck.deal_to_hand(&mut hands[1], 1);
            deck.deal_to_hand(&mut hands[2], 1);
            deck.deal_to_hand(&mut hands[3], 1);
            deck.deal_to_hand(&mut hands[4], 1);
            deck.deal_to_hand(&mut hands[0], 1);
        }
    }
    let mut i = 1;
    while i < 5 && !(hands[i].cards().is_empty()){
        println!("Hand: {}", hands[i]);
        i += 1;
    }
}

fn main() {
    // enum Values{
    //     Ace(String, [usize; 2]),
    //     Two(String, usize),
    //     Three(String, usize),
    //     Four(String, usize),
    //     Five(String, usize),
    //     Six(String, usize),
    //     Seven(String, usize),
    //     Eight(String, usize),
    //     Nine(String, usize),
    //     Ten(String, usize),
    //     Jack(String, usize),
    //     Queen(String, usize),
    //     King(String, usize),
    // }
    // enum Suits{
    //     Clubs(String),
    //     Diamonds(String),
    //     Hearts(String),
    //     Spades(String),
    // }
    // struct Card{
    //     value: Values,
    //     suit: Suits,
    // }

    //Deck crate might be more useful if using a switch and for multiple players moving forwards.

    //Make deck, player, and house (increasing number of max players later)
    let mut deck = Deck::new();
    let player_count = 2; //Temporary number until input is added
    let mut player1 = Hand::new();
    let mut player2 = Hand::new();
    let mut player3 = Hand::new();
    let mut player4 = Hand::new();
    let mut house = Hand::new();
    let hands: [&mut Hand; 5] = [
        &mut house,
        &mut player1,
        &mut player2,
        &mut player3,
        &mut player4,
    ];
    deck.shuffle();

    deal_hands(player_count, hands, &mut deck)
    // let ace_of_spades = Card{
    //     value: Values::Ace("Ace".to_string(), [1,11]),
    //     suit: Suits::Spades("Spades".to_string()),
    // };

    // deck.push(ace_of_spades);
    // for x in deck{
    //     println!("Card: {} {}", x.0, x.1);
    // }
}

// let mut deck: Vec<String> = Vec::new();
// for x in VALUES_IN_DECK{
//     for y in SUITES_IN_DECK{
//         let card_pieces = [x, "of", y];
//         let card = card_pieces.join(" ");
//         deck.push(card);
//     }

// }
// for x in values_iter {
//     for y in suites_iter {
//         let card_pieces = [x, y];
//         let card = card_pieces.join(" ");
//         deck.push(card);
//     }
// }

// }
// let mut rng = rand::thread_rng();
// deck.shuffle(&mut rng);
// for x in deck{
//     println!("Card: {}", x);
// }
