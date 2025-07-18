fn main() {
    let mut sequence: [u32; 10] = [0; 10];
    sequence[0] = 0;
    sequence[1] = 1;

    let mut count = 0;

    while count < 8 {
        sequence[count + 2] += sequence[count] + sequence[count + 1];
        count += 1;
    }

    for number in sequence {
        println!("{number}");
    }
}