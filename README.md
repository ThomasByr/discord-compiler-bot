# <img src="assets/code.png" alt="icon" width="3%"/> Discord compiler bot

````txt
;compile c++ -O3 -Wall -Wextra -Werror
argv1 argv2 argv3
```
stdin1
stdin2 not displayed
```
```cpp
#include <iostream>
#include <string>

int main(int argc, char** argv) {
    (void)argc;
    std::string str;
    std::getline(std::cin, str);
    std::cout << str << std::endl;
    std::cout << argv[1] << std::endl;
    return 0;
}
```
````

1. [In short](#in-short)
2. [Usage](#usage)
3. [Get Help](#get-help)
4. [Support](#support)
5. [Changelog, Bugs and TODO](#changelog-bugs-and-todo)

## In short

First, this project was done in a week so do not expect crazy behavior and be immune to bugs.

This is a Discord compiler bot which can compile / interpret code blocks and display the result. Keep in mind that we're working in discord. This means, of course, that we have many operating restraints. Here's a few of the big ones.

- Complicated syntax for non-trivial compilations
- Limited output length (limited to a couple hundred characters)
- Single-file input only

Please view [the Wiki](https://github.com/ThomasByr/discord-compiler-bot/wiki) for more.

## Usage

First you should install the [Rust programming language](https://www.rust-lang.org/learn/get-started) for your operating system. Then, you should create a new `.env` file on the model of the `.env.example`. You should at least set `BOT_TOKEN` and `APPLICATION_ID` (you can let other fields untouched) environnement variables.

```ps1
cargo run --release
```

The bot should then connect to the servers and if succesfull, show a list of debug commands on the go.

## Get Help

While in discord, please type `;help` to get generic help and show a list of available commands, or type `;help <cmd>` to get help about a specific command. Here is a list of the most usefull ones.

- `;format` on a code block with the language set will trigger a format command for the language
- `;compile` will compile and run a code block with the given language
- `;languages` will show a menu to display all languages
- `;asm` will transform the code block into Assembly _x86_64 for linux

If you want to contribute, please read the [contributing](.github/CONTRIBUTING.md) guideline, make pull requests and be kind.

## Support

Support for following languages : c++, c, java, python, ruby, rust, javascript, go, php, lua and many more.

> [Create a new issue](https://github.com/ThomasByr/discord-compiler-bot/issues/new)

## Changelog, Bugs and TODO

<details><summary>**v0.1.1** first release (fix)</summary>

- support for 30 more languages
- asm output
- embedded messages for discord
- `;invite`, `;botinfo` commands
- icons are now found online
- locked serenity dependency version to v0.11.1
- client now has privileged access to context

</details>

**TODO** (first implementation version)

- [x] connect to Azure (v0.1.0)
- [x] `;invite` (v0.1.0)
- [ ] should make slash commands (but I'm lazy)

**Bugs** (latest fix)

- ~~`;botinfo` command not working~~ (v0.1.0)
- ~~total number of servers joined always showing 0~~ (v0.1.1)
- ~~debug commands not showing up in console (might be linked to previous bug)~~ (v0.1.1)
