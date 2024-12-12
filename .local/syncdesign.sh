#!/usr/bin/env bash

rsync -rh --progress /home/jeleniel/obsidian/Master/ba-Projects/EpilogLite/sql_syntax/* $(dirname "$0")/../design/sql_syntax/
