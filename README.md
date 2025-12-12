# Universal Slider Bridge
This is my very first Rust code, I know it's rough. Right now, it's just a simple program to bridge the ChatMix dial on my SteelSeries Arctis Nova 7X headset to VoiceMeeter. I really don't like Sonar.

## To-Do:
There's many improvements I'd like to make to this. My (very) long term goal is a fully open source alternative to Sonar. But for now...
- [X] Retry on a timeout on connection failure to VoiceMeeter, instead of panicking
- [ ] Handle VoiceMeeter closing after a connection has been made
- [ ] Improve error handling
- [ ] Run in tray
- [ ] Auto start option
- [ ] Add support for more SteelSeries headsets
- [ ] GUI
- [ ] Allow custom mappings or plugins to control anything* from any* device