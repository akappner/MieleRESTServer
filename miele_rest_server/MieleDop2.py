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

from enum import Enum
import binascii
import struct
class MieleIntegerFormat(str, Enum):
    Unsigned="U",
    Signed="S",
    E="E"

class MieleAttribute:
    def to_dict(self):
        return {'wireLength' : self.wireLength, 'typeName': self.typeName, 'value':self.value}
    def __init__ (self, wireLength, typeName, value):
        self.wireLength = wireLength;
        self.typeName = typeName;
        self.value = value;
    def __str__ (self):
        if (isinstance(self.value, list)):
            valueStr=[str(x) for x in self.value];
        else:
            valueStr=str(self.value);
        return f"{self.typeName}={str(valueStr)}";

class EmptyMieleAttribute (MieleAttribute):
    def __init__(self):
        super().__init__(0, "empty", "");
class MieleAttributeDecoder:
    def __init__ (self, byteLength, typeName):
        self.byteLength=byteLength;
        self.typeName=typeName;
    def __str__ (self):
        return self.typeName;
class MieleFixedLengthAttributeDecoder (MieleAttributeDecoder):
    def __init__ (self, byteLength, typeName):
        super().__init__(byteLength, typeName);
    
    def decode (self, payload):
        return MieleAttribute (self.byteLength, 
        self.typeName,
        self.fixed_length_decode (payload[0:self.byteLength]));

class MieleVariableLengthAttributeDecoder (MieleAttributeDecoder):
    def __init__ (self, typeName):
        super().__init__(0, typeName);
    def decode (self, remainingPayload):
        [wireLength, values]=self.variableLengthDecode(remainingPayload);
        return MieleAttribute(wireLength, self.typeName, values);

class MieleAStruct(MieleVariableLengthAttributeDecoder):
    def __init__ (self):
        super().__init__("A_struct[]")
    def variableLengthDecode (self, remainingPayload):
        numberOfFields=(remainingPayload[0]<<8) + remainingPayload[1]; # 2 byte length field
        remainingPayload=remainingPayload[2:];
        if (numberOfFields==0):
            raise Exception("Cannot have empty struct");
        fields=[];
        totalWireLength=5;
        while (len(fields) < numberOfFields):
            id=(remainingPayload[0]<<8) + remainingPayload[1]; # 2 byte length field
            currentNumber=(remainingPayload[2]<<8) + remainingPayload[3]; # 2 byte length field
            dataType=remainingPayload[4];
#            if (dataType != 0x0a):
#                raise Exception(f"{dataType} found, should be 0x0a for struct[]");
            remainingPayload=remainingPayload[5:]
            currentStruct=MieleAttributeParser().parseBytes(remainingPayload);
            #totalWireLength=sum([x.wireLength for x in currentStruct]);
#            remainingPayload=remainingPayload[currentStruct.wireLength:];
            #totalWireLength=totalWireLength+currentStruct.wireLength ;
            fields.append(currentStruct);
            break;
        return [totalWireLength, fields]


class MieleBool(MieleFixedLengthAttributeDecoder):
    def __init__ (self):
        super().__init__(1, "bool");

    def fixed_length_decode (self, payload):
        payload=int.from_bytes(payload);
        if (payload == 0x01):
            return True;
        if (payload==0x00):
            return False;
        raise Exception(f"DOP2 Decode Error -- Boolean not 0x01 or 0x00 but {payload}");

class MieleInteger (MieleFixedLengthAttributeDecoder):
    def __init__ (self, byteLength, integerFormat):
        super().__init__(byteLength, f"{integerFormat}{byteLength*8}")
    def fixed_length_decode (self, data):
        return int.from_bytes(data);
#         return struct.unpack('<i', data);
#        return int(data[0]);
class MieleFloat (MieleFixedLengthAttributeDecoder):
    def __init__ (self, byteLength):
        super().__init__(byteLength, f"float{byteLength*8}")
    def fixed_length_decode (self, data):
         return struct.unpack('<f', data);
class MieleString (MieleVariableLengthAttributeDecoder):
    def __init__ (self):
        super().__init__("string");
    def variableLengthDecode (self, remainingPayload):
        headerLength=2;
        stringLength=(remainingPayload[0]<<8) + remainingPayload[1]; # 2 byte length field
        wireLength=headerLength+stringLength;
#        return [wireLength, remainingPayload[headerLength:wireLength].decode(encoding="utf-8", errors='ignore')]
        return [wireLength, bytes(remainingPayload[headerLength:wireLength]) ]
class MieleArray (MieleVariableLengthAttributeDecoder):
    def __init__ (self, elementDecoder):
        self.elementDecoder=elementDecoder;
        super().__init__(elementDecoder.typeName+"[]");
    def decode (self, data):
        numberElements=(data[0]<<8) + data[1]; # 2 byte element number field
        elements=[];
        elementWireLength = 0;
        while (len(elements) < numberElements):
            try:
                newElement=self.elementDecoder.decode(data[2:]);
            except Exception as e:
                raise Exception(f"Exception when decoding array element {len(elements)+1} of {numberElements}, elements so far {[str(x) for x in elements]}: {e}, elementWireLength={elementWireLength}, data={binascii.hexlify(data, '-')}")
            data=data[2+newElement.wireLength:];
            elementWireLength = elementWireLength + newElement.wireLength;
            elements.append(newElement);
        totalWireLength = elementWireLength + 2; #1 header + elements + 0x00 trailing byte?
        return MieleAttribute(totalWireLength, self.typeName, elements);

class MieleStruct(MieleAttributeDecoder):
    def __init__(self):
        super().__init__(0, "struct");
    def decode (self, data):
        hex=binascii.hexlify(data, " ");
        decoder=MieleAttributeParser();
        fields=[];
        fieldLength=0;
        headerLength=3; 
        [byte0, numberOfFields,byte2]=data[0:3];
        if (numberOfFields == 0 or numberOfFields > 160):
            raise Exception(f"Empty or implausibly large struct, {numberOfFields} number of fields");
        data=data[3:]
        while True:
            print(f"decoding struct field {data[0]}");
            fieldId=data[0];
            dataType=data[1];
#            if (fieldId != len(fields) + 1): ##removed this -- fields are NOT necessarily sequential
#                if (dataType==len(fields)+1): #mysterious padding sometimes shows up
#                    data=data[1:];
 #                   fieldLength=fieldLength+1;
 #                   continue;
 #               raise Exception(f"Struct Fields not sequentially numbered, expecting {len(fields)+1} but got {fieldId}")
            try:
                field=decoder.parseField(dataType, data[2:])
            except Exception as e:
                raise Exception(f"Struct decoding exception {e}, field id {fieldId}, last data type={dataType}, fields so far {[str(x) for x in fields]}");
                return MieleAttribute(headerLength + fieldLength + 1, f"incomplete struct, error {e}, {len(data)} bytes in, hex {hex}, last data type {data[1]}, last field id {fieldId}", fields);
            currentFieldLength = field.wireLength + 2;
            fieldLength=fieldLength + currentFieldLength;
            fields.append(field);
            if (len(fields) == numberOfFields):
                break;
            data=data[currentFieldLength:];
            if (data[0]==0x00): #padding
                data=data[1:];
                fieldLength=fieldLength+1; 

        totalWireLength = fieldLength + 3 ;
        return MieleAttribute(totalWireLength, "struct", fields);
class Dop2Payload:
    def __init__ (unit, node, fields):
        self.unit=unit;
        self.node=node,
        self.fields=fields;

class MieleAttributeParser():
    def registerDecoders(self):
        self.decoders={
            1: MieleBool(),
            2: MieleInteger(1, MieleIntegerFormat.Unsigned),
            3: MieleInteger(1, MieleIntegerFormat.Signed),
            4: MieleInteger(1, MieleIntegerFormat.E), ## 1-byte values end here
            5: MieleInteger(2, MieleIntegerFormat.Unsigned),
            6: MieleInteger(2, MieleIntegerFormat.Signed),
            7: MieleInteger(2, MieleIntegerFormat.E),
            8: MieleInteger(4, MieleIntegerFormat.Unsigned),
            9: MieleInteger(4, MieleIntegerFormat.Signed),
            10: MieleInteger(4, MieleIntegerFormat.E),
            11: MieleInteger(8, MieleIntegerFormat.Unsigned),
            12: MieleInteger(8, MieleIntegerFormat.Signed),
            13: MieleInteger(8, MieleIntegerFormat.E),
            14: MieleFloat(4),
            15: MieleFloat(8),
            16: MieleStruct (),
            17: MieleArray (MieleBool()),
            18: MieleString(), #this is really an U8[]
#            18: MieleArray (MieleInteger(1, MieleIntegerFormat.Unsigned)),
            20: MieleArray (MieleInteger(1, MieleIntegerFormat.E)),
            21: MieleArray (MieleInteger(2, MieleIntegerFormat.Unsigned)),
            22: MieleArray (MieleInteger(2, MieleIntegerFormat.Signed)),
            23: MieleArray (MieleInteger(2, MieleIntegerFormat.E)),
            25: MieleArray (MieleInteger(4, MieleIntegerFormat.Signed)),
            27: MieleArray (MieleInteger(8, MieleIntegerFormat.Unsigned)),
            32: MieleString(),
            33: MieleAStruct(),
            } 
    def __init__ (self):
        self.registerDecoders();
    def __str__(self):
        return f"MieleAttributeParser, decoders={[str(x) for x in self.decoders.values()]}";
    def parseField (self, fieldType, data):
        decoder=self.decoders[fieldType];
        fieldValue = decoder.decode (data);
        return fieldValue;

    def parseBytes (self, response):
        hex=binascii.hexlify(response, " ");
        payloadLength=response[1] + (response[0] << 8);
        unitId = (response[2] << 8) + response[3];
        attributeId = ( ( response[4] << 8 )* 1 + response[5]);
        padding_bytes_expected=len(response)-payloadLength-2;
        ## todo: add check that padding is 0x20 only

        payload=response[8:len(response)-padding_bytes_expected]

        if (len(payload)==0):
            print("empty response, returning");
            return [];
#            return Dop2Payload (unitId, attributeId, []);
        
        numberOfFields=(payload[3]) + (payload[4] << 8);
        print(f"Decoding {unitId}/{attributeId}, {payloadLength} stated length, {numberOfFields} root fields");

        fields=[];
        remainingPayload=payload[5:];
        fieldNumberingCorrection=0;
        try:
            while (True):
                if (len(fields) + 1 != remainingPayload[0] - fieldNumberingCorrection): #field index does not match
                     if (len(fields)+1 > remainingPayload[0]):
                         raise Exception("field index regressed");
                     else:
                         while ( (len(fields)+1) < remainingPayload[0]):
                             fields.append(EmptyMieleAttribute());
#                    if (fieldNumberingCorrection==0 and remainingPayload[0]==len(fields)+2):
#                        fieldNumberingCorrection=1;
#                    else:
#                        raise Exception(f"incorrect field numbering {hex}, expected total fields {numberOfFields} but field number {len(fields)++1} is labelled as field number {remainingPayload[0]}")
                fieldType = remainingPayload[1];
                decoder=self.decoders[fieldType];
                try:
                    fieldValue = decoder.decode (remainingPayload[2:]);
                except Exception as e:
                    raise Exception(f"Error while decodign with decoder {decoder}: {e}")
                if (fieldValue.wireLength==0):
                    raise Exception("zero-length field not possible");
                fields.append(fieldValue);
                if (len(fields)==numberOfFields - fieldNumberingCorrection):
                    break;
                remainingPayload=remainingPayload[3+fieldValue.wireLength:];
        except Exception as e:
            print("error during parsing, returning incomplete parse result");
            fields.append(f"short stop, inner exception {e}, hex={hex} bytes={len(hex)}, {numberOfFields-len(fields)} left in header (numbering correction={fieldNumberingCorrection}, padding bytes expected {padding_bytes_expected}, fields expected {numberOfFields}, remaining payload {binascii.hexlify(remainingPayload)}");
        return fields;
            
def hexdecode_main():
    import sys
    from MieleDop2Structures import DOP2Annotators
    node=int(sys.argv[1]);
    leaf=int(sys.argv[2]);
    hex=binascii.unhexlify(sys.argv[3]);
    print(f"MieleDop2.py called directly, decoding {len(hex)} DOP2 hex bytes")
    
    parser=MieleAttributeParser();
    currentStruct=parser.parseBytes(hex);
    print([str(x) for x in currentStruct])
    i=None;
    for annotator in DOP2Annotators:
        if (annotator.getLeaf()==[int(node), int(leaf)]):
            print(f"Decoding packet as {annotator}");

            i=annotator(currentStruct);
            try:
                i.readFields();
            except:
                pass;
    print(i);
#             print(annotator);

#    print(str(currentStruct));
    
if __name__ == "__main__":
    hexdecode_main();




