# Morse Code Codec

摩斯密码编解码器，支持 no_std 环境。

本项目仍不完善，欢迎贡献。

    Note: 支持数字编解码超简单的，只是作者犯懒不想写了，新手们快来参与一下练手呀！

路线图

- 支持字母编解码 [x]
- 支持数字编解码 [ ]
- 支持标点符号编解码 [ ]

编码

```rust
let morse_code_encoder = MorseCodeEncoder::new(b"HELLO WORLD".iter());

for action in morse_code_encoder.into_iter() {
    let delay = match action {
        morse_code::MorseCodeAction::Activate(delay) => {
            // 开启蜂鸣器或者点亮LED
            delay
        }
        morse_code::MorseCodeAction::Deactivate(delay) => {
            // 关闭蜂鸣器或者熄灭LED
            delay
        }
    };

    // 延时 delay 个单位时间

    // 比如使用 std 环境 thread::sleep
    // let delay = delay as u64;
    // thread::sleep(Duration::from_millis(500 * delay))
}
```

解码

```rust
let mut morse_code_decoder = MorseCodeDecoder::new(100);
let mut last_ticks = 0;

loop {
    wait_for_key_state_change(); // 等待摩斯密码的输入翻转

    let ticks = get_ticks(); // 通过某种方式获得运行时间
    let delta_ticks = ticks - last_ticks; // 计算距离上次切换状态过去的时间
    last_ticks = ticks;

    match morse_code_decoder.toggle(delta_time) {
        Some(res) => match res {
            morse_code::ToggleOutput::Done(ch) => { // 一个句子接收完毕
                info!("{}", ch as char);
            }
            morse_code::ToggleOutput::Char(ch) => { // 接收到一个新字符
                info!("{}", ch as char);
            }
        },
        None => {} // 还没有解析出任何内容
    }
}
```
