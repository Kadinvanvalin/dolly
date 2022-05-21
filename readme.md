# Dolly

Inspired by [gitclone](https://github.com/khoberg/gitclone).

I didn't like the dependence on npm so this is a re-write in rust. 

# Installation
#### dependencies
- tested on macos
- git
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
cp target/release/dolly /usr/local/bin/dolly
```
move optional scripts to your bin
```bash 
cp  ./open_remote.sh /usr/local/bin/remote
```
```bash 
cp  ./clone_and_cd.sh /usr/local/bin/dolly_INNER_clone_and_go.sh
```
## Behavior

dolly is just a tiny string parser - you will need the helper scripts to actually do the cloning


### helper scripts

remote will try to open the remote of your repo (macos only) - it will throw an error if you are not in a repo

```bash
remote
```



```bash
alias clone_and_go=". dolly_INNER_clone_and_go.sh"
```
```bash
clone_and_go git@github.com:Kadinvanvalin/dolly.git
```