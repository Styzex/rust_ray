# Rust Ray

A simple raycasting-based game engine written in Rust.

## License

Rust Ray is released under the [Apache 2.0 License](http://www.apache.org/licenses/).

## Dependencies

- glu-sys
- imgui
- libm
- sdl2

## Prerequisites
- Run the Visual Studio 2019 build tools installer
- For easy setup, select the Desktop development with C++ workload in the installer.
- For a minimal setup, follow these steps:
    - In the installer, navigate to Individual components
    - Select the latest MSVC for your architecture and version of Windows
    - Select the latest Windows SDK for your version of Windows
    - Select the C++ CMake tools for Windows component
    - Install the components

## Compiling

### Windows
To compile the game engine just run this commmand `cargo build` everything should work out of the box.

### Linux

#### Debian/Ubuntu
```bash
sudo apt install libsdl2-dev libgl1-mesa-dev libglu1-mesa-dev
```
#### Fedora
```bash
sudo dnf install SDL2-devel mesa-libGL-devel mesa-libGLU-devel
```
#### Arch Linux
```bash
sudo pacman -S sdl2 mesa
```
#### NixOS
Add this to your configuration.nix:
```
environment.systemPackages = with pkgs; [
  SDL2
  libGL
  glu
];
```
Or for a temporary shell:
```bash
nix-shell -p SDL2 libGL glu
```
After that you should be able compile the game engine with this commmand `cargo build` the build.rs will set everything you need for it to compile on linux.

### MacOS

To compile the game engine just run this commmand `cargo build` everything should work out of the box.

## Planned Features

- Implement a simple 3D renderer for obj models
- ImGui-based debug menu
  - Variable manipulation
  - Position viewing
  - Performance statistics
- A simple level editor
- Baked lighting (experimental)
- Map from a **_*map*_.rrm** file

## Implemented Features

- Fake 3D rendering from the raycasting
- 2D rendering
- Raycasting
- Player movement

## The custom map file format

- The name of the file will be shown in the map selection menu so map if its called map.rrm
- It doesn't support comments
- Any number that is a 1 or 0 will break it

map.rrm

```
size = u8

[1, 1, 1, 1, 1, 1, 1, 1],
[1, 0, 0, 0, 0, 0, 0, 1],
[1, 0, 0, 0, 0, 0, 0, 1],
[1, 0, 0, 0, 0, 0, 0, 1],
[1, 0, 0, 0, 0, 0, 0, 1],
[1, 0, 0, 0, 0, 0, 0, 1],
[1, 0, 0, 0, 0, 0, 0, 1],
[1, 1, 1, 1, 1, 1, 1, 1],
```
