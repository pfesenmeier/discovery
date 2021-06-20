#!/bin/bash

project_location="$HOME/discovery"

function openocd-stm32 {
	local current_directory=$PWD
	cd /tmp

	local script='openocd.exe -s ./scripts/ -f interface/stlink.cfg -f target/stm32f3x.cfg -c "bindto 0.0.0.0"'
	echo $script
	eval $script

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

windows_ip_address=$(grep "nameserver" /etc/resolv.conf | sed 's/nameserver //')

find . -name openocd.gdb -exec sed -Ei "s/^target remote.+\$/target remote ${windows_ip_address}:3333/" {} \;

