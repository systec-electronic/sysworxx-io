// SPDX-License-Identifier: LGPL-3.0-or-later
//
// (c) SYSTEC electronic AG, D-08468 Heinsdorfergrund, Am Windrad 2
//     www.systec-electronic.com

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
        thisNode.config = config;

        thisNode.on("input", (msg: NodeRedMsg, send, done) => {
            let analogOutput = +config.channel;

            if ((msg.topic != config.topic) && (config.topic != "#")) {
                return;
            }

            if (typeof (msg.payload) == 'number') {
                if ((msg.payload >= 0) && (msg.payload <= 100)) {
                    let dacValue = +thisNode.config.upperValData * (msg.payload / 100) + +thisNode.config.lowerValData;
                    io.analog_output_set(analogOutput, dacValue);
                    setNodeStatus(msg.payload);
                } else {
                    msg.payload = "Error: value out of range";
                }
            } else {
                msg.payload = "Error: wrong payload type";
            }
            send(msg);
            done();
        });

        function setNodeStatus(value: number) {
            if (value > 0) {
                thisNode.status({ fill: "green", shape: "dot", text: value });
            } else {
                thisNode.status({ fill: "grey", shape: "dot", text: "0" });
            }
        }
    }
    RED.nodes.registerType("Analog Out", nodeContstructor);
    RED.httpAdmin.get("/sysworxxInterfaces", RED.auth.needsPermission('sysworxx.read'), function(req, res) {
        res.json(sysworxxInterfaces);
    });
};

export = initNode;
