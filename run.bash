#!/usr/bin/env bash
# -*- coding: utf-8 -*-

( (./target/release/discord-compiler-bot) 2>&1 | tee bot.log )&
