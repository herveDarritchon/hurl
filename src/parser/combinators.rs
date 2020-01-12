use super::core::{ParseFunc, ParseResult, Parser};
use super::error::*;

#[allow(dead_code)]
pub fn optional<'a, T>(f: ParseFunc<'a, T>, p: &mut Parser) -> ParseResult<'a, Option<T>> {
    let start = p.state.clone();
    match f(p) {
        Ok(r) => {
            return Ok(Some(r));
        }
        Err(e) => {
            if e.recoverable {
                p.state = start;
                return Ok(None);
            } else {
                return Err(e);
            };
        }
    }
}

// make an error recoverable
// but does not reset cursor
#[allow(dead_code)]
pub fn recover<'a, T>(f: ParseFunc<'a, T>, p: &mut Parser) -> ParseResult<'a, T> {
    //   let start = p.state.clone();
    match f(p) {
        Ok(r) => {
            return Ok(r);
        }
        Err(e) => {
            return Err(Error {
                pos: e.pos,
                recoverable: true,
                inner: e.inner,
            });
        }
    }
}

#[allow(dead_code)]
pub fn nonrecover<'a, T>(f: ParseFunc<'a, T>, p: &mut Parser) -> ParseResult<'a, T> {
    //let start = p.state.clone();
    match f(p) {
        Ok(r) => {
            return Ok(r);
        }
        Err(e) => {
            return Err(Error {
                pos: e.pos,
                recoverable: false,
                inner: e.inner,
            });
        }
    }
}

#[allow(dead_code)]
pub fn zero_or_more<'a, T>(f: ParseFunc<'a, T>, p: &mut Parser) -> ParseResult<'a, Vec<T>> {
    let _start = p.state.clone();

    let mut v: Vec<T> = Vec::new();
    loop {
        let initial_state = p.state.clone();
        if p.is_eof() {
            return Ok(v);
        }

        match f(p) {
            Ok(r) => {
                v.push(r);
            }
            Err(e) => {
                if e.recoverable {
                    p.state.pos = initial_state.pos;
                    p.state.cursor = initial_state.cursor;
                    return Ok(v);
                } else {
                    return Err(e);
                };
            }
        }
    }
}

#[allow(dead_code)]
pub fn one_or_more<'a, T>(f: ParseFunc<'a, T>, p: &mut Parser) -> ParseResult<'a, Vec<T>> {
    let _initial_state = p.state.clone();
    match f(p) {
        Ok(r) => {
            let mut v = vec![r];
            loop {
                let initial_state = p.state.clone();
                match f(p) {
                    Ok(r) => {
                        v.push(r);
                    }
                    Err(e) => {
                        if e.recoverable {
                            p.state.pos = initial_state.pos;
                            p.state.cursor = initial_state.cursor;
                            return Ok(v);
                        } else {
                            return Err(e);
                        };
                    }
                }
            }
        }
        Err(Error { pos, inner, .. }) => {
            // if zero occurence => should fail?
            return Err(Error {
                pos,
                recoverable: false,
                inner,
            });
        }
    }
}


// return the last error when no default error is specified
// tipically this should be recoverable
pub fn choice<'a, T>(fs: Vec<ParseFunc<'a, T>>, p: &mut Parser) -> ParseResult<'a, T> {

    return match fs.get(0) {
        None => panic!("You can call choice with an empty vector of choice"),
        Some(f) => {
            let start = p.state.clone();
            if fs.len() == 1 {
                f(p)
            } else {
                match f(p) {
                    Err(Error {recoverable: true,..}) => {
                        p.state = start.clone();
                        choice(fs.clone().into_iter().skip(1).collect(), p)
                    },
                    x => x
                }
            }
        }
    };

}

//// not used yet?
//#[allow(dead_code)]
//pub fn choice_with_default_error<'a, T>(fs: Vec<ParseFunc<'a, T>>, default_error: Error, p: &mut Parser) -> ParseResult<'a, T> {
//    let start = p.state.clone();
//    for f in fs {
//        //println!("cursor {:?}", p.state.cursor);
//
//        match f(p) {
//            Ok(r) => {
//                return Ok(r);
//            }
//            Err(e) => {
//                //      println!("{:?}", e);
//                if !e.recoverable {
//                    return Err(e);
//                }
//                p.state = start.clone();
//            }
//        }
//    }
//    return Err(default_error);
//}


// peek does not change parser state
#[allow(dead_code)]
pub fn peek<'a, T>(f: ParseFunc<'a, T>, p: Parser) -> ParseResult<'a, T> {
    let start = p.state.clone();
    let mut p = p.clone();
    match f(&mut p) {
        Ok(r) => {
            p.state = start;
            return Ok(r);
        }
        Err(e) => {
            p.state = start;
            return Err(Error {
                pos: e.pos,
                recoverable: false,
                inner: e.inner,
            });
        }
    }
}
