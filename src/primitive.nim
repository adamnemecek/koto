from os import commandLineParams
import math as m

import vorbis/vorbisfile as VF
import portaudio as PA

import primitivepkg.oscillators.wave_table as wt
import primitivepkg.envelope as eg
import primitivepkg.utils

type
  SoundOut* = ref object
    channelNum*: int
    sampleFormat*: PA.TSampleFormat
    sampleRate*: float64
    bufferSize*: uint64

const seq_step = 16
type
  StepSequencer* = ref object
    tempo*: float64
    sequence*: string
    noteDuration*: int
    osc*: wt.WaveTableOcillator

    beat*: float64
    time*: float64
    env*: eg.Envelope

type
  TSound* = tuple[sndout: SoundOut, seq: StepSequencer]
  TStereo* = tuple[left, right: float32]

proc playWithPA(s: string) =
  echo "============== initialize pa  ==============="
  echo repr(PA.Initialize())

  proc fillingWithTable(inBuf, outBuf: pointer,
                        framesPerBuf: culong,
                        timeInfo: ptr TStreamCallbackTimeInfo,
                        stateusFlags: TStreamCallbackFlags,
                        userData: pointer): cint {.cdecl.} =
    let
      outBuf = cast[ptr array[int, TStereo]](outBuf)
      snd = cast[ptr TSound](userData)
      osc = snd.seq.osc
      freq = 440'f
      timeDelta = 1 / snd.sndout.sampleRate

    echo $(snd.seq.env.state) & ". " & $(snd.seq.time) & ", " & $(snd.seq.beat)
    for i in 0..<int(snd.sndout.bufferSize):
      let
        oscVal = oscillate(osc, freq, snd.sndout.sampleRate)
        envelope = generateEnvelope(snd.seq.env, snd.seq.time)
        val = oscVal * envelope
      outBuf[i] = (val, val)

      let before_beat = snd.seq.beat
      snd.seq.time = snd.seq.time + timeDelta
      snd.seq.beat = snd.seq.beat + timeDelta * snd.seq.tempo / 60

      # TODO: factor out

      # note on
      if m.floor(snd.seq.beat) - m.floor(before_beat) == 1:
        eg.noteOn(snd.seq.env, snd.seq.time)

  var
    stream: PStream
    sndout = SoundOut(
      channelNum: 2,
      sampleFormat: PA.TSampleFormat.sfFloat32,
      sampleRate: 44100,
      bufferSize: 1024)
    osc = wt.WaveTableOcillator(
      interpolFn: wt.linear_interpolate, tablePos: 0, volume: 0.5)
    env = Envelope(
      a: 0.1,
      d: 0.1,
      s: 0.5,
      r: 0.3,
      state: ASDR.None)
    stepseq = StepSequencer(
      tempo: 120,
      sequence: s,
      noteDuration: 100,
      osc: osc,
      time: 0,
      beat: 0,
      env: env)
    snd: TSound = (sndout, stepseq)

  osc.waveTable = wt.makeTable(osc, 256, wt.saw)

  discard PA.OpenDefaultStream(
    cast[PStream](stream.addr),
    numInputChannels = 0,
    numOutputChannels = cint(snd.sndout.channelNum),
    sampleFormat = snd.sndout.sampleFormat,
    sampleRate = cdouble(snd.sndout.sampleRate),
    framesPerBuffer = culong(snd.sndout.bufferSize),
    streamCallback = fillingWithTable,
    userData = cast[pointer](snd.addr))

  type KeyboardInterruptError = object of Exception
  proc handleError() {.noconv.} =
    echo "Keyboard Interrupt"
    raise newException(KeyboardInterruptError, "Keyboard Interrupt")

  setControlCHook(handleError)

  discard PA.StartStream(stream)
  try:
    while true:
      PA.Sleep(1)

  except KeyboardInterruptError:
    discard PA.StopStream(stream)
    discard PA.CloseStream(stream)
    echo repr(PA.Terminate())
    echo "============== terminate pa   ==============="
    quit 0


when isMainModule:
  let
    args = commandLineParams()

  if args.len == 0:
    playWithPA("0000000000000000")

  elif args.len == 1:
    playWithPA(args[0])

  # var vf: VF.TOggVorbis_File

  # if VF.fopen(args[0], vf.addr) == 0:
  #   echo "cannot open '" & args[0] & "'"
  #   quit(1)

  # echo "============== show .ogg info ==============="
  # echo "filename: '" & args[0] & "'"
  # echo repr(VF.info(vf.addr, -1))
