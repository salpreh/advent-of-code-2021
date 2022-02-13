use common::{FileConfig, get_input_file_path, load_data};
use day_4::{BingoCard, BingoMatch};

fn main() {
    let config = FileConfig::default("./day_4");
    let (sequence, cards) = load_bingo_data(get_input_file_path(&config));

    let mut bingo_match = BingoMatch::new(cards);

    let mut winner: Option<&BingoCard> = None;
    let mut last_num = 0;
    for num in sequence {
        winner = bingo_match.mark(num);
        last_num = num;

        if winner.is_some() { break; }
    }

    match winner {
        Some(card) => println!(
            "There is a winner, score: {} (unmarked: {}, last_num: {})",
            card.sum_unmarked() * last_num,
            card.sum_unmarked(),
            last_num
        ),
        None => println!("No winner in match")
    };
}

fn load_bingo_data(path: &str) -> (Vec<u32>, Vec<BingoCard>) {
    let data = load_data(path);

    // First line is bingo number sequence
    let mark_sequence: Vec<u32> = data[0].split(",").into_iter()
        .map(|it| it.trim().parse::<u32>().unwrap())
        .collect();

    // Next lines are cards data
    let mut cards = Vec::new();
    let mut current_rows = Vec::new();
    for line in &data[2..] {
        if line.trim().is_empty() {
            cards.push(BingoCard::from(current_rows));
            current_rows = Vec::new();
            continue;
        }

        let row: Vec<u32> = line.split_whitespace().into_iter()
            .map(|it| it.trim().parse::<u32>().unwrap())
            .collect();

        current_rows.push(row);
    }

    // If file do not end with additional linebreak last card will not be added in iteration
    if !current_rows.is_empty() { cards.push(BingoCard::from(current_rows)) }

    (mark_sequence, cards)
}