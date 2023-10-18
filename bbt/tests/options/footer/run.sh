#!/usr/bin/env bash

#******************************************************************************
#
# Footer HTML template.
#
#******************************************************************************

htop -b --footer="<div style='width:100%; font-size:12pt; font-weight:bold; text-align:center; border: 3px solid green;'>CUSTOM FOOTER</div>" --print-header-footer --margin='0 0 2.5cm 0' single H_000010.html actual.1 >> /dev/null 2>&1
