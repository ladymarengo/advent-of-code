use regex::Regex;
use std::fs::read_to_string;

fn main() {
    let input = read_to_string("input/14.txt").unwrap();
    let mut quadrants = vec![0; 4];
    let re = Regex::new(r"=(?<x>\d+),(?<y>\d+) v=(?<v_x>-?\d+),(?<v_y>-?\d+)").unwrap();
    let (width, height) = (101, 103);
    let seconds = 100;

    input.lines().for_each(|line| {
        let captures = re.captures(line).unwrap();
        let x = captures["x"].parse::<i32>().unwrap();
        let y = captures["y"].parse::<i32>().unwrap();
        let vel_x = captures["v_x"].parse::<i32>().unwrap();
        let vel_y = captures["v_y"].parse::<i32>().unwrap();

        let new_x = (((x + vel_x * seconds) % width) + width) % width;
        let new_y = (((y + vel_y * seconds) % height) + height) % height;

        if new_x < width / 2 {
            if new_y < height / 2 {
                quadrants[0] += 1;
            } else if new_y > height / 2 {
                quadrants[1] += 1;
            }
        } else if new_x > width / 2 {
            if new_y < height / 2 {
                quadrants[2] += 1;
            } else if new_y > height / 2 {
                quadrants[3] += 1;
            }
        }
    });

    println!(
        "First answer is {}",
        quadrants[0] * quadrants[1] * quadrants[2] * quadrants[3]
    );

    // part 2: 6587 seconds
    // * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * *
    // *                                                           *
    // *                                                           *
    // *                                                           *
    // *                                                           *
    // *                             *                             *
    // *                           * * *                           *
    // *                         * * * * *                         *
    // *                       * * * * * * *                       *
    // *                     * * * * * * * * *                     *
    // *                         * * * * *                         *
    // *                       * * * * * * *                       *
    // *                     * * * * * * * * *                     *
    // *                   * * * * * * * * * * *                   *
    // *                 * * * * * * * * * * * * *                 *
    // *                     * * * * * * * * *                     *
    // *                   * * * * * * * * * * *                   *
    // *                 * * * * * * * * * * * * *                 *
    // *               * * * * * * * * * * * * * * *               *
    // *             * * * * * * * * * * * * * * * * *             *
    // *                 * * * * * * * * * * * * *                 *
    // *               * * * * * * * * * * * * * * *               *
    // *             * * * * * * * * * * * * * * * * *             *
    // *           * * * * * * * * * * * * * * * * * * *           *
    // *         * * * * * * * * * * * * * * * * * * * * *         *
    // *                           * * *                           *
    // *                           * * *                           *
    // *                           * * *                           *
    // *                                                           *
    // *                                                           *
    // *                                                           *
    // *                                                           *
    // * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * *
}
