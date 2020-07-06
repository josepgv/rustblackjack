use rand::seq::SliceRandom;

enum HandResult {
    DealerBust,
    DealerTie,
    DealerLose,
    DealerWin,
    Unknown
}

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

fn play_hand(mut deck: Vec<i32>) -> HandResult {
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
    
    let d_sum = dealerhands.iter().sum::<i32>();
    let my_sum = myhands.iter().sum::<i32>();

    println!("Dealer: {:?} (suma: {})", dealerhands, d_sum);
    println!("Mine: {:?} (suma: {})", myhands, my_sum);

    if d_sum > 21 {
        return HandResult::DealerBust;
    }
    if d_sum == my_sum {
        return HandResult::DealerTie;
    }

    if d_sum > my_sum {
        return HandResult::DealerWin;
    }

    if d_sum < my_sum {
        return HandResult::DealerLose;
    }
    return HandResult::Unknown;
}

fn main() {
    let num_decks = 400000;
    let min_deck_len = 156;
    let num_simulations = 1000000;

    let mut deck = get_deck(num_decks);

    let mut wins = 0;
    let mut draws = 0;
    let mut loses = 0;

    for _i in 0..num_simulations {

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
        
        let d_sum = dealerhands.iter().sum::<i32>();
        let my_sum = myhands.iter().sum::<i32>();

        //println!("Dealer: {:?} (suma: {})", dealerhands, d_sum);
        //println!("Mine: {:?} (suma: {})\n", myhands, my_sum);

        if d_sum > 21 {
            wins += 1;
        }
        if d_sum == my_sum {
            draws += 1;
        }

        if d_sum > my_sum {
            loses += 1;
        }

        if d_sum < my_sum {
            wins += 1;
        }
        

        /*
        let deck_len = deck.len();
        if deck_len < min_deck_len {
            deck = get_deck(num_decks);
        }
       
        let result = play_hand(deck);

        match result {
            HandResult::DealerWin => loses += 1,
            HandResult::DealerBust => wins += 1,
            HandResult::DealerTie => draws += 1,
            HandResult::DealerLose => wins += 1,
            HandResult::Unknown => println!("WTF!"),
        }
         */
    }

    println!("Wins: {} - Draws: {} - Loses: {}", wins, draws, loses);


    
}
