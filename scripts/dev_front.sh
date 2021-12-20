#!/bin/bash
BASEDIR=$(dirname "$0")

watchexec -w zzg -w zzg_wasm -e rs -- $BASEDIR/start_front.sh