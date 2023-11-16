// structs should implement the Binary trait

trait Serialisable {
    fn serialise(&self) -> Vec<u8>;
}

fn usize_as_u16_as_bytes(len: usize) -> Vec<u8> {
    //TODO: check if the len is >= 2^16
    (len as u16).to_le_bytes().to_vec()
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
    fn serialise(&self) -> Vec<u8> {
        todo!()
    }
}

#[derive(Debug)]
struct Round {
    name: String,
    targets: Vec<Target>,
}

impl Serialisable for Round {
    fn serialise(&self) -> Vec<u8> {
        let mut res = vec!();

        res.append(&mut usize_as_u16_as_bytes(self.name.as_bytes().len()));
        res.extend_from_slice(self.name.as_bytes());

        for target in self.targets {
            res.append(&mut target.serialise());
        }

        res
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
    fn serialise(&self) -> Vec<u8> {
        let mut res = vec!();

        res.append(&mut usize_as_u16_as_bytes(self.name.as_bytes().len()));
        res.extend_from_slice(self.name.as_bytes());

        res.extend_from_slice(&self.distance.to_le_bytes());
        res.append(&mut usize_as_u16_as_bytes(self.distance_unit.as_bytes().len()));
        res.extend_from_slice(self.distance_unit.as_bytes());

        res.extend_from_slice(&self.face_size.to_le_bytes());
        res.append(&mut usize_as_u16_as_bytes(self.face_size_unit.as_bytes().len()));
        res.extend_from_slice(self.face_size_unit.as_bytes());

        res.extend_from_slice(&self.inclination.to_le_bytes());

        for end in self.ends {
            res.append(&mut end.serialise());
        }

        res
    }
}

#[derive(Debug)]
enum End {
    ScoredEnd(Vec<ValueScore>),
    MeasuredEnd(Vec<MeasuredScore>),
}

impl Serialisable for End {
    fn serialise(&self) -> Vec<u8> {
        match self {
            End::ScoredEnd(ends) => {
                let mut res = vec![0];

                for score in ends {
                    res.append(&mut score.serialise());
                }

                res
            },
            End::MeasuredEnd(ends) => {
                let mut res = vec![1];

                for score in ends {
                    res.append(&mut score.serialise());
                }

                res
            },
        }
    }
}

#[derive(Debug)]
struct ValueScore {
    value: u8,
    value_name: String,
}

#[derive(Debug)]
struct MeasuredScore {
    value: u8,
    value_name: String,
    r: u32,
    theta: u32
}

impl Serialisable for ValueScore {
    fn serialise(&self) -> Vec<u8> {
        let mut res = vec![self.value];

        res.extend_from_slice(usize_as_u16_as_bytes(self.value_name.as_bytes().len()).as_slice());
        res.extend_from_slice(self.value_name.as_bytes());

        res
    }
}

impl Serialisable for MeasuredScore {
    fn serialise(&self) -> Vec<u8> {
        let mut res = vec![self.value];

        res.append(&mut usize_as_u16_as_bytes(self.value_name.as_bytes().len()));
        res.extend_from_slice(self.value_name.as_bytes());

        res.extend_from_slice(&self.r.to_le_bytes());
        res.extend_from_slice(&self.theta.to_le_bytes());

        res
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

    println!("{:?}", e.serialise());
}
