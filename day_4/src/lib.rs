pub struct BingoCard {
    rows: Vec<Vec<(u32, bool)>>
}

/// Prints score of winning card
pub fn play_match(sequence: Vec<u32>, bingo_match: &mut BingoMatch) {

    let mut winner: Option<BingoCard> = None;
    let mut last_num = 0;
    for num in sequence {
        let winners = bingo_match.mark(num);
        last_num = num;

        if !winners.is_empty() {
            winner = Some(winners[0].clone());
            break;
        }
    }

    match winner {
        Some(card) => println!(
            "There is a winner, score: {} (unmarked: {}, last_num: {})",
            card.sum_unmarked() * last_num,
            card.sum_unmarked(),
            last_num
        ),
        None => println!("No winner in match")
    }

}

/// Prints score of last winning card
pub fn last_card_score(sequence: Vec<u32>, bingo_match: &mut BingoMatch) {
    let mut last_winner: Option<BingoCard> = None;
    let mut last_num = 0;

    for num in sequence {
        let winners = bingo_match.mark(num);
        winners.into_iter()
            .filter(|card| card.count_completed_lines() == 1)
            .for_each(|card| {
                last_num = num;
                println!("Completed count: {}", card.count_completed_lines());
                println!("Last selected board: {}, num: {}", card.sum_unmarked(), num);
                last_winner = Some(card.clone());
            })
    }

    match last_winner {
        Some(card) => println!(
            "There is a winner, score: {} (unmarked: {}, last_num: {})",
            card.sum_unmarked() * last_num,
            card.sum_unmarked(),
            last_num
        ),
        None => println!("No winner in match")
    };
}

impl BingoCard {
    pub fn from(rows: Vec<Vec<u32>>) -> BingoCard {
        let rows = rows.iter()
            .map(|r| {
                r.iter()
                    .map(|i| (*i, false))
                    .collect()
            })
            .collect();

        BingoCard { rows }
    }

    /// Get row values
    ///
    /// # Arguments
    /// * `row_n` - Row index starting by 0
    pub fn get_row(&self, row_n: usize) -> Option<&Vec<(u32, bool)>> {
        self.rows.get(row_n)
    }

    /// Get row values
    ///
    /// # Arguments
    /// * `row_n` - Row index starting by 0
    /// * `col_n` - Col index starting by 0
    pub fn get_value(&self, row_n: usize, col_n: usize) -> Option<&(u32, bool)> {
        self.rows.get(row_n).and_then(|row| {
            row.get(col_n)
        })
    }

    pub fn mark(&mut self, num: u32) -> bool {
        let mut mod_row: Option<usize> = None;
        let mut mod_col: Option<usize> = None;
        self.rows.iter_mut().enumerate().for_each(|(i, row)| {
            row.iter_mut().enumerate().for_each(|(j, it)| {
                let (row_num, marked) = it;

                if *row_num == num {
                    *marked = true;
                    mod_row = Some(i);
                    mod_col = Some(j);
                }
            })
        });

        match mod_row {
            Some(i) => self.is_row_completed(i)
                || self.is_col_completed(mod_col.unwrap()),
            None => false
        }
    }

    pub fn sum_unmarked(&self) -> u32 {
        self.rows.iter().map(|row| {
            row.iter()
                .filter(|(_, marked)| !*marked)
                .fold(0, |acc, (val, _)| acc + val)
        })
            .reduce(|acc, val| acc + val)
            .unwrap_or(0)
    }

    /// Counts completed rows and columns
    pub fn count_completed_lines(&self) -> u32 {
        if self.rows.len() == 0 || self.rows[0].len() == 0 { return 0}

        let mut col_status = vec![true;self.rows[0].len()];

        let count: u32 = self.rows.iter().map(|row| {
            row.iter().enumerate().fold(true, |agg, (i, it)| {
                let (_, marked) = it;
                col_status[i] &= *marked;
                agg && *marked
            })
        })
            .fold(0, |agg, row_res| agg + row_res as u32);

        count + col_status.iter()
            .fold(0, |agg, completed| agg + *completed as u32)
    }

    fn is_row_completed(&self, row_n: usize) -> bool {
        let row =  self.rows.get(row_n);
        if row.is_none() { return false }

        row.unwrap().iter()
            .map(|it| {
                let (_, marked) = it;
                marked
            })
            .fold(true, |acc, mk | acc && *mk)
    }

    fn is_col_completed(&self, col_n: usize) -> bool {
        self.rows.iter().all(|row| {
            let it = row.get(col_n);
            if it.is_none() { return false }

            it.unwrap().1
        })
    }
}

impl Clone for BingoCard {
    fn clone(&self) -> Self {
        let rows: Vec<Vec<(u32, bool)>> = self.rows.iter()
            .map(|row| row.clone())
            .collect();

        BingoCard { rows }
    }
}

pub struct BingoMatch {
    cards: Vec<BingoCard>
}

impl BingoMatch {
    pub fn new(cards: Vec<BingoCard>) -> BingoMatch {
        return BingoMatch { cards }
    }

    pub fn mark(&mut self, num: u32) -> Vec<&BingoCard> {
        let mut winners: Vec<&BingoCard> = Vec::new();
        self.cards.iter_mut().for_each(|card| {
            let finished_row = card.mark(num);
            if finished_row {
                winners.push(card);
            }
        });

        winners
    }
}

#[cfg(test)]
mod tests {
    use crate::BingoMatch;
    use super::BingoCard;

    #[test]
    fn should_mark_numbers() {
        let mut card = create_bingo_card();
        card.mark(11);

        let (num, marked) = card.get_value(1, 0).unwrap();

        assert_eq!(11, *num);
        assert!(marked);
    }

    #[test]
    fn should_return_true_when_row_completed() {
        let mut card = create_bingo_card();

        assert_eq!(false, card.mark(11));
        assert_eq!(false, card.mark(12));
        assert_eq!(true, card.mark(13));
    }

    #[test]
    fn should_return_true_when_col_completed() {
        let mut card = create_bingo_card();

        assert_eq!(false, card.mark(1));
        assert_eq!(false, card.mark(2));
        assert_eq!(false, card.mark(12));
        assert_eq!(true, card.mark(22));
    }

    #[test]
    fn should_return_sum_of_not_marked() {
        let mut card = create_bingo_card();
        card.mark(11);
        card.mark(12);
        card.mark(13);

        assert_eq!(72, card.sum_unmarked());
    }

    #[test]
    fn should_count_completed_rows() {
        let mut card = create_bingo_card();

        card.mark(11);
        card.mark(12);
        card.mark(13);
        assert_eq!(1, card.count_completed_lines());

        card.mark(1);
        card.mark(2);
        card.mark(3);
        assert_eq!(2, card.count_completed_lines());

        card.mark(22);
        assert_eq!(3, card.count_completed_lines());
    }

    #[test]
    fn should_return_winner_card() {
        let mut bingo_match = BingoMatch::new(vec![
            create_bingo_card_w_plus(0),
            create_bingo_card_w_plus(9)
        ]);

        assert!(bingo_match.mark(11).is_empty());

        assert!(bingo_match.mark(12).is_empty());

        let cards = bingo_match.mark(13);
        assert_eq!(1, cards.len());
        assert_eq!(72, cards[0].sum_unmarked());
    }

    fn create_bingo_card() -> BingoCard {
        create_bingo_card_w_plus(0)
    }

    fn create_bingo_card_w_plus(num: u32) -> BingoCard {
        let mut data = vec![
            vec![1, 2, 3],
            vec![11, 12, 13],
            vec![21, 22, 23]
        ];

        data.iter_mut().for_each(|row| {
            row.iter_mut().for_each(|it| {*it += num;})
        });

        BingoCard::from(data)
    }
}