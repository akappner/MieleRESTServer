#!/bin/bash
MIELEIP="$1"
TARGETFILE="$2"

echo "Trying HTTP"
wget --tries=2 --timeout=3 --connect-timeout=3 -O - --method=PUT --body-file="$TARGETFILE" http://$MIELEIP/Security/Commissioning
echo "Trying HTTPS"
wget  --header="Authorization: MielePairing:Pairing" --tries 2 --timeout=3 --no-check-certificate -O - --method=PUT --body-file="$TARGETFILE" https://$MIELEIP/Security/Commissioning
