# ttt

_Timeless Time Tracking_

## Tracking

```bash
$ ttt <project> [task] <action> [notes...]
```

Project can be anything, action is one of: `start`, `end`,
`estimate=<duration>`.  `start` and `end` are obvious, with the caveat that
"ending" may be any of "pausing with the intent of starting later", "finished",
"done for the day", etc. `estimate=<duration>` is special and indicates that
you're estimating the amount of work to be done on the `task` (or today, if the
`task` is not specified) to be `<duration>`. That duration should be expressed
in short form, e.g. `3h` or `30m` or `3/4h` or `1h15m` etc.

The format for `task` is free-form, but the convention is that subtasks are
delimited with a `.`.  I.e. `task.subtask` or `task.subtask.item`.

Anything else passed to the command is considered part of the free-form notes.

## What it does

All that command does is record (either locally using files or remotely to a
server as configured) all the information you've given it and the exact time
you've run it.

You can't record things in the past, nor can you in the future. The command
doesn't even care if you've already got a task "running" or if you're going
over your estimate, or if you've already set an estimate. It just records a
time and a payload of data.

## And then what?

The format is uber simple, so you can build your own reporting tools to make
this useful. For now this all falls on you, but as soon as we've got time we'll
add some demos and standard tools here.

## Why Timeless?

Most time tracking systems show a timer running up, or allow you to enter
times. This doesn't. It records the time itself and never shows it to you
immediately. I've found this increases productivity because you stop caring
about the time you spend and instead focus on the work. When you review your
day or week, *then* you can see how you've done, but not before.
