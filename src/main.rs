use std::collections::HashMap;
mod piece;

fn standart_positions() -> HashMap<(u8, u8), piece::Piece> {
    let mut piecies_positions: HashMap<(u8, u8), piece::Piece> = HashMap::new();

    // Kings
    let king_white = piece::Piece::king(piece::Colors::White, (8, 4));
    let king_black = piece::Piece::king(piece::Colors::Black, (1, 4));
    piecies_positions.insert(king_white.piece_position, king_white);
    piecies_positions.insert(king_black.piece_position, king_black);

    // White Pawns
    let pawn_white_1 = piece::Piece::pawn(piece::Colors::White, (7, 1));
    let pawn_white_2 = piece::Piece::pawn(piece::Colors::White, (7, 2));
    let pawn_white_3 = piece::Piece::pawn(piece::Colors::White, (7, 3));
    let pawn_white_4 = piece::Piece::pawn(piece::Colors::White, (7, 4));
    let pawn_white_5 = piece::Piece::pawn(piece::Colors::White, (7, 5));
    let pawn_white_6 = piece::Piece::pawn(piece::Colors::White, (7, 6));
    let pawn_white_7 = piece::Piece::pawn(piece::Colors::White, (7, 7));
    let pawn_white_8 = piece::Piece::pawn(piece::Colors::White, (7, 8));

    piecies_positions.insert(pawn_white_1.piece_position, pawn_white_1);
    piecies_positions.insert(pawn_white_2.piece_position, pawn_white_2);
    piecies_positions.insert(pawn_white_3.piece_position, pawn_white_3);
    piecies_positions.insert(pawn_white_4.piece_position, pawn_white_4);
    piecies_positions.insert(pawn_white_5.piece_position, pawn_white_5);
    piecies_positions.insert(pawn_white_6.piece_position, pawn_white_6);
    piecies_positions.insert(pawn_white_7.piece_position, pawn_white_7);
    piecies_positions.insert(pawn_white_8.piece_position, pawn_white_8);

    // Black Pawns
    let pawn_black_1 = piece::Piece::pawn(piece::Colors::Black, (2, 1));
    let pawn_black_2 = piece::Piece::pawn(piece::Colors::Black, (2, 2));
    let pawn_black_3 = piece::Piece::pawn(piece::Colors::Black, (2, 3));
    let pawn_black_4 = piece::Piece::pawn(piece::Colors::Black, (2, 4));
    let pawn_black_5 = piece::Piece::pawn(piece::Colors::Black, (2, 5));
    let pawn_black_6 = piece::Piece::pawn(piece::Colors::Black, (2, 6));
    let pawn_black_7 = piece::Piece::pawn(piece::Colors::Black, (2, 7));
    let pawn_black_8 = piece::Piece::pawn(piece::Colors::Black, (2, 8));

    piecies_positions.insert(pawn_black_1.piece_position, pawn_black_1);
    piecies_positions.insert(pawn_black_2.piece_position, pawn_black_2);
    piecies_positions.insert(pawn_black_3.piece_position, pawn_black_3);
    piecies_positions.insert(pawn_black_4.piece_position, pawn_black_4);
    piecies_positions.insert(pawn_black_5.piece_position, pawn_black_5);
    piecies_positions.insert(pawn_black_6.piece_position, pawn_black_6);
    piecies_positions.insert(pawn_black_7.piece_position, pawn_black_7);
    piecies_positions.insert(pawn_black_8.piece_position, pawn_black_8);

    return piecies_positions;
}

fn main() {
    const BLACK_BACK: &str = "\x1b[101m";
    const WHITE_BACK: &str = "\x1b[107m";
    const RESET_COLOR: &str = "\x1b[0m";
    let without_piece = " ".to_string();

    let piecies_positions = standart_positions();

    for row in 1..9 {
        for cell in 1..9 {
            let color = if (row + cell) % 2 == 0 {
                BLACK_BACK
            } else {
                WHITE_BACK
            };

            let piece = piecies_positions
                .get(&(row, cell))
                .map(|piece| &piece.piece_type)
                .unwrap_or(&without_piece);

            print!("{} {} {}", color, piece, RESET_COLOR)
        }
        println!("");
    }
}
