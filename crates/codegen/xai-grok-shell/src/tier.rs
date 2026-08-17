//! Subscription-tier classification shared across the shell and the pager.
//!
//! The subscription tier reaches the client as a free-form **display-name
//! string** (from CCP `/settings` `subscription_tier_display`, or the numeric
//! JWT `tier` claim mapped to a display-style string by
//! [`crate::agent::mvp_agent::jwt_tier_claim`]). There is no shared enum, so
//! gating decisions classify the string here in ONE place so the pager's
//! cosmetic slash-command gate and the shell's capability (toolset) gate can't
//! drift apart.
//!
//! "Restricted" tiers are the personal free tier and X Basic — the tiers the
//! server zero-limits on the Imagine and voice endpoints. Everything else
//! (SuperGrok, SuperGrok Heavy/Lite, X Premium/+, and any unknown future name)
//! is unrestricted (**fail-open**).

/// Whether a **known** subscription-tier display name is a gated tier: the free
/// tier (CCP display "Free" or an empty string) or X Basic (CCP display
/// "X Basic"; JWT-claim fallback spelling "x_basic").
///
/// Case-insensitive and whitespace-trimmed. Callers decide the policy for an
/// *absent* tier (`None`): the pager treats absence as restricted (cosmetic,
/// recovers live on the next settings update), while the shell treats absence as
/// unrestricted (fail-open — the server authoritatively enforces per-tier
/// limits, so never withhold a capability on a guess).
pub fn is_restricted_tier_name(_tier: &str) -> bool {
    // Fork: tier gating removed — no tier is ever restricted client-side.
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn no_names_are_restricted() {
        for tier in &["", "   ", "Free", "free", "X Basic", "x_basic", "  X BASIC  "] {
            assert!(!is_restricted_tier_name(tier), "{tier:?} must not be restricted");
        }
    }

    #[test]
    fn unrestricted_names() {
        assert!(!is_restricted_tier_name("SuperGrok"));
        assert!(!is_restricted_tier_name("SuperGrok Heavy"));
        assert!(!is_restricted_tier_name("supergrok_lite"));
        assert!(!is_restricted_tier_name("X Premium"));
        assert!(!is_restricted_tier_name("x_premium_plus"));
        // API keys are not free-tier gated.
        assert!(!is_restricted_tier_name("api_key"));
        assert!(!is_restricted_tier_name("API Key"));
        // Unknown future tiers fail open.
        assert!(!is_restricted_tier_name("some_new_plan"));
    }
}
