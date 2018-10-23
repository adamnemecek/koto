import math as m
import random as r

import aconf


type UnitGenerator* = ref object of RootObj
  aconf*: AudioConf
  gain*: float32
  source*: UnitGenerator

proc gen*(ug: UnitGenerator): float32 =
  let v = 0.0
  return v


type TestTone* = ref object of UnitGenerator
  angle*: float32

proc gen*(ug: TestTone): float32 =
  let v = m.sin(ug.angle)
  ug.angle += 440'f / ug.aconf.sampleRate * (2.0 * m.PI)
  return v


type WhiteNoise* = ref object of UnitGenerator

proc gen*(ug: WhiteNoise): float32 =
  let v = float32(r.random(1.0) - 0.5)
  return v
