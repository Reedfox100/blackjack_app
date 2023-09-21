use deckofcards::{Card, Cards, Deck, Hand, Rank, Suit};
use rand::{
    rngs::mock::StepRng,
    seq::{self, SliceRandom},
};
use std::io::{stdin, stdout, Read, Write};
use std::{io, mem, ptr::null};

//Easy function for a stop or pause in the code
fn press_enter() {
    let mut stdout = stdout();
    stdout.write(b"Press Enter to continue...").unwrap();
    stdout.flush().unwrap();
    stdin().read(&mut [0]).unwrap();
}

//Find the number of hands to deal at the start of the game and provides cards from the deck
fn num_players<'a>(
    player_count: usize,
    mut hands: [&'a mut Hand; 5],
    deck: &'a mut Deck,
) -> ([&'a mut Hand; 5], &'a mut Deck) {
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

//Calculated the hand value and returns it
fn calculate_hand(hand: &Hand) -> i32 {
    let mut result: i32 = 0;
    for x in &hand.cards {
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
    if (hand.cards.contains(&Card { rank: Rank::Ace, suit: Suit::Clubs }) || hand.cards.contains(&Card { rank: Rank::Ace, suit: Suit::Diamonds }) || hand.cards.contains(&Card { rank: Rank::Ace, suit: Suit::Hearts }) || hand.cards.contains(&Card { rank: Rank::Ace, suit: Suit::Spades })) && calculate_hand(hand) > 21{
        println!("Ace downgraded to a 1 to avoid going over.");
        result -= 10;
    }
    result
}

//Hits the player with a new card and returns the new version of the hand
fn hit_me<'a>(hand: &'a mut Hand, deck: &mut Deck) -> &'a mut Hand {
    deck.deal_to_hand(hand, 1);
    hand
}

fn player_choice(hands: &mut [&mut Hand; 5], mut deck: &mut Deck, iter: usize) {
    //Define basic variables and print out normal returns
    let mut input: String = "".to_string();
    let calc_house = calculate_hand(hands[0]);
    let calced_hand = calculate_hand(hands[iter]);
    let mut push = false;
    if calc_house == 21 {
        println!("$$ House $$: {}", calc_house);
        println!("------------------------");
        println!("{}", hands[iter]);
        println!("------------------------");
        println!("==========================================================");
        println!("Apologies, its a push. Dealer started with 21. No losses");
        println!("==========================================================");
        push = false;
        return;
    }
    //hand can be played VVV
    if push == false {
        if iter == 0 {
            return;
        } else {
            println!("$$ House $$: {}", calc_house);
            println!("Player#{}: {}", iter, calced_hand);
            println!("------------------------");
            println!("{}", hands[iter]);
            println!("------------------------");
        }
        //Allow splitting here ---------------->
        //Check if player has busted or blackjacked
        if iter != 0 {
            if calced_hand > 21 {
                println!("==========================================================");
                println!("You have busted and lost the round. Better luck next time!");
                println!("==========================================================");
                println!("");
                return;
            } else if calced_hand == 21 {
                println!("=============================================");
                println!("BLACKJACK!! You have won your hand for now...");
                println!("=============================================");
                println!("");
                return;
            } else {
                println!("You have not busted.");
                println!("");
            }

            //Query the player to return a hit or stay after giving new hand number
            println!("Hit or Stay? (Type 'Hit' or 'Stay'");
            io::stdin()
                .read_line(&mut input)
                .expect("Failure to read line...");

            //Determine hit or stay function
            if input.to_lowercase().contains("hit") {
                let _ = calced_hand + calculate_hand(&hit_me(hands[iter], &mut deck));
                player_choice(hands, deck, iter);
            } else if input.to_lowercase().contains("stay") {
                println!("=============================================");
                println!("     You have decided to stay your hand");
                println!("=============================================");
                println!("");
                return;
            } else {
                println!("Please return a value of 'Hit' or 'Stay'");
                player_choice(hands, deck, iter);
            }
        }
    }
}

fn calc_winners(player_count: usize, hands: &mut [&mut Hand; 5], deck: &mut Deck) {
    //calculate house score before determining winners
    let mut calc_house = calculate_hand(hands[0]);
    println!("$$ House $$: {}", calc_house);
    println!("Lets see what the house hits...");
    press_enter();
    while calc_house <= 16 {
        hit_me(hands[0], deck);
        calc_house = calculate_hand(hands[0]);
    }
    //Check bust or win from house
    println!("-------------------------------");
    println!("{}", hands[0]);
    print!("    Score: {}", calc_house);
    println!("-------------------------------");

    if calc_house > 22 {
        println!("=====================================");
        println!("The House has Busted!! Everyone wins.");
        println!("=====================================");
        println!("");
        return;
        //eventually replace with a "highest score" variable VVVV
    } else if calc_house == 22{
        println!("========================================");
        println!("The House has Pushed with a 22. Full tie");
        println!("========================================");
        println!("");
    }else if calc_house == 21
        && calculate_hand(hands[1]) != 21
        && calculate_hand(hands[2]) != 21
        && calculate_hand(hands[3]) != 21
        && calculate_hand(hands[4]) != 21
    {
        println!("===============================================");
        println!("Blackjack. The house wins and all players lose.");
        println!("===============================================");
        println!("");
        return;
    } else{
        println!("======================================");
        println!("The has has scored a {}. Losses scored", calc_house);
        println!("======================================");
        println!("");
    }
    //display winners and losers, as well as scores
    let mut i = 0;
    while i <= player_count{
        i += 1;
        println!("--------------------------");
        println!("Player#{}: {},  Score:{}", player_count, hands[player_count], calculate_hand(hands[player_count]));
        if calculate_hand(hands[player_count]) > calc_house{
            print!(" >>> WINNER");
        }
        else if calculate_hand(hands[player_count]) == calc_house{
            print!(" >>> TIE");
        }
        else{
            print!(" >>> LOSSER");
        }
    }
}

fn run_app<'a>(player_count: usize, mut hands: [&'a mut Hand; 5], mut deck: &'a mut Deck) {
    //Iterate through each
    let mut i: usize = 0;
    while i <= player_count && !(hands[i].cards.is_empty()) {
        player_choice(&mut hands, &mut deck, i);
        if calculate_hand(hands[0]) == 21 && hands[0].cards.len() == 2{
            return;
        }
        i += 1;
    }
    calc_winners(player_count, &mut hands, &mut deck);
}

fn rerun_app(){

}

fn main() {
    //Make deck, player, and house (increasing number of max players later)
    let mut deck = Deck::new();
    deck.shuffle();
    let player_count: usize = 2; //Temporary number until input is added
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

    //Returns the number of players and fills initial hands
    (hands, current) = num_players(player_count, hands, &mut deck);
    run_app(player_count, hands, current);
    rerun_app();

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
