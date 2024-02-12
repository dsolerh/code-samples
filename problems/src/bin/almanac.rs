// use rayon::prelude::*;
use std::str::FromStr;

fn main() {
    println!(
        "min location is: {}",
        calculate_almanac(
            SEEDS,
            SEED_TO_SOIL,
            SOIL_TO_FERTILIZER,
            FERTILIZER_TO_WATER,
            WATER_TO_LIGHT,
            LIGHT_TO_TEMPERATURE,
            TEMPERATURE_TO_HUMIDITY,
            HUMIDITY_TO_LOCATION
        )
        .unwrap()
    );
    println!(
        "min location in range is: {}",
        calculate_almanac_range(
            SEEDS_RANGE,
            SEED_TO_SOIL,
            SOIL_TO_FERTILIZER,
            FERTILIZER_TO_WATER,
            WATER_TO_LIGHT,
            LIGHT_TO_TEMPERATURE,
            TEMPERATURE_TO_HUMIDITY,
            HUMIDITY_TO_LOCATION
        )
        .unwrap()
    );
}

fn calculate_almanac_range(
    seeds_range: &[(i64, i64)],
    seed_to_soil_data: &str,
    soil_to_fertilizer_data: &str,
    fertilizer_to_water_data: &str,
    water_to_light_data: &str,
    light_to_temperature_data: &str,
    temperature_to_humidity_data: &str,
    humidity_to_location_data: &str,
) -> Result<i64, ParseAlmanacEntryError> {
    let seed_to_soil_map = seed_to_soil_data.parse::<AlmanacMap>()?;
    let soil_to_fertilizer_map = soil_to_fertilizer_data.parse::<AlmanacMap>()?;
    let fertilizer_to_water_map = fertilizer_to_water_data.parse::<AlmanacMap>()?;
    let water_to_light_map = water_to_light_data.parse::<AlmanacMap>()?;
    let light_to_temperature_map = light_to_temperature_data.parse::<AlmanacMap>()?;
    let temperature_to_humidity_map = temperature_to_humidity_data.parse::<AlmanacMap>()?;
    let humidity_to_location_map = humidity_to_location_data.parse::<AlmanacMap>()?;

    Ok(seeds_range
        .iter()
        .map(|&(start, len)| {
            let range = Range(start, start + len - 1);
            let soil = seed_to_soil_map.get_ranges(&[range]);
            let fertilizer = soil_to_fertilizer_map.get_ranges(&soil);
            let water = fertilizer_to_water_map.get_ranges(&fertilizer);
            let light = water_to_light_map.get_ranges(&water);
            let temperature = light_to_temperature_map.get_ranges(&light);
            let humidity = temperature_to_humidity_map.get_ranges(&temperature);
            let location = humidity_to_location_map.get_ranges(&humidity);

            // (seed, location)
            location.into_iter().map(|x| x.0).min().unwrap()
        })
        .min()
        .unwrap())
}

fn calculate_almanac(
    seeds: &[i64],
    seed_to_soil_data: &str,
    soil_to_fertilizer_data: &str,
    fertilizer_to_water_data: &str,
    water_to_light_data: &str,
    light_to_temperature_data: &str,
    temperature_to_humidity_data: &str,
    humidity_to_location_data: &str,
) -> Result<i64, ParseAlmanacEntryError> {
    let seed_to_soil_map = seed_to_soil_data.parse::<AlmanacMap>()?;
    let soil_to_fertilizer_map = soil_to_fertilizer_data.parse::<AlmanacMap>()?;
    let fertilizer_to_water_map = fertilizer_to_water_data.parse::<AlmanacMap>()?;
    let water_to_light_map = water_to_light_data.parse::<AlmanacMap>()?;
    let light_to_temperature_map = light_to_temperature_data.parse::<AlmanacMap>()?;
    let temperature_to_humidity_map = temperature_to_humidity_data.parse::<AlmanacMap>()?;
    let humidity_to_location_map = humidity_to_location_data.parse::<AlmanacMap>()?;

    Ok(seeds
        .iter()
        .map(|&seed| {
            let soil = seed_to_soil_map.get(seed);
            let fertilizer = soil_to_fertilizer_map.get(soil);
            let water = fertilizer_to_water_map.get(fertilizer);
            let light = water_to_light_map.get(water);
            let temperature = light_to_temperature_map.get(light);
            let humidity = temperature_to_humidity_map.get(temperature);
            let location = humidity_to_location_map.get(humidity);

            // (seed, location)
            location
        })
        // .inspect(|(seed, loc)| println!("seed: {} loc: {}", seed, loc))
        // .map(|(_, loc)| loc)
        .min()
        .unwrap())
}

#[derive(Debug, PartialEq)]
struct ParseAlmanacEntryError(String);

#[derive(Debug, PartialEq, Clone, Copy, Eq)]
struct Range(i64, i64);

impl Range {
    fn includes(&self, i: i64) -> bool {
        i >= self.0 && i <= self.1
    }

    fn includes_range(&self, range: &Self) -> bool {
        self.includes(range.0) && self.includes(range.1)
    }

    fn offset(&self, offset: i64) -> Self {
        Self(self.0 + offset, self.1 + offset)
    }

    // [    A  [ ]  B   ]
    // [    B  [ ]  A   ]
    fn intersect(&self, other: &Self) -> bool {
        self.includes(other.0) || self.includes(other.1)
    }

    fn expand(&mut self, other: &Self) {
        if other.0 < self.0 {
            self.0 = other.0
        }
        if other.1 > self.1 {
            self.1 = other.1;
        }
    }
}

#[derive(Debug, PartialEq)]
struct AlmanacEntry {
    src: Range,
    offset: i64,
}

impl FromStr for AlmanacEntry {
    type Err = ParseAlmanacEntryError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let nums = s
            .split(" ")
            .map(str::trim)
            .map(str::parse::<i64>)
            .collect::<Result<Vec<_>, _>>()
            .map_err(|_| ParseAlmanacEntryError(s.to_string()))?;
        let src_start = *nums.get(1).ok_or(ParseAlmanacEntryError(s.to_string()))?;
        let dst_start = *nums.get(0).ok_or(ParseAlmanacEntryError(s.to_string()))?;
        let len = *nums.get(2).ok_or(ParseAlmanacEntryError(s.to_string()))?;
        Ok(AlmanacEntry {
            src: Range(src_start, src_start + len - 1),
            offset: dst_start - src_start,
        })
    }
}

struct AlmanacMap {
    ranges: Vec<AlmanacEntry>,
}

impl AlmanacMap {
    fn get(&self, idx: i64) -> i64 {
        self.ranges
            .iter()
            .find(|entry| entry.src.includes(idx))
            .and_then(|x| Some(x.offset + idx))
            .unwrap_or(idx)
    }

    fn get_ranges(&self, src_ranges: &[Range]) -> Vec<Range> {
        let mut ranges = Vec::new();

        for range in src_ranges {
            let mut unexamined_ranges = vec![*range];

            for entry in &self.ranges {
                let range_b = if let Some(range) = unexamined_ranges.pop() {
                    range
                } else {
                    break;
                };

                let range_a = entry.src;
                //  A: the entry src range
                //  B: the src_range

                //  B might not intersect A
                if !range_a.intersect(&range_b) {
                    //  [ B ]  [   A   ] <- here wont be contained in any other A
                    if range_b.0 < range_a.0 {
                        ranges.push(range_b)
                    }
                    //  [   A   ]  [ B ] <- here B might be related to a subcequent A
                    if range_b.0 > range_a.0 {
                        unexamined_ranges.push(range_b);
                    }

                    continue;
                }

                // B might equal A
                if range_b == range_a {
                    ranges.push(range_a.offset(entry.offset));
                    continue;
                }

                // B might be included in A
                // in this case the src_range is contained inside the entry src range
                // [ A   [  B  ]  A     ]
                if range_a.includes_range(&range_b) {
                    ranges.push(range_b.offset(entry.offset));
                    continue;
                }

                //  B might englobe A
                //  [  B  [ A ]  B ] <- in this case B can be divided in 3 ranges and the las one needs to be further checked
                if range_b.includes_range(&range_a) {
                    if range_b.0 != range_a.0 {
                        ranges.push(Range(range_b.0, range_a.0 - 1));
                    }
                    ranges.push(range_a.offset(entry.offset));
                    unexamined_ranges.push(Range(range_a.1 + 1, range_b.1));
                    continue;
                }

                //  B is intercepting A
                //  [  B [ ]   A   ] <- here B can be divided in two ranges
                if range_b.0 < range_a.0 {
                    ranges.push(Range(range_b.0, range_a.0 - 1));
                    ranges.push(Range(range_a.0, range_b.1).offset(entry.offset));
                    continue;
                }
                //  [  A [ ]   B   ] <- here B can also be divided in two ranges but the second one should be checked
                if range_b.0 > range_a.0 {
                    ranges.push(Range(range_b.0, range_a.1).offset(entry.offset));
                    unexamined_ranges.push(Range(range_a.1 + 1, range_b.1));
                    continue;
                }
            }

            if let Some(range) = unexamined_ranges.pop() {
                ranges.push(range);
            }
        }

        ranges.sort_by(|a, b| a.0.cmp(&b.0));
        ranges.into_iter().fold(Vec::new(), |mut acc, x| {
            if let Some(last_range) = acc.last_mut() {
                if last_range.intersect(&x) || last_range.1 + 1 == x.0 {
                    last_range.expand(&x);
                    acc
                } else {
                    acc.push(x);
                    acc
                }
            } else {
                acc.push(x);
                acc
            }
        })
    }
}

impl FromStr for AlmanacMap {
    type Err = ParseAlmanacEntryError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut ranges = s
            .lines()
            .map(str::trim)
            .filter(|s| !s.is_empty())
            .map(str::parse::<AlmanacEntry>)
            .collect::<Result<Vec<_>, _>>()?;
        ranges.sort_by(|a, b| a.src.0.cmp(&b.src.0));

        Ok(Self { ranges })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_range_intersect() {
        // do intersect
        assert!(Range(1, 3).intersect(&Range(2, 5)));
        assert!(Range(1, 5).intersect(&Range(2, 3)));
        assert!(Range(3, 5).intersect(&Range(2, 3)));
        // do not intersect
        assert!(!Range(1, 2).intersect(&Range(3, 4)));
        assert!(!Range(3, 5).intersect(&Range(1, 2)));
    }

    #[test]
    fn test_almanac_entry_from_str() {
        assert_eq!(
            AlmanacEntry::from_str("10 20 30"),
            Ok(AlmanacEntry {
                src: Range(20, 49),
                offset: -10
            })
        )
    }

    #[test]
    fn test_almanac_map_from_str() {
        let data = "50 98 2\n52 50 10";
        let almanac = AlmanacMap::from_str(data).unwrap();

        assert_eq!(
            vec![
                AlmanacEntry {
                    src: Range(50, 59),
                    offset: 2
                },
                AlmanacEntry {
                    src: Range(98, 99),
                    offset: 50 - 98
                }
            ],
            almanac.ranges
        )
    }

    #[test]
    fn test_almanac_map_get() {
        let data = "50 98 2\n52 50 42";
        let map = AlmanacMap::from_str(data).unwrap();

        // no extra data
        assert_eq!(map.get(1), 1);
        // first range
        assert_eq!(map.get(98), 50);
        assert_eq!(map.get(99), 51);
        // second range
        assert_eq!(map.get(50), 52);
        assert_eq!(map.get(51), 53);
        assert_eq!(map.get(52), 54);
        assert_eq!(map.get(53), 55);
        assert_eq!(map.get(54), 56);
        assert_eq!(map.get(82), 84);

        let map = AlmanacMap::from_str(LIGHT_TO_TEMPERATURE).unwrap();

        assert_eq!(map.get(77), 45);
    }

    #[test]
    fn test_almanac_map_get_ranges() {
        let data = "50 98 2\n52 50 10";
        // [50 - 59] -> [52 - 61]
        // [60 - 97] -> [62 - 97]
        // [98 - 99] -> [50 - 51]
        let map = AlmanacMap::from_str(data).unwrap();

        assert_eq!(map.get_ranges(&[Range(52, 54)]), vec![Range(54, 56)]);

        // [50 - 59] [60 - 97] [98 - 99]
        //    [52 - 72]
        //    [52 - 59] [60 - 72]
        //    [54 - 61] [60 - 72]
        assert_eq!(map.get_ranges(&[Range(52, 72)]), vec![Range(54, 72)]);

        // [50 - 59] [60 - 97] [98 - 99]
        // [50 - 99]
        assert_eq!(map.get_ranges(&[Range(50, 99)]), vec![Range(50, 97)]);

        let map = AlmanacMap::from_str(SOIL_TO_FERTILIZER).unwrap();

        assert_eq!(map.get_ranges(&[Range(81, 94)]), vec![Range(81, 94)]);
    }

    #[test]
    fn test_calculate_almanac() {
        let location = calculate_almanac(
            TEST_SEEDS,
            SEED_TO_SOIL,
            SOIL_TO_FERTILIZER,
            FERTILIZER_TO_WATER,
            WATER_TO_LIGHT,
            LIGHT_TO_TEMPERATURE,
            TEMPERATURE_TO_HUMIDITY,
            HUMIDITY_TO_LOCATION,
        )
        .unwrap();

        assert_eq!(location, 35);
    }

    #[test]
    fn test_calculate_almanac_range() {
        let location = calculate_almanac_range(
            TEST_SEEDS_RANGE,
            SEED_TO_SOIL,
            SOIL_TO_FERTILIZER,
            FERTILIZER_TO_WATER,
            WATER_TO_LIGHT,
            LIGHT_TO_TEMPERATURE,
            TEMPERATURE_TO_HUMIDITY,
            HUMIDITY_TO_LOCATION,
        )
        .unwrap();

        assert_eq!(location, 46);
    }

    const TEST_SEEDS: &[i64] = &[79, 14, 55, 13];
    const TEST_SEEDS_RANGE: &[(i64, i64)] = &[(79, 14), (55, 13)];

    const SEED_TO_SOIL: &str = r"
    50 98 2
    52 50 48";

    const SOIL_TO_FERTILIZER: &str = r"
    0 15 37
    37 52 2
    39 0 15";

    const FERTILIZER_TO_WATER: &str = r"
    49 53 8
    0 11 42
    42 0 7
    57 7 4";

    const WATER_TO_LIGHT: &str = r"
    88 18 7
    18 25 70";

    const LIGHT_TO_TEMPERATURE: &str = r"
    45 77 23
    81 45 19
    68 64 13"; // [64 - 77) -> [68 - 81)

    const TEMPERATURE_TO_HUMIDITY: &str = r"
    0 69 1
    1 0 69";

    const HUMIDITY_TO_LOCATION: &str = r"
    60 56 37
    56 93 4";
}

const SEEDS: &[i64] = &[
    1347397244, 12212989, 2916488878, 1034516675, 2821376423, 8776260, 2240804122, 368941186,
    824872000, 124877531, 1597965637, 36057332, 4091290431, 159289722, 1875817275, 106230212,
    998513229, 159131132, 2671581775, 4213184,
];

const SEEDS_RANGE: &[(i64, i64)] = &[
    (1347397244, 12212989),
    (2916488878, 1034516675),
    (2821376423, 8776260),
    (2240804122, 368941186),
    (824872000, 124877531),
    (1597965637, 36057332),
    (4091290431, 159289722),
    (1875817275, 106230212),
    (998513229, 159131132),
    (2671581775, 4213184),
];

const SEED_TO_SOIL: &str = r"
2988689842 4194451945 100515351
2936009234 3353543976 52680608
588295233 2638661119 66434163
3932833115 2936009234 88315480
3525561241 3331695912 21848064
1622262003 1969921080 210668061
2160566101 909457337 162053391
1832930064 1887384181 82536899
3625461917 3024324714 307371198
3547409305 3680043285 78052612
1915466963 588295233 240773057
3089205193 3758095897 436356048
4021148595 3406224584 273818701
2156240020 2705095282 4326081
1164190025 2180589141 458071978
2477360206 829068290 80389047
2322619492 2709421363 154740714
654729396 1377923552 509460629
2557749253 1211672006 166251546
2724000799 1071510728 140161278";

const SOIL_TO_FERTILIZER: &str = r"
3961802244 3774724750 90737174
3164426550 3931513861 70563571
147221566 1279409424 704464
1394834067 2074132435 40845148
3795834030 2142537807 47621185
4083197470 4095560143 199407153
2722903919 2876212954 93296050
3467494732 2775293966 100918988
1809650294 1815421878 66426374
505665614 275280169 12031240
2142537807 4002077432 60985377
1577608496 331482268 177958690
2590855103 2196397738 132048816
1942888942 1978207624 24340152
756722275 120895382 4815243
3435289775 3899308904 32204957
1967229094 715059087 147748489
3955563498 2190158992 6238746
356096070 125710625 149569544
2520387506 2704826369 70467597
517696854 2041922514 32209921
1755567186 0 54083108
348569226 1280113888 7526844
761537518 1597328127 34415327
1114796271 1631743454 128592301
0 1494456232 102871895
1070625412 287311409 44170859
549906775 1287640732 206815500
102871895 710084154 4974933
1298474695 1881848252 96359372
3843455215 2549697361 112108283
3761987050 3865461924 33846980
4052539418 2674168317 30658052
1435679215 862807576 141929281
795952845 1004736857 274672567
3729489716 4063062809 32497334
2203523184 3085216895 316864322
2931907860 3402081217 11267883
107846828 2002547776 39374738
2816199969 2969509004 115707891
3568413720 3413349100 161075996
2943175743 2328446554 221250807
3234990121 3574425096 200299654
4282604623 2661805644 12362673
147926030 509440958 200643196
1876076668 54083108 66812274
1243388572 1760335755 55086123";

const FERTILIZER_TO_WATER: &str = r"
2460553918 850437816 63304366
1259757436 1986466040 193004355
2879827793 2638634287 61837387
39629536 0 3143529
2160922553 2535779758 68016930
2523858284 922523353 36811379
52449107 1199799263 207670511
2692884203 2603796688 34837599
3755186617 147251641 492169035
3266515480 3620937477 292130997
1596851845 4077877285 217090011
2727721802 913742182 8781171
3055087322 3913068474 164808811
2228939483 2179470395 229525704
3668162112 1052896909 87024505
3219896133 1539684314 30399976
1510550909 1679599925 86300936
0 3143529 39629536
2560669663 1407469774 132214540
1452761791 1142010145 57789118
260119618 2700471674 920465803
3250296109 1036677538 16219371
1813941856 1026924408 9753130
4247355652 2488168114 47611644
1823694986 959334732 67589676
2831305507 639420676 48522286
2736502973 52449107 94802534
1180585421 2408996099 79172015
2941665180 1765900861 113422142
2458465187 1139921414 2088731
3558646477 1570084290 109515635
1891284662 687942962 162494854
2053779516 1879323003 107143037";

const WATER_TO_LIGHT: &str = r"
2196302869 3170532562 121192468
3065704582 2916528129 254004433
2858667310 1154274853 9085577
3789349818 1163360430 70779786
2064226029 1434838179 90165206
1448515654 725716988 103420445
2690533041 2124509945 168134269
347894075 3345882022 38285799
3966625235 2618593488 35838159
4186823059 4134088017 89981817
1701108140 1088713231 65561622
2589948930 2518009377 100584111
4283393470 4230658428 11573826
0 3384167821 347894075
1638541363 2061943168 62566777
1296314418 573515752 152201236
1065865126 194284013 230449292
1766669762 1991838394 70104774
830054797 2292644214 110248534
3595088014 3291725030 12245358
1978385607 1905997972 85840422
3607333372 424733305 148782447
3511869770 2833309885 83218244
4134088017 4242232254 52735042
2154391235 3303970388 41911634
494720868 3732061896 335333929
1836774536 1234140216 141611071
2317495337 1633544379 272453593
4002463394 2402892748 64932431
940303331 829137433 125561795
1551936099 954699228 27518372
4276804876 4224069834 6588594
3756115819 2467825179 33233999
2867752887 0 19073457
386179874 1525003385 108540994
1579454471 1375751287 59086892
3319709015 2501059178 16950199
3336659214 19073457 175210556
3860129604 982217600 106495631
2886826344 2654431647 178878238";

const LIGHT_TO_TEMPERATURE: &str = r"
977891457 1797846421 453265654
3607226990 3913974738 161345346
2303244644 3266224873 12707372
1537599301 3278932245 264559714
354466514 3168465761 62294113
747844586 3543491959 55668994
2982698313 3599160953 269886589
2067998119 2251112075 27763866
59336731 230685734 266868096
3768572336 1500157846 31849471
3856743875 939537646 438223421
2095761985 59336731 146190926
326204827 4075320084 28261687
1502134302 3230759874 35464999
1431157111 868560455 70977191
2315952016 2579115195 227102616
1802159015 1532007317 265839104
2241952911 807268722 61291733
3297512098 497553830 309714892
852851251 4103581771 2643427
3252584902 3869047542 44927196
855494678 1377761067 122396779
2543054632 4106225198 139404427
2682459059 2278875941 300239254
803513580 4245629625 49337671
416760627 205527657 25158077
3800421807 2806217811 56322068
441918704 2862539879 305925882";

const TEMPERATURE_TO_HUMIDITY: &str = r"
3507573 490548898 11693081
545755853 699222305 569882925
3794976513 167435410 77260251
0 1526297837 3507573
1335234764 1766508370 36536350
2131780538 502241979 64264976
3707588652 1679120509 87387861
96082543 2288930706 220305732
1371771114 1529805410 106547120
481810045 3044354609 63945808
15200654 1426594789 7560739
3206337878 109359655 58075755
2445677382 2019348918 269581788
1909096745 3571407035 4209780
3138678479 4049712539 66833109
3400069156 3490205314 81201721
4084645800 3846924477 65868498
2353535073 1434155528 92142309
1716598669 3935506457 114206082
3264413633 4116545648 30901597
1913306525 3628450464 218474013
3205511588 3461720938 826290
2748703371 2770603565 195642711
2715259170 244990096 33149766
1478318234 3108300417 88629505
22761393 0 73321150
3295315230 3575616815 45887125
3872236764 278139862 212409036
2196045514 1269105230 157489559
2972004168 2509236438 123906332
316388275 3296299168 165421770
1685215763 3025113077 18894296
2748408936 244695661 294435
3704521599 4147447245 3067053
1215008024 566506955 120226740
1830804751 3912792975 22713482
1704110059 686733695 12488610
3488217401 1803044720 216304198
1872711004 3044007373 347236
1566947739 2652335541 118268024
3481270877 3621503940 6946524
2944346082 3462547228 27658086
3095910500 1636352530 42767979
1115638778 3196929922 99369246
3341202355 2966246276 58866801
1853518233 2633142770 19192771
1873058240 73321150 36038505";

const HUMIDITY_TO_LOCATION: &str = r"
336906655 0 11018487
4177510177 2085057023 105144397
1299579245 2985741466 175347598
643133711 2270603056 161424888
2404489601 1000033728 105953201
4282654574 2864154964 12312722
3409171342 3327025690 30826088
2119751049 2190201420 80401636
3393269098 3357851778 15902244
82121354 319849190 39107402
1953814423 3161089064 165936626
64524116 385149760 17597238
3439997430 2057119912 27937111
0 358956592 26193168
347925142 11018487 93152804
1484466972 2432027944 360604841
2510442802 643133711 347359888
26193168 402746998 38330948
1190305465 2946259551 39481915
2200152685 1105986929 204336916
3467934541 2792632785 71522179
804558599 3373754022 385746866
3539456720 1310323845 638053457
2857802690 3759500888 535466408
121228756 104171291 215677899
1229787380 2876467686 69791865
1474926843 990493599 9540129
1845071813 1948377302 108742610";
