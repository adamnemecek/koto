import portaudio as pa

import kotopkg.main


# fundamental proc and it should be replaced
proc procBuffer(inBuf, outBuf: pointer,
                framesPerBuf: culong,
                timeInfo: ptr pa.TStreamCallbackTimeInfo,
                stateusFlags: pa.TStreamCallbackFlags,
                userData: pointer): cint {.cdecl.} =
  return 0


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
