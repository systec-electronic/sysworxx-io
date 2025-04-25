// SPDX-License-Identifier: LGPL-3.0-or-later
// SPDX-FileCopyrightText: 2025 SYS TEC electronic AG <https://www.systec-electronic.com/>

import { NodeInitializer } from 'node-red';
import { SysworxxInterfaces, NodeRedMsg, NodeConfig } from './../types/interfaces';
import fs from 'fs';

declare function require(name: string);
const io = require('/usr/lib/libsysworxx_io_js.node');

const filePath = '/tmp/sysWORXX-Device-Information.json';
if (!fs.existsSync(filePath)) {
    io.get_hw_info(filePath);
}
const sysworxxInterfaces: SysworxxInterfaces = require(filePath);

const initNode: NodeInitializer = (RED): void => {
    function nodeContstructor(this: any, config: NodeConfig): void {
        RED.nodes.createNode(this, config);
        const thisNode = this;
        const initState = getInitState();
        io.output_set(+config.channel, initState);
        setNodeStatus(initState);

        thisNode.on("input", (msg: NodeRedMsg, send, done) => {
            let digitalOutput = +config.channel;

            if ((msg.topic != config.topic) && (config.topic != "#")) {
                return;
            }

            const newState = getValueFromConfig(msg.payload);
            if (newState != null) {
                io.output_set(digitalOutput, newState);
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
                thisNode.status({ fill: "green", shape: "dot", text: "1" });
            } else {
                thisNode.status({ fill: "grey", shape: "dot", text: "0" });
            }
        }
    }
    RED.nodes.registerType("Digital Out", nodeContstructor);
    RED.httpAdmin.get("/sysworxxInterfaces", RED.auth.needsPermission('sysworxx.read'), function(req, res) {
        res.json(sysworxxInterfaces);
    });
};

export = initNode;
