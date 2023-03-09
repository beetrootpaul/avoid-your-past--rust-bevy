# Bevy web game PoC

This is a personal PoC of a web (desktop&mobile) game setup based on Bevy game engine. I managed to make it playable on
my phone and some other people confirmed it works on their phones as well.

Based on https://github.com/beetrootpaul/avoid-your-past (partial rewrite)

Deployed to https://beetrootpaul.itch.io/bevy-web-game-poc

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

![progress 2023-03-07 mobile controls.jpg | width=256](progress%2Fprogress%202023-03-07%20mobile%20controls.jpg)

## License

Distributed under the terms of the MIT license.

See [LICENSE](LICENSE).