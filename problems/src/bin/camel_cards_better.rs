use std::{
    collections::HashMap,
    str::FromStr,
};

use lazy_static::lazy_static;

const VERSION: HandVersion = HandVersion::V2;

fn main() {
    let total = calculate_total_winnings(HANDS);

    println!("the total is {total}");

    // // let buf = BufReader::new(File::open("hands.txt").unwrap());
    // // let total: i32 = buf
    // //     .lines()
    // //     .filter_map(|s| s.ok())
    // //     .map(|s| {
    // //         let mut lines = s.split(' ');
    // //         let idx = lines.next().unwrap().parse::<i32>().unwrap();
    // //         let hand = lines.next().unwrap().parse::<Hand>().unwrap();
    // //         let amount = lines.next().unwrap().parse::<i32>().unwrap();

    // //         (idx, hand, amount)
    // //     })
    // //     // .inspect(|(idx, hand, amount)| println!("{} {} {}", idx, hand, amount))
    // //     .map(|(idx, _, amount)| (idx + 1) * amount)
    // //     .sum();

    // // println!("angel total is {total}");

    // let second_data = read_to_string("hands.txt").unwrap();

    // compare_hands(HANDS, &second_data)
}

// fn compare_hands(first_data: &str, second_data: &str) {
//     let mut first_data = first_data
//         .lines()
//         .map(|line| {
//             let mut parts = line.split(' ');
//             let hand = parts.next().unwrap().parse::<Hand>().unwrap();
//             let amount = parts.next().unwrap().parse::<i32>().unwrap();
//             (hand, amount)
//         })
//         .collect::<Vec<_>>();

//     first_data.sort_by(|a, b| a.0.cmp(&b.0));

//     let second_data = second_data
//         .lines()
//         .map(|s| {
//             let mut lines = s.split(' ');
//             let idx = lines.next().unwrap().parse::<i32>().unwrap();
//             let hand = lines.next().unwrap().parse::<Hand>().unwrap();
//             let amount = lines.next().unwrap().parse::<i32>().unwrap();

//             (idx, hand, amount)
//         })
//         .collect::<Vec<_>>();

//     for (d1, d2) in first_data
//         .into_iter()
//         .enumerate()
//         .map(|(idx, (hand, amount))| (idx, hand, amount))
//         .zip(second_data.into_iter())
//         .filter(|(a, b)| a.1 != b.1)
//     {
//         println!(
//             "diff | line: ({}/{}) hand: ({}/{}) amount: ({}/{})",
//             d1.0, d2.0, d1.1, d2.1, d1.2, d2.2
//         )
//     }
// }

fn calculate_total_winnings(data: &str) -> i32 {
    let mut data = data
        .lines()
        .filter(|line| !line.is_empty())
        .map(str::trim)
        .map(|line| {
            let mut parts = line.split(' ');
            let hand = parts.next().unwrap().parse::<Hand>().unwrap();
            let amount = parts.next().unwrap().parse::<i32>().unwrap();
            (hand, amount)
        })
        .collect::<Vec<_>>();

    data.sort();
    data.into_iter()
        .enumerate()
        // .inspect(|(index, (hand, value))| println!("{} {} {}", index, hand, value))
        .map(|(index, (_, value))| ((index + 1) as i32) * value)
        .sum()
}

lazy_static! {
    static ref CARD_MAP_V1: HashMap<char, u8> = {
        [
            '2', '3', '4', '5', '6', '7', '8', '9', 'T', 'J', 'Q', 'K', 'A',
        ]
        .into_iter()
        .enumerate()
        .map(|(i, ch)| (ch, i as u8))
        .collect::<HashMap<_, _>>()
    };
    static ref CARD_MAP_V2: HashMap<char, u8> = {
        [
            'J', '2', '3', '4', '5', '6', '7', '8', '9', 'T', 'Q', 'K', 'A',
        ]
        .into_iter()
        .enumerate()
        .map(|(i, ch)| (ch, i as u8))
        .collect::<HashMap<_, _>>()
    };
}

#[derive(Debug, PartialEq)]
struct ParseHandError;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct Card(char);

impl Card {
    fn from_char(c: char) -> Result<Self, ParseHandError> {
        match c {
            '2'..='9' => Ok(Card(c)),
            'J' => Ok(Card(c)),
            'A' => Ok(Card(c)),
            'K' => Ok(Card(c)),
            'Q' => Ok(Card(c)),
            'T' => Ok(Card(c)),
            _ => Err(ParseHandError),
        }
    }

    fn value(&self) -> &u8 {
        if VERSION == HandVersion::V1 {
            CARD_MAP_V1.get(&self.0).unwrap()
        } else {
            CARD_MAP_V2.get(&self.0).unwrap()
        }
    }
}

impl PartialOrd for Card {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.value().partial_cmp(other.value())
    }
}

impl Ord for Card {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.value().cmp(&other.value())
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Hand {
    cards: [Card; 5],
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum HandVersion {
    V1,
    V2,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum HandKind {
    HighCard = 0,
    OnePair = 1,
    TwoPair = 2,
    ThreeOrAKind = 3,
    FullHouse = 4,
    FourOfAKind = 5,
    FiveOfAKind = 6,
}

impl Hand {
    fn get_type(&self) -> HandKind {
        let mut cards_map = self.cards.iter().fold(HashMap::new(), |mut map, x| {
            map.entry(x.0).and_modify(|x| *x += 1).or_insert(1);
            map
        });

        let j_amount = if VERSION == HandVersion::V1 {
            0
        } else {
            cards_map.remove(&'J').unwrap_or(0)
        };

        let mut repeated_cards = cards_map.into_values().collect::<Vec<_>>();
        repeated_cards.sort();

        match repeated_cards.pop().unwrap_or_default() + j_amount {
            5 => HandKind::FiveOfAKind,
            4 => HandKind::FourOfAKind,
            3 => match repeated_cards.pop().unwrap_or_default() {
                2 => HandKind::FullHouse,
                1 => HandKind::ThreeOrAKind,
                _ => panic!("invalid case"),
            },
            2 => match repeated_cards.pop().unwrap_or_default() {
                2 => HandKind::TwoPair,
                1 => HandKind::OnePair,
                _ => panic!("invalid case"),
            },
            1 => HandKind::HighCard,
            _ => panic!("invalid case"),
        }
    }
}

impl FromStr for Hand {
    type Err = ParseHandError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            cards: s
                .chars()
                .take(5)
                .filter_map(|c| Card::from_char(c).ok())
                .take(5)
                .collect::<Vec<Card>>()
                .try_into()
                .map_err(|_| ParseHandError)?,
        })
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let kind1 = self.get_type();
        let kind2 = other.get_type();
        if kind1 == kind2 {
            for (card1, card2) in self.cards.iter().zip(other.cards.iter()) {
                if card1 != card2 {
                    return card1.cmp(card2);
                }
            }
        }
        kind1.cmp(&kind2)
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hand_get_type() {
        assert_eq!(
            Hand::from_str("22222").unwrap().get_type(),
            HandKind::FiveOfAKind
        );
        assert_eq!(
            Hand::from_str("222JJ").unwrap().get_type(),
            HandKind::FiveOfAKind
        );
        assert_eq!(
            Hand::from_str("JJJJJ").unwrap().get_type(),
            HandKind::FiveOfAKind
        );
        assert_eq!(
            Hand::from_str("22223").unwrap().get_type(),
            HandKind::FourOfAKind
        );
        assert_eq!(
            Hand::from_str("22JJ3").unwrap().get_type(),
            HandKind::FourOfAKind
        );
        assert_eq!(
            Hand::from_str("2JJJ3").unwrap().get_type(),
            HandKind::FourOfAKind
        );
        assert_eq!(
            Hand::from_str("3T3T3").unwrap().get_type(),
            HandKind::FullHouse
        );
        assert_eq!(
            Hand::from_str("3TJT3").unwrap().get_type(),
            HandKind::FullHouse
        );
        assert_eq!(
            Hand::from_str("33345").unwrap().get_type(),
            HandKind::ThreeOrAKind
        );
        assert_eq!(
            Hand::from_str("3JJ45").unwrap().get_type(),
            HandKind::ThreeOrAKind
        );
        assert_eq!(
            Hand::from_str("33442").unwrap().get_type(),
            HandKind::TwoPair
        );
        assert_eq!(
            Hand::from_str("3J425").unwrap().get_type(),
            HandKind::OnePair
        );
        assert_eq!(
            Hand::from_str("34425").unwrap().get_type(),
            HandKind::OnePair
        );
        assert_eq!(
            Hand::from_str("39425").unwrap().get_type(),
            HandKind::HighCard
        );

        // old ones
        assert_eq!(
            "AAAAA".parse::<Hand>().unwrap().get_type(),
            HandKind::FiveOfAKind
        );
        assert_eq!(
            "32T3K".parse::<Hand>().unwrap().get_type(),
            HandKind::OnePair
        );
        assert_eq!(
            "T55J5".parse::<Hand>().unwrap().get_type(),
            HandKind::FourOfAKind
        );
        assert_eq!(
            "KK677".parse::<Hand>().unwrap().get_type(),
            HandKind::TwoPair
        );
        assert_eq!(
            "KTJJT".parse::<Hand>().unwrap().get_type(),
            HandKind::FourOfAKind
        );
        assert_eq!(
            "QQQJA".parse::<Hand>().unwrap().get_type(),
            HandKind::FourOfAKind
        );
        assert_eq!(
            "J2332".parse::<Hand>().unwrap().get_type(),
            HandKind::FullHouse
        );
        assert_eq!(
            "2222Q".parse::<Hand>().unwrap().get_type(),
            HandKind::FourOfAKind
        )
    }

    #[test]
    fn test_hand_kind_order() {
        assert!(HandKind::FiveOfAKind > HandKind::HighCard);
        assert!(HandKind::FullHouse > HandKind::TwoPair);

        let mut kv = vec![
            HandKind::FiveOfAKind,
            HandKind::FourOfAKind,
            HandKind::ThreeOrAKind,
            HandKind::OnePair,
            HandKind::TwoPair,
        ];
        kv.sort();

        assert_eq!(
            kv,
            vec![
                HandKind::OnePair,
                HandKind::TwoPair,
                HandKind::ThreeOrAKind,
                HandKind::FourOfAKind,
                HandKind::FiveOfAKind
            ]
        );
    }

    #[test]
    fn test_hand_from_str() {
        let h = Hand::from_str("32T3K");

        assert_eq!(
            h,
            Ok(Hand {
                cards: [Card('3'), Card('2'), Card('T'), Card('3'), Card('K')]
            })
        );

        let h = "32t3_".parse::<Hand>();

        assert_eq!(h, Err(ParseHandError))
    }

    #[test]
    fn test_sort_hands() {
        let mut hands = vec![
            "J2883".parse::<Hand>().unwrap(),
            "J2524".parse::<Hand>().unwrap(),
            "J267K".parse::<Hand>().unwrap(),
        ];

        hands.sort();

        assert_eq!(
            hands,
            vec![
                "J267K".parse::<Hand>().unwrap(),
                "J2524".parse::<Hand>().unwrap(),
                "J2883".parse::<Hand>().unwrap(),
            ]
        )
    }

    #[test]
    fn test_calculate_total_winnings() {
        let total = calculate_total_winnings(TEST_DATA);

        assert_eq!(total, 6440);
    }

    const TEST_DATA: &str = r"
    32T3K 765
    T55J5 684
    KK677 28
    KTJJT 220
    QQQJA 483";
}

const HANDS: &str = r"
398KA 456
2J299 282
8939K 547
9TAA9 79
47TJ4 431
KJKKK 262
9Q75Q 826
JK9T3 213
82J22 956
T9QTT 251
93392 669
Q266J 278
682J6 385
33773 799
33233 648
9J999 543
657A5 540
34KA4 891
K269J 441
7442J 514
A3AAA 745
6Q8Q8 464
8AJ83 47
36J6K 474
5959T 505
222K8 615
J8228 208
J9285 892
TTATT 894
A96AT 256
9T979 472
T4447 922
5825J 106
8JA6Q 904
QJ777 217
33663 494
27252 640
Q8J63 587
53655 524
6T6TA 159
69J9T 915
T5T7K 229
KA694 504
45Q45 608
55A69 112
3TT49 552
T6T5A 601
QQQKK 180
3369J 488
TA92J 513
68T47 843
47JQT 574
7A274 604
4KKQQ 298
544JJ 878
92KKK 15
34833 502
55428 302
J7TT4 972
J9299 968
95A29 448
2785J 351
T8T8T 177
7Q767 974
3Q49Q 678
A3344 614
24455 304
Q8843 111
Q787J 10
9Q47T 576
8JT5A 423
QJ569 793
66242 563
QQ3J3 1000
4JJQQ 485
K57KK 875
98TJ4 172
45535 222
T8888 9
T3267 343
7A57A 190
A6A66 557
T97Q2 769
AAAA5 114
KKKAK 907
33533 117
6TA9T 559
J6496 933
9JAA5 32
99923 846
TJJT8 37
3AA3A 522
TK65K 714
4TQ44 363
J33A3 918
52885 286
ATJ8Q 436
TT7QQ 433
2K263 624
2AK22 258
AA77A 597
Q222K 653
6A459 220
AQ792 783
T5555 566
54444 564
68KT6 33
JAT28 491
QKKK3 750
4T334 884
K67KQ 154
82367 327
22AKJ 603
T3AT3 54
A9A9Q 729
994J6 517
QQ24Q 710
72J52 937
A8QKA 765
555T2 945
22772 396
43T96 391
2Q845 139
48854 632
25Q22 853
J69Q2 739
7K8T3 947
TQ278 508
22T22 345
K3T85 143
99KJ9 737
T9269 837
696J9 87
T4748 480
8J688 869
53KA6 538
T4534 585
22272 463
QJQ99 811
T4JT3 291
T4TT3 131
JKAAK 939
2KQA5 978
KQKKT 184
4J4Q8 903
KKKJJ 971
23J56 453
KKK6J 901
7JQ2Q 459
AAAAJ 690
95855 240
555QA 874
58AQ9 698
J7537 530
4QA2K 707
24T44 218
265K8 511
K4AK5 273
Q23QQ 983
665JA 492
777J4 5
87889 202
84444 760
A89A4 537
5JJJ2 509
886K6 44
AA997 27
7TQT2 786
66686 268
K7KK7 211
A4744 99
333J3 649
88A6T 212
Q7AK5 224
Q6QQQ 265
9Q995 358
AT843 373
75676 419
TTKTT 13
592AK 182
7TA58 610
J7322 905
J5A6K 704
JTTAA 357
7Q787 681
97766 71
86888 712
44A4A 264
3A535 680
7Q77Q 26
TTQQA 573
4TT42 142
88997 305
999KK 432
46TTT 446
JAAJJ 230
53892 165
33733 755
68668 181
45353 516
T73T7 209
35353 257
42272 285
3883Q 352
2AJAA 931
QJJ8Q 561
23342 162
7J8AT 595
44494 226
T43T7 272
QQ66Q 964
22223 296
8KQ39 353
JTTT7 751
8787J 866
2KKK6 926
36AQT 746
4J222 810
3KT56 126
K6K6K 660
QQ2AQ 790
9AKAK 521
44424 970
QJ2QQ 962
466A7 248
47K2J 215
42372 120
JTTQ3 949
93399 403
6J666 542
QA8AA 975
8J995 815
KKQQK 122
5KQ72 252
4T2TK 29
99J9J 205
KQQ6K 518
4KJQ8 394
J3JJA 203
7A8AJ 197
933KK 906
QQ354 460
K69TT 593
4Q34Q 620
A333Q 440
346T8 339
2Q294 409
3TAAJ 17
T5TTT 560
J5885 128
KA6A6 851
KQJ5K 194
6JK4Q 667
T2626 334
27377 380
43933 805
6Q68Q 133
Q2569 232
88887 376
QQ77Q 191
K27KJ 52
JJQ4A 481
K3274 798
J2Q2J 49
Q3Q4Q 963
2AAAA 754
939Q9 961
32553 470
5QQQQ 577
29KK2 287
99599 173
3838J 622
5TQ8Q 923
58558 98
Q997A 510
JJ922 709
Q67KT 461
2QKA8 764
7KKK9 928
88882 882
444AJ 546
Q2774 167
45KKK 668
32443 821
T4A76 307
2QQA2 324
T7353 850
55JJ5 553
555J2 28
4698K 443
77977 877
8T974 592
K474K 833
J44QA 636
QJ55A 697
742A6 787
7JJ77 584
64QK9 929
97JTJ 701
8A8AJ 468
28829 965
4K4TK 672
7J5QA 503
A7777 301
J565K 639
T6TJT 243
76477 756
3444J 598
43433 152
J7399 670
A525J 393
822T2 136
5687A 198
TJ2TT 483
972J2 844
6TJ6T 935
9QA82 759
KQ98J 45
53QQ5 966
K86T8 109
957TA 885
JK4KK 427
AAA8A 887
62626 705
3K29Q 223
73A9T 954
2T2JT 791
63J65 308
43888 161
73Q88 758
JQ557 938
32J42 618
5K635 148
562K4 870
AQAAT 176
86688 283
TT4JT 721
8KT94 145
78Q3J 895
5J355 527
KKJ22 637
999A9 654
6884J 941
36K7Q 890
TTQ65 312
A6759 471
64643 2
5Q5Q5 370
887AA 96
47478 325
3Q3AJ 477
AJKA8 420
25TQ2 716
54A7Q 156
422Q2 210
Q8232 487
8K52K 381
K9666 410
332JJ 55
Q9228 284
A8AQQ 845
5A65A 512
77384 398
8J382 386
22525 683
5AK55 412
36275 201
QK92A 12
3Q923 994
3J7AQ 736
92Q22 199
8JQ33 572
222AJ 361
89Q9K 355
576K2 551
7A82T 72
A97Q6 174
55353 84
5KKT9 46
55J55 782
4Q454 992
AAAA7 303
63TT6 830
8KJ35 839
J2KKK 921
946Q4 942
A3A5A 627
6KT4Q 534
KKJ9A 525
863T6 69
Q4444 719
293TA 789
TQTTT 196
7585J 428
2J2AQ 591
A5824 104
QJ83Q 713
65568 990
A9T48 580
K77J2 259
6342T 42
QK9QK 722
5K6A5 554
K222K 997
J8888 774
KT58K 21
JJ4T4 772
QKA88 451
3636J 48
K7529 687
K96QT 706
5JKK7 279
3333K 294
662Q6 88
QQQ55 364
82828 92
2TT2T 455
986A7 967
53252 227
T3J33 863
22JK6 141
696Q9 113
Q9TQT 836
6T66Q 596
QQ7QQ 164
25695 438
63QA5 802
4TT44 852
KKQKK 192
46TTK 70
32KTJ 768
A9AA2 809
88JT8 526
TT55T 16
J363A 812
944AA 619
AJ65A 401
49T44 741
J76A7 936
8TTJ8 388
2A244 219
QQK6Q 290
28K3T 717
586K7 86
TAKAT 880
92723 544
7AKQ4 873
4TTT8 276
34QT8 221
73777 407
894J4 175
8QQQQ 399
K45J2 239
K7777 214
2946T 338
KJ72J 562
5K3JK 132
J7722 415
ATAAA 362
44664 39
83588 51
393Q9 682
64A66 7
KJ224 31
45K74 541
22JJ2 497
9885A 555
KTJ96 899
T43Q2 195
286J2 700
3TJ24 35
85775 421
69T6A 605
29929 533
TTTT9 781
J333J 188
79Q66 115
66696 581
T3458 107
62555 74
AA66A 269
62296 703
35Q35 917
Q8888 633
T9TKK 630
QQ999 19
2259T 118
QA9Q7 776
32T22 950
KQJA7 467
89888 571
TTTT4 735
7T963 383
65KQK 193
9A996 246
9Q49T 817
K23K6 924
78J76 868
2J523 76
57765 943
QK77Q 392
666A3 807
2KKK2 958
662KJ 814
TAQ3T 267
865T4 818
97474 515
444JJ 664
748A7 536
4KK48 896
22582 189
55TT7 662
85658 359
384J4 677
TJTTJ 999
2Q22Q 796
Q2Q9Q 753
K5J55 8
TA5AT 628
A3955 185
Q888K 274
72AAK 838
9TQT9 641
8J686 482
777J7 89
8A68J 862
JQ392 650
2AJT5 65
Q664J 991
K8KK8 617
A22Q2 429
22737 82
2766A 770
774T9 613
KK3JK 245
9J698 333
53JQ4 948
A8A38 977
9T999 430
222JT 103
38Q2J 216
42KA4 80
68652 322
4634A 766
936JK 777
286K2 337
644JJ 1
J82K7 444
4AT63 982
43333 523
K6KKK 288
79494 400
3T446 663
5KJQ7 478
AA8K8 319
QQQQ2 865
JQQKK 575
64444 151
K2A23 646
838AQ 998
A3Q2Q 651
78246 266
828KA 688
4K4KJ 321
98898 404
A7227 724
QQAQ9 738
3J33Q 568
QAQQQ 439
94934 567
2726T 293
K495T 38
75777 41
666TT 744
26666 912
J25J3 349
77Q74 927
4T74K 102
7797T 897
J95K4 855
T7887 200
AA4AK 656
K585J 825
76227 631
KK766 861
T7886 135
85949 726
67KQ6 733
9A69A 629
T3233 170
7443T 823
T935T 171
85J4J 795
QK4QQ 384
3AQQ9 75
T5K34 841
T2K64 829
J3433 691
3KQA3 255
2KQ27 671
33J55 30
T2T22 748
KT33K 314
6564J 819
44J55 771
6A666 68
K8888 638
3JQ52 590
5552Q 447
TTJQQ 387
7K227 808
54J5J 253
9293T 888
9Q779 372
K322K 801
QK457 157
QQQ99 859
2KKKK 634
TQ8QQ 300
5A899 402
72245 411
6AKJ9 449
39698 602
65995 24
52553 295
8J7T8 405
93J99 731
T555J 740
JA9TQ 976
QJ5A4 81
59939 910
89J88 331
K35Q9 689
K4K94 490
53AA5 66
Q6Q66 6
J9KKK 803
744J7 178
3QKQ3 797
5K9A8 119
76JA5 476
46776 90
8Q289 925
6794T 160
94A97 686
K8K33 271
2KK98 62
44268 599
82T82 847
9QK75 702
88885 249
7T77T 153
24452 408
77733 495
T2TTT 378
645KQ 344
7TJ46 241
J5AAA 466
KKJA6 586
7A39K 275
J555Q 607
22AAT 742
7555K 469
55595 231
TKA5K 317
6A6A3 609
7J979 496
8JJJJ 842
TQTJT 860
3T352 78
88T26 832
32KT6 140
26957 462
38833 779
A3JJA 623
963QJ 980
53KA4 207
K8Q97 635
967J6 137
8T668 437
93762 281
Q7Q74 280
353J3 908
TQK66 382
3K25A 985
66J6J 395
3QQQK 445
4J2T9 187
KTJ7K 261
QJTT4 883
T9TJ9 360
T2658 588
J4T45 163
6A532 685
4256A 85
TTTKK 371
Q4343 757
J6727 840
A844A 506
46664 959
K7524 289
A6KT3 747
9537T 73
2Q9Q2 237
2K386 647
88787 277
343J7 775
4JQQ3 330
K3K3K 377
J7555 953
K4999 108
T967T 940
Q54Q7 493
AK8AA 499
Q8AT6 556
J3Q3K 166
8J8J8 179
77727 911
Q99Q4 695
39Q9J 375
736K5 397
AAA76 900
29854 749
55K5K 864
6466T 693
9A29A 788
3T3JT 889
5J33J 369
9J955 589
8K8K8 250
6K33A 233
64J95 824
J4444 645
2T327 715
93339 11
437T2 500
7AAJJ 879
T6T56 625
J74A8 655
57J84 952
KKTKT 582
47774 920
J8783 134
66996 661
8Q896 93
4A8T7 611
J49J4 548
552QQ 856
994A5 473
444A4 501
35478 110
Q4K23 909
44434 225
84648 63
AJAAJ 235
J48T6 951
3332J 365
TJ3T7 996
QAAAA 565
6Q674 626
449KJ 986
TTJKT 416
4232Q 785
356JJ 105
34T5T 727
TKT6T 489
5J5A6 734
838T8 763
6K944 263
TQQQ2 406
JA66Q 762
K3K32 150
T5TTQ 858
Q65AA 659
Q7A85 234
A5JTA 752
A5QQQ 238
948Q3 4
6J869 367
374JA 56
84TQA 318
89J98 993
T7837 916
5555K 138
83533 320
9J66A 465
KTTK4 366
6JT72 97
888A2 368
9J947 424
38737 732
75K6T 761
T4QQ4 728
Q926K 549
5T76Q 898
JQQJQ 780
68A77 930
5K59A 987
79979 442
47484 94
854AJ 457
97582 616
5Q9T4 144
26878 484
JTQ67 479
T2T72 18
66Q3Q 354
55752 168
65666 820
TT5T7 244
8T8KT 621
74J69 857
86973 872
TTTJT 315
557KK 725
635K6 730
AATAJ 792
84888 743
42944 989
882TT 389
K4799 806
423K8 957
7875J 158
8448K 822
6K99J 299
KKK48 475
3K33K 254
6A482 960
8T5AQ 848
Q44QQ 946
42KK2 913
Q2Q23 934
555A5 969
8KKKJ 59
626J2 675
686TJ 673
KA7KK 699
67785 23
3J363 569
888AA 834
36332 413
848KK 535
Q9J55 578
Q387A 124
89899 955
Q3AQA 50
52292 335
5KKKK 332
53555 867
83J33 854
995Q7 125
KKJ7K 665
TTTT8 247
5JAAK 718
55565 813
8JTTA 328
QQ9K9 458
99ATT 228
QTK6Q 652
6667J 827
JK788 871
448Q4 129
3494A 486
6A86A 83
J2J2J 876
67667 979
Q8234 711
AAQQQ 43
7QA7A 147
66646 186
AA4AA 77
A4222 390
35959 346
44423 902
666K6 529
99994 454
877J4 91
KTKK2 379
3Q7Q6 784
342K2 550
9JQT3 310
56A65 309
TQ54K 973
T4A47 657
46KKK 450
777T7 519
55293 340
3Q93Q 127
6A9J9 919
3A9JJ 452
A42T6 183
K33K7 242
T65Q4 323
695J9 658
J72K3 835
555T4 684
53K7T 40
J3Q5T 606
9A736 988
JT577 95
Q3333 804
KT9J4 130
J4KA4 849
A6AAA 36
AA8JA 767
3Q3Q2 146
TT522 886
42322 881
4395J 121
Q4T92 169
26922 507
363T6 831
67KAT 425
2T74J 64
7J755 696
AA3Q5 67
2999A 20
47737 932
Q23TK 816
58AJJ 270
KK23J 316
T3782 674
5K5KK 61
6AA76 57
Q5555 676
Q4558 531
T2TQT 417
QQ3QQ 583
3J733 123
7JQ44 297
AQQAA 53
QT9KA 116
79T86 342
JJJJJ 356
8AJQ9 666
6Q353 528
K27KK 329
8QJ88 545
75775 22
T6862 539
98745 336
9A99A 520
7373J 720
TTTT7 58
32J22 692
JQT8T 558
KKK4T 594
666AT 944
8A888 414
4TK37 347
Q9999 426
Q5Q33 341
227J2 643
97767 828
85399 570
83QQQ 292
4Q574 326
33T3T 723
84T46 3
25Q57 149
55999 25
67549 984
99966 995
7JJ7K 498
JQQQQ 204
7A7AJ 236
44849 34
AJ2A3 418
TK6QJ 612
33Q3Q 800
J83A3 206
T2359 306
A347A 435
KAAAA 101
7J7J4 708
T499T 679
99969 981
222J2 644
AAKTA 434
K88QK 778
K4J67 532
6JA9T 14
T93Q7 579
44247 794
75547 642
T383J 350
77JJJ 60
6QQ8Q 773
24J43 694
Q54J6 422
QQ5J5 914
66667 155
9KKKQ 893
4K4K4 600
QQJ9J 100
TJKQK 260
44945 374
K2AT3 313
33363 311
56655 348";
