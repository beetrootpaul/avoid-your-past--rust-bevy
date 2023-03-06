# Avoid Your Past (Bevy/Rust)

> a snake-like [PICO-8](https://www.lexaloffle.com/pico-8.php) game where each collected coin brings a harmful memory of
> your past steps. Rewritten from PICO-8/Lua to Bevy/Rust as an exercise.

Based on https://github.com/beetrootpaul/avoid-your-past

Temporarily deployed to https://beetrootpaul.itch.io/tmp-avoid-your-past-rust-bevy (behind password `qwerty` – yes, I am
OK with sharing it publicly here, because the only reason I restrict access is to not have this game listed publicly on
itch.io 😄 ).

Controls:

- keyboard: arrows to move
- gamepad: D-Pad to move
- gamepad: left stick to move as well
- keyboard, in debug build: `D` to enter debug pause, in which most of systems stop and pressing `.` advances the game
  by 1 frame. BTW movement works, since keyboard handling happens on a regular system, while all main game systems
  happen on a fixed FPS – meaning, you can change player's movement direction while being in debug pause, then press `.`
  to see its result
- keyboard, in debug build: `S` to toggle sprite boundaries visualization
- keyboard, in debug build: `C` to toggle hit circles visualization

## License

TODO: choose a license which allows non-commercial usage. It is probably "developer-friendly" type of a license.