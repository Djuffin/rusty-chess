//Implementation of Universal Chess Interface (UCI)
//http://wbec-ridderkerk.nl/html/UCIProtocol.html
use fen::parse_fen;
use std::str::{Chars, FromStr};
use std::fmt;
use std::io::{BufRead, Write};
use search::search;
pub use self::SearchOption::*;
pub use self::Command::*;
pub use self::Response::*;


#[derive(PartialEq, Debug, Clone, Copy)]
pub struct UciMove {
    from:Square,
    to:Square,
    promotion:Option<Kind>
}

#[derive(PartialEq, Debug, Clone, Copy)]
pub enum SearchOption {
    MovetimeMsc(usize),
    Depth(usize),
    Infinity
}

#[derive(PartialEq, Debug)]
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

#[derive(PartialEq, Debug)]
pub enum Response {
    RspId (String, String),
    RspUciOk,
    RspReadyOk,
    RspBestMove (UciMove),
    RspInfo (String),
}

#[derive(Clone, Copy, Debug)]
pub struct UciEngine {
    position: Position
}

impl fmt::Display for UciMove {
    fn fmt(&self, f:&mut fmt::Formatter) -> fmt::Result {
        match self.promotion {
            Some(promo) =>
                write!(f, "{}{}{}", self.from, self.to, promo),
            None =>
                write!(f, "{}{}", self.from, self.to)
        }
    }
}

impl fmt::Display for Response {
    fn fmt(&self, f:&mut fmt::Formatter) -> fmt::Result {
        match *self {
            RspId(ref name, ref value) => write!(f, "id {} {}", name, value),
            RspUciOk => write!(f, "uciok"),
            RspReadyOk => write!(f, "readyok"),
            RspInfo(ref info) => write!(f, "info {}", info),
            RspBestMove(ref mv) => write!(f, "bestmove {}", mv),
        }
    }
}

impl UciEngine {

    pub fn new() -> UciEngine {
        UciEngine {
            position: parse_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1").unwrap()
        }
    }

    pub fn std_main_loop(&mut self) {
        use std::io::{stdin, stdout};
        let input = stdin();
        let mut output = stdout();
        self.main_loop(&mut input.lock(), &mut output);
    }

    pub fn main_loop(&mut self, input:&mut BufRead, output:&mut Write) {
        for line in input.lines() {
            let line = line.unwrap();
            let cmd = match parse_command(&line) {
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
                CmdGo (opt) => {
                    match self.think(opt) {
                        Some(mv) => {
                            let uci_move = move_to_uci(&mv, self.position.next_to_move);
                            vec![RspInfo(format!("currmove {}", uci_move)), RspBestMove(uci_move)]
                        },
                        None => {
                            vec![RspInfo(format!("string, no moves found!"))]
                        }
                    }
                },
                CmdStop => vec![],
                CmdQuit => { break },
                CmdUnknown => vec![]
            };

            for r in responses.iter() {
                write!(output, "{}\n", r).ok();
                output.flush().ok();
            }
        }
    }

    fn think(&self, opt: SearchOption) -> Option<Move> {
        let depth = match opt {
            Depth(d) => d,
            Infinity => 5,
            MovetimeMsc(_) => 3,
        };
        search(&self.position, depth)
    }

    fn set_position(&mut self, pos: &Position, moves:&Vec<UciMove>) {
        self.position = *pos;
        for uci_move in moves.iter() {
            let mv = uci_to_move(&self.position.board, uci_move);
            self.position.apply_move(&mv);
        }
    }
}

fn move_to_uci(mv: &Move, color: Color) -> UciMove {
    use squares::*;
    match *mv {
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
        NullMove => panic!("null move is not supposed to get out to uci")
    }
}

fn uci_to_move(board: &Board, mv: &UciMove) -> Move {
    use squares::*;
    let piece = board.get_piece(mv.from);
    let piece = match piece {
        Some(p) => p,
        None => {
            //TODO: report error
            panic!("Uci move from an empty square");
        }
    };

    if piece == Piece(King, White) {
        if mv.from == e1 {
            if mv.to == g1 {
                return CastleKingSide;
            }
            if mv.to == c1 {
                return CastleQueenSide;
            }
        }
    }
    else if piece == Piece(King, Black) {
        if mv.from == e8 {
            if mv.to == g8 {
                return CastleKingSide;
            }
            if mv.to == c8 {
                return CastleQueenSide;
            }
        }
    }

    OrdinalMove (OrdinalMoveInfo {
        from: mv.from,
        to: mv.to,
        kind: piece.kind(),
        promotion: mv.promotion
    })
}

pub fn parse_command(line: &str) -> Result<Command, String> {
    let line = if line.ends_with("\n") { &line[0..line.len() - 1] } else { line };
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
        let mut position_str = &line[position_index..line.len()];
        let position = if position_str.starts_with("startpos") {
            //initial position
            parse_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1").unwrap()
        } else {
            if position_str.starts_with("fen") {
                position_str = &position_str["fen".len().. position_str.len()]
            }
            parse_fen(skip_spaces(position_str))?
        };
        let moves_index = match line.find("moves ") {
            Some(index) => index + "moves ".len(),
            None => 0
        };
        let moves:Vec<UciMove> = if moves_index > 0 && moves_index < line.len() {
            parse_moves(&line[moves_index..line.len()])?
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
        let option = pares_search_option(&line[option_index..line.len()])?;
        return Ok(CmdGo(option));
    }
    Err(format!("Unexpected command {}", line))
}

fn pares_search_option(input: &str) -> Result<SearchOption, String> {
    if input.starts_with("movetime") {
        let num_str = skip_spaces(&input["movetime".len()..input.len()]);
        let time:usize = match FromStr::from_str(num_str) {
            Ok(t) => t,
            _ => { return Err("Movetime is invalid or not provided".to_string()); }
        };
        return Ok (MovetimeMsc(time));
    } else if input.starts_with("depth") {
        let num_str = skip_spaces(&input["depth".len()..input.len()]);
        let depth:usize = match FromStr::from_str(num_str) {
            Ok(t) => t,
            _ => { return Err("Depth is invalid or not provided".to_string()); }
        };
        return Ok (Depth(depth));
    } else {
        return Ok (Infinity);
    }
}

fn skip_spaces<'a>(s: &'a str) ->&'a str {
    let index = s.find(|c: char| !c.is_whitespace());
    match index {
        Some(i) => &s[i..s.len()],
        None => &s[s.len()..s.len()]
    }
}

fn parse_moves(input: &str) -> Result<Vec<UciMove>, String> {
    let mut result = Vec::<UciMove>::new();
    for move_str in input.split(' ') {
        result.push(parse_move(move_str)?);
    }
    Ok(result)
}

fn parse_move(input: &str) -> Result<UciMove, String> {
    let mut chars = input.chars();
    let from = parse_square(&mut chars)?;
    let to   = parse_square(&mut chars)?;
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
        Some(c@'a'...'h') => (c as u32) - ('a' as u32),
        c => return Err(format!("Unexpected move file: {0:?}", c))
    };
    let rank = match iter.next() {
        Some(c@'1'...'8') => (c as u32) - ('1' as u32),
        c => return Err(format!("Unexpected move rank: {0:?}", c))
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

    let cmd = parse_command("position startpos\n").unwrap();
    let (pos, moves) = match cmd {
        CmdPosition(pos, moves) => (pos, moves),
        _ => panic!("wrong command")
    };
    assert_eq!(pos, startpos);
    assert_eq!(moves.len(), 0);

    let cmd = parse_command("position startpos moves e2e4 e7e5 a7a8r\n").unwrap();
    let (pos, moves) = match cmd {
        CmdPosition(pos, moves) => (pos, moves),
        _ => panic!("wrong command")
    };
    assert_eq!(pos, startpos);
    assert_eq!(moves[0], UciMove { from:e2, to:e4, promotion:None});
    assert_eq!(moves[1], UciMove { from:e7, to:e5, promotion:None});
    assert_eq!(moves[2], UciMove { from:a7, to:a8, promotion:Some(Rook)});

    let cmd = parse_command("position fen r1b1k1nr/ppp2ppp/2n5/2b1q3/3p4/P1P2pPN/1P5P/RNBQKB1R w KQkq - 0 10\n").unwrap();
    let (pos, moves) = match cmd {
        CmdPosition(pos, moves) => (pos, moves),
        _ => panic!("wrong command")
    };
    assert_eq!(pos, parse_fen("r1b1k1nr/ppp2ppp/2n5/2b1q3/3p4/P1P2pPN/1P5P/RNBQKB1R w KQkq - 0 10").unwrap());
    assert_eq!(moves.len(), 0);
}

#[test]
fn parse_simple_commands_test() {
    assert_eq!(parse_command("uci\n"), Ok(CmdUci));
    assert_eq!(parse_command("ucinewgame\n"), Ok(CmdUciNewGame));
    assert_eq!(parse_command("isready\n"), Ok(CmdIsReady));
    assert_eq!(parse_command("quit\n"), Ok(CmdQuit));
    assert_eq!(parse_command("stop\n"), Ok(CmdStop));
    assert_eq!(parse_command("go depth 3\n"), Ok(CmdGo(Depth(3))));
}

}

