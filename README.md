# Rickyisms

A searchable, single-page version of the [Trailer Park Boys Wiki's Rickyisms list](https://trailerpark.fandom.com/wiki/Rickyisms): Ricky's malapropisms and eggcorns.

The data is a cleaned snapshot of the wiki table, embedded in `index.html`. Duplicates were merged and vandalised entries removed. There is no build step; GitHub Pages serves the file as-is. `og.png` is the social share image, rendered from the page's own styles.

Content is from the Trailer Park Boys Wiki under [CC BY-SA 3.0](https://creativecommons.org/licenses/by-sa/3.0/).

## Rickyism of the day

`motd/` is a small CLI that prints one Rickyism per day, for a shell greeting or MOTD. It compiles `index.html` into the binary and reads the same `DATA` array, so the page stays the single source of the list. The pick is the day number since the Unix epoch (UTC) modulo the list length, so it changes once a day and is the same on every machine; `--random` picks one at random instead. Colour is used only on a terminal and is disabled by `NO_COLOR`.

```sh
cargo install --path motd
echo rickyism >> ~/.zshrc
```
