import math as m

import portaudio as pa

import kotopkg.main
import kotopkg.ug

type TestSin = ref object of UnitGenerator
  angle: float32

proc calc(ug: UnitGenerator): float32 =
  let v = m.sin(TestSin(ug).angle)
  TestSin(ug).angle += 0.08
  return v

let testsin = TestSin(angle: 0, calc: calc)

type TStereo = tuple[left, right: float32]

# fundamental proc and it should be replaced
proc procBuffer(inBuf, outBuf: pointer,
                framesPerBuf: culong,
                timeInfo: ptr pa.TStreamCallbackTimeInfo,
                stateusFlags: pa.TStreamCallbackFlags,
                userData: pointer): cint {.cdecl.} =
  var outBuf = cast[ptr array[int, TStereo]](outBuf)
  for i in 0..<(1024):
    let v = testsin.calc(testsin)
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
