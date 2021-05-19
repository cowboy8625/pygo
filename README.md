<p align="center">
  <br>
  PyGo
  <br>
  <a href="https://crates.io/crates/pygo"><img alt="crates.io" src="https://img.shields.io/crates/v/pygo.svg"></a>
  <a><img alt="lastupdated" src="https://img.shields.io/github/last-commit/cowboy8625/pygo"></a>
  <a><img alt="GitHub repo size" src="https://img.shields.io/github/repo-size/cowboy8625/pygo"></a>
  <a><img alt="issuse" src="https://img.shields.io/github/issues/cowboy8625/pygo"></a>
  <a><img alt="Lines of Code" src="https://img.shields.io/tokei/lines/github/cowboy8625/pygo"></a>
  <a><img alt="License" src="https://img.shields.io/badge/License-MIT-blue.svg"></a>
  <a href="https://discord.gg/KwnGX8P"><img alt="Discord Chat" src="https://img.shields.io/discord/509849754155614230"></a>
</p>

PyGo is a Cargo like manager for Python


`pygo new myproject`

Creates a file structure like

```
myproject
|
- src
| |
| - __main__.py
|
- .git
|
- .gitignore
|
- README.md
```

The `__main__.py` file will have a template like
```py
def main():
  print("Hello World")

if __name__ == "__main__":
  main()
```

`--bin` is a default argument

`--lib` will create a file structure like

`pygo new mylib --lib`

```
myproject
|
- src
| |
| - __init__.py
|
- .git
|
- .gitignore
|
- README.md
```


# RoadMap

  - More Commands Options
    1. `init` creates a project that has no `git` repo.
    2. `env` creates a virtual environment for your project.
    3. `add` command to install library's to your vm.
  - installable through `pip` or from package manager


