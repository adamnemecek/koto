import portaudio


type MasterPlayer* = ref object
  sampleRate*: float32
  framesPerBuffer*: int
  procPaBuffer*: TStreamCallback

  tempo*: float32
  tick*: int64
  time*: float32
  beat*: float32

proc procMasterPlayer*(mp: MasterPlayer) =
  mp.tick += 1
  mp.time += 1.0 / mp.sampleRate
  mp.beat += mp.tempo / mp.sampleRate
