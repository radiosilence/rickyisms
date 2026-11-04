# Rickyisms

A searchable, single-page version of the [Trailer Park Boys Wiki's Rickyisms list](https://trailerpark.fandom.com/wiki/Rickyisms): Ricky's malapropisms and eggcorns.

The data is a cleaned snapshot of the wiki table in `rickyisms.json`, which the page fetches and the CLI compiles in. Duplicates were merged and vandalised entries removed. There is no build step; GitHub Pages serves the files as-is, so opening `index.html` straight from disk will not load the list (serve the directory instead). `og.png` is the social share image, rendered from the page's own styles.

Content is from the Trailer Park Boys Wiki under [CC BY-SA 3.0](https://creativecommons.org/licenses/by-sa/3.0/).

## Rickyism of the day

`motd/` is a small CLI that prints one Rickyism per day, for a shell greeting or MOTD. It compiles `rickyisms.json` into the binary, so it needs no network or data files at runtime. The pick is the day number since the Unix epoch (UTC) modulo the list length, so it changes once a day and is the same on every machine; `--random` picks one at random instead. `-C/--no-correct` and `-E/--no-episode` hide what he meant and the episode. Colour is used only on a terminal and is disabled by `NO_COLOR`.

```sh
brew install radiosilence/tap/rickyism
echo rickyism >> ~/.zshrc
```

The Homebrew formula installs bash, zsh and fish completions. From source, `cargo install --path motd`, then `rickyism --completions zsh > "${fpath[1]}/_rickyism"`; `--completions` also takes `elvish` and `powershell`.

### Releasing

Bump `version` in `motd/Cargo.toml` and merge. On main, CI releases any version that has no GitHub release yet: it builds static Linux binaries (x86_64, arm64) and a signed, notarised macOS arm64 binary, publishes them as `v<version>`, and rewrites `Formula/rickyism.rb` in the shared [homebrew-tap](https://github.com/radiosilence/homebrew-tap). A manual run of the workflow on main retries a failed release.

The workflow needs these repository secrets: `MACOS_CERTIFICATE_P12`, `MACOS_CERTIFICATE_PASSWORD`, `APPLE_API_KEY_P8`, `APPLE_API_KEY_ID` and `APPLE_API_ISSUER_ID` for signing (a release fails without them), and `RELEASE_TOKEN`, a token that can push to the tap (the tap update is skipped without it).
