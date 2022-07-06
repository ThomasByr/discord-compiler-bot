# <img src="assets/code.png" alt="icon" width="3%"/> Discord compiler bot

[![Linux](https://svgshare.com/i/Zhy.svg)](https://docs.microsoft.com/en-us/windows/wsl/tutorials/gui-apps)
[![Windows](https://svgshare.com/i/ZhY.svg)](https://svgshare.com/i/ZhY.svg)
[![GitHub license](https://img.shields.io/github/license/ThomasByr/discord-compiler-bot)](https://github.com/ThomasByr/discord-compiler-bot/blob/master/LICENSE)
[![GitHub commits](https://badgen.net/github/commits/ThomasByr/discord-compiler-bot)](https://GitHub.com/ThomasByr/discord-compiler-bot/commit/)
[![GitHub latest commit](https://badgen.net/github/last-commit/ThomasByr/discord-compiler-bot)](https://gitHub.com/ThomasByr/discord-compiler-bot/commit/)
[![Maintenance](https://img.shields.io/badge/maintained%3F-yes-green.svg)](https://GitHub.com/ThomasByr/discord-compiler-bot/graphs/commit-activity)

[![Continuous Integration](https://github.com/ThomasByr/discord-compiler-bot/actions/workflows/main.yml/badge.svg)](https://github.com/ThomasByr/discord-compiler-bot/actions/workflows/main.yml)
[![rust-clippy analyze](https://github.com/ThomasByr/discord-compiler-bot/actions/workflows/rust-clippy.yml/badge.svg)](https://github.com/ThomasByr/discord-compiler-bot/actions/workflows/rust-clippy.yml)
[![Publish](https://github.com/ThomasByr/discord-compiler-bot/actions/workflows/publish.yml/badge.svg)](https://github.com/ThomasByr/discord-compiler-bot/actions/workflows/publish.yml)
[![GitHub version](https://badge.fury.io/gh/ThomasByr%2Fdiscord-compiler-bot.svg)](https://github.com/ThomasByr/discord-compiler-bot)
[![Author](https://img.shields.io/badge/author-@ThomasByr-blue)](https://github.com/ThomasByr)

````txt
;compile c++ -O3 -Wall -Wextra -Werror -Wpedantic
argv1 argv2 argv3
```
stdin1 on the first line
stdin2 on the second line
```
```cpp
#include <iostream>
#include <string>

int main(int argc, char** argv) {
    (void)argc;
    std::string str;
    std::getline(std::cin, str);
    std::cout << str << "\n"
              << argv[1] << std::endl;
    return EXIT_SUCCESS;
}
```
````

1. [✏️ In short](#️-in-short)
2. [👩‍🏫 Usage](#-usage)
3. [💁 Get Help](#-get-help)
4. [🔰 Support](#-support)
5. [⚖️ License](#️-license)
6. [🔄 Changelog and contributing](#-changelog-and-contributing)
7. [🐛 Bugs and TODO](#-bugs-and-todo)

## ✏️ In short

> **Note**
> This project was done in a week so do not expect crazy behavior and be immune to bugs.

This is a Discord compiler bot which can compile / interpret code blocks and display the result. Keep in mind that we're working in discord. This means, of course, that we have many operating restraints. Here's a few of the big ones.

- Complicated syntax for non-trivial compilations
- Limited output length (limited to a couple hundred characters)
- Single-file input only

Please read [the Wiki](https://github.com/ThomasByr/discord-compiler-bot/wiki) for more.

## 👩‍🏫 Usage

First you should install the [Rust programming language](https://www.rust-lang.org/learn/get-started) for your operating system. Then, you should create a new `.env` file on the model of the `.env.example`. You should at least set `BOT_TOKEN` and `APPLICATION_ID` (you can let other fields untouched) environnement variables.

```ps1
cargo run --release
```

The bot should then connect to the servers and if succesfull, show a list of debug commands on the go.

## 💁 Get Help

While in discord, please type `;help` to get generic help and show a list of available commands, or type `;help <cmd>` to get help about a specific command. Here is a list of the most usefull ones.

- `;format` on a code block with the language set will trigger a format command for the language
- `;compile` will compile and run a code block with the given language
- `;languages` will show a menu to display all languages
- `;asm` will transform the code block into Assembly \_x86_64 for linux

## 🔰 Support

Support for following languages : c++, c, java, python, ruby, rust, javascript, go, php, lua and many more.

On a side note, support has been added for ARM architectures. Meaning you can now host the bot yourself on a Raspberry PI without any tweaks to the code ! But you will need to compile it yourself though...

> [Create a new issue](https://github.com/ThomasByr/discord-compiler-bot/issues/new)

## ⚖️ License

This project is licensed under the GPL-3.0 new or revised license. Please read the [LICENSE](LICENSE) file.

- Redistributions of source code must retain the above copyright notice, this list of conditions and the following disclaimer.

- Redistributions in binary form must reproduce the above copyright notice, this list of conditions and the following disclaimer in the documentation and/or other materials provided with the distribution.

- Neither the name of the discord-compiler-bot authors nor the names of its contributors may be used to endorse or promote products derived from this software without specific prior written permission.

THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT HOLDER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.

## 🔄 Changelog and contributing

Please read the [changelog](changelog.md) file for the full history !

If you ever want to contribute to this project, either request the contributor status, or, more manually, fork the repo and make a full request ! On a more generic note, please do respect the [Rust Coding Conventions](https://rustc-dev-guide.rust-lang.org/conventions.html) and wait for your PR to be reviewed. Make sure you respect and read the [contributing](.github/CONTRIBUTING.md) guideline, make pull requests and be kind.

<details>
    <summary>  Beta first minor release (click here to expand) </summary>

**v0.1.0** first release

- support for 30 more languages
- asm output
- embedded messages for discord
- `;invite`, `;botinfo` commands
- icons are now found online

**v0.1.1** first patch

- locked serenity dependency version to v0.11
- client now has privileged access to context
- fix argument endpoint detection on replies

**v0.1.2** code cleaning and slash commands

- integration of some easy slash commands
- app commands to compile, format and make assembly
- cargo clippy to help keep the code cleaner
- deactivated local execution for some guilds
- `;block` and `;unblock` commands working (blacklist)
- added boilerplate code for some languages

**v0.1.3** local rights and corrections

- patched boilerplate code gen for java
- c detached from c++ boilerplate
- created alternate server count
- new `diff` slash command with colored output
- boilerplate code for php
- pinned serenity dependency to 0.11.1 to avoid headaches

**v0.1.4** hotfix

- compilation service unavailable

**v0.1.5** online services

- assume some service won't work
- unwrap leading to panic
- if api don't return status, assume we failed
- throw more compilation info in footer
- cared about performance for once

</details>

## 🐛 Bugs and TODO

**TODO** (first implementation version)

- [x] connect to Azure (v0.1.0)
- [x] `;invite` (v0.1.0)
- [x] should make slash commands (but I'm lazy) (v0.1.1)
- [ ] should make more slash commands :(

**Bugs** (latest fix)

- ~~`;botinfo` command not working~~ (v0.1.0)
- ~~total number of servers joined always showing 0~~ (v0.1.1)
- ~~debug commands not showing up in console (might be linked to previous bug)~~ (v0.1.1)
