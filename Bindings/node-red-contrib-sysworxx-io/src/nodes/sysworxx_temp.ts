// SPDX-License-Identifier: LGPL-3.0-or-later
//
// (c) SYSTEC electronic AG, D-08468 Heinsdorfergrund, Am Windrad 2
//     www.systec-electronic.com

import { NodeInitializer } from 'node-red';
import { SysworxxInterfaces, NodeRedMsg, NodeConfig, ADC_STATE, ADC_MODE, SAMPLE_UNIT, DECIMAL_PLACES, TMP_MODE, TMP_SENSOR_TYPE } from '../types/interfaces';
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
        let thisNode = this;

        // event functions
        thisNode.on("input", (msg: NodeRedMsg, send, done) => {
            publishAdcData();
        });

        thisNode.on("close", () => {
            clearInterval(thisNode.objStatusTimerId);
            thisNode.objStatusTimerId = -1;
        });

        onOpenInit();

        thisNode.objStatusTimerId = setInterval(publishAdcData, thisNode.sampleRate);

        // callable functions
        function getAnalogInput(): number {
            return +config.channel;
        }

        function sendMessage(value: number) {
            let sendingTopic = config.topic;
            let msg: NodeRedMsg = {
                topic: sendingTopic,
                payload: value
            };
            thisNode.send(msg);
        }

        function publishAdcData() {
            const analogValue = io.tmp_input_get(thisNode.channel);
            let deltaValue = Math.abs(analogValue - thisNode.lastValue);

            if (!(deltaValue <= thisNode.deltaProcVal)) {
                thisNode.lastValue = analogValue;
                let procValue = (analogValue / 10000);
                // The method toFixed() must be between 0 and 100.
                if ((thisNode.decimalPlaces >= 0) &&
                    (thisNode.decimalPlaces <= 100) &&
                    (thisNode.decimalPlaces != undefined)) {
                    procValue = +procValue.toFixed(thisNode.decimalPlaces);
                }
                sendMessage(procValue);
            }
        }

        function onOpenInit() {
            thisNode.config = config;
            thisNode.channel = getAnalogInput();
            thisNode.tmpMode = TMP_MODE[thisNode.config.tmpMode];
            thisNode.tmpType = TMP_SENSOR_TYPE[thisNode.config.tmpType];
            thisNode.sampleRate = calculateSampleRate(thisNode.config.sampleRate, thisNode.config.sampleUnit);
            thisNode.delta = parseInt(thisNode.config.delta, 10);
            thisNode.decimalPlaces = DECIMAL_PLACES[thisNode.config.decimalPlaces];
            thisNode.lastValue = 0;
            thisNode.deltaProcVal = thisNode.delta * 8;

            if (config.enableModeSetting) {
                setTmpMode(thisNode.channel, thisNode.tmpMode, thisNode.tmpType);
            }
        }

        function setTmpMode(channel: number, mode: TMP_MODE, type: TMP_SENSOR_TYPE) {
            io.tmp_set_mode(channel, mode, type);
        }

        function calculateSampleRate(sampleRate: string, sampleUnit: string): number {
            let calculatedSampleRate = parseInt(sampleRate);
            calculatedSampleRate *= SAMPLE_UNIT[sampleUnit];
            return calculatedSampleRate;
        }
    }
    RED.nodes.registerType("Temp Sensor", nodeContstructor);
    RED.httpAdmin.get("/sysworxxInterfaces", RED.auth.needsPermission('sysworxx.read'), function (req, res) {
        res.json(sysworxxInterfaces);
    });
};

export = initNode;
