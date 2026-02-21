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
import os
import struct
import numpy as np
import hmac
import hashlib
import binascii
import sys
import json
import pprint
from cryptography.hazmat.primitives.ciphers import Cipher, algorithms, modes
from MieleDop2 import MieleAttributeParser
from MieleDop2Structures import *

import requests
import secrets

from MieleErrors import *

class Dop2DataField:
    def __init__ (self, fieldNumber, payload, cursor):
        self.fieldType=payload[cursor];
    def __dict__ (self):
        return {}

class Dop2StringField:
    def __init__ (self, fieldNumber, payload, cursor):
        self.fieldType=payload[cursor];
        contentLength=(payload[cursor+1]<<8) + payload[cursor+2]
        if (cursor < len(payload) and payload[cursor]==0x00):
            contentLength=contentLength+1;

        cursor=cursor+2

        self.fieldNumber=fieldNumber;
        self.stringData=payload[cursor+1:cursor+contentLength].strip(bytes([0x00]));
        self.wireLength=contentLength+4;
    def __dict__ (self):
        return {}
        #return {"payload": str(self.stringData), "wireLength" : self.wireLength, "fieldNumber" : self.fieldNumber}
    def __str__ (self):
        return f"Field {self.fieldNumber}, type {self.fieldType}, wire length {self.wireLength}, string data {self.stringData}";
    def __repr__ (self):
        return self.__str__();
class MieleProvisioningInfo:
    def __init__ (self, groupid="", groupkey=""):
        self.groupkey=bytearray.fromhex(groupkey);
        self.groupid=groupid.upper();
    def get_aes_key (self):
        return self.groupkey[0:32];
    def get_signature_key(self):
        return self.groupkey

    @staticmethod
    def generate_random():
        return MieleProvisioningInfo(groupid=secrets.token_hex(8), groupkey=secrets.token_hex(64))
    def to_dict(self):
        return { "GroupID": self.groupid, "GroupKey": str(self.groupkey.hex().upper()) }
    def __str__(self):
        return f"""groupId: "{self.groupid}"
        groupKey: {str(self.groupkey.hex().upper())}"""

    def to_pairing_json(self):
        return json.dumps(self.to_dict(), sort_keys=True, indent=4)

    @staticmethod
    def from_paring_json(payload: str):
        data = json.loads(payload)
        return MieleProvisioningInfo(
            groupid=data["GroupID"],
            groupkey=data["GroupKey"],
        )


class MieleAuthHeader:
    def __init__ (self, groupid, iv, signature):
        self.groupid=groupid; #string
        self.signature=signature; #bytearray
        self.iv = iv;
    def __str__ (self):
        return
    @classmethod
    def from_string (cls, auth_header):
        return;

class MieleCryptoProvider:
    def __init__ (self, provisioningInfo):
        self.provisioningInfo = provisioningInfo;
    def iv_from_auth_header (self, authHeader):
        response_groupid, response_signature = authHeader[len("MieleH256 "):].split(":");
        response_signature = bytearray.fromhex(response_signature);
        response_iv=response_signature[0:16];
        print(response_iv)
        return response_iv;

    def decrypt_response (self, response):
        authHeader=response.headers["X-Signature"]
        response_iv=self.iv_from_auth_header(authHeader);
        response_plaintext = MieleCryptoProvider.decrypt_bytes(response.content, self.provisioningInfo.get_aes_key(), response_iv);
        #print(response_plaintext);
        return response_plaintext;
    def decrypt_bytes (ciphertext, key, iv):
        cipher = Cipher(algorithms.AES(key), modes.CBC(iv))
        decryptor = cipher.decryptor()
        plaintext = decryptor.update(ciphertext) + decryptor.finalize()
        return plaintext;
    def encrypt_bytes (plaintext, key, iv):
        cipher = Cipher(algorithms.AES(key), modes.CBC(iv))
        encryptor = cipher.encryptor()
        ciphertext = encryptor.update(plaintext) + encryptor.finalize()
        return ciphertext;

    def sign (self, httpMethod="PUT", host="127.0.0.1", date="Fri, 25 Jan 2025 19:57:40 GMT", acceptHeader="application/vnd.miele.v2+json; charset=utf-8",resourcePath="", contentTypeHeader="",body=""):
#        payload=f"{httpMethod}\n{host}/{resourcePath}\n{contentTypeHeader}\n{acceptHeader}\n{date}\n{body}";
#        payload_bytes=payload.encode('utf-8');
        if (isinstance(body,str)):
            body_bytes=body.encode('utf-8');
        else:
            body_bytes=body

        payload=f"{httpMethod}\n{host}/{resourcePath}\n{contentTypeHeader}\n{acceptHeader}\n{date}\n";
        payload_bytes=payload.encode('utf-8')+ body_bytes;
#        if (isinstance(body), str):
#            body=bytes(body, 'utf-8');
#        payload=f"{httpMethod}\n{host}/{resourcePath}\n{contentTypeHeader}\n{acceptHeader}\n{date}\n";
#        payload_bytes=payload.encode('utf-8') + body;

        hmac_obj = hmac.new(self.provisioningInfo.get_signature_key(), payload_bytes, hashlib.sha256);
        digest=hmac_obj.hexdigest().upper();
        print(digest);
        return digest;

    def get_auth_header (self, httpMethod="PUT", host="127.0.0.1", date="Fri, 25 Jan 2025 19:57:40 GMT", acceptHeader="application/vnd.miele.v2+json; charset=utf-8",resourcePath="", contentTypeHeader="", body=""):
        signature=self.sign(httpMethod, host, date, acceptHeader, resourcePath, contentTypeHeader, body=body);
        header= f"Authorization: MieleH256 {self.provisioningInfo.groupid}:{signature}"
        header= f"MieleH256 {self.provisioningInfo.groupid}:{signature}"
        return header;
    def pad_body_bytes (self, payload):
        blocksize=16;
        if (len(payload) % blocksize == 0): #no alignment needed
            return payload;
        padding=blocksize - (len(payload) % blocksize);
        print (f"padding with {padding} bytes")
        return payload.ljust(len(payload) + padding, b'\x20');
#        return payload;
    def pad_body_str (self, payload):
        if (len(payload)==0):
            return "";
        if (payload[-1]!="}"):
            raise Exception("Plaintext must be terminated with literal '}'")
        payload=payload[0:-1] + " "* (64-len(payload)) + "}";
#        payload=payload.encode('ascii')
        return payload;
    def encrypt_payload (self, payload, iv):
        if (isinstance(payload, str)):
            payload=payload.encode('utf-8');
        return MieleCryptoProvider.encrypt_bytes(payload, self.provisioningInfo.get_aes_key(), iv)
    def sendHttpRequest(self, httpMethod="GET", host="10.0.0.11", resourcePath="Devices/", payload=""):
        print(f"Sending HTTP request to {host}, resourcePath={resourcePath}");
        acceptHeader= "application/vnd.miele.v1+json"; #the device is not looking at this
        contentTypeHeader="application / vnd.miele.v1 + json; charset = utf - 8"
        date="Thu, 01 Jan 1970 02:09:22 GMT" # the device is not looking at this either
        if (isinstance (payload, str)):
            payload=self.pad_body_str(payload);
            print(f"STring payload: " + payload);
        else:
            payload=self.pad_body_bytes(payload);
        authHeader=self.get_auth_header(host=host, httpMethod=httpMethod, date=date, resourcePath=resourcePath, acceptHeader=acceptHeader, contentTypeHeader=contentTypeHeader, body=payload)
        if (len (payload) > 0):
            iv = self.iv_from_auth_header(authHeader);
            body_encrypted=self.encrypt_payload (payload, iv);
        else:
            body_encrypted=None;

        if (httpMethod=="GET"):
            response=requests.get("http://"+host+"/"+resourcePath, headers={"Content-Type": "application / vnd.miele.v1 + json; charset = utf - 8", "Host" : host, "User-Agent": "Miele@mobile 2.3.3 iOS", "Authorization": authHeader, "Date": date, "Accept":acceptHeader} , data=body_encrypted)

        elif (httpMethod=="PUT"):
            response=requests.put("http://"+host+"/"+resourcePath, headers={"Content-Type": "application / vnd.miele.v1 + json; charset = utf - 8", "Host" : host, "User-Agent": "Miele@mobile 2.3.3 iOS", "Authorization": authHeader, "Date": date, "Accept":acceptHeader},  data=body_encrypted )

#            response=requests.put("http://"+host+"/"+resourcePath, data={"Authorization": authHeader, "Date": "Fri, 25 Jan 2025 19:57:40 GMT", "Accept": "application/vnd.miele.v2+json; charset=utf-8"})
        print(response);
        print(response.headers);

        if (response.status_code == 200):
            decrypted=self.decrypt_response(response);
            return [decrypted, response];
        if (response.status_code == 204):
            return [b"", response];
        return [b"", response];

        raise Exception(f"HTTP Request failed" + str(response))
    def process_response(response):
        if (response.status_code != 200):
            raise Exception(f"Device sent error code: {response}");
        if (full.headers["Content-Type"].find("json") != -1):
            j=json.loads(response);
            return j;
        elif (full.headers["Content-Type"].find("DOP2") != -1):
            raise Exception("DOP2 decoding not ready for prime time");
        else:
            raise Exception(f"Unknown content type returned from device: {full.headers['Content-Type']}");
    def readDop2Node (self, host, deviceRoute, node=""):
#        deviceRoute="000187683192"
        resourcePath=f"Devices/{deviceRoute}/DOP2/{node}"
        [response, r]=self.sendHttpRequest(httpMethod="GET", host=host, resourcePath=resourcePath);
        return json.loads(response);
#        if (node==""): #we are reading root node
#
    def readDop2Recursive (self, host, deviceRoute):
        try:
            rootNode=self.readDop2Node(host, deviceRoute) #get root node
        except Exception as e:
            print(f"Error obtaining DOP2 root node, perhaps device is not exposing DOP2 endpoint.");
            raise MieleRESTException("DOP2 Root Node not found", host);
#        j=json.loads(rootNode);
        dopTreeBinary={}
        dopTree={};
        dopTreeAnnotated={};

        flattened={}
        for x in rootNode:  # visit each child node
            print(f"Exploring child node {x}");
            dopTree[x]={};
            dopTreeBinary[x]={};
            try:
                leaves=self.readDop2Node(host, deviceRoute, node=x); #read all leaves in child node
            except Exception as e:
                dopTree[x]=f"Error reading child node {x}, exception {e}";
                continue;
            print(f"Leaves {leaves} for node {x}");
            dopTreeAnnotated[x]={};

            for leafId in set(leaves):
                print(f"reading leaf {leafId} in node {x}");
                try:
                    dopTree[x][leafId]={};
                    [leafData, leafBytes]=self.readDop2Leaf(host, x, deviceRoute, leafId, 0, 0);

                    dopTreeBinary[x][leafId]=str(binascii.hexlify(leafBytes));
                    print(f"read leaf {leafId} in node {x}:");
                    for fieldId, fieldData in enumerate(leafData):
                        fieldId=fieldId+1 #DOP uses one-based index
                        flattened[f"{x}_{leafId}_{fieldId}"]=str(fieldData);
                        dopTree[x][leafId][fieldId]=fieldData;
#                        dopTree[x][leafId][fieldId]=fieldData;
                    print(f"successfully read {leafId} in node {x}")
                    for annotator in DOP2Annotators:
                        if (annotator.getLeaf()==[int(x), int(leafId)]):
#                           raise Exception(f"found annotator for {leafId}, {fieldId}");
                           try:
                               annotatorInstance=annotator(dopTree[x][leafId]);
                               annotatorInstance.readFields();
                               dopTreeAnnotated[x][str(type(annotatorInstance))]=annotatorInstance
                           except:
                               pass
#                           dopTree[x][str(leafId)+"_annotated"]=str(annotatorInstance)
                    for key, value in dopTree[x][leafId].items():
                        dopTree[x][leafId][key]=str(value);
                except Exception as e:
                    errorStr=f"Error reading node {x}, leaf {leafId}, error {str(e)}";
                    dopTree[x][leafId]=errorStr;
                    flattened[f"{x}_{leafId}"]=errorStr;
        print(dopTree);
#        dopTree=sorted(dopTree.items(), key=lambda kv: str(kv[1]));
        dump=json.dumps(flattened,indent=4);
#        with open("doptree.txt", "w+") as f:
#            f.write(dump);
#        print([x.keys() for x in dopTree.values()]);
        return {"dopTreeAnnotated": dopTreeAnnotated, "dopTreeDecoded": dopTree, "dopTreeBinary":dopTreeBinary};
    def writeDop2Leaf (self, host, deviceRoute, unit, attribute, payload, idx1=0, idx2=0):
        response=self.sendHttpRequest(httpMethod="PUT", host=host, resourcePath=f"Devices/{deviceRoute}/DOP2/{unit}/{attribute}?idx1={idx1}&idx2={idx2}", payload=payload);
        print(f"Sent PUT request to write {unit}/{attribute}, {len(payload)} bytes payload sent, got response {response}");
        return response
    def readDop2Leaf (self, host, node, deviceRoute, leaf, idx1=0, idx2=0):
        parser=MieleAttributeParser();
        fields=[];
        response=self.sendHttpRequest(httpMethod="GET", host=host, resourcePath=f"Devices/{deviceRoute}/DOP2/{node}/{leaf}?idx1={idx1}&idx2={idx2}");
        if (response):
            response=response[0]
        print(response)
        x=("DOP2/{node}/{leaf}");
        if (response):
            return [parser.parseBytes(response), response];

#         if (response):
#             response=response[0]
#             print (f"attempting to decode DOP2, {response}");
#             first_byte=response[1] + (response[0] << 8);
#             parent_attribute_id = (response[2] << 8) + response[3];
#             attribute_id = ( ( response[4] << 8 )* 1 + response[5]);

#             padding_bytes_expected=len(response)-first_byte-2;
#             print(f"reading parent attribute {parent_attribute_id}, attribute {attribute_id}, expecting {first_byte} payload bytes and {padding_bytes_expected} padding bytes");
#             if (padding_bytes_expected > 0):
#                 for x in response[-padding_bytes_expected:]:
#                     if (x!= 0x20):
#                         raise Exception("Error decoding; DOP2 Protocol Violation.")
#             payload=response[8:len(response)-padding_bytes_expected]
#             print("payload:" + str(len(payload)) + " bytes");
#             print(binascii.hexlify(payload, ' ', 1))

#             if (len(payload)==0):
#                 print("empty response, returning");
#                 payload_type=0;
#                 return [];
#             else:
#                 payload_type=(payload[3] << 0) + (payload[4] << 8); # 8 byte header
#             print(f"header indicates number of fields: {payload_type}");
#             numberFields=payload_type;
#             payload_hmm=(payload[5] << 0) ; # 8 byte header
#             print(f"{payload_hmm} data type:");

#             print(response[8:].decode("ascii", errors='ignore'))

#             # first field header starts at byte 6
#             cursor=5;
#             currentField=1;
#             if (payload[cursor]==0x02):
#                 print("field 0x01 suppressed, skipping?!")
#                 currentField=0x02;
#                 payload_type=payload_type+1;
# #            fieldsLeft=payload_type;
#             while (currentField <= payload_type):
#                 fieldHeader=payload[cursor];
#                 if (currentField != fieldHeader):
#                     break;
#                     raise Exception(f"Protocol violation -- fields not sequentially numbered; expected {currentField}, found {fieldHeader}, lastField={fields[-1]}");
#                 cursor=cursor+1;
#                 fieldType = payload[cursor];
#                 print (f"Field numbering correct. Decoding field {currentField}, type={fieldType}");
#                 match fieldType:
#                     case 21:
#                         print("skip 13")
#                         cursor=cursor+14;
#                     case 16:
#                         byte0=payload[cursor+1]
#                         byte1=payload[cursor+2]
#                         byte2=payload[cursor+3]
#                         startPacket=cursor;
#                         cursor=cursor+4;
#                         myCounter=1;
#                         counter=payload[cursor] # get counter element from wire
#                         elementLength=0;
#                         match byte1:
#                             case 0x06:
#                                 elementLength=3;
#                             case 0x03:
#                                 elementLength=2;
#                             case _:
#                                 elementLength=0
#                                 break;
#                         elementLength=elementLength+2;
#                         while (counter == myCounter):
#                             print(f"decoding {counter} array entry, elementLEngth={elementLength}");
#                             cursor=cursor+elementLength;
#                             counter=payload[cursor];
#                             myCounter=myCounter+1;
#                         info=f"{[byte0,byte1,byte2]} unknown array, packet counter={counter}, myCounter={myCounter}, element type {byte1}, detected element length {elementLength}, currentField={currentField}, totalFields={numberFields}, payload left={len(payload[startPacket:])}, contentLeft={binascii.hexlify(payload[startPacket:], sep=' ', bytes_per_sep=2)}";
#                         raise Exception(f"unknown array -- {info}");
#                     case 0x02:
#                         print("2-byte mystery")
#                         fields.append([fieldType, payload[cursor+1:cursor+2] ])
#                         cursor=cursor+3;
#                     case 0x07: #only 0x01 and 0x00 seen here
#                         print ("3-byte flags");
#                         fields.append([fieldType, payload[cursor+1:cursor+4]])
#                         cursor=cursor+4;
#                     case 0x03:
#                         print ("2-byte mystery");
#                         fields.append([fieldType, payload[cursor+1:cursor+3]])

#                         cursor=cursor+3;
#                     case 0x04:
#                         fields.append([fieldType, payload[cursor+1:cursor+3] ])
#                         cursor=cursor+3;
#                         print(f"3-byte mystery ")
#                     case 0x05:
#                         #contentBytes=payload[cursor+2] & 0xF;
#                         #elementLength=payload[cursor+5]
#                         fields.append([fieldType, payload[cursor+1:cursor+4] ])
#                         print("4 byte mystery?")
#                         #print(f"variable length mystery {contentBytes} elements/content bytes (could be strings), {elementLength} bytes per element");
#                         cursor=cursor+ 4; #3 byte mystery?
#                     case 0x06:
#                         print ("3-byte mystery");
#                         fields.append([fieldType, payload[cursor+1:cursor+3]])

#                         cursor=cursor+4;

#                     case 0x08:
#                         print(f"5-byte mystery");
#                         fields.append([fieldType, payload[cursor+1:cursor+6]])

#                         cursor=cursor+ 6;

#                     case 0x12: # confirmed to be a string
#                         print (f"String field");
#                         try:
#                             field=Dop2StringField(currentField, payload, cursor);
#                             cursor=cursor+field.wireLength
#                             print(field);
#                             print(payload[cursor:])
#                             fields.append([fieldType, field])
#                         except Exception as e:
#                             print(f"Error generating string field: " + str(e));
# #                            continue;
#                         print (f"String field: {field}");
#                     case 0x19:
#                         byte0=payload[cursor+1]
#                         byte1=payload[cursor+2]
#                         print(f"4-byte (dynamic length?) mystery {byte0}, {byte1}");
#                         payloadLength=(byte1*4)+4;
#                         fields.append([fieldType,payload[cursor+3:cursor+payloadLength]]);
#                         cursor=cursor+payloadLength;
# #                    case 23:
# #                        print(f"epic array -- payload length {byte1}");
# #                        byte0=payload[cursor+1]
# #                        byte1=payload[cursor+2]
# #                        payloadLength=byte1;

# #                        fields.append([fieldType, payload[cursor+3:cursor+3+payloadLength]]);
# #                        cursor=cursor+byte1+3
#                     case 0x09:
#                         print(f"4-byte mystery");
#                         fields.append([fieldType, payload[cursor+1:cursor+5]])

#                         cursor=cursor+4;
#                     case 0x01:
#                         arrayLength=(payload[cursor+1]<<0)
#                         cursor=cursor+1;
#                         arrayData=payload[cursor:cursor+arrayLength]
#                         print(f"array length {arrayLength}, data={binascii.hexlify(arrayData)}");
#                         cursor=cursor+arrayLength + 1 + (arrayLength==0) * 1;
#                         fields.append(arrayData)

#                     case 0x0b:
#                         print(f"9-byte mystery");
#                         fields.append(payload[cursor+1:cursor+9])

#                         cursor=cursor+10;
#                     case 0x20:
#                         fields.append(payload[cursor+1:cursor+5])

#                         print("4 byte mystery") # Devices/000187683192/DOP2/1/17
#                         cursor=cursor+5;
#                     case 0x21:
#                         field=Dop2StringField(currentField, payload, cursor);
#                         print("string array?") # Devices/000187683192/DOP2/1/17
#                         cursor=cursor+field.wireLength
#                         print(field);
#                         fields.append(field);
#                     case _:
#                         print("unknown");
#                         raise Exception(f"Unknown field type {fieldType} encountered in field {currentField}, {binascii.hexlify(payload[cursor:], sep=' ', bytes_per_sep=2)}, total fields {numberFields}")
#                 currentField=currentField+1;
        return fields;

if __name__ == '__main__':
#    print(MieleProvisioningInfo.generate_random());
    p=MieleProvisioningInfo("123456789ABCDEFE","123456789ABCDEFE123456789ABCDEFE123456789ABCDEFE123456789ABCDEFE123456789ABCDEFE123456789ABCDEFE123456789ABCDEFE123456789ABCDEFE");
    c=MieleCryptoProvider(p);
#    r=c.readDop2Leaf(sys.argv[1], "2", "15");
    r=c.readDop2Recursive(sys.argv[1]);
    exit(0);


    if (len(sys.argv)>3):
        method="PUT";
        payload=sys.argv[3];
    else:
        payload=""
        method="GET";

    response, full=c.sendHttpRequest(httpMethod=method, host=sys.argv[1], resourcePath=sys.argv[2], payload=payload);
    if (response != None):
        try:
            if (full.headers["Content-Type"].find("json")!=-1):
                j=json.loads(response);6
                print(json.dumps(j,indent=2));
            else:
                raise Exception("Exception");
        except:
            print(f"failed to decode as json, len={len(response)}");
            print(binascii.hexlify(response, ' ', 1));
            print ("attempting to decode DOP2");
            print(r);
            first_byte=response[1] + (response[0] << 8);
            parent_attribute_id = (response[2] << 8) + response[3];
            attribute_id = ( ( response[4] << 8 )* 1 + response[5]);

            padding_bytes_expected=len(response)-first_byte-2;
            print(f"reading parent attribute {parent_attribute_id}, attribute {attribute_id}, expecting {first_byte} payload bytes and {padding_bytes_expected} padding bytes");
            if (padding_bytes_expected > 0):
                for x in response[-padding_bytes_expected:]:
                    if (x!= 0x20):
                        raise Exception("Error decoding; DOP2 Protocol Violation.")
            payload=response[8:len(response)-padding_bytes_expected]
            print("payload:" + str(len(payload)) + " bytes");
            print(binascii.hexlify(payload, ' ', 1))

            if (len(payload)==0):
                print("empty response, returning");
                payload_type=0;
            else:
                payload_type=(payload[3] << 0) + (payload[4] << 8); # 8 byte header
            print(f"header indicates number of fields: {payload_type}");

            payload_hmm=(payload[5] << 0) ; # 8 byte header
            print(f"{payload_hmm} data type:");

            print(response[8:].decode("ascii", errors='ignore'))

            # first field header starts at byte 6
            cursor=5;
            currentField=1;
            if (payload[cursor]==0x02):
                print("field 0x01 suppressed, skipping?!")
                currentField=0x02;
                payload_type=payload_type+1;
#            fieldsLeft=payload_type;
            while (currentField <= payload_type):
                fieldHeader=payload[cursor];
                if (currentField != fieldHeader):
                    raise Exception(f"Protocol violation -- fields not sequentially numbered; expected {currentField}, found {fieldHeader}");
                cursor=cursor+1;
                fieldType = payload[cursor];
                print (f"Field numbering correct. Decoding field {currentField}, type={fieldType}");
                match fieldType:
                    case 0x02:
                        print("2-byte mystery")
                        cursor=cursor+3;
                    case 0x07:
                        print ("3-byte mystery");
                        cursor=cursor+4;
                    case 0x03:
                        print ("2-byte mystery");
                        cursor=cursor+3;
                    case 0x04:
                        elements=0;
#                        while (payload[cursor+elements]!= 0x00):
#                            elements=elements+1;
#                        variable=payload[cursor+1]
                        cursor=cursor+3;
#                        print(f"variable-length mystery (2-byte array?), elements={elements}")
                    case 0x05:
                        #contentBytes=payload[cursor+2] & 0xF;
                        #elementLength=payload[cursor+5]
                        print("4 byte mystery?")
                        #print(f"variable length mystery {contentBytes} elements/content bytes (could be strings), {elementLength} bytes per element");
                        cursor=cursor+ 4; #3 byte mystery?
                    case 0x08:
                        print(f"5-byte mystery");
                        cursor=cursor+ 6;
                    case 23:
                        print(f"64-byte array?");
                        cursor=cursor+64;
                    case 0x12:
                        cursor=cursor+2;
                        stringData=payload[cursor+1:cursor+stringLength]
                        print(f"string length {stringLength}, data={stringData}");
                        cursor=cursor+len(stringData)+1;
                        if (cursor < len(payload) and payload[cursor]==0x00):
                            cursor=cursor+1;
                    case 0x19:
                        byte0=payload[cursor+1]
                        byte1=payload[cursor+2]
                        print(f"4-byte (dynamic length?) mystery {byte0}, {byte1}");
                        cursor=cursor+(byte1*4)+4;
                    case 0x09:
                        print(f"4-byte mystery");
                        cursor=cursor+4;
                    case 0x01:
                        arrayLength=(payload[cursor+1]<<0)
                        cursor=cursor+1;
                        arrayData=payload[cursor:cursor+arrayLength]
                        print(f"array length {arrayLength}, data={binascii.hexlify(arrayData)}");
                        cursor=cursor+arrayLength + 1 + (arrayLength==0) * 1;
                    case 0x0b:
                        print(f"9-byte mystery");
                        cursor=cursor+10;
                    case 0x20:
                        print("4 byte mystery") # Devices/000187683192/DOP2/1/17
                        cursor=cursor+5;
                    case 0x21:
                        print("string array?") # Devices/000187683192/DOP2/1/17

                    case _:
                        print("unknown");
                        break;
                currentField=currentField+1;
#        elapsed=tuple_to_min(j["ElapsedTime"])
#        remaining=tuple_to_min(j["RemainingTime"])
#        total = elapsed + remaining;
#        if (total > 0.1):
#            progress=elapsed/(elapsed+remaining);
#            print(f"Progress: {100*progress:.2f}%");


