#!/bin/bash

# SPDX-License-Identifier: LGPL-3.0-or-later
#
# (c) SYSTEC electronic AG, D-08468 Heinsdorfergrund, Am Windrad 2
#     www.systec-electronic.com

function install {
    PACKAGE=$1

    if ! apt list --installed 2> /dev/null | grep ${PACKAGE} > /dev/null; then
        echo "Installing: ${PACKAGE}"
        apt install -y ${PACKAGE}
    else
        echo "Already installed: ${PACKAGE}"
    fi
}

install python-pip
install python-dev
install libffi-dev

pip2 install cffi
