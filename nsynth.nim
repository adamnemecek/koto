from os import commandLineParams

import sndfile


let args = commandLineParams()

if args.len == 1:
  var
    info: TINFO
    sndfile: ptr TSNDFILE

  sndfile = open(args[0], READ, cast[ptr TINFO](info.addr))
  echo repr(info)
