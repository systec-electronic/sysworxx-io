// SPDX-License-Identifier: LGPL-3.0-or-later
//
// (c) SYSTEC electronic AG, D-08468 Heinsdorfergrund, Am Windrad 2
//     www.systec-electronic.com

import { NodeInitializer } from 'node-red';
import { SysworxxInterfaces, NodeRedMsg, NodeConfig, SAMPLE_UNIT } from '../types/interfaces';
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
            let value: number = io.counter_get(thisNode.channel);
            sendMessage(value);
        });

        thisNode.on("close", () => {
            io.counter_enable(thisNode.channel, false);
            clearInterval(thisNode.objStatusTimerId);
            thisNode.objStatusTimerId = -1;
        });

        setupCounterInput();
        thisNode.objStatusTimerId = setInterval(publishCntValue, thisNode.sampleRate);

        function publishCntValue() {
            let value = io.counter_get(thisNode.channel);
            sendMessage(value);
        }

        function setupCounterInput() {
            thisNode.channel = +config.channel;
            thisNode.counterMode = +thisNode.config.counterMode;
            thisNode.counterTrigger = +thisNode.config.counterTrigger;
            thisNode.counterDirection = +thisNode.config.counterDirection;
            thisNode.sampleRate = calculateSampleRate(thisNode.config.sampleRate, thisNode.config.sampleUnit);

            io.counter_setup(
                thisNode.channel, thisNode.counterMode, thisNode.counterTrigger, thisNode.counterDirection);
            io.counter_enable(thisNode.channel, true);
        }

        function calculateSampleRate(sampleRate: string, sampleUnit: string): number {
            let calculatedSampleRate = parseInt(sampleRate);
            calculatedSampleRate *= SAMPLE_UNIT[sampleUnit];
            return calculatedSampleRate;
        }

        function sendMessage(value: number) {
            let sendingTopic = config.topic;
            let msg: NodeRedMsg = {
                topic: sendingTopic,
                payload: value
            };
            thisNode.send(msg);
            thisNode.status({ fill: "green", shape: "dot", text: value });
        }
    }

    RED.nodes.registerType("Counter In", nodeContstructor);
    RED.httpAdmin.get("/sysworxxInterfaces", RED.auth.needsPermission('sysworxx.read'), function(req, res) {
        res.json(sysworxxInterfaces);
    });
};

export = initNode;
