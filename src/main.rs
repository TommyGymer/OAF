// structs should implement the Binary trait

mod buffer;
use crate::buffer::Buffer;

// use std::io::Read;

trait Serialisable {
    fn serialise(&self) -> Buffer;
    fn deserialise(data: &mut Buffer) -> Self;
}

#[derive(Debug)]
struct Session {
    date: String,
    location: String,
    // bow: Bow,
    // archer: Archer,
    rounds: Vec<Round>,
}

impl Serialisable for Session {
    fn serialise(&self) -> Buffer {
        let mut res = Buffer::new();

        res.append_string(&self.date);

        res.append_string(&self.location);

        res.append_usize(self.rounds.len());
        for round in &self.rounds {
            res.append(&mut round.serialise());
        }

        res
    }

    fn deserialise(data: &mut Buffer) -> Self {
        todo!()
    }
}

#[derive(Debug)]
struct Round {
    name: String,
    targets: Vec<Target>,
}

impl Serialisable for Round {
    fn serialise(&self) -> Buffer {
        let mut res = Buffer::new();

        res.append_string(&self.name);

        res.append_usize(self.targets.len());
        for target in &self.targets {
            res.append(&mut target.serialise());
        }

        res
    }

    fn deserialise(data: &mut Buffer) -> Self {
        todo!()
    }
}

#[derive(Debug)]
struct Target {
    name: String,
    distance: u32,
    distance_unit: String,
    face_size: u32,
    face_size_unit: String,
    inclination: u32,
    ends: Vec<End>,
}

impl Serialisable for Target {
    fn serialise(&self) -> Buffer {
        let mut res = Buffer::new();

        res.append_string(&self.name);

        res.append_u32(self.distance);
        res.append_string(&self.distance_unit);

        res.append_u32(self.face_size);
        res.append_string(&self.face_size_unit);

        res.append_u32(self.inclination);

        res.append_usize(self.ends.len());
        for end in &self.ends {
            res.append(&mut end.serialise());
        }

        res
    }

    fn deserialise(data: &mut Buffer) -> Self {
        let name = data.pop_string();

        let dist = data.pop_u32();
        let dist_unit = data.pop_string();

        let face = data.pop_u32();
        let face_unit = data.pop_string();

        let inclination = data.pop_u32();

        let mut ends = vec![];
        let read = data.pop_usize();

        for _ in 0..read {
            let s = End::deserialise(data);
            ends.push(s);
        }

        Target {
            name,
            distance: dist,
            distance_unit: dist_unit,
            face_size: face,
            face_size_unit: face_unit,
            inclination,
            ends,
        }
    }
}

#[derive(Debug, PartialEq)]
enum End {
    ScoredEnd(Vec<ValueScore>),
    MeasuredEnd(Vec<MeasuredScore>),
}

impl Serialisable for End {
    fn serialise(&self) -> Buffer {
        match self {
            End::ScoredEnd(ends) => {
                let mut res = Buffer::from(vec![0]);

                res.append_usize(ends.len());
                for score in ends {
                    res.append(&mut score.serialise());
                }

                res
            },
            End::MeasuredEnd(ends) => {
                let mut res = Buffer::from(vec![1]);

                res.append_usize(ends.len());
                for score in ends {
                    res.append(&mut score.serialise());
                }

                res
            },
        }
    }

    fn deserialise(data: &mut Buffer) -> Self {
        let t = data.pop_u8();
        match t {
            0 => {
                let mut scores = vec![];
                let read = data.pop_usize();

                for _ in 0..read {
                    let s = ValueScore::deserialise(data);
                    scores.push(s);
                }

                End::ScoredEnd(scores)
            },
            1 => {
                let mut scores = vec![];
                let read = data.pop_usize();

                for _ in 0..read {
                    let s = MeasuredScore::deserialise(data);
                    scores.push(s);
                }

                End::MeasuredEnd(scores)
            },
            other => {
                panic!("unknown end type: {}", other)
            }
        }
    }
}

#[derive(Debug, PartialEq)]
struct ValueScore {
    value: u8,
    value_name: String,
}

#[derive(Debug, PartialEq)]
struct MeasuredScore {
    value: u8,
    value_name: String,
    r: u32,
    theta: u32
}

impl Serialisable for ValueScore {
    fn serialise(&self) -> Buffer {
        let mut res = Buffer::from(vec![self.value]);

        res.append_string(&self.value_name);

        res
    }

    fn deserialise(data: &mut Buffer) -> Self {
        Self {
            value: data.pop_u8(),
            value_name: data.pop_string(),
        }
    }
}

impl Serialisable for MeasuredScore {
    fn serialise(&self) -> Buffer {
        let mut res = Buffer::from(vec![self.value]);

        res.append_string(&self.value_name);

        res.append_u32(self.r);
        res.append_u32(self.theta);

        res
    }

    fn deserialise(data: &mut Buffer) -> Self {
        Self {
            value: data.pop_u8(),
            value_name: data.pop_string(),
            r: data.pop_u32(),
            theta: data.pop_u32(),
        }
    }
}

fn main() {
    let e = End::ScoredEnd (
        vec![
            ValueScore {
                value: 9,
                value_name: String::from("9"),
            },
            ValueScore {
                value: 9,
                value_name: String::from("9"),
            },
            ValueScore {
                value: 9,
                value_name: String::from("9"),
            },
        ]
    );

    let mut data = e.serialise();
    println!("data: {:?}", data);
    println!("deserialised: {:?}", End::deserialise(&mut data));

    let s = MeasuredScore {
        value: 7,
        value_name: "7".to_string(),
        r: 6,
        theta: 8,
    };

    println!("{:?}", MeasuredScore::deserialise(&mut s.serialise()))
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_measured_score_serialise() {
        let data = MeasuredScore {
            value: 7,
            value_name: "seven".to_string(),
            r: 255,
            theta: 6000,
        }.serialise();
        assert_eq!(
            data,
            vec![7, 5, 0, 115, 101, 118, 101, 110, 255, 0, 0, 0, 112, 23, 0, 0]
        )
    }

    #[test]
    fn test_measured_score_deserialise() {
        let data = MeasuredScore::deserialise(&mut Buffer::from(vec![7, 5, 0, 115, 101, 118, 101, 110, 255, 0, 0, 0, 112, 23, 0, 0]));
        let s = MeasuredScore {
            value: 7,
            value_name: "seven".to_string(),
            r: 255,
            theta: 6000,
        };
        assert_eq!(
            data,
            s,
        )
    }

    #[test]
    fn test_value_score_serialise() {
        let data = ValueScore {
            value: 7,
            value_name: "seven".to_string(),
        }.serialise();
        assert_eq!(
            data,
            vec![7, 5, 0, 115, 101, 118, 101, 110]
        )
    }

    #[test]
    fn test_value_score_deserialise() {
        let data = ValueScore::deserialise(&mut Buffer::from(vec![7, 5, 0, 115, 101, 118, 101, 110]));
        let s = ValueScore {
            value: 7,
            value_name: "seven".to_string(),
        };
        assert_eq!(
            data,
            s,
        )
    }

    #[test]
    fn test_measured_end_serialise() {
        let data = End::MeasuredEnd( vec![
            MeasuredScore {
                value: 7,
                value_name: "seven".to_string(),
                r: 255,
                theta: 6000,
            },
            MeasuredScore {
                value: 6,
                value_name: "six".to_string(),
                r: 1000,
                theta: 3000,
            },
            MeasuredScore {
                value: 5,
                value_name: "five".to_string(),
                r: 1500,
                theta: 50,
            }
        ] ).serialise();
        assert_eq!(
            data,
            vec![1, 3, 0,
                7, 5, 0, 115, 101, 118, 101, 110, 255, 0, 0, 0, 112, 23, 0, 0,
                6, 3, 0, 115, 105, 120, 232, 3, 0, 0, 184, 11, 0, 0,
                5, 4, 0, 102, 105, 118, 101, 220, 5, 0, 0, 50, 0, 0, 0,
            ]
        )
    }

    #[test]
    fn test_measured_end_deserialise() {
        let data = End::deserialise(
            &mut Buffer::from(
                vec![1, 3, 0,
                        7, 5, 0, 115, 101, 118, 101, 110, 255, 0, 0, 0, 112, 23, 0, 0,
                        6, 3, 0, 115, 105, 120, 232, 3, 0, 0, 184, 11, 0, 0,
                        5, 4, 0, 102, 105, 118, 101, 220, 5, 0, 0, 50, 0, 0, 0,
                ]
            ));
        let s = End::MeasuredEnd( vec![
            MeasuredScore {
                value: 7,
                value_name: "seven".to_string(),
                r: 255,
                theta: 6000,
            },
            MeasuredScore {
                value: 6,
                value_name: "six".to_string(),
                r: 1000,
                theta: 3000,
            },
            MeasuredScore {
                value: 5,
                value_name: "five".to_string(),
                r: 1500,
                theta: 50,
            }
        ] );
        assert_eq!(
            data,
            s,
        )
    }
    #[test]
    fn test_value_end_serialise() {
        let data = End::ScoredEnd( vec![
            ValueScore {
                value: 7,
                value_name: "seven".to_string(),
            },
            ValueScore {
                value: 6,
                value_name: "six".to_string(),
            },
            ValueScore {
                value: 5,
                value_name: "five".to_string(),
            }
        ] ).serialise();
        assert_eq!(
            data,
            vec![0, 3, 0,
                7, 5, 0, 115, 101, 118, 101, 110,
                6, 3, 0, 115, 105, 120,
                5, 4, 0, 102, 105, 118, 101,
            ]
        )
    }

    #[test]
    fn test_value_end_deserialise() {
        let data = End::deserialise(
            &mut Buffer::from(
                vec![0, 3, 0,
                        7, 5, 0, 115, 101, 118, 101, 110,
                        6, 3, 0, 115, 105, 120,
                        5, 4, 0, 102, 105, 118, 101,
                ]
            ));
        let s = End::ScoredEnd( vec![
            ValueScore {
                value: 7,
                value_name: "seven".to_string(),
            },
            ValueScore {
                value: 6,
                value_name: "six".to_string(),
            },
            ValueScore {
                value: 5,
                value_name: "five".to_string(),
            }
        ] );
        assert_eq!(
            data,
            s,
        )
    }
}
