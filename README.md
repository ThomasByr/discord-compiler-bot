# <img src="assets/code.png" alt="icon" width="4%"/> Discord Compiler Bot

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
;compile rust
argv1 argv2 argv3
```
stdin1
stdin2
stdin3
```
```rs
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().collect();

    let query = &args.get(1).expect("command line argument missing");

    let mut input = String::new();
    std::io::stdin().read_line(&mut input)?;
    let input = input.trim();

    println!(
        "first command line argument: {}\nfirst input line: {}",
        query, input
    );

    Ok(())
}
```
````

1. [✏️ In short](#️-in-short)
2. [👩‍🏫 Usage](#-usage)
3. [💁 Get Help](#-get-help)
4. [🔰 Support](#-support)
5. [🧪 Testing](#-testing)
6. [🧑‍🏫 Contributing](#-contributing)
7. [⚖️ License](#️-license)
8. [🖼️ Icons](#️-icons)
9. [🔄 Changelog](#-changelog)
10. [🐛 Bugs and TODO](#-bugs-and-todo)

## ✏️ In short

> **Note**
> This project was initially done in a week so do not expect crazy behavior and be immune to bugs.

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

## 🧪 Testing

Oh god... please don't.

Still, make sure you have `cargo` up and ready and then run :

```bash
cargo test --all
```

## 🧑‍🏫 Contributing

If you ever want to contribute, either request the contributor status, or, more manually, fork the repo and make a full request !. On a more generic note, please do respect the [Rust Coding Conventions](https://rustc-dev-guide.rust-lang.org/conventions.html) and wait for your PR to be reviewed. Make sure you respect and read the [Contributing Guidelines](.github/CONTRIBUTING.md), make pull requests and be kind.

> The standard procedure is :
>
> ```txt
> fork -> git branch -> push -> pull request
> ```
>
> Note that we won't accept any PR :
>
> - that does not follow our Contributing Guidelines
> - that is not sufficiently commented or isn't well formated
> - without any proper test suite
> - with a failing or incomplete test suite

Happy coding ! 🙂

## ⚖️ License

This project is licensed under the AGPL-3.0 new or revised license. Please read the [LICENSE](LICENSE) file.

- Redistributions of source code must retain the above copyright notice, this list of conditions and the following disclaimer.

- Redistributions in binary form must reproduce the above copyright notice, this list of conditions and the following disclaimer in the documentation and/or other materials provided with the distribution.

- Neither the name of the discord-compiler-bot authors nor the names of its contributors may be used to endorse or promote products derived from this software without specific prior written permission.

THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT HOLDER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.

## 🖼️ Icons

Icons (except [the logo](assets/code.png)) are made by [Freepik](https://www.flaticon.com/authors/freepik) and [pixelmeetup](https://www.flaticon.com/authors/pixelmeetup) from [www.flaticon.com](https://www.flaticon.com/).

## 🔄 Changelog

Please read the [changelog](changelog.md) file for the full history !

<details>
    <summary>  First stable release version (click here to expand) </summary>

**v1.0** unwrapping

- strongest cargo clippy analysis
- fixed some `panic!` on `.unwrap()`
- support for custom fail emoji
- the bot now uses 75B less ram on average, yay

**v1.1** swaps

- rewrote [CONTRIBUTING.md](.github/CONTRIBUTING.md)
- rules for background runner > .log
- TypeScript fix & shorthand `ts`
- `panic!` hook : prevent invalid panic logs from obfuscating errors
- rolled back to a lower output length limit

**v1.2** what is more useful when is broken ?

- reworked C and Java boilerplates
- check for existence of builds
- added `bash` and `sh` aliases for `bash script`

**v1.3** more languages

- added `c#` and `cs` aliases for `csharp`
- ... and many more

**v1.4** dependencies

- added `.rustfmt.toml` to enforce a consistent style
- updated dependencies (notably serenity to `0.11.5`)
- finally unified embed dispatching
- went back to dispatching embeds manually for `;asm` and `;compile` commands
- upgraded godbolt
- restored proper order of operation when loading shards

**v1.5** bench

- ensured regular expressions are compiled statically
- minor bump to serenity `0.11.6` and use of unstable features (forum threads)

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
- ~~some shards are randomly disconnecting~~ (v1.4.2)
