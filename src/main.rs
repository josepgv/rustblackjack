use rand::seq::SliceRandom;

fn get_deck(num_decks: i32) -> Vec<i32> {
    let mut deck: Vec<i32> = Vec::new();
    for _i in 1..(4*num_decks) {
        for j in 1..12 {
            deck.push(j)
        }
    }
    let mut rng = rand::thread_rng();

    deck.shuffle(&mut rng);
    deck
}

fn main() {
    let mut deck = get_deck(4);
    let mut dealerhands: Vec<i32> = Vec::new();
    let mut myhands: Vec<i32> = Vec::new();
//println!("{:?}", deck);
    dealerhands.push(deck.pop().unwrap());
    myhands.push(deck.pop().unwrap());
    dealerhands.push(deck.pop().unwrap());
    myhands.push(deck.pop().unwrap());

    while myhands.iter().sum::<i32>() < 12 {
        myhands.push(deck.pop().unwrap());
    }

    while dealerhands.iter().sum::<i32>() < 18 {
        let mut exit = false;

        if dealerhands.iter().sum::<i32>() == 17 {
            exit = true;
            for i in 0..dealerhands.len() {
                if dealerhands[i] == 11 {
                    exit = false;
                    dealerhands[i] = 1;
                }
            }
        }

        if exit == true {
            break;
        }

        dealerhands.push(deck.pop().unwrap());
    }
    
    println!("Dealer: {:?} (suma: {})", dealerhands, dealerhands.iter().sum::<i32>());
    println!("Mine: {:?} (suma: {})", myhands, myhands.iter().sum::<i32>());
}
