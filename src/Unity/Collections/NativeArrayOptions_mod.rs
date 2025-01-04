#[cfg(feature = "Unity+Collections+NativeArrayOptions")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum NativeArrayOptions {
    #[default]
    ClearMemory = 1i32,
    UninitializedMemory = 0i32,
}
#[cfg(feature = "Unity+Collections+NativeArrayOptions")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::Unity::Collections::NativeArrayOptions =>
    "Unity.Collections"."NativeArrayOptions"
);
