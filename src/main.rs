use deckofcards::{Cards, Deck, Hand};
use rand::{
    rngs::mock::StepRng,
    seq::{self, SliceRandom},
};
use std::{mem, ptr::null, io};


fn num_players<'a>(player_count: i32, mut hands: [&'a mut Hand; 5], deck: &'a mut Deck) -> [&'a mut Hand; 5]{
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

    hands
    }

fn calculate_hand(hand: &mut Hand) -> i32{
    let mut result: i32 = 0;
    print!("|--");
    for x in &mut hand.cards{
        print!(" {} ", x);
        match x.rank.to_str(){
            "Two" =>result += 2,
            "Three" =>result += 3,
            "Four" =>result += 4,
            "Five" =>result += 5,
            "Six" =>result += 6,
            "Seven" =>result += 7,
            "Eight" =>result += 8,
            "Nine" =>result += 9,
            "Ten"|"King"|"Queen"|"Jack" =>result += 10,
            "Ace" =>result += 11,
            _ =>result += 1,
        };
    }
    print!("--|");
result
}

fn hit_me(hand: &mut Hand, deck: &mut Deck){
    deck.deal_to_hand(hand, 1);
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
    //Returns the number of players and fills initial hands
    hands = num_players(player_count, hands, &mut deck);

    let mut i = 0;
    while i < 5 && !(hands[i].cards().is_empty()){
            let mut input: String = Default::default();
            println!("$$ House $$: {}", calculate_hand(hands[i]));
            println!("  Total: {}", calculate_hand(hands[i]));
            //INSERT SPLITTING HERE
            println!("Hit or Stay? (Type Hit/H or Stay/S");
            io::stdin().read_line(&mut input).expect("Failure to read line...");
            if input.to_lowercase() == "hit" || input.to_lowercase() == "h"{
                hit_me(hands[i], &mut deck);
            }
        i += 1;
    }
    // let mut input = String::new();
    // let mut i = 1;
    // while i < 5 && !(hands[i].cards().is_empty()){
    //     println!("Hand: {}", hands[i]);
    //     i += 1;
    // }
}

