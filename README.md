# Pueue

[![GitHub release](https://img.shields.io/github/tag/nukesor/pueue.svg)](https://github.com/nukesor/pueue/releases/latest)
[![Crates.io](https://img.shields.io/crates/v/pueue)](https://crates.io/crates/pueue)
[![MIT Licence](https://img.shields.io/pypi/l/pueue.svg)](https://github.com/Nukesor/pueue/blob/master/LICENSE)
[![Patreon](https://github.com/Nukesor/images/blob/master/patreon-donate-blue.svg)](https://www.patreon.com/nukesor)


![Pueue](https://raw.githubusercontent.com/Nukesor/images/master/pueue.png)

Pueue is a command-line task management tool for sequential and parallel execution of long running tasks.
If you break it down, it's a manager that processes a queue of shell commands.
On top of that, there are a lot of convenience features and abstractions.

Since it's not bound to any terminal, you can control your tasks from any terminal on the same machine and even remotely.
The best part probably is, that the queue will be continuously processed, even if you no longer have any active ssh session.

It provides functionality for:
- Easy output inspection.
- Interaction with running processes
- Manipulation of the scheduled task order
- Running multiple tasks at once (You can decide how many concurrent tasks you want to run)

**Pueue has been rewritten in Rust!!** If you want the old version that's build with python, please install via pip.

## Why should I use it?

I simply guess many of us know this situation, when one needs to unzip or transfer huge amounts of data into different directories.
This normally ends with about 10 open terminals/tmux sessions and an overchallenged hard drive.

Pueue is specifically designed for these situations. It executes long running tasks in their respective directories, without being bound to any terminal.  

*A few possible applications:*
- Copying huge amounts of stuff
- Compression tasks
- Movie encoding
- `rsync` tasks

Give it a try, If I got your attention.
Pueue made at least my life a lot easier on many occasions.

If you like the project and feel like something is missing, feel free to create an issue suggesting improvements.  
I'm always open to suggestions and already implemented a few user requested features.

PRs are of course always welcome!

## Installation:

There are three different ways to install Pueue.

1. Use an Arch Linux AUR package manager e.g. yay: `yay -S pueue` . This will deploy the service file and completions automatically.
2. Install by using cargo: `cargo install pueue`.
3. Clone the repository and execute `cargo install --path .`.

## How to use it:

For normal operation it's recommended to add an alias to your shell rc for `pueue add --`, e.g. `alias pad=pueue add --`

To add a command just write: `pueue add -- ls -al`  
The command will then be added and executed as if you executed it right now and then.

To get the status of currently running commands, just type `pueue status`.

There is a help option (-h) for all commands.
```
Pueue client 0.1.0
Arne Beer <contact@arne.beer>
Interact with the Pueue daemon

USAGE:
    pueue [FLAGS] [OPTIONS] [config] <SUBCOMMAND>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information
    -v, --verbose    Verbose mode (-v, -vv, -vvv)

OPTIONS:
    -a, --address <address>    The url for the daemon. Overwrites the address in the config file
    -p, --port <port>          The port for the daemon. Overwrites the port in the config file

ARGS:
    <config>    Optional custom config path

SUBCOMMANDS:
    add         Enqueue a task for execution
    clean       Remove all finished tasks from the list (also clears logs).
    edit        Edit the command of a stashed or queued task.
    enqueue     Enqueue stashed tasks. They'll be handled normally afterwards.
    help        Prints this message or the help of the given subcommand(s)
    kill        Kill running tasks.
    log         Display the log output of finished tasks
    parallel    Set the amount of allowed parallel tasks
    pause       Pause the daemon and all running tasks. A paused daemon won't start any new tasks. Daemon and tasks
                can be continued with `start`
    remove      Remove a tasks from the list. Running or paused tasks need to be killed first.
    reset       Kill all running tasks, remove all tasks and reset max_id.
    restart     Enqueue tasks again.
    send        Send something to a task. Useful for sending confirmations ('y\n')
    show        Show the output of a currently running task This command allows following (like `tail -f`)
    start       Wake the daemon from its paused state. Also continues all paused tasks.
    stash       Stashed tasks won't be automatically started. Either `enqueue` them, to be normally handled or
                explicitely `start` them.
    status      Display the current status of all tasks
    switch      Switches the queue position of two commands. Only works on queued and stashed commands
```

## Configs

The configuration file of Pueue is located in `~/.config/pueue.yml`.

```
---
client:
  daemon_port: "6924"
  secret: "your_secret"
daemon:
  pueue_directory: /home/$USER/.local/share/pueue
  default_parallel_tasks: 1
  port: "6924"
  secret: "your_secret"
```
**Client**: 
- `daemon_port` The port the client tries to connect to.  
- `secret` The secret, that's used for authentication

**Daemon**: 
- `pueue_directory` The location Pueue uses for it's intermediate files and logs.
- `default_parallel_tasks` Determines how many tasks should be processed concurrently.  
- `port` The port the daemon should listen to.  
- `secret` The secret, that's used for authentication


## Logs 

All logs can be found in `${pueue_directory}/logs`.
Logs of previous Pueue sessions will be created whenever you issue a `reset` or `clean`.  
In case the daemon fails or something goes wrong, the daemon will print to `stdout`/`stderr`.
If the daemon crashes or something goes wrong, please set the debug level to `-vvvv` and create an issue with the log!

If you want to dig right into it, you can compile and run it yourself with a debug build.
This would help me a lot!


## Utilities

### Systemd
If you use Systemd and don't install Pueue with Yay, place `pueuedaemon.service` in `/etc/systemd/user/`.  
Afterwards every user can start/enable their own session with:  

        systemctl --user start pueuedaemon.service
        systemctl --user enable pueuedaemon.service

### Json Support

The Pueue client `status` and `log` commands support JSON output with the `-j` flag.
This can be used to easily incorporate it into window manager bars, such as i3bar.


Copyright &copy; 2019 Arne Beer ([@Nukesor](https://github.com/Nukesor))
