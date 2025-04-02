#!/bin/bash

# SPDX-License-Identifier: LGPL-3.0-or-later
#
# (c) SYSTEC electronic AG, D-08468 Heinsdorfergrund, Am Windrad 2
#     www.systec-electronic.com

set -e

mkdir -p /usr/src/rootfs
wget -qO- http://srv-gitlab.intern.systec-electronic.com/121903/debian-var/-/jobs/6389/artifacts/raw/rootfs_dev.tgz?inline=false | tar xzC /usr/src/rootfs
chroot /usr/src/rootfs
apt-get update
apt-get install -y libiio-dev
exit
