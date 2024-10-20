<h1 align="center">
<img src="/docs/public/samples/splashscreen.png" />
Hermes-Studio
</h1>

<div align="center">

[![License](https://img.shields.io/github/license/dclause/hermes-studio?color=success)](https://github.com/dclause/hermes-studio/blob/develop/LICENSE)
[![Documentation](https://img.shields.io/badge/documentation-_online-success)](https://dclause.github.io/hermes-studio/)
[![Build FRONTEND](https://github.com/dclause/hermes-studio/actions/workflows/build_frontend.yml/badge.svg)](https://github.com/dclause/hermes-studio/actions/workflows/build_frontend.yml)
[![Build BACKEND](https://github.com/dclause/hermes-studio/actions/workflows/build_backend.yml/badge.svg)](https://github.com/dclause/hermes-studio/actions/workflows/build_backend.yml)

</div>

**_Hermes-Studio_ is an open-source intuitive drag-and-drop interface to manage and remote control your Arduino-based
robot in minutes. No programming skills required.**

_Written as a decoupled backend in Rust and frontend in Vue, the whole project serves as an advanced demonstration of
the power of the [Hermes-Five](https://github.com/dclause/hermes-five) API for remote control of IoT boards._

_I personally use this project to control my homemade [InMoov](http://inmoov.fr/) robot._

<div align="center">
<img src="/docs/public/samples/boards_connected.png?raw=true" width=200 />
<img src="/docs/public/samples/control_connected.png?raw=true" width=200 />
<img src="/docs/public/samples/groups_connected.png?raw=true" width=200 />
<img src="/docs/public/samples/timeline_connected.png?raw=true" width=200 />
</div>

## Documentation

To check out docs, visit [Github Pages](https://dclause.github.io/hermes-studio/).

## Installation

You can find pre-built releases for Windows, Linux, macOS and
raspberryPiOS [here](https://github.com/dclause/hermes-studio/releases). If you'd like to compile from source,
you can follow the compilation guide.

> [!CAUTION]  
> All Arduino boards used in the project must be configured to use
> the [StandardFirmataPlus.ino](https://github.com/firmata/arduino/blob/main/examples/StandardFirmataPlus/StandardFirmataPlus.ino)
> sketch.

## Features

**Hermes-Studio** provides both an API (REST + WebSocket) and an Interface to remote control your Arduino (and
other IoT compatible boards) using [Hermes-Five](https://github.com/dclause/hermes-five) under-the-hood.

- Configure your remote controllable boards (Arduino currently)
- Control boards over Serial connection
- Access remote control website from any device
- Remote control all devices (LEDs, servos, etc.) individually
- Create and run states (static positions) of your robot
- Create and play animations from the timeline tool

## Contribution

All contributions are more than welcome through [PR](https://github.com/dclause/hermes-studio/pulls) and
the [issue queue](https://github.com/dclause/hermes-studio/issues).

- Fork the repository
- Create a new branch: `git checkout -b feature-branch`
- Commit your changes: `git commit -am 'Add new feature'`
- Push to the branch: `git push origin feature-branch`
- Create a new Pull Request

**_The author does not claim to know everything about Rust programming or IoT, and all ideas are welcome as long as they
respect the project's original philosophy._**

## License

This project is licensed under the MIT License. See
the [LICENSE](https://github.com/dclause/hermes-studio/blob/develop/LICENSE) file for details.

## Contact

For support, please open an issue or reach out to the [author](https://github.com/dclause).