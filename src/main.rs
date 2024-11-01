// #[derive(Debug)]
// struct Deck {
//     cards: Vec<String>,
// }

// // 繼承
// impl Deck {
//     fn new_deck() -> Self {
//         let values = ["1", "2", "3", "4", "5", "6", "7", "8", "9", "10", "J", "Q", "K", "ACE"];
//         let suits = ["Heart", "Spade", "Diamond", "Club"];

//         let mut cards = vec![];

//         for suit in suits {
//             for value in values {
//                 let card = format!("{}-{}", suit, value);
//                 cards.push(card);
//             }
//         }

//         // return Deck { cards }; 可以直接移除 return 跟 ;
//         return Deck { cards };
//     }

//     fn shuffle(&self) {}
// }
// fn main() {
//     let deck = Deck::new_deck();
//     deck.shuffle();

//     println!("{:#?}", deck.cards)
// }

struct Deck {
    cards: Vec<String>,
}

fn main() {
    const a: char = '😊';
    println!("{:#?}", a)
}
