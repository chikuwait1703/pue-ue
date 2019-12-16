complete -c pueue -n "__fish_use_subcommand" -s a -l address -d 'The url for the daemon. Overwrites the address in the config file'
complete -c pueue -n "__fish_use_subcommand" -s p -l port -d 'The port for the daemon. Overwrites the port in the config file'
complete -c pueue -n "__fish_use_subcommand" -s v -l verbose -d 'Verbose mode (-v, -vv, -vvv)'
complete -c pueue -n "__fish_use_subcommand" -s h -l help -d 'Prints help information'
complete -c pueue -n "__fish_use_subcommand" -s V -l version -d 'Prints version information'
complete -c pueue -n "__fish_use_subcommand" -f -a "add" -d 'Enqueue a task for execution'
complete -c pueue -n "__fish_use_subcommand" -f -a "remove" -d 'Remove a tasks from the list. Running or paused tasks need to be killed first.'
complete -c pueue -n "__fish_use_subcommand" -f -a "switch" -d 'Switches the queue position of two commands. Only works on queued and stashed commands'
complete -c pueue -n "__fish_use_subcommand" -f -a "stash" -d 'Stashed tasks won\'t be automatically started. Either `enqueue` them, to be normally handled or explicitely `start` them.'
complete -c pueue -n "__fish_use_subcommand" -f -a "enqueue" -d 'Enqueue stashed tasks. They\'ll be handled normally afterwards.'
complete -c pueue -n "__fish_use_subcommand" -f -a "start" -d 'Wake the daemon from its paused state. Also continues all paused tasks.'
complete -c pueue -n "__fish_use_subcommand" -f -a "restart" -d 'Enqueue tasks again.'
complete -c pueue -n "__fish_use_subcommand" -f -a "pause" -d 'Pause the daemon and all running tasks. A paused daemon won\'t start any new tasks. Daemon and tasks can be continued with `start`'
complete -c pueue -n "__fish_use_subcommand" -f -a "kill" -d 'Kill running tasks.'
complete -c pueue -n "__fish_use_subcommand" -f -a "send" -d 'Send something to a task. Useful for sending confirmations (\'y\\n\')'
complete -c pueue -n "__fish_use_subcommand" -f -a "edit" -d 'Edit the command of a stashed or queued task.'
complete -c pueue -n "__fish_use_subcommand" -f -a "status" -d 'Display the current status of all tasks'
complete -c pueue -n "__fish_use_subcommand" -f -a "log" -d 'Display the log output of finished tasks'
complete -c pueue -n "__fish_use_subcommand" -f -a "show" -d 'Show the output of a currently running task This command allows following (like `tail -f`)'
complete -c pueue -n "__fish_use_subcommand" -f -a "reset" -d 'Kill all running tasks, remove all tasks and reset max_id.'
complete -c pueue -n "__fish_use_subcommand" -f -a "clean" -d 'Remove all finished tasks from the list (also clears logs).'
complete -c pueue -n "__fish_use_subcommand" -f -a "parallel" -d 'Set the amount of allowed parallel tasks'
complete -c pueue -n "__fish_use_subcommand" -f -a "help" -d 'Prints this message or the help of the given subcommand(s)'
complete -c pueue -n "__fish_seen_subcommand_from add" -s i -l immediate -d 'Start the task immediately'
complete -c pueue -n "__fish_seen_subcommand_from add" -s h -l help -d 'Prints help information'
complete -c pueue -n "__fish_seen_subcommand_from add" -s V -l version -d 'Prints version information'
complete -c pueue -n "__fish_seen_subcommand_from remove" -s h -l help -d 'Prints help information'
complete -c pueue -n "__fish_seen_subcommand_from remove" -s V -l version -d 'Prints version information'
complete -c pueue -n "__fish_seen_subcommand_from switch" -s h -l help -d 'Prints help information'
complete -c pueue -n "__fish_seen_subcommand_from switch" -s V -l version -d 'Prints version information'
complete -c pueue -n "__fish_seen_subcommand_from stash" -s h -l help -d 'Prints help information'
complete -c pueue -n "__fish_seen_subcommand_from stash" -s V -l version -d 'Prints version information'
complete -c pueue -n "__fish_seen_subcommand_from enqueue" -s h -l help -d 'Prints help information'
complete -c pueue -n "__fish_seen_subcommand_from enqueue" -s V -l version -d 'Prints version information'
complete -c pueue -n "__fish_seen_subcommand_from start" -s t -l task-ids -d 'Enforce starting these tasks. This doesn\'t affect the daemon or any other tasks and works on a paused deamon.'
complete -c pueue -n "__fish_seen_subcommand_from start" -s h -l help -d 'Prints help information'
complete -c pueue -n "__fish_seen_subcommand_from start" -s V -l version -d 'Prints version information'
complete -c pueue -n "__fish_seen_subcommand_from restart" -s i -l immediate -d 'Start the task(s) immediately'
complete -c pueue -n "__fish_seen_subcommand_from restart" -s h -l help -d 'Prints help information'
complete -c pueue -n "__fish_seen_subcommand_from restart" -s V -l version -d 'Prints version information'
complete -c pueue -n "__fish_seen_subcommand_from pause" -s t -l task-ids -d 'Enforce starting these tasks. Doesn\'t affect the daemon or any other tasks.'
complete -c pueue -n "__fish_seen_subcommand_from pause" -s w -l wait -d 'Pause the daemon, but let any running tasks finish by themselves.'
complete -c pueue -n "__fish_seen_subcommand_from pause" -s h -l help -d 'Prints help information'
complete -c pueue -n "__fish_seen_subcommand_from pause" -s V -l version -d 'Prints version information'
complete -c pueue -n "__fish_seen_subcommand_from kill" -s a -l all -d 'Kill all running tasks, this also pauses the daemon.'
complete -c pueue -n "__fish_seen_subcommand_from kill" -s h -l help -d 'Prints help information'
complete -c pueue -n "__fish_seen_subcommand_from kill" -s V -l version -d 'Prints version information'
complete -c pueue -n "__fish_seen_subcommand_from send" -s h -l help -d 'Prints help information'
complete -c pueue -n "__fish_seen_subcommand_from send" -s V -l version -d 'Prints version information'
complete -c pueue -n "__fish_seen_subcommand_from edit" -s h -l help -d 'Prints help information'
complete -c pueue -n "__fish_seen_subcommand_from edit" -s V -l version -d 'Prints version information'
complete -c pueue -n "__fish_seen_subcommand_from status" -s j -l json -d 'Print the current state as json to stdout This doesn\'t include stdout/stderr of tasks. Use `log -j` if you want everything'
complete -c pueue -n "__fish_seen_subcommand_from status" -s h -l help -d 'Prints help information'
complete -c pueue -n "__fish_seen_subcommand_from status" -s V -l version -d 'Prints version information'
complete -c pueue -n "__fish_seen_subcommand_from log" -s t -l task-ids -d 'Specify for which specific tasks you want to see the output'
complete -c pueue -n "__fish_seen_subcommand_from log" -s j -l json -d 'Print the current state as json Includes EVERYTHING'
complete -c pueue -n "__fish_seen_subcommand_from log" -s h -l help -d 'Prints help information'
complete -c pueue -n "__fish_seen_subcommand_from log" -s V -l version -d 'Prints version information'
complete -c pueue -n "__fish_seen_subcommand_from show" -s f -l follow -d 'Continuously print stdout (like `tail -f`)'
complete -c pueue -n "__fish_seen_subcommand_from show" -s e -l err -d 'Like -f, but shows stderr instead of stdeout.'
complete -c pueue -n "__fish_seen_subcommand_from show" -s h -l help -d 'Prints help information'
complete -c pueue -n "__fish_seen_subcommand_from show" -s V -l version -d 'Prints version information'
complete -c pueue -n "__fish_seen_subcommand_from reset" -s h -l help -d 'Prints help information'
complete -c pueue -n "__fish_seen_subcommand_from reset" -s V -l version -d 'Prints version information'
complete -c pueue -n "__fish_seen_subcommand_from clean" -s h -l help -d 'Prints help information'
complete -c pueue -n "__fish_seen_subcommand_from clean" -s V -l version -d 'Prints version information'
complete -c pueue -n "__fish_seen_subcommand_from parallel" -s h -l help -d 'Prints help information'
complete -c pueue -n "__fish_seen_subcommand_from parallel" -s V -l version -d 'Prints version information'
complete -c pueue -n "__fish_seen_subcommand_from help" -s h -l help -d 'Prints help information'
complete -c pueue -n "__fish_seen_subcommand_from help" -s V -l version -d 'Prints version information'
