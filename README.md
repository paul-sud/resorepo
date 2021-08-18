# resorepo

REcursively Search On remote REPO

If you're like me, you often need to search through other people's code on GitHub, but only need to do it a couple of times. Usually this involves cloning, searching for what I need with [`ripgrep`](https://github.com/BurntSushi/ripgrep/), and deleting the directory when I'm done. This utility allows you to use the full `ripgrep` (aka `rg`) functionality on any remote repository, and also caches the repositories so cloning is not required every time.

## Installation

First ensure Rust and `git` are installed. Then clone this repo, `cd` into it and run `cargo build --release`. If desired, you can then add it to your `PATH`, for instance by copying the binary `./target/release/resorepo` to `/usr/local/bin`

Usage requires `rg` installed and on your path as it is executed as a subprocess.

## Usage

```bash
resorepo <repo-url> [rg-args]...
```

You must specify the remote repository as either a full URL or in the `owner/repository` shorthand form for GitHub repositories. The `rg-args` are passed to `ripgrep`, see its [documentation](https://github.com/BurntSushi/ripgrep) for details or consult the `rg` help with `rg -h`/`rg --help`.

### Examples

Find all occurrences of `foo` in the remote repository https://github.com/paul-sud/s3-md5

```bash
resorepo https://github.com/paul-sud/s3-md5 foo
```

Same as above but using short form for GitHub repos:

```bash
resorepo paul-sud/s3-md5 foo
```

Same as above, except display 10 lines of context after the match. Note the `--` in order to indicate that `-A` should be treated as a positional argument and not a flag for `resorepo`.

```bash
resorepo paul-sud/s3-md5 foo -- -A 10
```

To use regular expressions starting with dashes, you can supply the `--` separator twice:

```bash
resorepo paul-sud/s3-md5 -- -- "-md5"
```

## Configuration

The time-to-live (TTL) for the cached repositories can be configured via setting the value of `cache_ttl_days` in the config file `~/.resorepo/resorepo_config.yaml`. Once it is implemented, after searching `resosepo` will automatically clean up any repositories older than the expiration date.

## Future Ideas/Features

* When clone capabilities are available in `gitoxide` repo cloning should be updated to use it. Currently using `git2`, `gitoxide` seems faster and also doesn't depend on `libgit2`
* Use the `rg` API instead of invoking as a subprocess. This is difficult because useful functions like [`search_parallel`](https://github.com/BurntSushi/ripgrep/blob/9b01a8f9ae53ebcd05c27ec21843758c2c1e823f/crates/core/main.rs#L127) are not part of `ripgrep`'s public API, and there would be a lot of copy-pasting code in here. Parsing the command line arguments like `ripgrep` would be similarly non-trivial.
* Choose a branch or tag
* Organize repos by organization to avoid conflicts between repos with the same name in the cache
* Clone over SSH (git2-rs defaults to HTTPS)
* Pull in new changes if the cached repo is still around: https://stackoverflow.com/questions/58768910/how-to-perform-git-pull-with-the-rust-git2-crate
* A web interface or Chrome extension could be pretty cool so you don't have to copy-paste URLs from browser to the terminal
