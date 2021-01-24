const DATA: [i32; 153] = [
    1, 12, 2, 3, 1, 1, 2, 3, 1, 3, 4, 3, 1, 5, 0, 3, 2, 13, 1, 19, 1, 19, 10, 23, 1, 23, 13, 27, 1,
    6, 27, 31, 1, 9, 31, 35, 2, 10, 35, 39, 1, 39, 6, 43, 1, 6, 43, 47, 2, 13, 47, 51, 1, 51, 6,
    55, 2, 6, 55, 59, 2, 59, 6, 63, 2, 63, 13, 67, 1, 5, 67, 71, 2, 9, 71, 75, 1, 5, 75, 79, 1, 5,
    79, 83, 1, 83, 6, 87, 1, 87, 6, 91, 1, 91, 5, 95, 2, 10, 95, 99, 1, 5, 99, 103, 1, 10, 103,
    107, 1, 107, 9, 111, 2, 111, 10, 115, 1, 115, 9, 119, 1, 13, 119, 123, 1, 123, 9, 127, 1, 5,
    127, 131, 2, 13, 131, 135, 1, 9, 135, 139, 1, 2, 139, 143, 1, 13, 143, 0, 99, 2, 0, 14, 0
];

fn int_program(program: [i32; 153]) -> i32 {
    let mut i: usize = 0;
    let n = 153;
    let mut p: [i32; 153] = program;

    while i < n && p[i] < 3 {
        let p1: usize = p[i + 1] as usize;
        let p2: usize = p[i + 2] as usize;
        let p3: usize = p[i + 3] as usize;
        p[p3] = if p[i] == 1 { p[p1] + p[p2] } else { p[p1] * p[p2] };
        i = i + 4;
    }
    p[0]
}

fn noun_and_verb(program: [i32; 153]) -> i32 {
    for i in 1..100 {
        for j in 1..100 {
            let mut p: [i32; 153] = program;
            p[1] = i;
            p[2] = j;

            if int_program(p) == 19690720 {
                return 100 * i + j;
            }
        }

    }
    0
}

fn main() {
    println!("Part 1: {}", int_program(DATA));  // Part 1: 7210630
    println!("Part 2: {}", noun_and_verb(DATA));  // Part 2: 3892
}
