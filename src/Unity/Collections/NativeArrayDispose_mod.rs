#[cfg(feature = "Unity+Collections+NativeArrayDispose")]
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct NativeArrayDispose {
    pub m_Buffer: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    pub m_AllocatorLabel: crate::Unity::Collections::Allocator,
}
#[cfg(feature = "Unity+Collections+NativeArrayDispose")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::Unity::Collections::NativeArrayDispose =>
    "Unity.Collections"."NativeArrayDispose"
);
#[cfg(feature = "Unity+Collections+NativeArrayDispose")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::Unity::Collections::NativeArrayDispose {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "Unity+Collections+NativeArrayDispose")]
impl crate::Unity::Collections::NativeArrayDispose {
    pub fn Dispose(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Dispose",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
}
