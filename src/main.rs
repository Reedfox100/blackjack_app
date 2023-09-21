use deckofcards::{Card, Cards, Deck, Hand, Rank, Suit};
use std::io;
use std::io::{stdin, stdout, Read, Write};
use std::process::exit;

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
fn calculate_hand(hand: &Hand, verbose: bool) -> i32 {
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
            _ => result += 0,
        };
    }
    for x in &hand.cards {
        if x.rank.to_str().eq("Ace") && result > 21 {
            if verbose {
                println!("Ace downgraded to a 1 to avoid going over.");
            }
            result -= 10;
        }
    }
    result
}

//Hits the player with a new card and returns the new version of the hand
fn hit_me<'a>(hand: &'a mut Hand, deck: &mut Deck) -> &'a mut Hand {
    deck.deal_to_hand(hand, 1);
    hand
}

fn player_choice(hands: &mut [&mut Hand; 5], mut deck: &mut Deck, iter: usize) {
    let mut calced_hand = calculate_hand(hands[iter], true);
    let calc_house = calculate_hand(hands[0], true);
    while true {
        //Define basic variables and print out normal returns
        let mut input: String = "".to_string();
        if calc_house == 21 {
            println!("$$ House $$: {}", calc_house);
            println!("------------------------");
            println!("{}", hands[iter]);
            println!("------------------------");
            println!("==========================================================");
            println!("Apologies, its a push. Dealer started with 21. No losses");
            println!("==========================================================");
            return;
        }
        //hand can be played VVV
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
                calced_hand = calculate_hand(&hit_me(hands[iter], &mut deck), true);
                continue;
            } else if input.to_lowercase().contains("stay") {
                println!("=============================================");
                println!("     You have decided to stay your hand");
                println!("=============================================");
                println!("");
                return;
            } else {
                println!("Please return a value of 'Hit' or 'Stay'");
                continue;
            }
        }
    }
}

fn calc_winners(player_count: usize, hands: &mut [&mut Hand; 5], deck: &mut Deck) {
    //calculate house score before determining winners
    let mut calc_house = calculate_hand(hands[0], true);
    print!("\x1B[2J\x1B[1;1H");
    println!("$$ House $$: {}", calc_house);
    println!("Lets see what the house hits...");
    press_enter();
    while calc_house <= 16 {
        hit_me(hands[0], deck);
        calc_house = calculate_hand(hands[0], true);
    }
    //Check bust or win from house
    println!("-------------------------------");
    print!("{}", hands[0]);
    println!("    Score: {}", calc_house);
    println!("-------------------------------");
    println!("");

    if calc_house > 22 {
        println!("=====================================");
        println!("The House has Busted!! Everyone wins.");
        println!("=====================================");
        println!("");
        //eventually replace with a "highest score" variable VVVV
    } else if calc_house == 22 {
        println!("========================================");
        println!("The House has Pushed with a 22. Full tie");
        println!("========================================");
        println!("");
    } else if calc_house == 21
        && calculate_hand(hands[1], true) != 21
        && calculate_hand(hands[2], true) != 21
        && calculate_hand(hands[3], true) != 21
        && calculate_hand(hands[4], true) != 21
    {
        println!("===============================================");
        println!("Blackjack. The house wins and all players lose.");
        println!("===============================================");
        println!("");
    } else {
        println!("======================================");
        println!("The has has scored a {}. Losses scored", calc_house);
        println!("======================================");
        println!("");
    }
    //display winners and losers, as well as scores
    let mut i = 0;
    while i < player_count {
        i += 1;
        println!("--------------------------");
        println!(
            "Player#{}: {},  Score:{}",
            i,
            hands[i],
            calculate_hand(hands[i], true)
        );
        if calculate_hand(hands[i], false) > calc_house {
            println!(" >>> WINNER");
        } else if calculate_hand(hands[i], false) == calc_house {
            println!(" >>> TIE");
        } else {
            println!(" >>> LOSER");
        }
    }
}

fn run_app<'a>(player_count: usize, mut hands: [&'a mut Hand; 5], mut deck: &'a mut Deck) {
    //Iterate through each
    let mut i: usize = 0;
    while i <= player_count && !(hands[i].cards.is_empty()) {
        player_choice(&mut hands, &mut deck, i);
        if calculate_hand(hands[0], true) == 21 && hands[0].cards.len() == 2 {
            return;
        }
        i += 1;
    }
    calc_winners(player_count, &mut hands, &mut deck);
}

fn player_amount() -> usize {
    println!("Please choose the number of players, maximum of 4, at the table.");
    let mut input: String = "".to_string();
    io::stdin()
        .read_line(&mut input)
        .expect("Failure to read line...");
    let mut output: usize = match input.trim().parse() {
        Ok(x) => x,
        Err(_) => 0,
    };
    if output == 0 {
        println!("Please enter a valid number (1-4)");
        output = player_amount();
    }
    output
}

fn main() {
    std::thread::Builder::new()
        .stack_size(33554432)
        .spawn(|| {
            // your code here
        })
        .unwrap()
        .join()
        .unwrap();
    //Make deck, player, and house (increasing number of max players later)
    let mut deck = Deck::new();
    deck.shuffle();
    let player_count: usize; //Temporary number until input is added
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
    let mut current: &mut Deck = &mut deck;

    //Returns the number of players and fills initial hands
    player_count = player_amount();
    (hands, current) = num_players(player_count, hands, &mut deck);
    run_app(player_count, hands, current);
    //Add replay button or question VVVV

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
