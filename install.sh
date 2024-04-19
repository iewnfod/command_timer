#!/bin/bash

echo "This script will install this program as \`t\` in \`/usr/local/bin\`"
echo "You may need adminitrator rights to run this script"
echo ""

read -r -p "Are you sure that you want to continue? [y/N] " ins
case $ins in
	[yY][eE][sS]|[yY])
		cargo build --release
		if [ $? -ne 0 ]; then
			echo "Failed to build binary"
			exit 2
		fi

		cp target/release/command_timer /usr/local/bin/t
		if [ $? -ne 0 ]; then
			echo "Failed to copy binary into \`/usr/local/bin\`"
			echo "You may need to use \`sudo\` to run this script"
			exit 3
		fi

		echo ""
		echo "Install successfully"
		exit 0
		;;

	*)
		echo ""
		echo "Stopped by user"
		exit 1
		;;
esac
