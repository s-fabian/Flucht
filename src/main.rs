//! ```cargo
//! [dependencies]
//! log = "0.4"
//! colog = "1.1"
//! colored = "2"
//! ```

extern crate colored;
extern crate log;
extern crate colog;

use colored::Colorize;
use log::info;
use std::fs;
use std::collections::HashMap;
use std::char;
use std::fmt;
// use std::str::from_utf8;

const MY_COLORS: [(u8, u8, u8); 10] = [
    (1u8, 111u8, 185u8),   // 0
    (236u8, 78u8, 32u8),   // 1
    (100u8, 149u8, 5u8),   // 2
    (125u8, 120u8, 90u8),  // 3
    (66u8, 72u8, 116u8),   // 4
    (125u8, 78u8, 87u8),   // 5
    (147u8, 225u8, 216u8), // 6
    (63u8, 239u8, 45u8),   // 7
    (238u8, 99u8, 102u8),  // 8
    (224u8, 202u8, 60u8),  // 9
];

#[derive(Clone)]
#[allow(dead_code)]
struct Element {
    top: i32,
    left: i32,
    width: i32,
    height: i32,
    type_id: i32,
}

#[derive(Clone)]
struct Move {
    nr: i32,
    direction: Direction,
}

#[derive(Clone)]
pub struct Field {
    data: [Element; 10],
    hash: String,
    moves: Vec<Move>,
    rating: i32,
}

#[derive(Clone)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}

impl Element {
    fn get_index(self: &Element) -> u8 {
        ((self.top - 1) * 4 + self.left - 1) as u8
    }
}

impl Field {
    fn new() -> Field {
        let data = [
            Element {
                // red
                top: 1,
                left: 2,
                width: 2,
                height: 2,
                type_id: 0,
            },
            Element {
                // yellow 1
                top: 2,
                left: 1,
                width: 1,
                height: 2,
                type_id: 1,
            },
            Element {
                // yellow 2
                top: 2,
                left: 4,
                width: 1,
                height: 2,
                type_id: 1,
            },
            Element {
                // yellow 3
                top: 3,
                left: 2,
                width: 2,
                height: 1,
                type_id: 1,
            },
            Element {
                // yellow 4
                top: 4,
                left: 1,
                width: 1,
                height: 2,
                type_id: 1,
            },
            Element {
                // yellow 5
                top: 4,
                left: 4,
                width: 1,
                height: 2,
                type_id: 1,
            },
            Element {
                // blue 1
                top: 4,
                left: 2,
                width: 1,
                height: 1,
                type_id: 2,
            },
            Element {
                // blue 2
                top: 4,
                left: 3,
                width: 1,
                height: 1,
                type_id: 2,
            },
            Element {
                // blue 3
                top: 5,
                left: 2,
                width: 1,
                height: 1,
                type_id: 2,
            },
            Element {
                // blue 4
                top: 5,
                left: 3,
                width: 1,
                height: 1,
                type_id: 2,
            },
        ];
        let mut f = Field {
            data,
            hash: "".to_string(),
            moves: vec![],
            rating: 0,
        };

        f.do_hash();
        f.save_choices();
        f
    }

    fn do_hash(self: &mut Field) {
        self.hash.clear();

        for i in &self.data {
            let byte = (((i.top - 1) * 4 + i.left) + 64) as u8 as char;
            self.hash.push(byte);
        }

        let mut r: i32 = 0;
        let mut anz = 0;
        for i in &self.data {
            anz += 1;
            if anz == 1 {
                r += i.top * 20;
                if i.left == 1 || i.left == 3 {
                    r += 10;
                }
            } else {
                r += 5 - i.top;
                if i.width == 2 {
                    if i.left == 1 || i.left == 3 {
                        r += 10;
                    }
                }
            }
        }

        self.rating = r;
        if self.data[0].top == 4 && self.data[0].left == 2 {
            self.rating = 250;
        }
    }

    fn save_choices(self: &mut Field) {
        let board = self.to_u8a();
        let mut indexes: Vec<i32> = vec![];

        let mut done: Vec<Move> = vec![];

        for (i, val) in board.iter().enumerate() {
            if *val == 32 as u8 {
                indexes.push(i as i32);
            }
        }
        for i in indexes {
            // left
            if (i % 4) != 0 && i > 0 {
                let e_i = (i - 1) as usize;
                let e = board[e_i];
                if e != 32 {
                    let e_e: &Element = &self.data[e as usize];
                    if e_e.get_index() + (e_e.width as u8) - 1 != e_i as u8 {
                    } else if e_e.height > 1 {
                        if board[(i + 4) as usize] == 32 {
                            done.push(Move {
                                nr: e as i32,
                                direction: Direction::Right,
                            });
                        }
                    } else {
                        done.push(Move {
                            nr: e as i32,
                            direction: Direction::Right,
                        });
                    };
                };
            };
            // right
            if (i % 4) != 3 && i < 19 {
                let e_i = (i + 1) as usize;
                let e = board[e_i];
                if e != 32 {
                    let e_e = &self.data[e as usize];
                    if e_e.get_index() != e_i as u8 {
                    } else if e_e.height > 1 {
                        if board[(i + 4) as usize] == 32 {
                            done.push(Move {
                                nr: e as i32,
                                direction: Direction::Left,
                            });
                        }
                    } else {
                        done.push(Move {
                            nr: e as i32,
                            direction: Direction::Left,
                        });
                    };
                };
            };
            // up
            if i > 3 {
                let e_i = (i - 4) as usize;
                let e = board[e_i];
                if e != 32 {
                    let e_e = &self.data[e as usize];
                    if e_e.get_index() + ((e_e.height - 1) as u8) * 4 != e_i as u8 {
                    } else if e_e.width > 1 {
                        if board[(i + 1) as usize] == 32 {
                            done.push(Move {
                                nr: e as i32,
                                direction: Direction::Down,
                            });
                        }
                    } else {
                        done.push(Move {
                            nr: e as i32,
                            direction: Direction::Down,
                        });
                    };
                };
            };
            // down
            if i < 16 {
                let e_i = (i + 4) as usize;
                let e = board[e_i];
                if e != 32 {
                    let e_e = &self.data[e as usize];
                    if e_e.get_index() != e_i as u8 {
                    } else if e_e.width > 1 {
                        if board[(i + 1) as usize] == 32 {
                            done.push(Move {
                                nr: e as i32,
                                direction: Direction::Up,
                            });
                        }
                    } else {
                        done.push(Move {
                            nr: e as i32,
                            direction: Direction::Up,
                        });
                    };
                };
            };
        }
        self.moves = done;
    }

    fn to_u8a(self: &Field) -> [u8; 20] {
        let mut s: [u8; 20] = b"                    ".to_owned();

        for (i, val) in (&self.data).iter().enumerate() {
            let mut p: u8;
            for iy in 1..=val.height {
                p = val.get_index() + ((iy - 1) * 4) as u8;

                for _ in 1..=val.width {
                    s[p as usize] = i as u8;
                    p += 1;
                }
            }
        }
        s
    }

    fn move_ref(self: &mut Field, move__: &Move) {
        let index = move__.nr;
        let direction: (i32, i32) = match move__.direction {
            Direction::Down => (0, 1),
            Direction::Up => (0, -1),
            Direction::Left => (-1, 0),
            Direction::Right => (1, 0),
        };
        self.data[index as usize].left += direction.0;
        self.data[index as usize].top += direction.1;
        self.do_hash();
        self.save_choices();
    }

    fn show_hist(self: &Field, moves: &Vec<u8>) -> String {
        let mut show_f = self.clone();
        let mut s: Vec<String> = vec![];
        let mut z = moves.clone();

        s.push(format!("{}", show_f));
        while z.len() > 0 {
            let p = z.pop().unwrap() as usize;

            if p > 0 {
                if p <= show_f.moves.len() {
                    show_f.move_ref(&show_f.moves[p - 1].clone());
                    s.push(format!("{}", show_f));
                }
            }
        }
        let mut out = String::new();
        const ANZ: usize = 8;
        const EMPTY_STRING: String = String::new();

        for i in 1..=s.len() / ANZ + 1 {
            // sorry for bad code
            let mut a: [String; ANZ] = [EMPTY_STRING; ANZ];
            for c in 1..=ANZ {
                let c2 = (i - 1) * ANZ + c;
                if c2 > s.len() {
                    continue;
                }
                let this = &s[c2 - 1];
                let that: Vec<&str> = this.split('\n').collect();

                for (n, j) in that.iter().enumerate() {
                    if n >= ANZ {
                        continue;
                    }
                    a[n].push_str(&("  ".to_string() + j))
                }
            }
            for x in a {
                out.push_str(&(x + "\n"));
            }
        }

        out
    }

    fn to_html(self: &Field, moves: Vec<u8>) -> String {
        let mut show_f = self.clone();
        let mut s = String::new();
        let mut z = moves.clone();

        s.push_str(&*self._to_html_help());
        while z.len() > 0 {
            let p = z.pop().unwrap() as usize;

            if p > 0 {
                if p <= show_f.moves.len() {
                    show_f.move_ref(&show_f.moves[p - 1].clone());
                    s.push_str(&show_f._to_html_help());
                }
            }
        }
        let s = format!("<h1 style=\"text-align: center; font-family: Arial, Helvetica, sans-serif\">Solution</h1>\n<div style=\"display: flex; flex-wrap: wrap; justify-content: space-around; gap: 20px;\">\n\n{}</div>\n", s);

        format!("<html>\n<head>\n<title>Solution</title>\n<style> html {} color-scheme: dark; zoom: 200%; user-select: none; {} </style>\n</head>\n<body>\n{}</body>\n</html>", "{", "}", s)
    }

    fn _to_html_help(&self) -> String {
        let mut out = String::new();
        let mut said: Vec<u8> = vec![];
        let u8a = self.to_u8a();

        for c in 0..20 {
            if c % 4 == 0 {
                out.push('\n');
            }
            let i = u8a[c];
            if i == 32 {
                out.push_str("   ");
            } else {
                let color = MY_COLORS[i as usize];

                if !said.contains(&i) {
                    let text_color = if (color.0 as i32 + color.1 as i32 + color.2 as i32) > 346 {
                        (0, 0, 0)
                    } else {
                        (255, 255, 255)
                    };

                    out.push_str(&*format!(
                        "<mark style=\"color: {}; background-color: {}\"> {} </mark>",
                        format!("rgb({}, {}, {})", text_color.0, text_color.1, text_color.2),
                        format!("rgb({}, {}, {})", color.0, color.1, color.2),
                        i
                    ));
                    said.push(i);
                } else {
                    out.push_str(&*format!(
                        "<mark style=\"background-color: rgb({}, {}, {});\">   </mark>",
                        color.0, color.1, color.2
                    ));
                }
            }
        }

        format!("<pre>{}\n</pre>\n\n", out)
    }
}

impl fmt::Display for Field {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut out = String::new();
        let u8a = self.to_u8a();

        let mut said: Vec<u8> = vec![];
        for c in 0..20 {
            if c % 4 == 0 {
                out.push('\n');
            }
            let i = u8a[c];
            if i == 32 {
                out.push_str("   ");
            } else {
                let color = MY_COLORS[i as usize];

                if !said.contains(&i) {
                    let colors = if (color.0 as i32 + color.1 as i32 + color.2 as i32) > 346 {
                        (0, 0, 0)
                    } else {
                        (255, 255, 255)
                    };

                    out.push_str(&format!(
                        "{}",
                        format!(
                            " {} ",
                            format!("{}", i).truecolor(colors.0, colors.1, colors.2)
                        )
                        .on_truecolor(color.0, color.1, color.2)
                    ));
                    said.push(i);
                } else {
                    out.push_str(&format!(
                        "{}",
                        "   ".on_truecolor(color.0, color.1, color.2)
                    ));
                }
            }
        }

        writeln!(f, "{}", out)
    }
}

impl fmt::Debug for Element {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let type_str = format!("{}x{}", &self.width, &self.height);
        write!(
            f,
            "(top, left) = ({}, {}); type = {}",
            &self.top, &self.left, type_str
        )
    }
}

impl fmt::Display for Direction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Direction::Down => "↓".blue(),
                Direction::Up => "↑".red(),
                Direction::Left => "←".yellow(),
                Direction::Right => "→".green(),
            }
        )
    }
}

impl ToString for Move {
    fn to_string(&self) -> String {
        let colors = MY_COLORS[self.nr as usize];
        let nr = &self.nr.to_string().truecolor(colors.0, colors.1, colors.2);

        format!("id {}: {}  ", nr, &self.direction)
    }
}

type OurHash = HashMap<String, u8>;

pub fn search_best(f: Field) {
    let move_list: Vec<u8> = vec![];
    let mut i = 0;
    loop {
        i += 1;
        info!("new search depth: {}", i);

        let mut hash: OurHash = OurHash::new();
        let erg = search(&mut hash, 1, i, &f, &move_list);
        let best_rating = erg.0;
        let best_moves = erg.1;
        let s = f.show_hist(&best_moves);
        info!("Best moves: {}", s);
        info!("Depth {} finished, rating {}", i, best_rating);
        fs::write("solution.html", f.to_html(best_moves)).unwrap();
        if best_rating >= 250 {
            break;
        };
    }
    info!("{}", String::from("FOUND!").green());
}

fn search(hash_table: &mut OurHash, depth: i32, max_depth: i32, f: &Field, move_list: &Vec<u8>) -> (i32, Vec<u8>) {
    let mut best_move: i32 = 0;
    let mut best_rating: i32 = 0;
    let move_list = move_list.clone();
    let mut best_moves_list: Vec<u8> = vec![];

    if depth >= max_depth {
        return (f.rating, move_list);
    }

    let mut move_ = 1;
    for i in &f.moves {
        // dbg!(c);
        let mut hash_ok: bool = true;
        let mut f2 = f.clone();
        f2.move_ref(i);

        match hash_table.get(&f2.hash) {
            Some(v) => {
                if *v <= (depth as u8) {
                    hash_ok = false;
                } else if (*v as i32) <= (depth - 2) {
                    return (-1, move_list);
                } else {
                    hash_table.remove(&f2.hash.clone());
                }
            }
            None => (),
        };

        if hash_ok {
            hash_table.insert(f2.hash.clone(), depth as u8);
            let solution = search(hash_table, depth + 1, max_depth, &f2, &move_list);
            let rating = solution.0;
            if rating > best_rating {
                best_moves_list = solution.1;
                best_move = move_;
                best_rating = rating;
            }
        }

        move_ += 1;
    }

    best_moves_list.push(best_move as u8);
    return (best_rating as i32, best_moves_list);
}

fn main() {
    colog::init();

    let field = Field::new();
    info!("{}", &field);


    let mut s = String::new();

    for i in &field.moves {
        s.push_str(&*format!("{}", i.to_string()));
        s.push_str("\n");
    }
    info!("Moves: {}", s);

    search_best(field);

}
