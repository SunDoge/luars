extern crate regex;

// #[macro_use]
// extern crate serde_derive;

// extern crate serde;



pub mod prob_start;
pub mod prob_trans;
pub mod prob_emit;


use regex::Regex;
use std::collections::HashMap as Map;
use super::compact::{char_slice, SplitCaptures, SplitState};

pub type ProbEmit = Map<char, Map<char, f64>>;
pub type ProbStart = Map<char, f64>;
pub type ProbTrans = Map<char, Map<char, f64>>;

const MIN_FLOAT: f64 = -3.14e100;

lazy_static! {
    static ref PrevStatus: Map<char, &'static str> = {
        let mut m: Map<char, &'static str> = Map::new();
        m.insert('B', "ES");
        m.insert('M', "MB");
        m.insert('S', "SE");
        m.insert('E', "BM");
        m
    };
    // static ref FORCE_SPLIT_WORDS:
}

// lazy_static! {
//     static ref EMIT_P: ProbEmit = {

//     }
// }

fn viterbi(
    obs: &str,
    states: &str,
    start_p: &ProbStart,
    trans_p: &ProbTrans,
    emit_p: &ProbEmit,
) -> (f64, Vec<char>) {
    let mut v: Vec<ProbStart> = vec![Map::new()];
    let mut path: Map<char, Vec<char>> = Map::new();
    for y in states.chars() {
        if let Some(ob) = emit_p[&y].get(&obs.chars().nth(0).unwrap()) {
            v[0].insert(y, start_p[&y] + ob);
        } else {
            v[0].insert(y, start_p[&y] + MIN_FLOAT);
        };

        path.insert(y, vec![y]);
    }

    for t in 1..obs.chars().count() {
        let mut newpath: Map<char, Vec<char>> = Map::new();
        for y in states.chars() {
            let em_p = if let Some(ob) = emit_p[&y].get(&obs.chars().nth(t).unwrap()) {
                *ob
            } else {
                MIN_FLOAT
            };

            let xs: Vec<(f64, char)> = PrevStatus[&y]
                .chars()
                .map(|y0| {
                    if let Some(ob) = trans_p[&y0].get(&y) {
                        return (v[t - 1][&y0] + ob, y0);
                    } else {
                        return (v[t - 1][&y0] + MIN_FLOAT, y0);
                    };
                })
                .collect();

            let (prob, state) = *xs.iter()
                .max_by(|x, y| x.0.partial_cmp(&y.0).unwrap())
                .unwrap();

            let mut m: ProbStart = Map::new();
            m.insert(y, prob);
            v.push(m);
            let mut tpath = path[&state].clone();
            tpath.push(y);
            &newpath.insert(y, tpath);
        }
        path = newpath;
    }
    let (prob, state) = "ES".chars()
        .map(|y| (v[obs.len() - 1][&y], y))
        .max_by(|x, y| x.0.partial_cmp(&y.0).unwrap())
        .unwrap();

    (prob, path[&state].clone())
}

fn __cut<'a>(sentence: &'a str) -> Vec<&'a str> {
    let (prob, pos_list) = viterbi(
        sentence,
        "BMES",
        &*prob_start::P,
        &*prob_trans::P,
        &*prob_emit::P,
    );

    // println!("{}-{:?}", prob, pos_list);
    // println!("{:?}", &prob_start::P);
    vec!["fuck"]
}

pub fn cut<'a>(sentence: &'a str) -> Vec<String> {
    let re_han = Regex::new(r"([\x{4E00}-\x{9FD5}]+)").unwrap();
    let re_skip = Regex::new(r"([a-zA-Z0-9]+(?:\.\d+)?%?)").unwrap();
    let blocks = SplitCaptures::new(&re_han, &sentence);
    let mut segs: Vec<String> = Vec::new();
    for blk in blocks {
        match blk {
            SplitState::Captured(caps) => {
                for word in __cut(&caps[0]) {
                    // TODO: Force split words
                    segs.push(word.to_string());
                }
            }
            SplitState::Unmatched(t) => {
                let tmp = SplitCaptures::new(&re_skip, &t);
                for x in tmp {
                    match x {
                        SplitState::Captured(caps) => segs.push(caps[0].to_string()),
                        SplitState::Unmatched(t) => segs.push(t.to_string()),
                    }
                }
            }
        }
    }
    segs
}