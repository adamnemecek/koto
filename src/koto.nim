from os import commandLineParams

import portaudio as PA

import snd_types
import pa_thread as pat

import kotopkg.oscillators.wave_table as wt
import kotopkg.envelope as eg
import kotopkg.utils


when isMainModule:
  let
    args = commandLineParams()

  var
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
      r: 0.1,
      state: ASDR.None)
    stepseq = StepSequencer(
      tempo: 120,
      sequence: "0000000000000000",
      noteDuration: 100,
      osc: osc,
      time: 0,
      beat: 0,
      env: env)
    snd = TSoundRef(
      sndout: sndout,
      seq: stepseq)

  osc.waveTable = wt.makeTable(osc, 256, wt.square)

  pat.playWithPA(snd)

  # var vf: VF.TOggVorbis_File

  # if VF.fopen(args[0], vf.addr) == 0:
  #   echo "cannot open '" & args[0] & "'"
  #   quit(1)

  # echo "============== show .ogg info ==============="
  # echo "filename: '" & args[0] & "'"
  # echo repr(VF.info(vf.addr, -1))
