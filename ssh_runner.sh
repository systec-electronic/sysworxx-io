#!/usr/bin/env bash

# SPDX-License-Identifier: LGPL-3.0-or-later
#
# (c) SYSTEC electronic AG, D-08468 Heinsdorfergrund, Am Windrad 2
#     www.systec-electronic.com

set -e
trap 'ssh "root@${TARGETIP}" "killall -s SIGINT /tmp/$@"' 2

ssh "root@${TARGETIP}" "rm -f /tmp/$1; mkdir -p /tmp/$1; rmdir /tmp/$1"
scp "$1" "root@${TARGETIP}:/tmp/$1"
ssh "root@${TARGETIP}" "IO_LOG=$IO_LOG RUST_BACKTRACE=1 /tmp/$@"
