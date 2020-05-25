# bb

This is bibucket cli tool to get opened Bibucket PRs.

## Install

```sh
$ git clone https://github.com/t-yng/bb.git
$ cd bb
$ ./install.sh
```

## Usage

### config
First you must setup user_name, password, Bibucket workspace.  
Workspace is name space where your repository is placed in.

```sh
$ bb config
user_name: xxxx
password: xxxx
workspace: xxxx
```

### get pr list

```sh
$ bb pr list
```

## Advanced Usage

You write alias bellow into your shell config file like `~/.zshrc`

```
alias gchpr='bb pr list | peco | xargs git checkout'
```

You can git checkout branch opened pr in one command.  

```
$ source ~/.zshrc
$ gchpr
```
