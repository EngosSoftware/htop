#!/usr/bin/env bash

#******************************************************************************
#
# Header HTML template.
#
#******************************************************************************

htop -b --header="<div style='width:100%; font-size:12pt; font-weight:bold; text-align:center; border: 3px solid red;'>CUSTOM HEADER</div>" --print-header-footer --margin='2.5cm 0 0 0' single H_000010.html actual.1 >> /dev/null 2>&1