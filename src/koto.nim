import math as m

import portaudio as pa

import kotopkg.player
import kotopkg.main
import kotopkg.ug as ug


let
  mp = MasterPlayer(
    sampleRate: 44100, framesPerBuffer: 1024,
    tempo: 120.0'f32, tick: 0, time: 0.0'f32, beat: 0.0'f32)
  testtone = ug.TestTone(mp: mp, gain: 0.4, angle: 0.0, sources: @[])
  wnoise = ug.WhiteNoise(mp: mp, gain: 0.3, sources: @[])
  wn = ug.WhiteNoise(mp: mp, gain: 0.0, sources: @[wnoise, testtone])

type TStereo = tuple[left, right: float32]

# fundamental proc and it should be replaced
proc procBuffer(inBuf, outBuf: pointer,
                framesPerBuf: culong,
                timeInfo: ptr pa.TStreamCallbackTimeInfo,
                stateusFlags: pa.TStreamCallbackFlags,
                userData: pointer): cint {.cdecl.} =
  var outBuf = cast[ptr array[int, TStereo]](outBuf)
  for i in 0..<(1024):
    let v = ug.gen(wnoise)
    outBuf[i] = (v, v)
    procMasterPlayer(mp)

mp.procPaBuffer = procBuffer

when isMainModule:
  echo "hi, Koto!"

  init()
  var s = start(mp)
  try:
    while true:
      pa.Sleep(1)

  except KeyboardInterruptError:
    echo "Keyboard Interrupted"
    term(s)
    quit 0

  term(s)
