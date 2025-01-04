#[cfg(feature = "UnityEngine+UIElements+UIR+CommandType")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum CommandType {
    #[default]
    BlitToPreviousRT = 9i32,
    Draw = 0i32,
    Immediate = 2i32,
    ImmediateCull = 1i32,
    PopDefaultMaterial = 11i32,
    PopRenderTexture = 8i32,
    PopScissor = 6i32,
    PopView = 4i32,
    PushDefaultMaterial = 10i32,
    PushRenderTexture = 7i32,
    PushScissor = 5i32,
    PushView = 3i32,
}
#[cfg(feature = "UnityEngine+UIElements+UIR+CommandType")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::UIElements::UIR::CommandType =>
    "UnityEngine.UIElements.UIR"."CommandType"
);
