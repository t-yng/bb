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
