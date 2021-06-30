# https://github.com/rust-lang/cargo/issues/7557#issuecomment-791320960
# open-browser.sh
#! /bin/bash

winPath="file://///wsl$//Ubuntu${1}"

#firefox
#/mnt/c/Program\ Files/Mozilla\ Firefox/firefox.exe $winPath

#chrome
/mnt/c/Program\ Files/Google/Chrome/Application/chrome.exe $winPath

