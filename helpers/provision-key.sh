#!/bin/bash
MIELEIP="$1"
TARGETFILE="$2"

DATE=$(date -u '+%a, %d %b %Y %H:%M:%S GMT')

echo $DATE
echo "Trying HTTP"
wget --header "Date: $DATE" --header="Authorization: MielePairing:Pairing" --tries=2 --timeout=3 --connect-timeout=3 -O - --method=PUT --body-file="$TARGETFILE" http://$MIELEIP/Security/Commissioning
echo "Trying HTTPS"
wget  --header "Date: $DATE" --header="Authorization: MielePairing:Pairing" --tries 2 --timeout=3 --no-check-certificate -O - --method=PUT --body-file="$TARGETFILE" https://$MIELEIP/Security/Commissioning
