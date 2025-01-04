#[cfg(feature = "System+Reflection+PInvokeAttributes")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum PInvokeAttributes {
    #[default]
    BestFitDisabled = 32i32,
    BestFitEnabled = 16i32,
    BestFitMask = 48i32,
    BestFitUseAssem = 0i32,
    CallConvCdecl = 512i32,
    CallConvFastcall = 1280i32,
    CallConvMask = 1792i32,
    CallConvStdcall = 768i32,
    CallConvThiscall = 1024i32,
    CallConvWinapi = 256i32,
    CharSetAnsi = 2i32,
    CharSetAuto = 6i32,
    CharSetUnicode = 4i32,
    MaxValue = 65535i32,
    NoMangle = 1i32,
    SupportsLastError = 64i32,
    ThrowOnUnmappableCharDisabled = 8192i32,
    ThrowOnUnmappableCharEnabled = 4096i32,
    ThrowOnUnmappableCharMask = 12288i32,
}
#[cfg(feature = "System+Reflection+PInvokeAttributes")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::System::Reflection::PInvokeAttributes =>
    "System.Reflection"."PInvokeAttributes"
);
