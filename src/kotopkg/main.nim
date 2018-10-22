import portaudio as pa


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

proc startPA(procPaBuffer: proc): PStream =
  var stream: PStream

  discard pa.OpenDefaultStream(
    cast[PStream](stream.addr),
    numInputChannels = 0,
    numOutputChannels = 2,
    sampleFormat = pa.TSampleFormat.sfFloat32,
    sampleRate = cdouble(44100),
    framesPerBuffer = culong(1024),
    streamCallback = procPaBuffer,
    userData = nil)

  discard pa.StartStream(stream)
  return stream


proc init*(): void =
  echo "----initializing----"
  initPA()

proc start*(procPaBuffer: proc): PStream =
  return startPA(procPaBuffer)

proc term*(stream: PStream): void =
  echo "----terminating----"
  termPA(stream)
