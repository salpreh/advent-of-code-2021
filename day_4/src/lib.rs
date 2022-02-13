pub struct BingoCard {
    rows: Vec<Vec<(u32, bool)>>
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
        let mut mod_index: Option<usize> = None;
        self.rows.iter_mut().enumerate().for_each(|(i, row)| {
            row.iter_mut().for_each(|it| {
                let (row_num, marked) = it;

                if *row_num == num {
                    *marked = true;
                    mod_index = Some(i);
                }
            })
        });

        match mod_index {
            Some(i) => self.is_row_completed(i),
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
}

pub struct BingoMatch {
    cards: Vec<BingoCard>
}

impl BingoMatch {
    pub fn new(cards: Vec<BingoCard>) -> BingoMatch {
        return BingoMatch { cards }
    }

    pub fn mark(&mut self, num: u32) -> Option<&BingoCard> {
        let mut winner: Option<&BingoCard> = None;
        self.cards.iter_mut().for_each(|card| {
            let finished_row = card.mark(num);
            if finished_row {
                winner = Some(card);
            }
        });

        winner
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
    fn should_return_sum_of_not_marked() {
        let mut card = create_bingo_card();
        card.mark(11);
        card.mark(12);
        card.mark(13);

        assert_eq!(72, card.sum_unmarked());
    }

    #[test]
    fn should_return_winner_card() {
        let mut bingo_match = BingoMatch::new(vec![
            create_bingo_card_w_plus(0),
            create_bingo_card_w_plus(9)
        ]);

        match bingo_match.mark(11) {
            Some(_) => assert!(false, "Should not return winner"),
            None => assert!(true)
        };

        match bingo_match.mark(12) {
            Some(_) => assert!(false, "Should not return winner"),
            None => assert!(true)
        };

        match bingo_match.mark(13) {
            Some(card) => assert_eq!(72, card.sum_unmarked()),
            None => assert!(true)
        };
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