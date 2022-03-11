Intel 8080 emulator written in rust that runs natively via sdl and on web with webassembly. Currently space invaders has been implemented.

![Screen Shot](/screenshot.jpeg?raw=true "Screenshot")

[Live Demo](https://jgautier.github.io/eightyeighty/web/index.html)

To run on ubuntu:

```
sudo apt-get install libsdl2-dev
sudo apt-get install libsdl2-mixer-dev
cargo run
```

To rebuild web version:
```
cd web
wasm-pack build --target web
```

Resources used to develop and debug:

[Emulator 101](http://emulator101.com/)

[Rust-8080](https://github.com/Tom-Goring/Rust-8080/)

[space-invaders.rs](https://github.com/cbeust/space-invade.rs)

[i8080 opcodes](https://www.pastraiser.com/cpu/i8080/i8080_opcodes.html)
