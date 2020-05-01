# Pueue

[![GitHub Actions Workflow](https://github.com/nukesor/pueue/workflows/Test%20build/badge.svg)](https://github.com/Nukesor/comfy-table/actions)
[![Crates.io](https://img.shields.io/crates/v/pueue)](https://crates.io/crates/pueue)
[![MIT Licence](https://img.shields.io/pypi/l/pueue.svg)](https://github.com/Nukesor/pueue/blob/master/LICENSE)
[![dependency status](https://deps.rs/repo/github/nukesor/pueue/status.svg)](https://deps.rs/repo/github/nukesor/pueue)
[![Patreon](https://github.com/Nukesor/images/blob/master/patreon-donate-blue.svg)](https://www.patreon.com/nukesor)
[![Paypal](https://github.com/Nukesor/images/blob/master/paypal-donate-blue.svg)](https://www.paypal.me/arnebeer/)


![Pueue](https://raw.githubusercontent.com/Nukesor/images/master/pueue.gif)

Pueue is a command-line task management tool for sequential and parallel execution of long-running tasks.  

Simply put, it's a tool that processes a queue of shell commands.
On top of that, there are a lot of convenience features and abstractions.


Since Pueue is not bound to any terminal, you can control your tasks from any terminal on the same machine.
The queue will be continuously processed, even if you no longer have any active ssh session.

It provides functionality for:
- Scheduling commands that will be executed in their respective working directories
- Easy output inspection.
- Interaction with running processes
- Pausing/Resuming tasks, when you need some processing power right NOW!
- Manipulation of the scheduled task order
- Running multiple tasks at once (You can decide how many concurrent tasks you want to run)
- Works on Linux and MacOS and partially on Windows.

**Disclaimer:** Windows support is highly experimental right now. This means:
- Pueue uses `powershell` to execute commands, keep this in mind when writing commands.
- Starting/resuming commands doesn't work for now.

**Pueue has been rewritten in Rust!!** If you want the old version that's build with python, please install via pip.

## Why should I use it?

Imagine having to unpack or transfer large amounts of data from different directories to other directories.
Usually something like this ends with about 10 open terminals/tmux sessions and an over-challenged hard drive.

Pueue is specifically designed for these situations. It executes long-running tasks in their respective directories, without being bound to any terminal.  

**A few possible applications:**
- Copying huge amounts of stuff
- Compression tasks
- Movie encoding
- `rsync` tasks
- Anything that takes longer than 5 minutes

Pueue made at least my life a lot easier on many occasions.

If you like the project and feel like something is missing, please create an issue.  
I'm always open to suggestions and already implemented a few users requested features.

PRs are of course very welcome!

## Installation:

There are three different ways to install Pueue.

**Package Manager**  
Use your system's package manager.  
This will usually deploy service files and completions automatically.  
Pueue has been packaged for:

- Arch Linux's AUR: e.g. `yay -S pueue`.  
- NixOS
- Homebrew

**Via Cargo**  
You'll need Rust version `>=1.39`
```
cargo install pueue
```
This will install pueue to `~/.cargo/bin/pueue`

**From Source**  
You'll need Rust version `>=1.39`
```
git clone git@github.com:Nukesor/pueue
cd pueue
cargo install --path .
```
This will install pueue to `~/.cargo/bin/pueue`

## Starting the Daemon

### Local
Just run `pueued` anywhere on your commandline. It'll exit if you close the terminal, though.

### Background
To fork `pueued` into the background, add the `-d` or `--daemonize` flag. E.g. `pueued -d`. \
The daemon can be then shut down using the client: `pueue shutdown`

### Systemd
If you use Systemd and don't install Pueue with a package manager, place `pueued.service` in `/etc/systemd/user/`.  
Afterward, every user can start/enable their own session with:  

        systemctl --user start pueued.service
        systemctl --user enable pueued.service


## How to use it:

**Adding Commands:**

To add a command just write: `pueue add sleep 60`\
If you want to add flags to the command, you can either:
- add `--` => `pueue add -- ls -al`
- surround the command with a string `pueue add 'ls -al'`

The command will then be added and scheduled for execution, as if you executed it right now and then.

For normal operation it's recommended to add an alias to your shell's rc.\
E.g.: `alias pad='pueue add --'`

Surrounding a command with quotes is also required, if your command contains escaped characters.\
For instance `pueue add ls /tmp/long\ path` will result in the execution of `sh -c ls /tmp/long path`, which will then break, as the escaped space is not passed to Pueue.

**See what's going on:**

To get the status of currently running commands, just type `pueue status`.

To look at the output of a finished command use `pueue log` or `pueue log $task_id`.

If you want to see output of a running command use `git show $task_id`.
You can also use the `-f` and `-e` flag to get a live view of the output.


**Pitfalls:**

To avoid common pitfalls, please read the [FAQ Section](https://github.com/Nukesor/pueue/blob/master/FAQ.md).

There is a help option (-h) for all commands.
```

Pueue client 0.3.0
Arne Beer <contact@arne.beer>
Interact with the Pueue daemon

USAGE:
    pueue [FLAGS] [OPTIONS] <SUBCOMMAND>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information
    -v, --verbose    Verbose mode (-v, -vv, -vvv)

OPTIONS:
    -p, --port <port>    The port for the daemon. Overwrites the port in the config file

SUBCOMMANDS:
    add            Enqueue a task for execution
    clean          Remove all finished tasks from the list (also clears logs)
    completions    Generates shell completion files. Ingore for normal operations
    edit           Edit the command or the path of a stashed or queued task
    enqueue        Enqueue stashed tasks. They'll be handled normally afterwards
    help           Prints this message or the help of the given subcommand(s)
    kill           Kill either all or only specific running tasks
    log            Display the log output of finished tasks
    parallel       Set the amount of allowed parallel tasks
    pause          Pause the daemon and all running tasks. A paused daemon won't start any new tasks. Daemon and
                   tasks can be continued with `start` Can also be used to pause specific tasks
    remove         Remove tasks from the list. Running or paused tasks need to be killed first
    reset          Kill all running tasks, remove all tasks and reset max_id
    restart        Enqueue tasks again
    send           Send something to a task. Useful for sending confirmations ('y\n')
    show           Show the output of a currently running task This command allows following (like `tail -f`)
    shutdown       Remotely shut down the daemon. Should only be used if the daemon isn't started by a service
                   manager
    start          Wake the daemon from its paused state and continue all paused tasks. Can be used to resume or
                   start specific tasks
    stash          Stashed tasks won't be automatically started. Either `enqueue` them, to be normally handled or
                   explicitly `start` them
    status         Display the current status of all tasks
    switch         Switches the queue position of two commands. Only works on queued and stashed commands
```

## Configs

The configuration file of Pueue is located in `~/.config/pueue.yml`.  
The default will be generated after starting pueue once.

```
---
client:
  daemon_port: "6924"
  secret: "your_secret"
  read_local_logs: true

daemon:
  pueue_directory: /home/$USER/.local/share/pueue
  default_parallel_tasks: 1
  pause_on_failure: false
  port: "6924"
  secret: "your_secret"
```
**Client**: 
- `daemon_port` The port the client tries to connect to.  
- `secret` The secret, that's used for authentication
- `read_local_logs` If the client runs as the same user (and on the same machine) as the daemon, logs don't have to be sent via the socket but rather read directly.

**Daemon**: 
- `pueue_directory` The location Pueue uses for its intermediate files and logs.
- `default_parallel_tasks` Determines how many tasks should be processed concurrently.  
- `pause_on_failure` If set to `true`, the daemon no longer start new task as soon as a single task fails. Already running tasks will continue.
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

### Shell completion files
Shell completion files can be created on the fly with `pueue completions $shell $directory`.
There's also a `build_completions.sh` script, which creates all completion files in the `utiles/completions` directory.

### JSON Support

The Pueue client `status` and `log` commands support JSON output with the `-j` flag.
This can be used to easily incorporate it into window manager bars, such as i3bar.

## Scripting

When calling pueue commands in a script, you might need to sleep for a short amount of time for now.
The pueue server processes requests asynchronously, whilst the TaskManager runs it's own update loop with a small sleep. 
(The TaskManager handles everything related to starting, stopping and communicating with processes.)

A sleep in scripts will probably become irrelevant, as soon as this bug in rust-lang is fixed: https://github.com/rust-lang/rust/issues/39364


Copyright &copy; 2019 Arne Beer ([@Nukesor](https://github.com/Nukesor))
