// SPDX-License-Identifier: LGPL-3.0-or-later
// SPDX-FileCopyrightText: 2025 SYS TEC electronic AG <https://www.systec-electronic.com/>

import { NodeInitializer } from 'node-red';
import { NodeRedMsg, NodeConfig } from '../types/interfaces';

declare function require(name: string);
const io = require('/usr/lib/libsysworxx_io_js.node');

const initNode: NodeInitializer = (RED): void => {
    function nodeContstructor(this: any, config: NodeConfig): void {
        RED.nodes.createNode(this, config);
        const thisNode = this;
        const initState = getInitState();
        io.set_err_led(initState);
        setNodeStatus(initState);

        thisNode.on("input", (msg: NodeRedMsg, send, done) => {
            if ((msg.topic != config.topic) && (config.topic != "#")) {
                return;
            }

            const newState = getValueFromConfig(msg.payload);
            if (newState != null) {
                io.set_err_led(newState);
                setNodeStatus(newState);
                send(msg);
            }
            done();
        });

        function getInitState(): boolean {
            if (config.initialState == "initStateActive") {
                return true;
            }
            return false;
        }

        function getValueFromConfig(value: boolean | number | string): boolean | null {
            const valueHigh = convertToType(config.dataHigh, config.typeHigh);
            const valueLow = convertToType(config.dataLow, config.typeLow);

            if (value === valueHigh) {
                return true;
            } else if (value === valueLow) {
                return false;
            }
            return null;
        }

        function convertToType(value: boolean | string | number, type: string) {
            switch (type) {
                case "num":
                    return +value;
                case "bool":
                    return (value == "true") ? true : false;
                case "str":
                    return value;
            }
        }

        function setNodeStatus(status: boolean) {
            if (status) {
                thisNode.status({ fill: "red", shape: "dot", text: "on" });
            } else {
                thisNode.status({ fill: "grey", shape: "dot", text: "off" });
            }
        }
    }
    RED.nodes.registerType("Error LED", nodeContstructor);
};

export = initNode;
