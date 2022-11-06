# Changelog

<summary>The full history, or so was I told...</summary>

## Beta first minor release

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

## First stable release version

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
