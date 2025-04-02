// SPDX-License-Identifier: LGPL-3.0-or-later
//
// (c) SYSTEC electronic AG, D-08468 Heinsdorfergrund, Am Windrad 2
//     www.systec-electronic.com

import { NodeInitializer } from 'node-red';
import { SysworxxInterfaces, NodeRedMsg, NodeConfig } from '../types/interfaces';
import fs from 'fs';

declare function require(name: string);
const io = require('/usr/lib/libsysworxx_io_js.node');

const filePath = '/tmp/sysWORXX-Device-Information.json';
if (!fs.existsSync(filePath)) {
    io.get_hw_info(filePath);
}
const sysworxxInterfaces: SysworxxInterfaces = require(filePath);
const runSwitchChannel = 38;

const initNode: NodeInitializer = (RED): void => {
    function nodeContstructor(this: any, config: NodeConfig): void {
        RED.nodes.createNode(this, config);
        const thisNode = this;
        thisNode.uid = 0;

        thisNode.on("input", (msg: NodeRedMsg, send, done) => {
            let state: boolean = io.input_get(getDigitalInput());
            sendMessage(state);
        });

        thisNode.on("close", () => {
            io.unregister_input_interrupt(getDigitalInput(), thisNode.uid);
        });

        let state: boolean = io.input_get(getDigitalInput());
        sendMessage(state);

        registerInputInterrupt();

        function getDigitalInput(): number {
            return runSwitchChannel;
        }

        function registerInputInterrupt() {
            thisNode.uid = io.register_input_interrupt(getDigitalInput(), callbackFunction, 3);
        }

        function callbackFunction(channel: number, state: number) {
            sendMessage(!!state);
            setNodeStatus(!!state);
        }

        function sendMessage(state: boolean) {
            let sendingTopic = config.topic;
            let sendingPayload = getPayloadFromConfig(state);
            let msg: NodeRedMsg = {
                topic: sendingTopic,
                payload: sendingPayload
            };
            setNodeStatus(state);
            thisNode.send(msg);
        }

        function setNodeStatus(status: boolean) {
            if (status) {
                thisNode.status({ fill: "green", shape: "dot", text: "run" });
            } else {
                thisNode.status({ fill: "grey", shape: "dot", text: "stop" });
            }
        }

        function getPayloadFromConfig(state: boolean): boolean | string | number {
            let new_value: boolean | string | number;
            if (state) {
                new_value = convertToType(config.dataHigh, config.typeHigh);
            } else {
                new_value = convertToType(config.dataLow, config.typeLow);
            }
            return new_value;
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
    }
    RED.nodes.registerType("Run Switch", nodeContstructor);
    RED.httpAdmin.get("/sysworxxInterfaces", RED.auth.needsPermission('sysworxx.read'), function (req, res) {
        res.json(sysworxxInterfaces);
    });
};

export = initNode;
