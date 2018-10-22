type UnitGenerator* = ref object of RootObj
  calc*: proc(ug: UnitGenerator): float32
