//Position evaluation
use types::*;
pub use self::GameStage::*;

#[derive(PartialEq, Debug, Clone, Copy)]
pub enum GameStage {
    Opening, Middlegame, Endgame
}

pub type Score = i32;
pub static INFINITY: Score = ::std::i32::MAX;

pub trait Evaluator {
    //Assigns for each position a score, that measures adventage of whites (positive score) or blacks (negative score)
    //in centipawns (1/100 of a pawn)
    fn eval(&self, position: &Position) -> Score;

    //tells what stage of the game we are at
    fn classify(&self, position: &Position) -> GameStage;
}


//very simple position evaluation based mostly on https://www.chessprogramming.org/Simplified_Evaluation_Function
#[derive(Copy)]
pub struct SimpleEvaluator {
    //arrays that eastimate relative value of each a piece in each square
    white_pawn_weights : [i8; 64],
    black_pawn_weights : [i8; 64],

    white_knight_weights : [i8; 64],
    black_knight_weights : [i8; 64],

    white_bishop_weights : [i8; 64],
    black_bishop_weights : [i8; 64],

    white_rook_weights : [i8; 64],
    black_rook_weights : [i8; 64],

    white_queen_weights : [i8; 64],
    black_queen_weights : [i8; 64],

    white_king_weights : [i8; 64],
    black_king_weights : [i8; 64],

    white_endgame_king_weights : [i8; 64],
    black_endgame_king_weights : [i8; 64]

}

impl Clone for SimpleEvaluator {
    fn clone(&self) -> Self {
            *self
    }
}

impl SimpleEvaluator {
    pub fn new() -> SimpleEvaluator {
        let white_pawn_weights : [i8; 64] = mirror_weights_table(
          &[0,  0,  0,  0,  0,  0,  0,  0,
            50, 50, 50, 50, 50, 50, 50, 50,
            10, 10, 20, 30, 30, 20, 10, 10,
             5,  5, 10, 25, 25, 10,  5,  5,
             0,  0,  0, 20, 20,  0,  0,  0,
             5, -5,-10,  0,  0,-10, -5,  5,
             5, 10, 10,-20,-20, 10, 10,  5,
             0,  0,  0,  0,  0,  0,  0,  0]);

        let white_knight_weights : [i8; 64] = mirror_weights_table(
          &[-50,-40,-30,-30,-30,-30,-40,-50,
            -40,-20,  0,  0,  0,  0,-20,-40,
            -30,  0, 10, 15, 15, 10,  0,-30,
            -30,  5, 15, 20, 20, 15,  5,-30,
            -30,  0, 15, 20, 20, 15,  0,-30,
            -30,  5, 10, 15, 15, 10,  5,-30,
            -40,-20,  0,  5,  5,  0,-20,-40,
            -50,-40,-30,-30,-30,-30,-40,-50]);

        let white_bishop_weights : [i8; 64] = mirror_weights_table(
          &[-20,-10,-10,-10,-10,-10,-10,-20,
            -10,  0,  0,  0,  0,  0,  0,-10,
            -10,  0,  5, 10, 10,  5,  0,-10,
            -10,  5,  5, 10, 10,  5,  5,-10,
            -10,  0, 10, 10, 10, 10,  0,-10,
            -10, 10, 10, 10, 10, 10, 10,-10,
            -10,  5,  0,  0,  0,  0,  5,-10,
            -20,-10,-10,-10,-10,-10,-10,-20]);

        let white_rook_weights : [i8; 64] = mirror_weights_table(
           &[0,  0,  0,  0,  0,  0,  0,  0,
             5, 10, 10, 10, 10, 10, 10,  5,
            -5,  0,  0,  0,  0,  0,  0, -5,
            -5,  0,  0,  0,  0,  0,  0, -5,
            -5,  0,  0,  0,  0,  0,  0, -5,
            -5,  0,  0,  0,  0,  0,  0, -5,
            -5,  0,  0,  0,  0,  0,  0, -5,
             0,  0,  0,  5,  5,  0,  0,  0]);

        let white_queen_weights : [i8; 64] = mirror_weights_table(
          &[-20,-10,-10, -5, -5,-10,-10,-20,
            -10,  0,  0,  0,  0,  0,  0,-10,
            -10,  0,  5,  5,  5,  5,  0,-10,
             -5,  0,  5,  5,  5,  5,  0, -5,
              0,  0,  5,  5,  5,  5,  0, -5,
            -10,  5,  5,  5,  5,  5,  0,-10,
            -10,  0,  5,  0,  0,  0,  0,-10,
            -20,-10,-10, -5, -5,-10,-10,-20]);

        let white_king_weights : [i8; 64] = mirror_weights_table(
          &[-30,-40,-40,-50,-50,-40,-40,-30,
            -30,-40,-40,-50,-50,-40,-40,-30,
            -30,-40,-40,-50,-50,-40,-40,-30,
            -30,-40,-40,-50,-50,-40,-40,-30,
            -20,-30,-30,-40,-40,-30,-30,-20,
            -10,-20,-20,-20,-20,-20,-20,-10,
             20, 20,  0,  0,  0,  0, 20, 20,
             20, 30, 10,  0,  0, 10, 30, 20]);

        let white_endgame_king_weights : [i8; 64] = mirror_weights_table(
          &[-50,-40,-30,-20,-20,-30,-40,-50,
            -30,-20,-10,  0,  0,-10,-20,-30,
            -30,-10, 20, 30, 30, 20,-10,-30,
            -30,-10, 30, 40, 40, 30,-10,-30,
            -30,-10, 30, 40, 40, 30,-10,-30,
            -30,-10, 20, 30, 30, 20,-10,-30,
            -30,-30,  0,  0,  0,  0,-30,-30,
            -50,-30,-30,-30,-30,-30,-30,-50]);

        SimpleEvaluator {
            white_pawn_weights : white_pawn_weights,
            black_pawn_weights : mirror_weights_table(&white_pawn_weights),

            white_knight_weights : white_knight_weights,
            black_knight_weights : mirror_weights_table(&white_knight_weights),

            white_bishop_weights : white_bishop_weights,
            black_bishop_weights : mirror_weights_table(&white_bishop_weights),

            white_rook_weights : white_rook_weights,
            black_rook_weights : mirror_weights_table(&white_rook_weights),

            white_queen_weights : white_queen_weights,
            black_queen_weights : mirror_weights_table(&white_queen_weights),

            white_king_weights : white_king_weights,
            black_king_weights : mirror_weights_table(&white_king_weights),

            white_endgame_king_weights : white_endgame_king_weights,
            black_endgame_king_weights : mirror_weights_table(&white_endgame_king_weights),
        }
    }

    fn eval_material(&self, position: &Position) -> Score {
        let board = &position.board;
        let queens = (board.queens & board.whites).count() as i32 -
                     (board.queens & board.blacks).count() as i32;

        let rooks =  (board.rooks & board.whites).count() as i32 -
                     (board.rooks & board.blacks).count() as i32;

        let bishops =(board.bishops & board.whites).count() as i32 -
                     (board.bishops & board.blacks).count() as i32;

        let knights =(board.knights & board.whites).count() as i32 -
                     (board.knights & board.blacks).count() as i32;

        let kings =  (board.kings & board.whites).count() as i32 -
                     (board.kings & board.blacks).count() as i32;

        let pawns =  (board.pawns & board.whites).count() as i32 -
                     (board.pawns & board.blacks).count() as i32;

        (pawns * 100) + (knights * 320) + (bishops * 330) + (rooks * 500) + (queens * 900) + (kings * 20000)
    }

    fn eval_piece_positions(&self, position: &Position) -> Score {
        let board = &position.board;
        let stage = self.classify(position);
        let mut result = 0i32;
        for &color in [White, Black].iter() {
            for &kind in [Pawn, Knight, Bishop, Rook, Queen, King].iter() {
                for sq in board.get_pieces(kind, color) {
                    result += self.eval_one_piece_position(Piece(kind, color), sq, stage);
                }
            }
        }
        result
    }

    #[inline]
    fn eval_one_piece_position(&self, piece: Piece, sq:Square, stage: GameStage) -> Score {
        let table = match piece {
            Piece(Pawn, White) => self.white_pawn_weights,
            Piece(Pawn, Black) => self.black_pawn_weights,  

            Piece(Knight, White) => self.white_knight_weights,
            Piece(Knight, Black) => self.black_knight_weights,  

            Piece(Bishop, White) => self.white_bishop_weights,
            Piece(Bishop, Black) => self.black_bishop_weights,  

            Piece(Rook, White) => self.white_rook_weights,
            Piece(Rook, Black) => self.black_rook_weights,  

            Piece(Queen, White) => self.white_queen_weights,
            Piece(Queen, Black) => self.black_queen_weights,  

            Piece(King, White) if stage == Endgame => self.white_endgame_king_weights,
            Piece(King, White) => self.white_king_weights,

            Piece(King, Black) if stage == Endgame => self.black_endgame_king_weights,
            Piece(King, Black) => self.black_king_weights, 
        };

        if piece.color() == White {
            table[sq.file_and_rank() as usize] as Score
        } else {
            -table[sq.file_and_rank() as usize] as Score
        } 
    }
}

impl Evaluator for SimpleEvaluator {
    fn eval(&self, position: &Position) -> Score {
        self.eval_material(position) +
        self.eval_piece_positions(position)
    }

    fn classify(&self, position: &Position) -> GameStage {
        let board = &position.board;
        let no_queens = board.queens.is_empty();
        let no_rooks = board.rooks.is_empty();
        let minor_pieces_count = (board.bishops | board.knights).count();

        if  no_queens || (no_rooks && minor_pieces_count <= 2) {
            return Endgame;
        }

        if position.full_moves < 8 {
            return Opening;
        } 

        Middlegame
    }
}


fn mirror_weights_table(table: &[i8; 64]) -> [i8; 64] {
    let mut result = [0i8; 64]; 
    for rank in 0..8 {
        for file in 0..8 {
            let input_sq = Square::new(file, rank);
            let output_sq = Square::new(file, 7 - rank);
            result[output_sq.file_and_rank() as usize] = table[input_sq.file_and_rank() as usize];
        } 
    }
    result
}

#[cfg(test)]
mod tests {
use fen::parse_fen;
use eval::*;

#[test]
fn simple_eval_test() {
    let evaluator = SimpleEvaluator::new();

    //eval for whites
    let position = parse_fen("N7/1BR5/8/3Q4/4P3/8/8/8 w KQkq - 0 1").unwrap(); 
    //score = (100 for a pawn) + (320 for a knight) + (330 for a bishop) + (500 for a rook) + (900 for a queen) = 2150
    //knight on a8 = -50
    //bishop on b7 = 0
    //rook on c7 = 10
    //queen on d5 = 5
    //pawn on e4 = 20
    //------------------
    // 2135
    let score = evaluator.eval(&position);
    assert_eq!(score, 2135);

    //initial position
    let position = parse_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1").unwrap(); 
    let score = evaluator.eval(&position);
    assert_eq!(score, 0);

    //middle game position
    let position = parse_fen("qk5r/8/8/3K4/8/8/8/R6Q w - - 0 40").unwrap(); 
    let stage = evaluator.classify(&position);
    assert_eq!(stage, Middlegame);
    let score = evaluator.eval(&position);
    //middlegame position
    //white king on d5 = -50
    //black king in b8 = 30
    //result = -50 - 30 = -80
    //rooks and queens nullify each other
    assert_eq!(score, -80); 

    //endgame position
    let position = parse_fen("k7/8/8/3K4/8/8/8/8 w - - 0 40").unwrap(); 
    let stage = evaluator.classify(&position);
    assert_eq!(stage, Endgame);
    let score = evaluator.eval(&position);
    //endgame table is used
    //white king in the center = 40
    //black king in the corner = -50
    //result = 40 - (-50) = 90
    assert_eq!(score, 90);
}

#[test]
fn classify_test() {
    let evaluator = SimpleEvaluator::new();

    let position = parse_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w - - 0 1").unwrap(); 
    let stage = evaluator.classify(&position);
    assert_eq!(stage, Opening);

    let position = parse_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w - - 0 9").unwrap(); 
    let stage = evaluator.classify(&position);
    assert_eq!(stage, Middlegame);

    let position = parse_fen("rnb1kbnr/pppppppp/8/8/8/8/PPPPPPPP/RNB1KBNR w - - 0 9").unwrap(); 
    let stage = evaluator.classify(&position);
    assert_eq!(stage, Endgame);

    let position = parse_fen("2bqk3/pppppppp/8/8/8/8/PPPPPPPP/3QKN2 w - - 0 9").unwrap(); 
    let stage = evaluator.classify(&position);
    assert_eq!(stage, Endgame);
    

}

}