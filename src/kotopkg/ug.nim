import math as m
import random as r

import player


type UnitGenerator* = ref object of RootObj
  mp*: MasterPlayer
  gain*: float32
  sources*: seq[UnitGenerator]


method gen*(ug: UnitGenerator): float32 =
  var v: float32 = 0.0
  for source in ug.sources:
    v += source.gain * gen(source)
  return v


type TestTone* = ref object of UnitGenerator
  angle*: float32

method gen*(ug: TestTone): float32 =
  let v = m.sin(ug.angle)
  ug.angle += 440'f / ug.mp.sampleRate * (2.0 * m.PI)
  return v * ug.gain + gen(UnitGenerator(ug))


type WhiteNoise* = ref object of UnitGenerator

method gen*(ug: WhiteNoise): float32 =
  let v = float32(r.random(1.0) - 0.5)
  return v * ug.gain + gen(UnitGenerator(ug))
