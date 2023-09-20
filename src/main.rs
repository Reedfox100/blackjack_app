use deckofcards::{Cards, Deck, Hand};
use rand::{
    rngs::mock::StepRng,
    seq::{self, SliceRandom},
};
use std::io::{stdin, stdout, Read, Write};
use std::{io, mem, ptr::null};

fn press_enter() {
    let mut stdout = stdout();
    stdout.write(b"Press Enter to continue...").unwrap();
    stdout.flush().unwrap();
    stdin().read(&mut [0]).unwrap();
}

fn num_players<'a>(
    player_count: i32,
    mut hands: [&'a mut Hand; 5],
    deck: &'a mut Deck,
) -> ([&'a mut Hand; 5], &'a mut Deck){
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
(hands, deck)
    
}

fn calculate_hand(hand: & Hand) -> i32 {
    let mut result: i32 = 0;
    print!("|--");
    for x in &hand.cards {
        print!(" {} ", x);
        match x.rank.to_str() {
            "Two" => result += 2,
            "Three" => result += 3,
            "Four" => result += 4,
            "Five" => result += 5,
            "Six" => result += 6,
            "Seven" => result += 7,
            "Eight" => result += 8,
            "Nine" => result += 9,
            "Ten" | "King" | "Queen" | "Jack" => result += 10,
            "Ace" => result += 11,
            _ => result += 1,
        };
    }
    print!("--|");
    result
}

fn hit_me<'a>(hand: &'a mut Hand, deck: &mut Deck) -> &'a mut Hand {
    deck.deal_to_hand(hand, 1);
    hand
}

fn player_choice(hands: &mut[&mut Hand; 5], mut deck: &mut Deck, iter: usize) {
    let mut input: String = Default::default();
    let calced_hand = calculate_hand(hands[iter]);
    if iter == 0 {
        println!("$$ House $$: {}", calced_hand);
    } else {
        println!("Player#{}: {}", iter, calced_hand);
    }

    println!("  Total: {}", calced_hand);
    if iter != 0 {
        if calced_hand > 21 {
            println!("You have busted and lost the round. Better luck next time!");
        } else if calced_hand == 21 {
            println!("BLACKJACK!! You have won your hand for now...");
        } else {
            println!("You have not busted.");
        }
        press_enter();
    }

    //Query the player to return a hit or stay after giving new hand number
    println!("Hit or Stay? (Type Hit/H or Stay/S");
    io::stdin()
        .read_line(&mut input)
        .expect("Failure to read line...");
    if input.to_lowercase() == "hit" || input.to_lowercase() == "h" {
        hit_me(hands[iter], &mut deck);
    } else if input.to_lowercase() == "stay" || input.to_lowercase() == "s" {
    } else {
        println!("Please return a value of 'Hit' or 'Stay'");
        player_choice(hands, deck, iter);
    }
}

fn run_app<'a>(player_count: i32, mut hands: [&'a mut Hand; 5], mut deck: &'a mut Deck) {
    //Returns the number of players and fills initial hands

    //Iterate through each
    let mut i = 0;
    while i < 5 && !(hands[i].cards.is_empty()) {
        //INSERT SPLITTING HERE
        player_choice(&mut hands, &mut deck, i);
        i += 1;
    }
}

fn main() {
    //Make deck, player, and house (increasing number of max players later)
    let mut deck = Deck::new();
    deck.shuffle();
    let player_count = 2; //Temporary number until input is added
    let mut player1 = Hand::new();
    let mut player2 = Hand::new();
    let mut player3 = Hand::new();
    let mut player4 = Hand::new();
    let mut house = Hand::new();
    let mut hands: [&mut Hand; 5] = [
        &mut house,
        &mut player1,
        &mut player2,
        &mut player3,
        &mut player4,
    ];
    let current: &mut Deck;
    (hands, current) = num_players(player_count, hands, &mut deck);

    run_app(player_count, hands, current);

    // let mut input = String::new();
    // let mut i = 1;
    // while i < 5 && !(hands[i].cards().is_empty()){
    //     println!("Hand: {}", hands[i]);
    //     i += 1;
    // }

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
}
