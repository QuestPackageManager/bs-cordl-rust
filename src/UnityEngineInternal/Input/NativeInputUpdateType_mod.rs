#[cfg(feature = "UnityEngineInternal+Input+NativeInputUpdateType")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum NativeInputUpdateType {
    #[default]
    BeforeRender = 4i32,
    Dynamic = 1i32,
    Editor = 8i32,
    Fixed = 2i32,
    IgnoreFocus = -2147483648i32,
}
#[cfg(feature = "UnityEngineInternal+Input+NativeInputUpdateType")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngineInternal::Input::NativeInputUpdateType => "UnityEngineInternal.Input"
    ."NativeInputUpdateType"
);
