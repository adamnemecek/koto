import math as m

import portaudio as pa

import kotopkg.aconf as conf
import kotopkg.main
import kotopkg.ug as ug


let
  aconf = conf.AudioConf(sampleRate: 44100)
  testtone = ug.TestTone(aconf: aconf, angle: 0.0)
  wn = ug.WhiteNoise(aconf: aconf)


type TStereo = tuple[left, right: float32]

# fundamental proc and it should be replaced
proc procBuffer(inBuf, outBuf: pointer,
                framesPerBuf: culong,
                timeInfo: ptr pa.TStreamCallbackTimeInfo,
                stateusFlags: pa.TStreamCallbackFlags,
                userData: pointer): cint {.cdecl.} =
  var outBuf = cast[ptr array[int, TStereo]](outBuf)
  for i in 0..<(1024):
    let v = ug.gen(wn)
    outBuf[i] = (v, v)


when isMainModule:
  echo "hi, Koto!"

  init()
  var s = start(procBuffer)
  try:
    while true:
      pa.Sleep(1)

  except KeyboardInterruptError:
    echo "Keyboard Interrupted"
    term(s)
    quit 0

  term(s)
