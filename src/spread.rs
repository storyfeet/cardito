use card_format::Card;

pub struct SpreadIter<'a> {
    cards: &'a [Card],
    n: usize,
    pos: usize,
}

impl<'a> SpreadIter<'a> {
    pub fn new(cards: &'a [Card]) -> Self {
        let n = cards.get(0).map(|c| c.num).unwrap_or(0);
        SpreadIter { cards, n, pos: 0 }
    }
}

impl<'a> Iterator for SpreadIter<'a> {
    type Item = (&'a Card, usize, usize);
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if self.n > 0 {
                self.n -= 1;
                return self.cards.get(self.pos).map(|c| (c, self.pos, self.n));
            }
            self.pos += 1;
            self.n = self.cards.get(self.pos)?.num;
        }
    }
}
