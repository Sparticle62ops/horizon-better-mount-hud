//! better-mount-hud for Horizon: the mod API wiring of the 1:1 core.
//!
//! The 1:1 logic core lives in `hud_mixin.rs`: upstream `mixin/HudMixin.java`
//! (62 lines) carried exactly, its five handlers running against the shadowed
//! `Hud` state (`minecraft`, `getPlayerVehicleWithHealth`,
//! `getVehicleMaxHearts`) and the player/game-mode surface they read, over the
//! `HudMixinEnv` seam:
//!
//! - `bettermounthud$moveMountHealthUp` (`extractVehicleHealth`, @STORE yLine1)
//! - `bettermounthud$alwaysRenderFood` (`extractPlayerHealth` redirect)
//! - `bettermounthud$moveAirUp` (`getAirBubbleYLine`, @HEAD)
//! - `bettermounthud$switchBar` (`nextContextualInfoState` redirect)
//! - `bettermounthud$renderExperienceLevel` (`extractHotbarAndDecorations` redirect)
//!
//! Upstream has no config and no fabric entrypoints (client-side mixins only),
//! so there are no settings to register and no client init to run: the
//! registration is empty, matching the original.
//!
//! Gaps, tracked on horizon-rust/horizon: the five handlers are HUD
//! injections into vanilla's `Hud` extraction pipeline (the vehicle-health row
//! placement, the always-render food redirect, the air-bubble line, the
//! contextual info bar switch and the experience-level render condition). The
//! mod API has no HUD-injection surface yet, so the handlers compile and test
//! 1:1 but the in-game HUD wiring waits on that surface.
use mod_api::{RegistrationContext, Registrar};

mod hud_mixin;

#[derive(Default)]
struct Mod;

#[mod_api::horizon_mod]
impl mod_api::Hooks for Mod {
    /// Upstream has no config: nothing to register on the settings page.
    fn register(&self, _registrar: &mut dyn Registrar, _context: &RegistrationContext) {}
}
