# PyGo

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
```
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
