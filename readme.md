# Dolly

Inspired by [gitclone](https://github.com/khoberg/gitclone).

I didn't like the dependence on npm so this is a re-write in rust. 

# Installation
#### dependencies
- ecdsa ssh key located at "$HOME/.ssh/id_ecdsa"
- rust (if you want to compile)


## download 
https://github.com/Kadinvanvalin/dolly/releases/tag/v0.1.0-alpha

## build it yourself
```bash
git clone git@github.com:Kadinvanvalin/dolly.git && cd dolly
```
### compile a release binary
```bash
cargo b -r
```
move dolly to your bin
```bash 
mv target/release/dolly /usr/local/bin
```


## Behavior

Dolly will try to clone your repo to your home directory with directories for the domain and slug of the project
```bash
dolly git@github.com:Kadinvanvalin/dolly.git
```
will clone dolly into 

```bash
$HOME/github.com/kadinvanvalin/dolly
```