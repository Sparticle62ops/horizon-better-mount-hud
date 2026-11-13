# horizon-better-mount-hud (1:1 port)

Exact 1:1 Rust translation of [better-mount-hud](https://github.com/Lortseam/better-mount-hud)
(Lortseam, GPL-3.0) for Horizon.

Upstream logic: `mixin/HudMixin.java` (62 lines), carried 1:1 in
`mixins/better_mount_hud/runtime.rs` as a Horizon client runtime splice
(`crates/horizon-client/src/game.rs`). The mixin's injections into `Hud`
(extractVehicleHealth / extractPlayerHealth / getAirBubbleYLine /
nextContextualInfoState / extractHotbarAndDecorations) run their exact handler
bodies against a `HudMixinEnv` seam that supplies the shadowed `Hud` state
(`minecraft`, `getPlayerVehicleWithHealth`, `getVehicleMaxHearts`) and the
player/game-mode surface the handlers read.

Ported by sparticle.

## License

Upstream better-mount-hud is Copyright (C) Lortseam and contributors,
GPL-3.0-or-later. This port carries the upstream license and credit per the
port policy.
