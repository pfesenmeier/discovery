# https://github.com/rust-lang/cargo/issues/7557#issuecomment-791320960
# open_browser.sh
#! /bin/bash
#echo $1
winPath="file://///wsl$//Ubuntu${1}"
#echo ${winPath}
#firefox
#/mnt/c/Program\ Files/Mozilla\ Firefox/firefox.exe $winPath
#chrome
/mnt/c/Program\ Files/Google/Chrome/Application/chrome.exe $winPath

