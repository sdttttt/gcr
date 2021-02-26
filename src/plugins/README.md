# Plug

This is where all of GRC's built-in plugins are available.
The lifecycle of plug-in execution occurs before and after GRC `Commit`.

Currently, GRC provides very simple lifecycle hook functions.

Let's take a look at these plugins:

## Push

Maybe you're tired of pushing after each commit. Luck ~ **push** plug-in can take care of that.

This action occurs **after** the GRC completes the **commit**.

Push's job is **simple**.

- First, reads the name of the current branch.
- Then find the remote link (example: `origin`) corresponding to this branch.
- Read `$HOME/.ssh/id_rsa` from your user folder.
- finally push to the remote branch.

There are two things to note about this process:

- You need ssh-key. In other words, Push requires you to use SSH as validation. (Do not use password authentication)
- The whole push needs to be `fast-forward`.

> Because `push plug-in` has no way to deal with complex situations. If there is a **branch conflict** or **inconsistent commit record**, you need to resolve it yourself.

In the end, all you need is to **enable** it:

```toml
# grc.toml
plug = ["push"]
```


The output after using the `push` plug-in will look like this:

```
$ grc -a .
✔ Which scope? (Optional) · module
✔ Commit Message ? · update view of part module selector.
✔ Provide a longer description? (Optional) ·
✔ PR & Issues this commit closes, e.g 123: (Optional) ·
feat(module): 🎉 update part view module selector.
[-] running push ...
Remote: origin
Branch: sdttttt
[~] push is end.

$
```

Well, it looks clean. I like the feeling. :)

# Contribution

plug-in access is a difficult task.
How to have a good design? highly flexible interface and strong expansibility all need to be carefully considered.
If you have any good ideas, you are welcome to make issue or PR.

