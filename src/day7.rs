use std::cmp::Ordering;
use std::collections::HashMap;
use std::fs;
use std::io::{BufRead, BufReader};
use std::ops::Deref;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Copy, Clone)]
enum Card {
  WithoutJoker(CardWithoutJoker),
  WithJoker(CardWithJoker),
}

impl Card {
  fn parse(c: char, joker: bool) -> Card {
    if joker {
      Card::WithJoker(CardWithJoker::parse(c))
    } else {
      Card::WithoutJoker(CardWithoutJoker::parse(c))
    }
  }
}

impl ToString for Card {
  fn to_string(&self) -> String {
    match self {
      Card::WithoutJoker(c) => c.to_string(),
      Card::WithJoker(c) => c.to_string(),
    }
  }
}

#[derive(Eq, Ord, PartialEq, PartialOrd, Debug, Hash, Copy, Clone)]
enum CardWithoutJoker {
  Ace,
  King,
  Queen,
  Jack,
  Ten,
  Nine,
  Eight,
  Seven,
  Six,
  Five,
  Four,
  Three,
  Two,
}

#[derive(Eq, Ord, PartialEq, PartialOrd, Debug, Hash, Copy, Clone)]
enum CardWithJoker {
  Ace,
  King,
  Queen,
  Ten,
  Nine,
  Eight,
  Seven,
  Six,
  Five,
  Four,
  Three,
  Two,
  Joker,
}

impl CardWithoutJoker {
  fn parse(c: char) -> CardWithoutJoker {
    match c {
      'A' => CardWithoutJoker::Ace,
      'K' => CardWithoutJoker::King,
      'Q' => CardWithoutJoker::Queen,
      'J' => CardWithoutJoker::Jack,
      'T' => CardWithoutJoker::Ten,
      '9' => CardWithoutJoker::Nine,
      '8' => CardWithoutJoker::Eight,
      '7' => CardWithoutJoker::Seven,
      '6' => CardWithoutJoker::Six,
      '5' => CardWithoutJoker::Five,
      '4' => CardWithoutJoker::Four,
      '3' => CardWithoutJoker::Three,
      '2' => CardWithoutJoker::Two,
      x => panic!("Invalid card: {}", x),
    }
  }
}

impl ToString for CardWithoutJoker {
  fn to_string(&self) -> String {
    match self {
      CardWithoutJoker::Ace => "A".to_string(),
      CardWithoutJoker::King => "K".to_string(),
      CardWithoutJoker::Queen => "Q".to_string(),
      CardWithoutJoker::Jack => "J".to_string(),
      CardWithoutJoker::Ten => "T".to_string(),
      CardWithoutJoker::Nine => "9".to_string(),
      CardWithoutJoker::Eight => "8".to_string(),
      CardWithoutJoker::Seven => "7".to_string(),
      CardWithoutJoker::Six => "6".to_string(),
      CardWithoutJoker::Five => "5".to_string(),
      CardWithoutJoker::Four => "4".to_string(),
      CardWithoutJoker::Three => "3".to_string(),
      CardWithoutJoker::Two => "2".to_string(),
    }
  }
}

impl CardWithJoker {
  fn parse(c: char) -> CardWithJoker {
    match c {
      'A' => CardWithJoker::Ace,
      'K' => CardWithJoker::King,
      'Q' => CardWithJoker::Queen,
      'T' => CardWithJoker::Ten,
      '9' => CardWithJoker::Nine,
      '8' => CardWithJoker::Eight,
      '7' => CardWithJoker::Seven,
      '6' => CardWithJoker::Six,
      '5' => CardWithJoker::Five,
      '4' => CardWithJoker::Four,
      '3' => CardWithJoker::Three,
      '2' => CardWithJoker::Two,
      'J' => CardWithJoker::Joker,
      x => panic!("Invalid card: {}", x),
    }
  }
}

impl ToString for CardWithJoker {
  fn to_string(&self) -> String {
    match self {
      CardWithJoker::Ace => "A".to_string(),
      CardWithJoker::King => "K".to_string(),
      CardWithJoker::Queen => "Q".to_string(),
      CardWithJoker::Ten => "T".to_string(),
      CardWithJoker::Nine => "9".to_string(),
      CardWithJoker::Eight => "8".to_string(),
      CardWithJoker::Seven => "7".to_string(),
      CardWithJoker::Six => "6".to_string(),
      CardWithJoker::Five => "5".to_string(),
      CardWithJoker::Four => "4".to_string(),
      CardWithJoker::Three => "3".to_string(),
      CardWithJoker::Two => "2".to_string(),
      CardWithJoker::Joker => "J".to_string(),
    }
  }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum HandValue {
  FiveOfAKind,
  FourOfAKind,
  FullHouse,
  ThreeOfAKind,
  TwoPairs,
  OnePair,
  HighCard,
}

impl HandValue {
  fn new(cards: Vec<Card>, joker: bool) -> HandValue {
    let mut cards = cards;
    cards.sort();

    let mut counts = HashMap::new();
    for card in cards {
      let count = counts.entry(card).or_insert(0);
      *count += 1;
    }

    if joker {
      let mut joker_count = 0;
      for (card, count) in counts.iter() {
        if *card == Card::WithJoker(CardWithJoker::Joker) {
          joker_count = *count;
        }
      }

      counts.remove(&Card::WithJoker(CardWithJoker::Joker));
      // add the joker counts to the highest card in the counts
      let mut highest_card = Card::WithoutJoker(CardWithoutJoker::Two);
      let mut highest_count = 0;
      for (card, count) in counts.iter() {
        if *count > highest_count {
          highest_card = *card;
          highest_count = *count;
        }
      }

      let count = counts.entry(highest_card).or_insert(0);
      *count += joker_count;
    }

    let mut counts = counts.into_iter().collect::<Vec<(Card, u32)>>();
    counts.sort_by(|a, b| b.1.cmp(&a.1));
    match counts[0].1 {
      5 => HandValue::FiveOfAKind,
      4 => HandValue::FourOfAKind,
      3 => {
        if counts[1].1 == 2 {
          HandValue::FullHouse
        } else {
          HandValue::ThreeOfAKind
        }
      }
      2 => {
        if counts[1].1 == 2 {
          HandValue::TwoPairs
        } else {
          HandValue::OnePair
        }
      }
      _ => HandValue::HighCard,
    }
  }
}

#[derive(Debug)]
struct Cards(Vec<Card>);

impl PartialEq for Cards {
  fn eq(&self, other: &Self) -> bool {
    self.0.iter().all(|c| other.0.contains(c))
  }
}

impl Eq for Cards {}

impl PartialOrd for Cards {
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    Some(self.cmp(other))
  }
}

impl Ord for Cards {
  fn cmp(&self, other: &Self) -> Ordering {
    let self_cards = self.0.clone();
    let other_cards = other.0.clone();

    for (self_card, other_card) in self_cards.iter().zip(other_cards.iter()) {
      match self_card.cmp(other_card) {
        Ordering::Equal => continue,
        Ordering::Less => return Ordering::Less,
        Ordering::Greater => return Ordering::Greater,
      }
    }

    Ordering::Equal
  }
}

impl Deref for Cards {
  type Target = Vec<Card>;

  fn deref(&self) -> &Self::Target {
    &self.0
  }
}

impl Cards {
  fn iter(&self) -> std::slice::Iter<Card> {
    self.0.iter()
  }

  fn contains(&self, card: &Card) -> bool {
    self.0.contains(card)
  }
}

#[derive(Debug)]
struct Hand {
  cards: Cards,
  value: HandValue,
  bid: u32,
}

impl Hand {
  fn new(cards: &Vec<Card>, bid: u32, joker: bool) -> Hand {
    Hand {
      cards: Cards(cards.clone()),
      value: HandValue::new(cards.clone(), joker),
      bid,
    }
  }

  // fn debug(&self) {
  //   println!(
  //     "Cards: {} ({:?})",
  //     self.cards.iter().map(|c| c.to_string()).collect::<String>(),
  //     self.value
  //   );
  // }
}

impl PartialEq for Hand {
  fn eq(&self, other: &Self) -> bool {
    self.value == other.value && self.cards.iter().all(|c| other.cards.contains(c))
  }
}

impl Eq for Hand {}

impl PartialOrd for Hand {
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    Some(self.cmp(other))
  }
}

impl Ord for Hand {
  fn cmp(&self, other: &Self) -> Ordering {
    if self.value == other.value {
      self.cards.cmp(&other.cards)
    } else {
      self.value.cmp(&other.value)
    }
  }
}

fn parse_input(input: &str, joker: bool) -> Hand {
  let (cards, bid) = input.split_once(" ").unwrap();
  let cards = cards
    .chars()
    .map(|c| Card::parse(c, joker))
    .collect::<Vec<Card>>();

  let bid = bid.parse::<u32>().unwrap();

  Hand::new(&cards, bid, joker)
}

fn solve(input_file_path: &str, part: &str) -> u32 {
  let mut hands = BufReader::new(fs::File::open(input_file_path).unwrap())
    .lines()
    .map(|line| parse_input(&line.unwrap(), part == "b"))
    .collect::<Vec<Hand>>();

  hands.sort();
  hands.reverse();
  // hands.iter().for_each(|h| h.debug());

  hands
    .iter()
    .enumerate()
    .map(|(i, hand)| hand.bid * (i as u32 + 1))
    .sum()
}

pub fn solve_a(input_file_path: &str) -> u64 {
  solve(input_file_path, "a").try_into().unwrap()
}

pub fn solve_b(input_file_path: &str) -> u64 {
  solve(input_file_path, "b").try_into().unwrap()
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_solve_a() {
    assert_eq!(solve_a(".\\src\\test_input\\day7.txt"), 6440);
  }

  #[test]
  fn test_solve_b() {
    assert_eq!(solve_b(".\\src\\test_input\\day7.txt"), 5905);
  }
}
