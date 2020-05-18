aprs-tools
==========
command line utilities to generate aprs messages written in rust

Usage
-----

aprs-location : generates a location aprs message
aprs-message : generate an aprs message 

aprs-encode : encode an aprs message into a wave file

Examples
--------
```
aprs-message -d BLN1BALON hello | aprs-encode -s 4x6ub-11
```
will generate a Bulletin with text "hello" and then encode it as sent from my balloon

