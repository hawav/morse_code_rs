# Morse Code Codec

Morse code codec supporting no_std environment.

This project is still incomplete, and contributions are welcome.

    Note: Supporting digital encoding and decoding is super simple, but the author is too lazy to write it. Newbies, come and join in the practice!

Roadmap

- Support alphabetical codec [x]
- Support digital codec [ ]
- Support punctuation codec [ ]

Encoding

```rust
let morse_code_encoder = MorseCodeEncoder::new(b"HELLO WORLD".iter());

for action in morse_code_encoder.into_iter() {
    let delay = match action {
        morse_code::MorseCodeAction::Activate(delay) => {
            // Activate the buzzer or light up the LED
            delay
        }
        morse_code::MorseCodeAction::Deactivate(delay) => {
            // Deactivate the buzzer or turn off the LED
            delay
        }
    };

    // Delay for 'delay' time units

    // For example, in a std environment using thread::sleep
    // let delay = delay as u64;
    // thread::sleep(Duration::from_millis(500 * delay))
}
```

Decoding

```rust
let mut morse_code_decoder = MorseCodeDecoder::new(100);
let mut last_ticks = 0;

loop {
    wait_for_key_state_change(); // Wait for a toggle in Morse code input

    let ticks = get_ticks(); // Obtain the current time using some method
    let delta_ticks = ticks - last_ticks; // Calculate the time elapsed since the last state change
    last_ticks = ticks;

    match morse_code_decoder.toggle(delta_time) {
        Some(res) => match res {
            morse_code::ToggleOutput::Done(ch) => { // A complete sentence has been received
                info!("{}", ch as char);
            }
            morse_code::ToggleOutput::Char(ch) => { // A new character has been received
                info!("{}", ch as char);
            }
        },
        None => {} // No content has been decoded yet
    }
}
```
