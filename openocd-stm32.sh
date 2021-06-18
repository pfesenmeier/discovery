#!/bin/bash

project_location='discovery'

function openocd-stm32 {
	local current_directory=$PWD
	cd /tmp

	local script='openocd.exe -s ./scripts/ -f interface/stlink.cfg -f target/stm32f3x.cfg'
	echo $script
	$script

	cd $current_directory
}

function itmd {
	local current_directory=$PWD
	cd /tmp

	if [ ! -f itm.txt ]; then 
	  touch itm.txt
	fi
        >|itm.txt

	local script='itmdump -F -f itm.txt'
	echo $script
	$script

	trap 'cd $current_directory' EXIT
}

deno run "$project_location"/get-ip-address-for-gdb-target.ts
