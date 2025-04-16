#!/bin/bash

# SPDX-License-Identifier: LGPL-3.0-or-later
# SPDX-FileCopyrightText: 2025 SYS TEC electronic AG <https://www.systec-electronic.com/>

function install {
    PACKAGE=$1

    if ! apt list --installed 2> /dev/null | grep ${PACKAGE} > /dev/null; then
        echo "Installing: ${PACKAGE}"
        apt install -y ${PACKAGE}
    else
        echo "Already installed: ${PACKAGE}"
    fi
}

install python3-pip
install python3-dev
install libffi-dev

pip3 install cffi
