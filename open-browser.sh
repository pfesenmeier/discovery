#! /bin/bash

# firefox
BROWSER="/mnt/c/Program Files/Mozilla Firefox/firefox.exe"

# chrome
# BROWSER="/mnt/c/Program Files/Google/Chrome/Application/chrome.exe" 

# if launched from gh
if echo $1 | grep -q "github"; then
 "$BROWSER" $1
 exit 0
fi

# if launched from cargo
"$BROWSER" "file://///wsl$//Ubuntu${1}"

