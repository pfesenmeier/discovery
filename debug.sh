#!/bin/bash

project_location="$HOME/discovery"

function start-openocd {
	local current_directory=$PWD
	cd /tmp

	local script='openocd.exe -s ./scripts/ -f interface/stlink.cfg -f target/stm32f3x.cfg -c "bindto 0.0.0.0"'
	echo $script
	eval $script

	cd $current_directory
}

function start-itm {
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

# update gdb commands with current windows ip address
find . -name openocd.gdb -exec sed -Ei "s/^target remote.+\$/target remote ${windows_ip_address}:3333/" {} \;

