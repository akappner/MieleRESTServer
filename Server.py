#!/usr/bin/python3
# Copyright (c) 2025 Alexander Kappner.
#
# This file is part of MieleRESTServer
# (see github).
#
# This program is free software: you can redistribute it and/or modify
# it under the terms of the GNU General Public License as published by
# the Free Software Foundation, either version 3 of the License, or
# (at your option) any later version.
#
# This program is distributed in the hope that it will be useful,
# but WITHOUT ANY WARRANTY; without even the implied warranty of
# MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
# GNU General Public License for more details.
#
# You should have received a copy of the GNU General Public License
# along with this program. If not, see <http://www.gnu.org/licenses/>.
#
import argparse
import binascii
import json
import logging
import sys
import time
from pathlib import Path

import yaml
from flask import Flask, abort, jsonify, make_response, render_template, request
from flask_restful import Api, Resource, fields, marshal, reqparse

from _version import __version__
from flask import Flask, jsonify, abort, make_response
from flask_restful import Api, Resource, reqparse, fields, marshal
from MieleCrypto import MieleProvisioningInfo, MieleCryptoProvider
from MieleApi import *
from MieleErrors import *
from MieleDop2 import *
from MieleDop2Structures import *
from flask import render_template,request

import json
import time
import yaml
import sys
import argparse

import binascii

PRODUCTNAME = "MieleRESTServer"
endpoints = {}
logger = logging.getLogger(__name__)


def resolve_template_dir():
    for candidate in (
        Path.cwd() / "templates",
        Path(__file__).resolve().parent / "templates",
    ):
        if candidate.is_dir():
            return str(candidate)
    return "templates"


class MieleHelpers:
    def tuple_to_min (t):
        return t[0]*60+t[1];

class EndpointLastComm:
    def __init__ (self):
        self.time=None;
    def reset(self):
        self.time = time.monotonic();
    def __str__(self):
        if (self.time==None):
            return "never"
        else:
            return f"{time.monotonic()-self.time}";
class MieleEndpointConfig:
    def __init__ (self, host, groupId, groupKey, device_route):
#        self.id = id;
        self.host = host;
        self.provisioningInfo = MieleProvisioningInfo(groupId, groupKey);
        self.cryptoProvider = MieleCryptoProvider(self.provisioningInfo);
        self.device_route = device_route;
        self.last_comm=EndpointLastComm();
    def __init__(self, d):
        self.host=d["host"];
        self.provisioningInfo = MieleProvisioningInfo(d["groupId"], d["groupKey"]);
        self.cryptoProvider = MieleCryptoProvider(self.provisioningInfo);
        self.last_comm=EndpointLastComm();

        if ("route" in d and d["route"] != "auto"):
            self.device_route=d["route"];
        else:
            self.autodetect_route()
            logger.info(
                f'Autodetected device route for host {self.host}; please add "deviceRoute: "{self.device_route}" in your config file'
            )

    def tryDecodeAndAdd (j, key, e):
        try:
            toDecode=j[key];
            j["Decoded"+key]=e(toDecode).name;
            return j;
        except:
            return j;
    def autodetect_route(self):
        response = self.send_get(f"Devices")
        j = json.loads(response)
        logger.debug(j)
        if len(j.keys()) == 1:
            self.device_route = list(j.keys())[0]
        else:
            raise Exception("Error autodetecting route");
    def readDop2Leaf (self, unit, attribute, idx1, idx2):
        [attributes, leafData] = self.cryptoProvider.readDop2Leaf(self.host, unit, self.device_route, attribute, idx1, idx2);
        return [ [str(x) for x in attributes], leafData];

    def readDop2LeafRaw (self, unit, attribute, idx1, idx2):
        [attributes, leafData] = self.cryptoProvider.readDop2Leaf(self.host, unit, self.device_route, attribute, idx1, idx2);
        return [attributes, leafData]

    def writeDop2Leaf (self, unit, attribute, payload):
        return self.cryptoProvider.writeDop2Leaf(self.host, self.device_route, unit, attribute, payload);
    def walkdop2tree (self):
        return self.cryptoProvider.readDop2Recursive(self.host, self.device_route);
    def get_device_summary_raw(self):
        return self.send_get(f"Devices/{self.device_route}/State");
    def get_device_ident_raw (self):
        return self.send_get(f"Devices/{self.device_route}/Ident");
    def get_device_summary_annotated (self):
        response=self.get_device_summary_raw();
        j=json.loads(response)
        response2=self.get_device_ident_raw();
        j2=json.loads(response2)
        j=j| j2;
        MieleEndpointConfig.tryDecodeAndAdd(j, "ProgramPhase", ProgramPhase);
        MieleEndpointConfig.tryDecodeAndAdd(j, "ProgramID", ProgramId);
        MieleEndpointConfig.tryDecodeAndAdd(j, "Status", Status);
        MieleEndpointConfig.tryDecodeAndAdd(j, "DeviceType", DeviceType);
        MieleEndpointConfig.tryDecodeAndAdd(j, "DryingStep", DryingStep);
        try:
            elapsed=MieleHelpers.tuple_to_min(j["ElapsedTime"])
            remaining=MieleHelpers.tuple_to_min(j["RemainingTime"])
            total = elapsed + remaining;
            if (total < 0.1):
                progress=0.0;
            else:
                progress = elapsed / (elapsed + remaining)
                logger.debug(f"Progress: {100 * progress:.2f}%")
            j["RemainingMinutes"] = remaining
            j["ElapsedMinutes"] = elapsed
            j["Progress"] = str(progress)
        except:
            j["Progress"] = -1
            j["RemainingMinutes"] = -1
            j["ElapsedMinutes"] = -1
            pass
        return j

    def set_process_action(self):
        command = json.dumps({"ProcessAction": 1})
        logger.debug(command)
        decrypted, response = self.cryptoProvider.sendHttpRequest(
            httpMethod="PUT",
            host=self.host,
            resourcePath=f"Devices/{self.device_route}/State",
            payload=command,
        )
        logger.debug(decrypted)
        return json.loads(decrypted)

    def set_device_action(self):
        command = json.dumps({"DeviceAction": 2})
        # command=json.dumps({"StandbyState": 0});
        logger.debug(command)
        decrypted, response = self.cryptoProvider.sendHttpRequest(
            httpMethod="PUT",
            host=self.host,
            resourcePath=f"Devices/{self.device_route}/State",
            payload=command,
        )
        logger.debug(decrypted)
        return json.loads(decrypted)

    def send_get (self, path):
        try:
            response = self.cryptoProvider.sendHttpRequest(
                httpMethod="GET", host=self.host, resourcePath=path
            )[0]
            logger.debug("response body: %s", response)
            self.last_comm.reset()
            return response
        except:
            logger.exception("Communication error")
            raise

    def serialize(self):
        return json.dumps( {"host":self.host, "groupid": self.provisioningInfo.groupid, "route":self.device_route, "last_comm": self.last_comm.__str__()} )
    def last_comm (self):
        self.last_comm.reset();
class CommandPassthroughAPI(Resource):
    def __init__ (self):
        self.reqparse = reqparse.RequestParser();
        self.reqparse.add_argument('endpoint', type=str, required=True, help='',location='json');
        self.reqparse.add_argument('command', type=str, required=True, help='',location='json');
    def get (self, endpoint, command):
        endpoint=endpoints[endpoint];
        command=command.replace("_","/").replace("-","");
        response=endpoint.send_get(command)
        try:
            return json.loads(response);
        except:
            parser=MieleAttributeParser();
            return [str(x) for x in parser.parseBytes(response)];
#            return str(binascii.hexlify(response, " "));
class Dop2SettingAPI(Resource):
    def __init__(self):
        self.reqparse = reqparse.RequestParser();
        self.reqparse.add_argument('endpoint', type=str, required=True, help='',location='json');
        self.reqparse.add_argument('setting', type=str, required=True, help='',location='json');
    def get (self, endpoint, setting):
        endpoint=endpoints[endpoint];
        try:
            settingInt=int(setting);
        except:
            settingInt=SfValueId[setting];
        [leafData, leafBytes] = endpoint.readDop2LeafRaw(2, 105, idx1=settingInt, idx2=0);
        leaf={}
        for fieldId, fieldData in enumerate(leafData):
            fieldId = fieldId + 1  # DOP uses one-based index
            leaf[fieldId] = fieldData
        logger.debug(leaf)
        ann = DOP2_SF_Value(leaf)
        ann.readFields()
        return ann
        # return {"decoded": [str(x) for x in leaf], "binary":str(binascii.hexlify(data))}


class Dop2LeafAPI(Resource):
    def __init__(self):
        self.reqparse = reqparse.RequestParser();
        self.reqparse.add_argument('endpoint', type=str, required=True, help='',location='json');
        self.reqparse.add_argument('unit', type=int, required=True, help='',location='json');
        self.reqparse.add_argument('attribute', type=int, required=True, help='',location='json');
    def post(self, endpoint, unit, attribute, idx1=0, idx2=0):
        endpoint = endpoints[endpoint]
        payload = request.get_data()
        if request.headers.get("Content-Type", "") == "text/plain":
            payload = binascii.unhexlify(payload)
        logger.debug(f"PUT {unit}/{attribute}, payload={binascii.hexlify(payload)}")
        ##        b="000e000e008200010001000100010400";
        #        b="FFFF000e007a00010001000200010b0000000000000000000209000000002020"
        ##        b="001c000e007a00010001000200010b000000000010000000020900001000" #this sets the time 14/122
        #       b="00190002062f0000000000030001070000000205000000030500002020202020" #test user request
        #        b="00070002062f0000000000010001070000000020202020202020202020202020" #user request message has only ONE byte
        #       b="00070002062f0000000000010001070000000020202020202020202020202020" #user request message has only ONE byte
        #        b="000e0002062f0001000100010001070000002020202020202020202020202020"
        ##      payload=binascii.unhexlify(b);
        [attributes, data] = endpoint.writeDop2Leaf(unit, attribute, payload)
        return [str(x) for x in attributes]

    def get(self, endpoint, unit, attribute, idx1=0, idx2=0):
        endpoint = endpoints[endpoint]
        [attributes, data] = endpoint.readDop2Leaf(unit, attribute, idx1, idx2)
        return {"decoded": attributes, "binary": str(binascii.hexlify(data, " "))}


class SetProcessActionAPI(Resource):
    def post(self, endpoint):
        endpoint=endpoints[endpoint];
        return endpoint.set_process_action();
    def get(self, endpoint):
        endpoint=endpoints[endpoint];
        summary=endpoint.get_device_summary_annotated();
        is_remote_start_capable = summary["RemoteEnable"][2];
        is_remote_start_enabled = summary["ProgramType"]==2 and summary["ProgramID"] != ProgramId.UNKNOWN.value
        return {"DeviceRemoteStartCapable": is_remote_start_capable,
                "DeviceRemoteStartEnabled": is_remote_start_enabled,
                "message":"Set and start timer first on device if not currently enabled" }
class SetDeviceActionAPI(Resource):
    def post(self, endpoint):
        endpoint=endpoints[endpoint];
        return endpoint.set_device_action();
class WalkDOP2TreeAPI(Resource):
    def get(self, endpoint):
        endpoint=endpoints[endpoint];
        return endpoint.walkdop2tree();
class EndpointAPI(Resource):
    def get(self, endpoint=None):
        if endpoint==None:
            return { e: json.loads(x.serialize()) for e, x in endpoints.items()}
        return json.loads(endpoints[endpoint].serialize())


def handle_invalid_usage(e):
    return jsonify(e.asdict()), e.status_code

class JSON_Val:
    def __init__(self, key, expectedType=None):
        self.key=key;
        self.expectedType=expectedType;

def parseAndValidateJSON(request, vals):
    j=request.get_json();
    if (j==None):
        raise MieleRESTException("No valid JSON in request", 400);
    for v in vals:
        if not (v.key in j):
            raise MieleRESTException(f'Required key "{v.key}" missing from payload', 400)
        if (v.expectedType !=None):
            t=type(j[v.key]);
            if (t!=v.expectedType):
                raise MieleRESTException(f'Unexpected type for field "{v.key}": expected "{v.expectedType}", got "{t}"', 400)


def parseJSONrequest(*vals):
    def _parseJSONrequest(f):
        def inner(*args, **kwargs):
            try:
                parseAndValidateJSON(request, vals);
                t=f(*args, **kwargs)
            except MieleRESTException as e:
                return e.asdict(), 400;
            return t;
        return inner;
    return _parseJSONrequest;
class DeviceSummaryAPI(Resource):
    def __init__ (self):
        self.reqparse = reqparse.RequestParser();
        self.reqparse.add_argument('endpoint', type=str, required=False, help='',location='json');
        super(DeviceSummaryAPI,self).__init__()
    def get(self, endpoint):
        if (len(endpoint)>0):
            endpoint=endpoints[endpoint];
            j=endpoint.get_device_summary_annotated();
            return j;
class DevicesSummaryAPI(Resource):
    def get(self):
        j={e: x.get_device_summary_annotated() for e, x in endpoints.items() }
        return j;


def main(argv=None):
    parser = argparse.ArgumentParser(
                    prog=PRODUCTNAME,
                    description='Provides RESTful interface to Miele@home devices')

    parser.add_argument('-v', '--version', action='version', version='%(prog)s ' + __version__)

    parser.add_argument(
        "-c",
        "--config",
        default=f"/etc/{PRODUCTNAME}.config",
        help="path to configuration file",
    )
    parser.add_argument(
        "-b",
        "--bind",
        default=f"127.0.0.1",
        help="IP address to bind to, default is local only",
    )
    parser.add_argument(
        "-p", "--port", default=5001, help="port to bind to, default is port 5001"
    )
    parser.add_argument(
        "--webui", action="store_true", help="enable experimental web UI, default off"
    )
    parser.add_argument(
        "--debug",
        action="store_true",
        help="run REST server in debug mode, default off",
    )
    parser.add_argument(
        "-l", "--log-level",
        default="INFO",
        type=str.upper,
        choices=["CRITICAL", "ERROR", "WARNING", "INFO", "DEBUG", "NOTSET"],
        help="set Python logging level (default: INFO)",
    )

    cmdargs = parser.parse_args(argv)
    logging.basicConfig(
        level=getattr(logging, cmdargs.log_level),
        format="%(asctime)s %(levelname)s %(name)s: %(message)s",
    )

    with open(cmdargs.config) as stream:
        try:
            config_file = yaml.safe_load(stream)
        except yaml.YAMLError:
            logger.exception("Error loading configuration file, exiting")
            return 1

    endpoints.clear()
    for key, value in config_file["endpoints"].items():
        endpoints[key]=MieleEndpointConfig(value);


    app = Flask(__name__, static_url_path="")
    app.register_error_handler(MieleRESTException, handle_invalid_usage)
    api = Api(app)
    api.add_resource(DevicesSummaryAPI, '/generate-summary')
    api.add_resource(DeviceSummaryAPI, '/generate-summary/<string:endpoint>')
    api.add_resource(WalkDOP2TreeAPI, '/walkdop2tree/<string:endpoint>')
    api.add_resource(EndpointAPI, '/endpoints', '/endpoints/<string:endpoint>')
    api.add_resource(SetProcessActionAPI, '/start/<string:endpoint>')
    api.add_resource(SetDeviceActionAPI, '/wakeup/<string:endpoint>')
    api.add_resource(CommandPassthroughAPI, '/command/<string:endpoint>/<string:command>')

    api.add_resource(Dop2LeafAPI,
        '/dop2leaf/<string:endpoint>/<int:unit>/<int:attribute>',
        '/dop2leaf/<string:endpoint>/<int:unit>/<int:attribute>/<int:idx1>',
        '/dop2leaf/<string:endpoint>/<int:unit>/<int:attribute>/<int:idx1>/<int:idx2>'
)
    api.add_resource(Dop2SettingAPI, '/dop2setting/<string:endpoint>/<string:setting>')

    if (cmdargs.webui):
        @app.route("/webui", strict_slashes=False)
        @app.route("/", strict_slashes=False)
        def webui_index():
            return render_template("generate_summary.html", endpoint_names=list(endpoints.keys()));
        @app.route("/webui/<string:endpoint>")
        def webui_endpoint(endpoint):
            #        context=EndpointAPI.get(endpoint);
            context = endpoints[endpoint].get_device_summary_annotated()
            logger.debug(context)
            return render_template(
                "generate_summary.html", endpoint=context, endpointName=endpoint
            )

    else:
        logger.info("WebUI disabled.")

    app.run(debug=cmdargs.debug, host=cmdargs.bind, port=cmdargs.port);
    return 0


if __name__ == '__main__':
    raise SystemExit(main())
