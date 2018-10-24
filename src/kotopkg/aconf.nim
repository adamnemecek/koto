import portaudio

type AudioConf* = ref object
  sampleRate*: float32
  framesPerBuffer*: int
  procPaBuffer*: TStreamCallback
