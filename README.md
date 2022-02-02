# pront

`pront` is a project with the goal of creating an affordable printer/plotter that supports standard protocols and is inexpensive to operate.

## Working draft for order of operations

1. Printing a document via IPP
   1. `prontd-ippd` - Runs the IPP server. => `http://svgd/pdf`
   2. `prontd-svgd` - Converts the document's pages to SVG. => `http://pland/svg`
   3. `prontd-pland` - Converts the SVG to pen plotter instructions. => `http://gcd/`
   4. `prontd-gcd` - Generates GCode for the pen plotter instructions it receives. => `http://pend/`
   5. `prontd-pend` - Interprets the GCode and runs the motors accordingly. => GPIO
2. Printing a document via Telnet/TCP
   1. `prontd-ttyd` - Runs the TCP socket, wraps text. => `http://pland/txt`
   2. `prontd-pland` - Converts the input text to pen plotter instructions. => `http://gcd/`
   3. `prontd-gcd` - Generates GCode for the pen plotter instructions it receives. => `http://pend/`
   4. `prontd-pend` - Interprets the GCode and runs the motors accordingly. => GPIO

## Things that actually exist right now

* `prontd-ttyd`