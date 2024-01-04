use morse_code::MorseCodeDecoder;

fn main() {
    // let encoder = MorseCodeEncoder::new(b"hello".iter());
    // println!("{:?}", encoder.into_iter().collect::<Vec<_>>());
    // let encoder = MorseCodeEncoder::new(b"hello".iter());
    // for action in encoder.into_iter() {
    //     let delay = match action {
    //         morse_code::MorseCodeAction::Activate(delay) => {
    //             println!("哔！！！！",);
    //             delay
    //         }
    //         morse_code::MorseCodeAction::Deactivate(delay) => {
    //             println!("（关掉了）",);
    //             delay
    //         }
    //     };
    //     let delay = delay as u64;
    //     thread::sleep(Duration::from_millis(500 * delay))
    // }

    let mut decode = MorseCodeDecoder::new(100);
    dbg!(decode.toggle(100));
    dbg!(decode.toggle(100));
    dbg!(decode.toggle(100));
    dbg!(decode.toggle(100));
    dbg!(decode.toggle(100));
    dbg!(decode.toggle(100));
    dbg!(decode.toggle(100));
    dbg!(decode.toggle(100));

    dbg!(decode.toggle(1000));
}
