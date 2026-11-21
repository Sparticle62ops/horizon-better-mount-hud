# horizon-better-mount-hud

Port of [better-mount-hud](https://github.com/Lortseam/better-mount-hud)
(GPL-3.0 (c) Lortseam and contributors) to Horizon's native-mod Rust API.

Upstream improves the in-game HUD while riding a mount: the mount-health row
moves up next to the player health row when the game mode can hurt the player,
the player food bar always renders while riding, the air-bubble line sits at
the vehicle's max-hearts height, the contextual info bar stays the jump bar
while the jump key is down (unless there is no jumpable vehicle), and the
experience level still renders under the same condition.

## What is ported

The mod's entire logic is upstream `mixin/HudMixin.java` (62 lines), carried
1:1 in `hud_mixin.rs`:

- `bettermounthud$moveMountHealthUp` - `extractVehicleHealth` @STORE yLine1:
  `y -= 10` when `gameMode.canHurtPlayer()`.
- `bettermounthud$alwaysRenderFood` - `extractPlayerHealth` redirect: the
  vehicle-max-hearts call returns 0 so the food bar renders while riding.
- `bettermounthud$moveAirUp` - `getAirBubbleYLine` @HEAD: the heart count
  becomes the vehicle's max hearts when a vehicle with health is present.
- `bettermounthud$switchBar` - `nextContextualInfoState` redirect: the
  jumpable vehicle is dropped from the info bar when the game mode has
  experience, the jump key is not down and the jump-riding scale is 0.
- `bettermounthud$renderExperienceLevel` - `extractHotbarAndDecorations`
  redirect: `hasExperience()` ANDed with the same jump-bar condition.

The handlers run against a `HudMixinEnv` seam supplying the shadowed `Hud`
state (`minecraft`, `getPlayerVehicleWithHealth`, `getVehicleMaxHearts`) and
the player/game-mode surface they read.

## Known degradation

The five handlers are HUD injections into vanilla's `Hud` extraction pipeline.
The mod API has no HUD-injection surface yet, so the handlers compile and test
1:1 but the in-game HUD wiring waits on that surface. Filed on
horizon-rust/horizon.

Pinned to Horizon f00125f61f794aaa3d4e950d301f353b35b6ee3d and horizon-mixins
e59da9094f0c2fdda6c627fe9eff22dec6a906f0; tested against those revisions.

## License

GPL-3.0-or-later, per upstream. See `LICENSE`.
