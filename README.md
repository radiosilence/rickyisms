# Rickyisms

A searchable, single-page version of the [Trailer Park Boys Wiki's Rickyisms list](https://trailerpark.fandom.com/wiki/Rickyisms): Ricky's malapropisms and eggcorns.

The data is a cleaned snapshot of the wiki table in `rickyisms.json`, which the page fetches and the CLI compiles in. Duplicates were merged and vandalised entries removed. There is no build step; GitHub Pages serves the files as-is, so opening `index.html` straight from disk will not load the list (serve the directory instead). `og.png` is the social share image, rendered from the page's own styles.

Content is from the Trailer Park Boys Wiki under [CC BY-SA 3.0](https://creativecommons.org/licenses/by-sa/3.0/).

## Rickyism of the day

`motd/` is a small CLI that prints one Rickyism per day, for a shell greeting or MOTD. It compiles `rickyisms.json` into the binary, so it needs no network or data files at runtime. The pick is the day number since the Unix epoch (UTC) modulo the list length, so it changes once a day and is the same on every machine; `--random` picks one at random instead. `-C/--no-correct` and `-E/--no-episode` hide what he meant and the episode. Colour is used only on a terminal and is disabled by `NO_COLOR`.

```sh
cargo install --path motd
echo rickyism >> ~/.zshrc
rickyism --completions zsh > "${fpath[1]}/_rickyism"
```

`--completions` also takes `bash`, `fish`, `elvish` and `powershell`.
