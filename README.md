# <img src="assets/code.png" alt="icon" width="3%"/> Discord compiler bot

[![Linux](https://svgshare.com/i/Zhy.svg)](https://docs.microsoft.com/en-us/windows/wsl/tutorials/gui-apps)
[![Windows](https://svgshare.com/i/ZhY.svg)](https://svgshare.com/i/ZhY.svg)
[![GitHub license](https://img.shields.io/github/license/ThomasByr/discord-compiler-bot)](https://github.com/ThomasByr/discord-compiler-bot/blob/master/LICENSE)
[![GitHub commits](https://badgen.net/github/commits/ThomasByr/discord-compiler-bot)](https://GitHub.com/ThomasByr/discord-compiler-bot/commit/)
[![GitHub latest commit](https://badgen.net/github/last-commit/ThomasByr/discord-compiler-bot)](https://gitHub.com/ThomasByr/discord-compiler-bot/commit/)
[![Maintenance](https://img.shields.io/badge/Maintained%3F-yes-green.svg)](https://GitHub.com/ThomasByr/discord-compiler-bot/graphs/commit-activity)

[![Continuous Integration](https://github.com/ThomasByr/discord-compiler-bot/actions/workflows/main.yml/badge.svg)](https://github.com/ThomasByr/discord-compiler-bot/actions/workflows/main.yml)
[![rust-clippy analyze](https://github.com/ThomasByr/discord-compiler-bot/actions/workflows/rust-clippy.yml/badge.svg)](https://github.com/ThomasByr/discord-compiler-bot/actions/workflows/rust-clippy.yml)
[![GitHub version](https://badge.fury.io/gh/ThomasByr%2Fdiscord-compiler-bot.svg)](https://github.com/ThomasByr/discord-compiler-bot)
[![Author](https://img.shields.io/badge/author-@ThomasByr-blue)](https://github.com/ThomasByr)

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
5. [License](#license)
6. [Changelog, Bugs and TODO](#changelog-bugs-and-todo)

## In short

First, this project was done in a week so do not expect crazy behavior and be immune to bugs.

This is a Discord compiler bot which can compile / interpret code blocks and display the result. Keep in mind that we're working in discord. This means, of course, that we have many operating restraints. Here's a few of the big ones.

- Complicated syntax for non-trivial compilations
- Limited output length (limited to a couple hundred characters)
- Single-file input only

Please read [the Wiki](https://github.com/ThomasByr/discord-compiler-bot/wiki) for more.

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
- `;asm` will transform the code block into Assembly \_x86_64 for linux

If you want to contribute, please read the [contributing](.github/CONTRIBUTING.md) guideline, make pull requests and be kind.

## Support

Support for following languages : c++, c, java, python, ruby, rust, javascript, go, php, lua and many more.

> [Create a new issue](https://github.com/ThomasByr/discord-compiler-bot/issues/new)

## License

This project is licensed under the GPL-3.0 new or revised license. Please read the [LICENSE](LICENSE) file.

- Redistributions of source code must retain the above copyright notice, this list of conditions and the following disclaimer.

- Redistributions in binary form must reproduce the above copyright notice, this list of conditions and the following disclaimer in the documentation and/or other materials provided with the distribution.

- Neither the name of the discord-compiler-bot authors nor the names of its contributors may be used to endorse or promote products derived from this software without specific prior written permission.

THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT HOLDER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.

## Changelog, Bugs and TODO

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

</details>

**TODO** (first implementation version)

- [x] connect to Azure (v0.1.0)
- [x] `;invite` (v0.1.0)
- [x] should make slash commands (but I'm lazy) (v0.1.1)
- [ ] should make more slash commands :(

**Bugs** (latest fix)

- ~~`;botinfo` command not working~~ (v0.1.0)
- ~~total number of servers joined always showing 0~~ (v0.1.1)
- ~~debug commands not showing up in console (might be linked to previous bug)~~ (v0.1.1)
