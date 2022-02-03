# pront

`pront` is a project with the goal of creating an affordable printer/plotter that supports standard protocols and is inexpensive to operate.

## Working draft for order of operations

1. Printing a document via IPP
   1. `prontd-ippd` - Runs the IPP server. => `http://svgd/pdf`
   2. `prontd-svgd` - Converts the document's pages to SVG, and writes the embedded fonts to the font cache if they exist. => `http://pland/svg`, `/fonts`
   3. `prontd-pland` - Converts the SVG to GCode, using cached fonts. => `tcp://pend:23`
   4. `prontd-pend` - Interprets the GCode and runs the motors accordingly. => GPIO
2. Printing a document via Telnet/TCP
   1. `prontd-ttyd` - Runs the TCP socket, wraps text. => `http://pland/txt`
   2. `prontd-pland` - Converts the SVG to GCode. => `tcp://pend:23`
   3. `prontd-pend` - Interprets the GCode and runs the motors accordingly. => GPIO

## Things that actually exist right now

* `prontd-ttyd`