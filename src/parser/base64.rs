use super::core::*;



// part of hurl
// just reuse Parser/Error Position
// do not depend on external separator
// stop parsing when there is no more base64 character
//
// what kind of errors can you have?
// can only fail if using bad padding?
// if padding is used it must be used properly
// you can only have an Expecting padding error (missing one for example)


fn value(c: &char) -> Option<i32> {
    return match c {
        'A' => Some(0),
        'B' => Some(1),
        'C' => Some(2),
        'D' => Some(3),
        'E' => Some(4),
        'F' => Some(5),
        'G' => Some(6),
        'H' => Some(7),
        'I' => Some(8),
        'J' => Some(9),
        'K' => Some(10),
        'L' => Some(11),
        'M' => Some(12),
        'N' => Some(13),
        'O' => Some(14),
        'P' => Some(15),
        'Q' => Some(16),
        'R' => Some(17),
        'S' => Some(18),
        'T' => Some(19),
        'U' => Some(20),
        'V' => Some(21),
        'W' => Some(22),
        'X' => Some(23),
        'Y' => Some(24),
        'Z' => Some(25),
        'a' => Some(26),
        'b' => Some(27),
        'c' => Some(28),
        'd' => Some(29),
        'e' => Some(30),
        'f' => Some(31),
        'g' => Some(32),
        'h' => Some(33),
        'i' => Some(34),
        'j' => Some(35),
        'k' => Some(36),
        'l' => Some(37),
        'm' => Some(38),
        'n' => Some(39),
        'o' => Some(40),
        'p' => Some(41),
        'q' => Some(42),
        'r' => Some(43),
        's' => Some(44),
        't' => Some(45),
        'u' => Some(46),
        'v' => Some(47),
        'w' => Some(48),
        'x' => Some(49),
        'y' => Some(50),
        'z' => Some(51),
        '0' => Some(52),
        '1' => Some(53),
        '2' => Some(54),
        '3' => Some(55),
        '4' => Some(56),
        '5' => Some(57),
        '6' => Some(58),
        '7' => Some(59),
        '8' => Some(60),
        '9' => Some(61),
        '+' => Some(62),
        '/' => Some(63),
        _ => None,
    };
}


// consume padding can not fail
pub fn padding(p: &mut Parser) -> String {
    let mut buf = String::from("");

    loop {
        let save = p.state.clone();
        match p.next_char() {
            Some('=') => { buf.push('='); }
            _ => {
                p.state = save;
                break;
            }
        }
    }
    return buf;
}


// support whitespaces
// keep original encoding by the caller
// eat for meaningful character at a time? for decoding
// Decoding Base64 without padding
//// Can not fail => just move parser cursor
//pub fn deprecated_parse(p: &mut Parser) -> ParseResult<'static, Vec<u8>> {
//    let mut bytes = vec![];
//    let mut buf = vec![]; // base64 text
//    loop {
//        let pad = padding(p);
//        if pad != "" {
//            break;
//        }
//        let save = p.state.clone();
//        match p.next_char() {
//            None => { break; }
//            Some(' ') | Some('\n') | Some('\t') => {}
//            Some(c) => {
//
//                match value(&c) {
//                    None => { p.state = save; break; }
//                    Some(v) => {
//                        buf.push(v);
//                        if buf.len() == 4 {
//                            let bs = decode_four_chars(
//                                *buf.get(0).unwrap(),
//                                *buf.get(1).unwrap(),
//                                *buf.get(2).unwrap(),
//                                *buf.get(3).unwrap(),
//                            );
////println!(">> bs {:?}", bs);
//                            for b in bs {
//                                bytes.push(b);
//                            }
//                            buf = vec![];
//                        }
//                    }
//                }
//            }
//        }
//    }
//    match buf.as_slice() {
//        [c1, c2] => bytes.append(&mut decode_two_chars(*c1, *c2)),
//        [c1, c2, c3] => bytes.append(&mut decode_three_chars(*c1, *c2, *c3)),
//        _ => {}
//    }
//    return Ok(bytes);
//}


pub fn parse2(p: &mut Parser) -> Vec<u8> {
    let mut bytes = vec![];
    let mut buf = vec![]; // base64 text
    loop {
        let pad = padding(p);
        if pad != "" {
            break;
        }
        let save = p.state.clone();
        match p.next_char() {
            None => { break; }
            Some(' ') | Some('\n') | Some('\t') => {}
            Some(c) => {

                match value(&c) {
                    None => { p.state = save; break; }
                    Some(v) => {
                        buf.push(v);
                        if buf.len() == 4 {
                            let bs = decode_four_chars(
                                *buf.get(0).unwrap(),
                                *buf.get(1).unwrap(),
                                *buf.get(2).unwrap(),
                                *buf.get(3).unwrap(),
                            );
//println!(">> bs {:?}", bs);
                            for b in bs {
                                bytes.push(b);
                            }
                            buf = vec![];
                        }
                    }
                }
            }
        }
    }
    match buf.as_slice() {
        [c1, c2] => bytes.append(&mut decode_two_chars(*c1, *c2)),
        [c1, c2, c3] => bytes.append(&mut decode_three_chars(*c1, *c2, *c3)),
        _ => {}
    }
    return bytes;
}

#[test]
fn test_decode_one_block() {
    let mut parser = Parser::init("");
    assert_eq!(parse2(&mut parser), vec![] as Vec<u8>);
    assert_eq!(parser.state.cursor, 0);

    let mut parser = Parser::init("AA==;");
    assert_eq!(parse2(&mut parser), vec![0]);
    assert_eq!(parser.state.cursor, 4);

    let mut parser = Parser::init("AA");
    assert_eq!(parse2(&mut parser), vec![0]);
    assert_eq!(parser.state.cursor, 2);

    let mut parser = Parser::init("AA;");
    assert_eq!(parse2(&mut parser), vec![0]);
    assert_eq!(parser.state.cursor, 2);

    let mut parser = Parser::init("TWE=;");
    assert_eq!(parse2(&mut parser), vec![77, 97]);
    assert_eq!(parser.state.cursor, 4);

    let mut parser = Parser::init("TWFu;");
    assert_eq!(parse2(&mut parser), vec![77, 97, 110]);
    assert_eq!(parser.state.cursor, 4);
}

/*
https://en.wikipedia.org/wiki/Base64
Test padding/no-padding

Encoded
YW55IGNhcm5hbCBwbGVhcw==		any carnal pleas   # [97, 110, 121, 32, 99, 97, 114, 110, 97, 108, 32, 112, 108, 101, 97, 115]



|   Y       |     W     |     5     |     5     |
|     24    |    22     |      57   |     57    |
|0|1|1|0|0|0|0|1|0|1|1|0|1|1|1|0|0|1|1|1|1|0|0|1|
|      97       |     110       |      121      |
*/

#[test]
fn test_decode_with_padding() {
    let mut parser = Parser::init("YW55IGNhcm5hbCBwbGVhcw==;");
    let decoded = parse2(&mut parser);
    assert_eq!(decoded, "any carnal pleas".as_bytes());

    let mut parser = Parser::init("YW55IGNhcm5hbCBwbGVhc3U=;");
    assert_eq!(parse2(&mut parser), "any carnal pleasu".as_bytes());

    let mut parser = Parser::init("YW55IGNhcm5hbCBwbGVhc3Vy;");
    assert_eq!(parse2(&mut parser), "any carnal pleasur".as_bytes());
}

#[test]
fn test_decode_without_padding() {
    let mut parser = Parser::init("YW55IGNhcm5hbCBwbGVhcw;");
    assert_eq!(parse2(&mut parser), "any carnal pleas".as_bytes());

    let mut parser = Parser::init("YW55IGNhcm5hbCBwbGVhc3U;");
    assert_eq!(parse2(&mut parser), "any carnal pleasu".as_bytes());
}

#[test]
fn test_decode_with_whitespace() {
    let mut parser = Parser::init("TW E=\n;");
    assert_eq!(parse2(&mut parser), vec![77, 97]);
    assert_eq!(parser.state.cursor, 5);
}




// region decode-chunk

fn decode_two_chars(c1: i32, c2: i32) -> Vec<u8> {
    return vec![((c1 << 2 & 255) + (c2 >> 4)) as u8];
}

fn decode_three_chars(c1: i32, c2: i32, c3: i32) -> Vec<u8> {
    return vec![
        ((c1 << 2 & 255) + (c2 >> 4)) as u8,
        ((c2 << 4 & 255) + (c3 >> 2)) as u8,
    ];
}

// region decode 4 chars

/*
|   Y       |     W     |     5     |     5     |
|     24    |    22     |      57   |     57    |
|0|1|1|0|0|0|0|1|0|1|1|0|1|1|1|0|0|1|1|1|1|0|0|1|
|      97       |     110       |      121      |
*/

fn decode_four_chars(c1: i32, c2: i32, c3: i32, c4: i32) -> Vec<u8> {
    return vec![
        ((c1 << 2 & 255) + (c2 >> 4)) as u8,
        ((c2 << 4 & 255) + (c3 >> 2)) as u8,
        (((c3 << 6) & 255) + c4) as u8,
    ];
}

#[test]
fn test_decode_four_chars() {
    assert_eq!(
        decode_four_chars(
            value(&'Y').unwrap(),
            value(&'W').unwrap(),
            value(&'5').unwrap(),
            value(&'5').unwrap(),
        ),
        vec![97, 110, 121]
    );
    assert_eq!(
        decode_four_chars(
            value(&'T').unwrap(),
            value(&'W').unwrap(),
            value(&'F').unwrap(),
            value(&'u').unwrap(),
        ),
        vec![77, 97, 110]
    );
}

// endregion

#[test]
fn test_decode_two_chars() {
    assert_eq!(
        decode_two_chars(value(&'A').unwrap(), value(&'A').unwrap()),
        vec![0]
    );
    assert_eq!(
        decode_two_chars(value(&'A').unwrap(), value(&'Q').unwrap()),
        vec![1]
    );
    assert_eq!(
        decode_two_chars(value(&'T').unwrap(), value(&'Q').unwrap()),
        vec![77]
    );
}

#[test]
fn test_decode_three_chars() {
    assert_eq!(
        decode_three_chars(
            value(&'T').unwrap(),
            value(&'W').unwrap(),
            value(&'E').unwrap(),
        ),
        vec![77, 97]
    );
}

// endregion
