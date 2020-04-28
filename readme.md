# format-shell-command

I liked [this script][0] that @bbkane had [posted on Reddit](https://www.reddit.com/r/vim/comments/g1lx7e/i_made_a_command_to_autoformat_shell_commands/), especially in conjunction with the suggestion to make it into a [nvim command](https://github.com/bbkane/dotfiles/blob/3e25d662b2dbb84e3056987c1027fa87440abd7b/nvim/.config/nvim/init.vim#L397).

I [forked it][1] to make a minor style change: unix pipes `|` can actually function as line continuations. Instead of

```console
echo foo \
    | awk '{ print $1}'
```

I much prefer

```console
echo foo |
    awk '{ print $1}'
```

Anyway, I think that responsiveness is pretty important when writing code, and the experience with this kind of code formatting may really benefit from shaving off a few milliseconds. It seemed like a good excuse for me to keep practicing my rust, even more so since much of the python code looked like it was *begging* to be made into an enum or a match, and I thought that the python generator (something I *really* miss in rust) would probaby be not terribly difficult to rewrite as an iterator.

My very naive rewrite took a few hours -- possibly longer than it would have taken me to write the python from scratch -- but its relative performance is nothing to shrug off.

I strongly suspect that opportunities for further optimizations (e.g. decreasing `String` allocations, implement `FromStr` to parse `Expr`, get rid of `panic` / proper error handling, implement the tests) would be obvious for a better rust programmer, but that's part of the point -- this was simply the most obvious way that I could get the code to compile, and it is 30 times faster.

```console
$ hyperfine --warmup=5 $'echo "echo -n \'hi there\' | awk \'{ print $1 }\' | sed -n \'s/i/aha/p\'" | ~/gists/9acab6b3bd1478343296d
9fd02e08bc4/format_shell_cmd.py'
Benchmark #1: echo "echo -n 'hi there' | awk '{ print $1 }' | sed -n 's/i/aha/p'" | ~/gists/9acab6b3bd1478343296d9fd02e08bc4/format_shell_cmd.py
  Time (mean ± σ):     163.3 ms ±   7.4 ms    [User: 71.7 ms, System: 65.8 ms]
  Range (min … max):   153.8 ms … 182.2 ms    18 runs
 
$ hyperfine --warmup=5 $'echo "echo -n \'hi there\' | awk \'{ print $1 }\' | sed -n \'s/i/aha/p\'" | target/release/format-shell-command'
Benchmark #1: echo "echo -n 'hi there' | awk '{ print $1 }' | sed -n 's/i/aha/p'" | target/release/format-shell-command  
  Time (mean ± σ):       4.8 ms ±   3.8 ms    [User: 1.6 ms, System: 2.2 ms]
  Range (min … max):     1.6 ms …  24.4 ms    145 runs
 
  Warning: Command took less than 5 ms to complete. Results might be inaccurate.
  Warning: Statistical outliers were detected. Consider re-running this benchmark on a quiet PC without any interferences from other programs. It might help to use the '--warmup' 
or '--prepare' options.
```

[0]: https://github.com/bbkane/dotfiles/blob/master/bin_common/bin_common/format_shell_cmd.py
[1]: https://gist.github.com/n8henrie/9acab6b3bd1478343296d9fd02e08bc4
