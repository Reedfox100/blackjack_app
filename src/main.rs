use rand::{
    rngs::mock::StepRng,
    seq::{self, SliceRandom},
};


fn main() {
    enum Suites{
        Clubs(String),
        Diamonds(String),
        Hearts(String),
        Spades(String),
    }

    enum Values{
        Ace(String, [usize; 2]),
        Two(String, usize),
        Three(String, usize),
        Four(String, usize),
        Five(String, usize),
        Six(String, usize),
        Seven(String, usize),
        Eight(String, usize),
        Nine(String, usize),
        Ten(String, usize),
        Jack(String, usize),
        Queen(String, usize),
        King(String, usize),
    }

    struct Card{
        suit: Suites,
        value: Values,

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
}
