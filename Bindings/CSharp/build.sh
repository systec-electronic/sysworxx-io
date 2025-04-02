#!/bin/sh

# SPDX-License-Identifier: LGPL-3.0-or-later
#
# (c) SYSTEC electronic AG, D-08468 Heinsdorfergrund, Am Windrad 2
#     www.systec-electronic.com

set -e
if [ -x "$(command -v mcs)" ]; then
	csc -out:Demo.exe Demo/Demo.cs Ctr700Driver/Ctr700.cs
	csc -out:Runlight.exe Runlight/Runlight.cs Ctr700Driver/Ctr700.cs /reference:Mono.Posix.dll
	echo
	echo Start the programs: mono Demo.exe
	echo or: mono Runlight.exe
	exit 0
fi
echo "Error: No build environment available. Please install .NET SDK or Mono." >&2
exit 1
