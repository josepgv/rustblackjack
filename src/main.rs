use rand::seq::SliceRandom;
use std::time::Instant;
use std::thread;

enum HandResult {
    DealerBust,
    DealerTie,
    DealerLose,
    DealerWin,
    Unknown
}

struct SimResult {
    wins: i32,
    draws: i32,
    loses: i32
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

    if my_sum > 21 {
        return HandResult::DealerWin;
    }

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

    let cpu_cores = num_cpus::get();
    let batch_size = num_simulations / cpu_cores;

    let now = Instant::now();

    let mut handles = vec![];

    for _t in 0..cpu_cores {
    
        let handle = thread::spawn(move || {
            let mut simresult = SimResult {
                wins: 0,
                draws: 0,
                loses: 0
            };
            let mut deck = get_deck(num_decks);
            println!("Starting thread {}", _t);
            for _i in 0..batch_size {
                if deck.len() < min_deck_len {
                    deck = get_deck(num_decks);
                }
                
                let result = play_hand(&mut deck);

                if _i == (batch_size/2) {
                    println!("Thread {} at 50%", _t);
                }
    
                match result {
                    HandResult::DealerWin => simresult.loses += 1,
                    HandResult::DealerBust => simresult.wins += 1,
                    HandResult::DealerTie => simresult.draws += 1,
                    HandResult::DealerLose => simresult.wins += 1,
                    HandResult::Unknown => println!("WTF!"),
                }
            }
            return simresult;
        });
        handles.push(handle);
    }

    let mut totals_sims = SimResult {
        wins: 0,
        draws: 0,
        loses: 0
    };

    for handle in handles {
        let info = handle.join().unwrap();
        totals_sims.wins += info.wins;
        totals_sims.draws += info.draws;
        totals_sims.loses += info.loses;
    }

    let elapsed = now.elapsed();

    println!("\nTotal simulations: {}", num_simulations);
    println!(" - Wins: {}\n - Draws: {}\n - Loses: {}\n", totals_sims.wins, totals_sims.draws, totals_sims.loses);
    println! ("Win percentage: {}" , ((totals_sims.wins as f32 / num_simulations as f32) * 100.00));
    println! ("Draw percentage: {}" , ((totals_sims.draws as f32 / num_simulations as f32) * 100.00));
    println! ("Lose percentage: {}" , ((totals_sims.loses as f32 / num_simulations as f32) * 100.00));
    println!("Elapsed time: {:?}", elapsed);
    
}
