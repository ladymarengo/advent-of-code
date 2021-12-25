/* Rules are made by reverse ingeneering the input ("input/24" with comments after each step) 

Rules:
w4 = w3 + 8
w7 = w6 - 2
w9 = w8 + 7
w10 = w5
w11 = w2 - 5
w12 = w1 + 2
w13 = w0 - 4
*/

fn main() {

    'outer: for i in (0..=9999999).rev() {
        let ints: Vec<i32> = i.to_string().chars().map(|c| c.to_digit(10).unwrap() as i32).collect();
        for w in &ints {
            if *w == 0 {
                continue 'outer;
            }
        }
        let w0 = ints[0];
        let w1 = ints[1];
        let w2 = ints[2];
        let w3 = ints[3];
        let w5 = ints[4];
        let w6 = ints[5];
        let w8 = ints[6];
        let w4 = w3 + 8;
        let w7 = w6 - 2;
        let w9 = w8 + 7;
        let w10 = w5;
        let w11 = w2 - 5;
        let w12 = w1 + 2;
        let w13 = w0 - 4;
        let mut result = true;
        for w in [w4, w7, w9, w10, w11, w12, w13].iter() {
            if *w < 1 || *w > 9 {
                result = false;
                break;
            }
        }
        if result {
            println!("First answer is {}{}{}{}{}{}{}{}{}{}{}{}{}{}", w0, w1, w2, w3, w4, w5, w6, w7, w8, w9, w10, w11, w12, w13);
            break;
        }
    }
    'outer: for i in 1111111..=9999999 {
        let ints: Vec<i32> = i.to_string().chars().map(|c| c.to_digit(10).unwrap() as i32).collect();
        for w in &ints {
            if *w == 0 {
                continue 'outer;
            }
        }
        let w0 = ints[0];
        let w1 = ints[1];
        let w2 = ints[2];
        let w3 = ints[3];
        let w5 = ints[4];
        let w6 = ints[5];
        let w8 = ints[6];
        let w4 = w3 + 8;
        let w7 = w6 - 2;
        let w9 = w8 + 7;
        let w10 = w5;
        let w11 = w2 - 5;
        let w12 = w1 + 2;
        let w13 = w0 - 4;
        let mut result = true;
        for w in [w4, w7, w9, w10, w11, w12, w13].iter() {
            if *w < 1 || *w > 9 {
                result = false;
                break;
            }
        }
        if result {
            println!("Second answer is {}{}{}{}{}{}{}{}{}{}{}{}{}{}", w0, w1, w2, w3, w4, w5, w6, w7, w8, w9, w10, w11, w12, w13);
            break;
        }
    }
}