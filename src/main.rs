// struct Piece {
//     color: String,
// }

fn main() {
    const BLACK_BACK: &str = "\x1b[101m";
    const WHITE_BACK: &str = "\x1b[107m";
    const RESET_COLOR: &str = "\x1b[0m";

    //♚♛♜♝♞♟︎♟︎♔♕♖♗♘♙

    for row in 1..9 {
        for cell in 1..9 {
            let color = if (row + cell) % 2 == 0 {
                BLACK_BACK
            } else {
                WHITE_BACK
            };
            print!("{}   {}", color, RESET_COLOR)
        }
        println!("")
    }
}
