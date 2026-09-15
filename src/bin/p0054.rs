use pe_rs::fetch_input;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Rank {
    HighCard,
    OnePair,
    TwoPairs,
    ThreeOfAKind,
    Straight,
    Flush,
    FullHouse,
    FourOfAKind,
    StraightFlush,
    RoyalFlush,
}

fn get_rank(card: &[(u8, &str)]) -> (Rank, Vec<u8>) {
    let (mut val, suit): (Vec<u8>, Vec<&str>) = card.iter().copied().unzip();
    val.sort_unstable_by(|a, b| b.cmp(a));

    let is_ace_low = val == [14, 5, 4, 3, 2];
    let is_straight = is_ace_low || val.windows(2).all(|w| w[0] - w[1] == 1);
    let straight_high = if is_ace_low { 5 } else { val[0] };
    let is_same_suit = suit.iter().all(|&s| s == suit[0]);

    if is_straight && is_same_suit && straight_high == 14 {
        return (Rank::RoyalFlush, vec![14]);
    } else if is_straight && is_same_suit {
        return (Rank::StraightFlush, vec![straight_high]);
    } else if is_same_suit {
        return (Rank::Flush, val);
    } else if is_straight {
        return (Rank::Straight, vec![straight_high]);
    }

    let mut cnt = [0_u8; 15];
    for &v in &val {
        cnt[v as usize] += 1;
    }

    let mut groups = (2u8..=14)
        .filter_map(|val| {
            let c = cnt[val as usize];
            (c > 0).then_some((c, val))
        })
        .collect::<Vec<_>>();

    groups.sort_unstable_by(|a, b| b.cmp(a));

    let rank = match groups.as_slice() {
        [(4, _), ..] => Rank::FourOfAKind,
        [(3, _), (2, _)] => Rank::FullHouse,
        [(3, _), ..] => Rank::ThreeOfAKind,
        [(2, _), (2, _), ..] => Rank::TwoPairs,
        [(2, _), ..] => Rank::OnePair,
        _ => Rank::HighCard,
    };

    let tie_breakers = groups.iter().map(|&(_, val)| val).collect();

    (rank, tie_breakers)
}

fn main() {
    let input = fetch_input("0054_poker.txt").unwrap();
    let cards = input
        .lines()
        .map(|row| {
            row.split_ascii_whitespace()
                .map(|ele| {
                    let (value, suit) = ele.split_at(1);
                    let parsed_val = match value {
                        "A" => 14,
                        "K" => 13,
                        "Q" => 12,
                        "J" => 11,
                        "T" => 10,
                        _ => value.parse::<u8>().unwrap(),
                    };
                    (parsed_val, suit)
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let mut ans = 0;
    for row in cards {
        if get_rank(&row[..5]) > get_rank(&row[5..]) {
            ans += 1;
        }
    }
    println!("{ans}");
}
