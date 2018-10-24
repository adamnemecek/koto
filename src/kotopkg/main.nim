import portaudio as pa

import aconf


type KeyboardInterruptError* = object of Exception
proc handleError() {.noconv.} =
  raise newException(KeyboardInterruptError, "Keyboard Interrupt")

setControlCHook(handleError)


proc initPA(): void =
  discard pa.Initialize()

proc termPA(stream: PStream): void =
  discard pa.StopStream(stream)
  discard pa.CloseStream(stream)
  discard pa.Terminate()

proc startPA(aconf: AudioConf): PStream =
  var stream: PStream

  discard pa.OpenDefaultStream(
    cast[PStream](stream.addr),
    numInputChannels = 0,
    numOutputChannels = 2,
    sampleFormat = pa.TSampleFormat.sfFloat32,
    sampleRate = cdouble(aconf.sampleRate),
    framesPerBuffer = culong(aconf.framesPerBuffer),
    streamCallback = aconf.procPaBuffer,
    userData = nil)

  discard pa.StartStream(stream)
  return stream


proc init*(): void =
  echo "----initializing----"
  initPA()

proc start*(aconf: AudioConf): PStream =
  return startPA(aconf)

proc term*(stream: PStream): void =
  echo "----terminating----"
  termPA(stream)
