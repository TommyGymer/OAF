mod session;
mod buffer;
use crate::session::*;

fn main() {
    let e = End::Scored (
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

    let mut data = e.serialise().unwrap();
    println!("data: {:?}", data);
    println!("deserialised: {:?}", End::deserialise(&mut data));

    let s = MeasuredScore {
        value: 7,
        value_name: "7".to_string(),
        r: 6,
        theta: 8,
    };

    println!("{:?}", MeasuredScore::deserialise(&mut s.serialise().unwrap()))
}