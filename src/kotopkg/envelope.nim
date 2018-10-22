type
  ASDR* = enum
    None, Attack, Decay, Sustin, Release
  Envelope* = ref object
    a*: float32
    d*: float32
    s*: float32
    r*: float32

    state*: ASDR
    startTime*: float32
    endTime*: float32

proc noteOn*(env: Envelope, startTime: float32) =
  env.startTime = startTime
  env.state = ASDR.Attack

proc noteOff*(env: Envelope, endTime: float32) =
  if env.state in [ASDR.Attack, ASDR.Decay, ASDR.Sustin]:
    env.endTime = endTime
    env.state = ASDR.Release

proc generateEnvelope*(env: Envelope, time: float32): float32 =
  let
    noteTime = time - env.startTime
    gateTime = env.endTime - env.startTime

  if env.state in [ASDR.None, ASDR.Attack] and noteTime < env.a:
    env.state = ASDR.Attack
    return noteTime / env.a

  elif env.state in [ASDR.Attack, ASDR.Decay] and noteTime < env.a + env.d:
    env.state = ASDR.Decay
    return 1 - (noteTime - env.a) / env.d + env.s

  elif env.state in [ASDR.Decay, ASDR.Sustin] and noteTime > env.a + env.d:
    env.state = ASDR.Sustin
    return env.s

  elif env.state == ASDR.Release and noteTime < gateTime + env.r:
    env.state = ASDR.Release
    return env.s * (1 - (noteTime - gateTime) / env.r)

  elif env.state == ASDR.Release and noteTime >= gateTime + env.r:
    env.state = ASDR.None
    return 0