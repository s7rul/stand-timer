# Stand timer

A simple timer to remind you to stand up and sit down when you work. Will also send notifications.

## Install
Install from crates.io:
```
cargo install stand-timer

```
Install from local source:
```
cargo install --path ./ --force
```

## Run
Start the timer with:
```
stand-timer --sit-time [minutes_to_sit] --stand-time [minutes_to_stand]
```
The program will then alternate between siting and standing based on the times given and sending a notification when it is time to change.
