# rogad

A rust daemon that manages various input events for the Odroid Go Advance.

## Goals

- System hotkeys that don't get passed to applications
- Robust headphone jack support
- Convenience functions for hardware commands (change brightness / audio output)
- Learn a bit more rust
- Build the smallest possible rust executable
- Make efficient use of CPU time and memory
- Provide platform-agnostic features that will work with any distribution

## Installation

Assuming your Go Advance is on your network and you've set up key-based ssh,
just run `./deploy.sh` or `./deploy.sh --release` to build and start the daemon.
Note that, for the smallest possible binary, the release has almost no error
handling or logging.

## Usage

Run as root; the "real" gamepad will be seized and a proxy will be created.
The proxy will receive all events except when the hotkey is held down.

When the hotkey is held down and released, the events are sent once the
key is released, allowing it to function as a normal button.

When the hotkey is held down and a button is pressed, a handler is called.
If nothing handles this key combination, hotkey mode is exited and the
original events are sent in order, allowing hotkeys to be shared with
other applications.

If the key combination has a handler, the event is consumed. Holding down
a key combination fires the handler every 50ms.

## TODO

- Learn rust.
- Clean up file reference passing / find a more elegant solution.
- Consider rewriting with lazy_static.
- Modularize gamepad event handling.
- Add configuration support.
- Rewrite the keydown handlers to fire on independent threads.
  - This means all threads will either have long timeouts or block on input.
  - Currently, holding down the hotkey imposes a 2-3% CPU load and delays
    event delivery by up to 10ms; the idle blocking input loop consumes
    roughly 0% CPU.
