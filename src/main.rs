mod session;
mod buffer;
use crate::session::*;

fn main() {
    // let e = End::Scored (
    //     vec![
    //         ValueScore {
    //             value: 9,
    //             value_name: String::from("9"),
    //         },
    //         ValueScore {
    //             value: 9,
    //             value_name: String::from("9"),
    //         },
    //         ValueScore {
    //             value: 9,
    //             value_name: String::from("9"),
    //         },
    //     ]
    // );
    //
    // let mut data = e.serialise().unwrap();
    // println!("data: {:?}", data);
    // println!("deserialised: {:?}", End::deserialise(&mut data));
    //
    // let s = MeasuredScore {
    //     value: 7,
    //     value_name: "7".to_string(),
    //     r: 6,
    //     theta: 8,
    // };
    //
    // println!("{:?}", MeasuredScore::deserialise(&mut s.serialise().unwrap()))

    let s = Session {
        date: "2024-01-02".to_string(),
        location: "Home".to_string(),
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
                                    value: 10,
                                    value_name: "10".to_string(),
                                },
                            ])
                        ],
                    }
                ],
            }
        ],
    };

    println!("before: {:?}", s);

    s.encode("tmp.oaf".to_string()).unwrap();

    let decoded_s = Session::decode("tmp.oaf".to_string()).unwrap();

    println!("after: {:?}", decoded_s);
}