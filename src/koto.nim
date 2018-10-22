import portaudio as pa

import kotopkg.main


type TStereo = tuple[left, right: float32]

# fundamental proc and it should be replaced
proc procBuffer(inBuf, outBuf: pointer,
                framesPerBuf: culong,
                timeInfo: ptr pa.TStreamCallbackTimeInfo,
                stateusFlags: pa.TStreamCallbackFlags,
                userData: pointer): cint {.cdecl.} =
  var outBuf = cast[ptr array[int, TStereo]](outBuf)
  for i in 0..<(1024):
    outBuf[i] = (0'f32, 0'f32)


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
