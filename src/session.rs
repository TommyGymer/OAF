// structs should implement the Binary trait
use crate::buffer::{Buffer, BufferError};

pub trait Serialisable<T> {
    /// Returns a Buffer containing self as bytes
    fn serialise(&self) -> Result<Buffer, BufferError>;
    /// Returns a copy of self built from the bytes in the provided Buffer
    fn deserialise(data: &mut Buffer) -> Result<T, BufferError>;
}

#[derive(Debug, PartialEq)]
pub struct Session {
    pub date: String,
    pub location: String,
    // pub bow: Bow,
    // pub archer: Archer,
    pub rounds: Vec<Round>,
}

// Magic bytes: 4F 41 46 46 (OAFF)

#[derive(Debug, Clone)]
enum EncodingError {
    BufferError(BufferError),
}

impl Session {
    fn encode(&self, filename: String) -> Result<(), EncodingError> {
        Ok(())
    }
}

impl Serialisable<Session> for Session {
    fn serialise(&self) -> Result<Buffer, BufferError> {
        let mut res = Buffer::new();

        res.append_string(&self.date)?;

        res.append_string(&self.location)?;

        res.append_usize(self.rounds.len())?;
        for round in &self.rounds {
            res.append(&mut round.serialise()?);
        }

        Ok(res)
    }

    fn deserialise(data: &mut Buffer) -> Result<Self, BufferError> {
        let date = data.pop_string()?;
        let location = data.pop_string()?;

        let mut rounds = vec![];
        let read = data.pop_usize()?;

        for _ in 0..read {
            rounds.push(Round::deserialise(data)?);
        }

        Ok(Session {
            date,
            location,
            rounds,
        })
    }
}

#[derive(Debug, PartialEq)]
pub struct Round {
    pub name: String,
    pub targets: Vec<Target>,
}

impl Serialisable<Round> for Round {
    fn serialise(&self) -> Result<Buffer, BufferError> {
        let mut res = Buffer::new();

        res.append_string(&self.name)?;

        res.append_usize(self.targets.len())?;
        for target in &self.targets {
            res.append(&mut target.serialise()?);
        }

        Ok(res)
    }

    fn deserialise(data: &mut Buffer) -> Result<Self, BufferError> {
        let name = data.pop_string()?;

        let mut targets = vec![];
        let read = data.pop_usize()?;

        for _ in 0..read {
            targets.push(Target::deserialise(data)?);
        }

        Ok(Round {
            name,
            targets,
        })
    }
}

#[derive(Debug, PartialEq)]
pub struct Target {
    pub name: String,
    pub distance: u32,
    pub distance_unit: String,
    pub face_size: u32,
    pub face_size_unit: String,
    pub inclination: u32,
    pub ends: Vec<End>,
}

impl Serialisable<Target> for Target {
    fn serialise(&self) -> Result<Buffer, BufferError> {
        let mut res = Buffer::new();

        res.append_string(&self.name)?;

        res.append_u32(self.distance);
        res.append_string(&self.distance_unit)?;

        res.append_u32(self.face_size);
        res.append_string(&self.face_size_unit)?;

        res.append_u32(self.inclination);

        res.append_usize(self.ends.len())?;
        for end in &self.ends {
            res.append(&mut end.serialise()?);
        }

        Ok(res)
    }

    fn deserialise(data: &mut Buffer) -> Result<Self, BufferError> {
        let name = data.pop_string()?;

        let dist = data.pop_u32()?;
        let dist_unit = data.pop_string()?;

        let face = data.pop_u32()?;
        let face_unit = data.pop_string()?;

        let inclination = data.pop_u32()?;

        let mut ends = vec![];
        let read = data.pop_usize()?;

        for _ in 0..read {
            ends.push(End::deserialise(data)?);
        }

        Ok(Target {
            name,
            distance: dist,
            distance_unit: dist_unit,
            face_size: face,
            face_size_unit: face_unit,
            inclination,
            ends,
        })
    }
}

#[derive(Debug, PartialEq)]
pub enum End {
    Scored(Vec<ValueScore>),
    Measured(Vec<MeasuredScore>),
    Blank(u32),
    ShotTrainer(u32),
    BareShaft(Vec<BareShaft>),
    BowDraws(u32),
}

impl Serialisable<End> for End {
    fn serialise(&self) -> Result<Buffer, BufferError> {
        Ok(match self {
            End::Scored(scores) => {
                let mut res = Buffer::from(vec![0]);

                res.append_usize(scores.len())?;
                for score in scores {
                    res.append(&mut score.serialise()?);
                }

                res
            },
            End::Measured(scores) => {
                let mut res = Buffer::from(vec![1]);

                res.append_usize(scores.len())?;
                for score in scores {
                    res.append(&mut score.serialise()?);
                }

                res
            },
            End::Blank(count) => {
                let mut res = Buffer::from(vec![2]);

                res.append_u32(*count);

                res
            },
            End::ShotTrainer(count) => {
                let mut res = Buffer::from(vec![3]);

                res.append_u32(*count);

                res
            },
            End::BareShaft(scores) => {
                let mut res = Buffer::from(vec![4]);

                res.append_usize(scores.len())?;
                for score in scores {
                    res.append(&mut score.serialise()?);
                }

                res
            },
            End::BowDraws(count) => {
                let mut res = Buffer::from(vec![5]);

                res.append_u32(*count);

                res
            },
        })
    }

    fn deserialise(data: &mut Buffer) -> Result<Self, BufferError> {
        let t = data.pop_u8()?;
        Ok(match t {
            0 => {
                let mut scores = vec![];
                let read = data.pop_usize()?;

                for _ in 0..read {
                    let s = ValueScore::deserialise(data)?;
                    scores.push(s);
                }

                End::Scored(scores)
            },
            1 => {
                let mut scores = vec![];
                let read = data.pop_usize()?;

                for _ in 0..read {
                    let s = MeasuredScore::deserialise(data)?;
                    scores.push(s);
                }

                End::Measured(scores)
            },
            2 => {
                End::Blank(data.pop_u32()?)
            },
            3 => {
                End::ShotTrainer(data.pop_u32()?)
            },
            4 => {
                let mut scores = vec![];
                let read = data.pop_usize()?;

                for _ in 0..read {
                    let s = BareShaft::deserialise(data)?;
                    scores.push(s);
                }

                End::BareShaft(scores)
            },
            5 => {
                End::BowDraws(data.pop_u32()?)
            },
            other => {
                panic!("unknown end type: {}", other)
            }
        })
    }
}

#[derive(Debug, PartialEq)]
pub struct ValueScore {
    pub value: u8,
    pub value_name: String,
}

#[derive(Debug, PartialEq)]
pub struct MeasuredScore {
    pub value: u8,
    pub value_name: String,
    pub r: u32,
    pub theta: u32,
}

#[derive(Debug, PartialEq)]
pub struct BareShaft {
    pub r: u32,
    pub theta: u32,
}

impl Serialisable<ValueScore> for ValueScore {
    fn serialise(&self) -> Result<Buffer, BufferError> {
        let mut res = Buffer::from(vec![self.value]);

        res.append_string(&self.value_name)?;

        Ok(res)
    }

    fn deserialise(data: &mut Buffer) -> Result<Self, BufferError> {
        Ok(Self {
            value: data.pop_u8()?,
            value_name: data.pop_string()?,
        })
    }
}

impl Serialisable<MeasuredScore> for MeasuredScore {
    fn serialise(&self) -> Result<Buffer, BufferError> {
        let mut res = Buffer::from(vec![self.value]);

        res.append_string(&self.value_name)?;

        res.append_u32(self.r);
        res.append_u32(self.theta);

        Ok(res)
    }

    fn deserialise(data: &mut Buffer) -> Result<Self, BufferError> {
        Ok(Self {
            value: data.pop_u8()?,
            value_name: data.pop_string()?,
            r: data.pop_u32()?,
            theta: data.pop_u32()?,
        })
    }
}

impl Serialisable<BareShaft> for BareShaft {
    fn serialise(&self) -> Result<Buffer, BufferError> {
        let mut res = Buffer::from(vec![]);

        res.append_u32(self.r);
        res.append_u32(self.theta);

        Ok(res)
    }

    fn deserialise(data: &mut Buffer) -> Result<Self, BufferError> {
        Ok(Self {
            r: data.pop_u32()?,
            theta: data.pop_u32()?,
        })
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_full() {
        let s = Session {
            date: "4/12/2023".to_string(),
            location: "St Andrews".to_string(),
            rounds: vec![
                Round {
                    name: "Portsmouth".to_string(),
                    targets: vec![
                        Target {
                            name: "WA 60cm Indoor".to_string(),
                            distance: 18,
                            distance_unit: "m".to_string(),
                            face_size: 60,
                            face_size_unit: "cm".to_string(),
                            inclination: 0,
                            ends: vec![
                                End::Scored(vec![
                                    ValueScore {
                                        value: 10,
                                        value_name: "10".to_string(),
                                    },
                                    ValueScore {
                                        value: 10,
                                        value_name: "10".to_string(),
                                    },
                                    ValueScore {
                                        value: 9,
                                        value_name: "9".to_string(),
                                    }
                                ]),
                                End::Scored(vec![
                                    ValueScore {
                                        value: 10,
                                        value_name: "10".to_string(),
                                    },
                                    ValueScore {
                                        value: 9,
                                        value_name: "9".to_string(),
                                    },
                                    ValueScore {
                                        value: 9,
                                        value_name: "9".to_string(),
                                    }
                                ])
                            ],
                        }
                    ],
                }
            ],
        };

        assert_eq!(s, Session::deserialise(&mut s.serialise().unwrap()).unwrap())
    }

    #[test]
    fn test_measured_score_serialise() {
        let data = MeasuredScore {
            value: 7,
            value_name: "seven".to_string(),
            r: 255,
            theta: 6000,
        }.serialise().unwrap();
        assert_eq!(
            data,
            vec![7, 5, 0, 115, 101, 118, 101, 110, 255, 0, 0, 0, 112, 23, 0, 0]
        )
    }

    #[test]
    fn test_measured_score_deserialise() {
        let data = MeasuredScore::deserialise(&mut Buffer::from(vec![7, 5, 0, 115, 101, 118, 101, 110, 255, 0, 0, 0, 112, 23, 0, 0])).unwrap();
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
        }.serialise().unwrap();
        assert_eq!(
            data,
            vec![7, 5, 0, 115, 101, 118, 101, 110]
        )
    }

    #[test]
    fn test_value_score_deserialise() {
        let data = ValueScore::deserialise(&mut Buffer::from(vec![7, 5, 0, 115, 101, 118, 101, 110])).unwrap();
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
        let data = End::Measured( vec![
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
        ] ).serialise().unwrap();
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
            )).unwrap();
        let s = End::Measured( vec![
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
        let data = End::Scored( vec![
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
        ] ).serialise().unwrap();
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
            )).unwrap();
        let s = End::Scored( vec![
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
