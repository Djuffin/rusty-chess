//Implementation of Universal Chess Interface (UCI)
//http://wbec-ridderkerk.nl/html/UCIProtocol.html
use types::{Kind, Move, Position, Square, Rook, Bishop, Knight, Queen};
use fen::parse_fen;
use std::str::{Chars, StrSlice};


#[deriving(PartialEq, Show)]
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
    CmdQuit
}

#[deriving(PartialEq)]
pub enum Response {
    RspId (String, String),
    RspUciOk,
    RspReadyOk,
    RspBestMove (Move),
    RspInfo (String),
    RspOption (String) 
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
        let position_str = line.slice_from(position_index);
        let position = if position_str.starts_with("startpos") {
            //initial position
            parse_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1").unwrap()
        } else {
            try!(parse_fen(position_str))
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

    let cmd = parse_command("position 8/8/8/8/8/8/8/8 w - - 200 999 moves").unwrap();
    let (pos, moves) = match cmd {
        CmdPosition(pos, moves) => (pos, moves),
        _ => fail!("wrong command")
    };
    assert_eq!(pos, parse_fen("8/8/8/8/8/8/8/8 w - - 200 999").unwrap());
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

