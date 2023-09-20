# blackjack_app

# Welcome to Blackjack!
This is a simple application made in my first attempt at Rust allowing you to practice blackjack! It is completely text-based and using some *special* ASCII art,
looks as pretty as a text-based Blackjack table can look.
Take the time to test it out, have some fun with local friends (it is multiplayer up to 4 at a table), and hone your Blackjack skills!

## Details:
* Up to 4 Players (Not including the house)
* Runnable and playable all in a Command Line
* Developed for the purpose of practice, not betting

## Future Changes:
* Correct splitting
* Shoe changes (Different sizes of decks)
* UI??


## Notes:
#1: There is no betting. This is meant for practice, so focusing on money should not be a concern.

#2: The current shoe size is 1 deck (52 cards), but will be expanded later on to a multitude of sizes for better practice when in a competitive environment or a casino.

#3: Splitting will be added next, given time.


# How to Play
- Run the Program in a CMD line by navigating to main.rs in SRC.
- Pick how many players you want to play with.
- You will be dealt a hand of two cards, the goal is to hit 21 exactly or as close as possible without going over. Going over 21 (A "bust") is an immediate loss, you are fighting against the house, not each other.
- You will decide to "Hit" or to "Stay". Hitting will give you another card, increasing your total. Staying will pass you and not give you any more cards this hand. It is recommended by the house to stay once you hit 17 but that is only if you aren't feeling lucky.
- The house busting will make every player win, busting yourself will cause a loss.
- Cards are scored as listed on them, Jacks, Queens, and Kings are worth 10 points, and Aces are worth either 1 or 11 points, depending on what benefits your hand the best.
- Once a hand has been played, the deck is reshuffled and you may continue with a new hand, or quit the game to reassign new amounts of players, deck size (future improvement), or to go outside and do something else, like fishing!

## Structure
The current structure of the program includes some help from a Rust crate known as [deckofcards](https://docs.rs/deckofcards/latest/deckofcards/).
This wonderful crate has simple logic that could have been re-produced in my project, but if we have the resources, why not save some time and use an already-created deck!
The following is a layout of the program:

>Insert information regarding syntax and inner functions
