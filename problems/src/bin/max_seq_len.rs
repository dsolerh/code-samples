fn main() {
    let max_len = calculate_max_len(&DATA);

    println!("max len: {max_len}");
}

fn calculate_max_len(data: &[i32]) -> i32 {
    let mut data = data.iter().map(|&x| (x, 1)).collect::<Vec<_>>();

    for i in (0..data.len() - 1).rev() {
        println!("i: {} val: {} len: {}", i, data[i].0, data[i].1);
        let curr_val = data[i].0;
        let mut max_len = 0;
        for (val, len) in &data[i..] {
            if *val > curr_val && *len > max_len {
                max_len = *len;
            }
        }

        data[i].1 += max_len;
    }

    // for (i, (val, len)) in data.iter_mut() {
    //     println!("i: {i} val: {val} len: {len}");
    //     let ss = data.iter().take(*i).collect::<Vec<_>>();
    //     println!("vec: {:?}", ss);
    // }

    let mut max = 0;
    println!("");
    for (val, len) in data {
        println!("val: {val} len: {len}");
        if len > max {
            max = len;
        }
    }
    max
    // let mut i = 0;
    // let mut start = data[i];
    // let i_end = data.len();

    // let mut max_len = 0;

    // while i < i_end {
    //     start = data[i];
    //     let mut len = 1;
    //     for j in i + 1..i_end {
    //         let val = data[j];
    //         if val > start {
    //             start = val;
    //             len += 1;
    //         }
    //     }

    //     if len > max_len {
    //         max_len = len;
    //     }

    //     i += 1

    // }

    // max_len
}

const DATA: [i32; 539] = [
    34, 12, 57, 89, 43, 67, 23, 1, 8, 94, 29, 47, 11, 39, 72, 16, 66, 88, 45, 36, 75, 28, 55, 90,
    21, 83, 64, 53, 5, 14, 97, 19, 76, 3, 49, 71, 81, 7, 92, 62, 32, 78, 51, 20, 37, 26, 99, 58,
    69, 24, 84, 68, 31, 42, 82, 16, 98, 2, 55, 69, 18, 53, 77, 44, 6, 27, 76, 50, 35, 93, 17, 58,
    44, 10, 4, 61, 82, 96, 87, 63, 80, 74, 9, 30, 56, 46, 95, 13, 60, 86, 70, 33, 8, 59, 91, 12,
    65, 15, 40, 67, 98, 38, 79, 88, 54, 66, 39, 52, 73, 25, 95, 50, 89, 21, 37, 62, 47, 28, 75, 85,
    14, 59, 33, 86, 74, 31, 49, 29, 72, 56, 61, 57, 68, 11, 90, 48, 7, 60, 41, 84, 27, 46, 41, 24,
    78, 92, 66, 97, 20, 53, 43, 23, 83, 87, 54, 64, 69, 4, 77, 26, 94, 16, 71, 89, 5, 58, 43, 72,
    10, 28, 31, 99, 18, 39, 45, 65, 81, 25, 30, 3, 85, 50, 14, 19, 73, 93, 22, 63, 47, 8, 34, 44,
    51, 42, 94, 74, 6, 82, 36, 12, 56, 79, 68, 2, 69, 86, 16, 52, 88, 66, 23, 41, 74, 56, 17, 98,
    80, 8, 42, 70, 22, 60, 32, 45, 59, 87, 30, 4, 94, 75, 33, 71, 18, 49, 62, 46, 98, 58, 28, 96,
    7, 64, 40, 95, 21, 11, 66, 31, 74, 16, 50, 79, 37, 88, 3, 54, 13, 67, 63, 26, 41, 27, 52, 17,
    97, 84, 45, 61, 77, 9, 25, 91, 59, 44, 69, 19, 80, 29, 66, 84, 13, 74, 32, 65, 1, 42, 89, 51,
    11, 55, 39, 82, 14, 71, 55, 25, 58, 9, 85, 70, 20, 36, 7, 31, 81, 28, 46, 10, 65, 93, 37, 55,
    4, 38, 69, 18, 82, 27, 79, 50, 96, 66, 21, 44, 89, 64, 3, 53, 19, 58, 26, 74, 1, 68, 52, 98,
    23, 76, 60, 12, 50, 97, 39, 29, 82, 59, 8, 43, 67, 22, 48, 15, 31, 74, 7, 62, 91, 50, 66, 34,
    73, 41, 98, 57, 30, 55, 79, 15, 47, 83, 24, 89, 51, 93, 8, 33, 70, 46, 4, 90, 56, 12, 77, 21,
    68, 38, 79, 29, 45, 61, 18, 82, 65, 37, 83, 15, 93, 63, 9, 48, 43, 80, 16, 71, 30, 59, 66, 19,
    89, 53, 32, 92, 77, 10, 56, 36, 84, 44, 75, 57, 6, 67, 98, 45, 21, 83, 76, 28, 13, 50, 32, 64,
    56, 90, 35, 47, 73, 66, 40, 94, 59, 11, 52, 27, 55, 36, 63, 2, 68, 45, 86, 22, 41, 57, 35, 90,
    62, 30, 76, 18, 47, 12, 72, 31, 75, 6, 84, 67, 13, 49, 60, 39, 92, 52, 26, 71, 11, 86, 60, 93,
    37, 29, 63, 14, 38, 74, 80, 3, 19, 70, 67, 45, 92, 24, 58, 84, 8, 81, 56, 50, 89, 22, 94, 71,
    2, 47, 43, 27, 67, 90, 61, 4, 26, 80, 52, 13, 58, 95, 34, 91, 3, 76, 59, 11, 66, 28, 63, 54,
    36, 68, 29, 42, 21, 87, 52, 66, 17, 95, 60,
];

/*
    [10,    22, 9,  33, 21, 50, 41, 60, 80, 70, 75, 90]
    [7,     6,  6,  5,  5,  4,  4,  3,  1,  2,  1,  0]

    10:
        22:
            33:
                50:
                    60:
                        80:
                            90
                        70:
                            75:
                                90
                    80
                41:
                    60:
                        80
                60:
                    80
                80
            50:
                60:
                    80
                80
            41:
                60:
                    80
                80
            60:
                80
            80
        33:
            50:
                60:
                    80
                80
            60:
                80
            80
        21:
            50:
                60:
                    80
                80
            41:
                60:
                    80
                80
            60:
                80
            80
        50:
            60:
                80
            80
        60:
            80
        80
*/
