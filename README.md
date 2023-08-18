# intranq

Bevy implementation of intranq DAW.

## Goals

- Performance
    - Low latency
    - Low CPU usage
    - RAM is cheap
    - Background processing: https://www.cocporn.com/blog/autofreeze
    - Background compilation (signal network and internal effects/instruments)
    - Separate Bevy worlds for UI and audio (?)
    - This is not a "wish", if it cannot be achieved, the project is a failure and should be abandoned
- Modularity
- 3D UI albeit with 2D usage patterns
- Internal effects/instruments
    - Simple porting from Jesusonic?
    - Simple generation of UI
- Keybindings
    - Vim-like motion
    - Chorded
    - Customizable
- Command bar
- Good stuff from REAPER
    - Slip editing
    - Dry/wet on effects
    - Clip handles
- Git integration
- Signal path is static
- Timeline is dynamic

## UI components needed

- Transport
- Timeline editor
- Shell

## Previous notes

- Clip
    - Waveform
    - Note data
    - Automation stuff?
- Inspector
- Main menu
- Mixer
- Effects rack