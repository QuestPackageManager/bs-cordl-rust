#[cfg(feature = "UnityEngineInternal+Input+NativeInputEventType")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum NativeInputEventType {
    #[default]
    Delta = 1145852993i32,
    DeviceConfigChanged = 1145259591i32,
    DeviceRemoved = 1146242381i32,
    State = 1398030676i32,
    Text = 1413830740i32,
}
#[cfg(feature = "UnityEngineInternal+Input+NativeInputEventType")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngineInternal::Input::NativeInputEventType
    => "UnityEngineInternal.Input"."NativeInputEventType"
);
