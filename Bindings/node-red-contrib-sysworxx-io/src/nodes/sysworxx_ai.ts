// SPDX-License-Identifier: LGPL-3.0-or-later
//
// (c) SYSTEC electronic AG, D-08468 Heinsdorfergrund, Am Windrad 2
//     www.systec-electronic.com

import { NodeInitializer } from 'node-red';
import { SysworxxInterfaces, NodeRedMsg, NodeConfig, ADC_STATE, ADC_MODE, SAMPLE_UNIT, DECIMAL_PLACES } from '../types/interfaces';
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
            let value: number = io.analog_input_get(thisNode.channel);
            sendMessage(value);
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
            setNodeStatus(ADC_STATE.ACTIVE);
            startTimerStatus(1000);
        }

        function publishAdcData() {
            let analogValue = io.analog_input_get(thisNode.channel);

            // Check, if the read value is different "enough" to be published
            let deltaValue = Math.abs(analogValue - thisNode.lastValue);
            if (!(deltaValue <= thisNode.deltaProcVal)) {
                thisNode.lastValue = analogValue;
                let procValue = (analogValue * thisNode.digitValue) + thisNode.lowerValData;
                // The method toFixed() must be between 0 and 100.
                if ((thisNode.decimalPlaces >= 0) &&
                    (thisNode.decimalPlaces <= 100) &&
                    (thisNode.decimalPlaces != undefined)) {
                    procValue = +procValue.toFixed(thisNode.decimalPlaces);
                }
                sendMessage(+procValue);
            }
        }

        function onOpenInit() {
            thisNode.config = config;
            thisNode.channel = getAnalogInput();
            thisNode.mode = ADC_MODE[thisNode.config.mode];
            thisNode.sampleRate = calculateSampleRate(thisNode.config.sampleRate, thisNode.config.sampleUnit);
            thisNode.delta = parseInt(thisNode.config.delta, 10);
            thisNode.unit = thisNode.config.unit;
            thisNode.upperValData = parseFloat(thisNode.config.upperValData);
            thisNode.lowerValData = parseFloat(thisNode.config.lowerValData);
            thisNode.decimalPlaces = DECIMAL_PLACES[thisNode.config.decimalPlaces];
            thisNode.lastValue = 0;

            // convert user configured 'delta' value into ADC data format (12bit ADC -> SHL(3) -> 15bit ADC)
            thisNode.deltaProcVal = thisNode.delta * 8;
            thisNode.digitValue = getDigitValue(thisNode.upperValData, thisNode.lowerValData);

            if (config.enableModeSetting) {
                setAdcMode(thisNode.channel, thisNode.mode);
            }
            setNodeStatus(ADC_STATE.IDLE);
        }

        function setAdcMode(channel: number, mode: ADC_MODE) {
            io.analog_mode_set(channel, mode);
        }

        function getDigitValue(upperValData: number, lowerValData: number) {
            return ((upperValData - lowerValData) / 32768);
        }

        function calculateSampleRate(sampleRate: string, sampleUnit: string): number {
            let calculatedSampleRate = parseInt(sampleRate);
            calculatedSampleRate *= SAMPLE_UNIT[sampleUnit];
            return calculatedSampleRate;
        }

        function setNodeStatus(status: ADC_STATE) {
            let statusColor = "grey";
            let statusShape = "dot";
            let statusText = "settled";

            switch (status) {
                case ADC_STATE.ACTIVE:
                    statusColor = "green";
                    statusText = "altered";
                    break;

                case ADC_STATE.ERROR:
                    statusColor = "red";
                    statusText = "error";
                    break;

                case ADC_STATE.IDLE:
                default:
                    break;
            }
            thisNode.status({ fill: statusColor, shape: statusShape, text: statusText });
        }

        function startTimerStatus(timeout: number) {
            setTimeout(() => {
                setNodeStatus(ADC_STATE.IDLE);
            }, timeout);
        }
    }
    RED.nodes.registerType("Analog In", nodeContstructor);
    RED.httpAdmin.get("/sysworxxInterfaces", RED.auth.needsPermission('sysworxx.read'), function(req, res) {
        res.json(sysworxxInterfaces);
    });
};

export = initNode;
