```fish
function __do_completion
    set cmd (commandline -j)
    set cmd_arr (string split ' ' $cmd)
    if [ -z "$cmd_arr[-1]" ]
        # preserve the last white space
        eval "my-comp $cmd ''"
    else
        eval my-comp $cmd
    end
end

complete -k -c my-app -x -a "(__do_completion)"
```
