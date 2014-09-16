//Implementation of Universal Chess Interface (UCI)
//http://wbec-ridderkerk.nl/html/UCIProtocol.html
use types::*;
use fen::parse_fen;
use std::str::{Chars, StrSlice};
use std::fmt;


#[deriving(PartialEq)]
pub struct UciMove {
    from:Square,
    to:Square,
    promotion:Option<Kind>
}

#[deriving(PartialEq, Show)]
pub enum SearchOption {
    MovetimeMsc(uint),
    Infinity
}

#[deriving(PartialEq, Show)]
pub enum Command {
    CmdUci,
    CmdIsReady,
    CmdUciNewGame,
    CmdPosition (Position, Vec<UciMove>),
    CmdGo (SearchOption),
    CmdStop,
    CmdQuit,
    CmdUnknown
}

#[deriving(PartialEq)]
pub enum Response {
    RspId (String, String),
    RspUciOk,
    RspReadyOk,
    RspBestMove (UciMove),
    RspInfo (String),
}

pub struct UciEngine {
    position: Position
}

impl fmt::Show for UciMove {
    fn fmt(&self, f:&mut fmt::Formatter) -> fmt::Result {
        match self.promotion {
            Some(promo) => 
                write!(f, "{}{}{}", self.from, self.to, promo),
            None => 
                write!(f, "{}{}", self.from, self.to)
        }        
    }
}

impl fmt::Show for Response {
    fn fmt(&self, f:&mut fmt::Formatter) -> fmt::Result {
        match *self {
            RspId(ref name, ref value) => write!(f, "id {} {}", name, value),
            RspUciOk => write!(f, "uciok"),
            RspReadyOk => write!(f, "readyok"),
            RspInfo(ref info) => write!(f, "info {}", info),
            RspBestMove(ref move) => write!(f, "bestmove {}", move),
        }
    }
}

impl UciEngine {

    pub fn new() -> UciEngine {
        UciEngine {
            position: parse_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1").unwrap()
        }
    }

    pub fn main_loop(&mut self) {
        use std::io;
        for line in io::stdin().lines() {
            let line = line.unwrap();
            let cmd = match parse_command(line.as_slice()) {
                Ok(cmd) => cmd,
                Err(_) => CmdUnknown 
            };

            let responses  = match cmd {
                CmdUci => vec![RspId("name".to_string(), "rchess".to_string()), 
                           RspId("author".to_string(), "EZ".to_string()), RspUciOk],
                CmdIsReady => vec![RspReadyOk],
                CmdUciNewGame => vec![],
                CmdPosition (ref pos, ref moves) => {
                    self.set_position(pos, moves);
                    vec![]
                }
                CmdGo (_) => {
                    let move = self.think();
                    let uci_move = move_to_uci(&move, self.position.next_to_move);
                    vec![RspInfo(format!("currmove {}", uci_move)), RspBestMove(uci_move)]
                },
                CmdStop => vec![],
                CmdQuit => { break },
                CmdUnknown => vec![]
            };

            for r in responses.iter() {
                println!("{}", r);
            }
            // {
            //     use std::io::{File, Append, ReadWrite};
            //     let p = Path::new("/home/eugene/projects/rusty-chess/rchess.log");
            //     let mut file = match File::open_mode(&p, Append, ReadWrite) {
            //         Ok(f) => f,
            //         Err(e) => fail!("file error: {}", e),
            //     };
            //     write!(&mut file, ">>{}", line);
            //     writeln!(&mut file, "{}", cmd);
            //     for r in responses.iter() {
            //         writeln!(&mut file, "<<{}", r);
            //     }
            // }
        }
    }

    fn think(&self) -> Move {
        use std::rand;
        let moves:Vec<Move> = self.position.gen_moves().collect();
        let index = rand::random::<uint>() % moves.len();
        moves[index]
    }

    fn set_position(&mut self, pos: &Position, moves:&Vec<UciMove>) {
        self.position = *pos;
        for uci_move in moves.iter() {
            let move = uci_to_move(&self.position.board, uci_move);
            self.position.apply_move(&move);
        }
    }
}

fn move_to_uci(move: &Move, color: Color) -> UciMove {
    use squares::*;
    match *move {
        OrdinalMove(ref mi) => UciMove {
            from: mi.from,
            to: mi.to,
            promotion: mi.promotion
        },
        CastleKingSide => { 
            if color == White  { 
                UciMove {
                    from: e1,
                    to: g1,
                    promotion: None
                }
            } else {
                UciMove {
                    from: e8,
                    to: g8,
                    promotion: None
                }
            }
        }
        CastleQueenSide => {
            if color == White {
                UciMove {
                    from: e1,
                    to: c1,
                    promotion: None
                }
            } else {
                UciMove {
                    from: e8,
                    to: c8,
                    promotion: None
                }
            }
        }
        NullMove => fail!("null move is not supposed to get out to uci")
    }
}

fn uci_to_move(board: &Board, move: &UciMove) -> Move {
    use squares::*;
    let piece = board.get_piece(move.from);
    let piece = match piece {
        Some(p) => p,
        None => {
            //TODO: report error
            fail!("Uci move from an empty square");
        }
    };

    if piece == Piece(King, White) {
        if move.from == e1 {
            if move.to == g1 {
                return CastleKingSide;
            } 
            if move.to == c1 {
                return CastleQueenSide;
            } 
        }
    } 
    else if piece == Piece(King, Black) {
        if move.from == e8 {
            if move.to == g8 {
                return CastleKingSide;
            } 
            if move.to == c8 {
                return CastleQueenSide;
            } 
        }
    }

    OrdinalMove (OrdinalMoveInfo {
        from: move.from,
        to: move.to,
        kind: piece.kind(),
        promotion: move.promotion
    })
}

pub fn parse_command(line: &str) -> Result<Command, String> {
    if line.starts_with("ucinewgame") {
        return Ok(CmdUciNewGame);
    }    
    if line.starts_with("uci") {
        return Ok(CmdUci);
    } 
    if line.starts_with("isready") {
        return Ok(CmdIsReady);
    }
    if line.starts_with("stop") {
        return Ok(CmdStop);
    }
    if line.starts_with("quit") {
        return Ok(CmdQuit);
    }
    if line.starts_with("position") {
        let position_index = match line.find(' ') {
            Some(index) => index + 1,
            None => return Err("fen or startpos is expected after 'position' command".to_string())
        };
        let mut position_str = line.slice_from(position_index);
        let position = if position_str.starts_with("startpos") {
            //initial position
            parse_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1").unwrap()
        } else {
            if position_str.starts_with("fen") {
                position_str = position_str.slice_from("fen".len())
            }
            try!(parse_fen(skip_spaces(position_str)))
        };
        let moves_index = match line.find_str("moves ") {
            Some(index) => index + "moves ".len(),
            None => 0
        };
        let moves:Vec<UciMove> = if moves_index > 0 && moves_index < line.len() {
            try!(parse_moves(line.slice_from(moves_index))) 
        } else {
            Vec::new()
        };
        return Ok(CmdPosition (position, moves))
    }
    if line.starts_with("go") {
        let option_index = match line.find(' ') {
            Some(index) => index + 1,
            None => return Ok(CmdGo(Infinity))
        };
        let option = try!(pares_search_option(line.slice_from(option_index)));
        return Ok(CmdGo(option));         
    }
    Err(format!("Unexpected command {}", line))
} 

fn pares_search_option(input: &str) -> Result<SearchOption, String> {
    if input.starts_with("movetime") {
        let num_str = skip_spaces(input.slice_from("movetime".len()));
        let time:uint = match from_str(num_str) {
            Some(t) => t,
            None => { return Err("Movetime is invalid or not provided".to_string()); }
        };
        return Ok (MovetimeMsc(time));
    } else {
        return Ok (Infinity);
    }
}

fn skip_spaces<'a>(s: &'a str) ->&'a str {
    let index = s.find(|c: char| !c.is_whitespace());
    match index {
        Some(i) => s.slice_from(i),
        None => s.slice_from(s.len())
    }
}

fn parse_moves(input: &str) -> Result<Vec<UciMove>, String> {
    let mut result = Vec::<UciMove>::new();
    for move_str in input.split(' ') {
        result.push(try!(parse_move(move_str)));
    }
    Ok(result)
}

fn parse_move(input: &str) -> Result<UciMove, String> {
    let mut chars = input.chars();
    let from = try!(parse_square(&mut chars));
    let to   = try!(parse_square(&mut chars));
    let promotion = match chars.next() {
        Some('q') => Some(Queen),
        Some('n') => Some(Knight),
        Some('b') => Some(Bishop),
        Some('r') => Some(Rook),
        _ => None
    };

    Ok (UciMove {
        from: from,
        to: to,
        promotion: promotion
    })
}

fn parse_square(iter: &mut Chars) -> Result<Square, String> {
    let file = match iter.next() {
        Some(c@'a'..'h') => (c as u32) - ('a' as u32),
        c => return Err(format!("Unexpected move file: {0}", c))
    };
    let rank = match iter.next() {
        Some(c@'1'..'8') => (c as u32) - ('1' as u32),
        c => return Err(format!("Unexpected move rank: {0}", c))
    };
    Ok(Square::new(file as u8, rank as u8)) 
}

#[cfg(test)]
mod tests {
use uci::*;
use fen::{parse_fen};
use types::*;
use squares::*;

#[test]
fn parse_go_command_test() {
    assert_eq!(parse_command("go infinite"), Ok(CmdGo(Infinity)));
    assert_eq!(parse_command("go"), Ok(CmdGo(Infinity)));
    assert_eq!(parse_command("go movetime 123"), Ok(CmdGo(MovetimeMsc(123)))); 
}

#[test]
fn parse_position_command_test() {
    let startpos = parse_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1").unwrap();

    let cmd = parse_command("position startpos").unwrap();
    let (pos, moves) = match cmd {
        CmdPosition(pos, moves) => (pos, moves),
        _ => fail!("wrong command")
    };
    assert_eq!(pos, startpos);
    assert_eq!(moves.len(), 0);

    let cmd = parse_command("position startpos moves e2e4 e7e5 a7a8r").unwrap();
    let (pos, moves) = match cmd {
        CmdPosition(pos, moves) => (pos, moves),
        _ => fail!("wrong command")
    };
    assert_eq!(pos, startpos);
    assert_eq!(moves[0], UciMove { from:e2, to:e4, promotion:None});
    assert_eq!(moves[1], UciMove { from:e7, to:e5, promotion:None});
    assert_eq!(moves[2], UciMove { from:a7, to:a8, promotion:Some(Rook)});

    let cmd = parse_command("position fen r1b1k1nr/ppp2ppp/2n5/2b1q3/3p4/P1P2pPN/1P5P/RNBQKB1R w KQkq - 0 10\n").unwrap();
    let (pos, moves) = match cmd {
        CmdPosition(pos, moves) => (pos, moves),
        _ => fail!("wrong command")
    };
    assert_eq!(pos, parse_fen("r1b1k1nr/ppp2ppp/2n5/2b1q3/3p4/P1P2pPN/1P5P/RNBQKB1R w KQkq - 0 10").unwrap());
    assert_eq!(moves.len(), 0);
}

#[test]
fn parse_simple_commands_test() {
    assert_eq!(parse_command("uci"), Ok(CmdUci));
    assert_eq!(parse_command("ucinewgame"), Ok(CmdUciNewGame));
    assert_eq!(parse_command("isready"), Ok(CmdIsReady));
    assert_eq!(parse_command("quit"), Ok(CmdQuit));
    assert_eq!(parse_command("stop"), Ok(CmdStop));
}

}

