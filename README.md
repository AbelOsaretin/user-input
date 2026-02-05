# user-input

A simple Rust CLI program that displays a power menu, reads user input, and prints the corresponding action.

## Features

- Menu with power options: Off, Sleep, Reboot, Shutdown, Hibernate
- Case-insensitive input handling
- Continuous prompt loop

## Requirements

- Rust (stable) and Cargo

## Run

From the project root:

- `cargo run`

Type one of the menu options (e.g., `off`, `sleep`, `reboot`, `shutdown`, `hibernate`) and press Enter.

## Example

```
Welcome
Please enter one of the menu below
Off
Sleep
Reboot
Shutdown
Hibernate

shutdown
-------------------------------
-------------------------------
Shutting Down
-------------------------------
-------------------------------
```

## Notes

- Use Ctrl+C to exit the loop.
