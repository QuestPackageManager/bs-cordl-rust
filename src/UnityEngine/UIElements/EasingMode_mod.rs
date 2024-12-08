#[cfg(feature = "UnityEngine+UIElements+EasingMode")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EasingMode {
    Ease = 0i32,
    EaseIn = 1i32,
    EaseInBack = 17i32,
    EaseInBounce = 20i32,
    EaseInCirc = 11i32,
    EaseInCubic = 8i32,
    EaseInElastic = 14i32,
    EaseInOut = 3i32,
    EaseInOutBack = 19i32,
    EaseInOutBounce = 22i32,
    EaseInOutCirc = 13i32,
    EaseInOutCubic = 10i32,
    EaseInOutElastic = 16i32,
    EaseInOutSine = 7i32,
    EaseInSine = 5i32,
    EaseOut = 2i32,
    EaseOutBack = 18i32,
    EaseOutBounce = 21i32,
    EaseOutCirc = 12i32,
    EaseOutCubic = 9i32,
    EaseOutElastic = 15i32,
    EaseOutSine = 6i32,
    Linear = 4i32,
}
#[cfg(feature = "UnityEngine+UIElements+EasingMode")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::UIElements::EasingMode =>
    "UnityEngine.UIElements"."EasingMode"
);
