#!/bin/bash

URL="https://schema.mau.se/setup/jsp/SchemaICAL.ics?startDatum=idag&intervallTyp=m&intervallAntal=6&moment=philip&sokMedAND=true&sprak=SV&resurser="
ICS_FILE="calendar.ics"

# Fetch the iCal file
curl -o $ICS_FILE $URL
