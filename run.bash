#!/usr/bin/env bash
# -*- coding: utf-8 -*-

processes=$(pgrep -u "$USER" -f "discord-compiler-bot")
xargs kill -9 <<< "$processes" > /dev/null 2>&1
printf "\033[97mkilled processe(s):\033[0m\n%s\n\n" "$processes"

( (./target/release/discord-compiler-bot) 2>&1 | tee bot.log )&
