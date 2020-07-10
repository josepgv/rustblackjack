use rand::seq::SliceRandom;
use std::time::Instant;
use std::sync::{Arc, Mutex};
use std::thread;
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

fn play_hand(deck: &mut Vec<i32>) -> HandResult {
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
    //println!("Mine: {:?} (suma: {})", myhands, my_sum);

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
    let num_decks = 4;
    let min_deck_len = 126;
    let num_simulations = 250000;

    let cpu_cores = 4;
    let batch_size = num_simulations / cpu_cores;

    let now = Instant::now();

    let wins = Arc::new(Mutex::new(0));
    let draws = Arc::new(Mutex::new(0));
    let loses = Arc::new(Mutex::new(0));

    //let mut deck = get_deck(num_decks);

    let mut handles = vec![];

    for _t in 0..cpu_cores {
        let wins = Arc::clone(&wins);
        let draws = Arc::clone(&draws);
        let loses = Arc::clone(&loses);
    
        let handle = thread::spawn(move || {
            let mut num_wins = wins.lock().unwrap();
            let mut num_draws = draws.lock().unwrap();
            let mut num_loses = loses.lock().unwrap();
            let mut deck = get_deck(num_decks);
            println!("Starting thread");
            for _i in 0..batch_size {
                if deck.len() < min_deck_len {
                    deck = get_deck(num_decks);
                }
                
                let result = play_hand(&mut deck);
    
                match result {
                    HandResult::DealerWin => *num_loses += 1,
                    HandResult::DealerBust => *num_wins += 1,
                    HandResult::DealerTie => *num_draws += 1,
                    HandResult::DealerLose => *num_wins += 1,
                    HandResult::Unknown => println!("WTF!"),
                }
            }
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    let elapsed = now.elapsed();

    println!("\nTotal simulations: {}", num_simulations);
    println!(" - Wins: {}\n - Draws: {}\n - Loses: {}\n", *wins.lock().unwrap(), *draws.lock().unwrap(), *loses.lock().unwrap());
    println! ("Win percentage: {}" , ((*wins.lock().unwrap() as f32 / num_simulations as f32) * 100.00));
    println! ("Draw percentage: {}" , ((*draws.lock().unwrap() as f32 / num_simulations as f32) * 100.00));
    println! ("Lose percentage: {}" , ((*loses.lock().unwrap() as f32 / num_simulations as f32) * 100.00));
    println!("Elapsed time: {:?}", elapsed);
    
}
