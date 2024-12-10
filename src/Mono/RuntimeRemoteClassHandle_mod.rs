#[cfg(feature = "Mono+RuntimeRemoteClassHandle")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct RuntimeRemoteClassHandle {
    pub value: *mut quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "Mono+RuntimeRemoteClassHandle")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::Mono::RuntimeRemoteClassHandle => "Mono"
    ."RuntimeRemoteClassHandle"
);
#[cfg(feature = "Mono+RuntimeRemoteClassHandle")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::Mono::RuntimeRemoteClassHandle {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "Mono+RuntimeRemoteClassHandle")]
impl crate::Mono::RuntimeRemoteClassHandle {
    pub fn get_ProxyClass(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Mono::RuntimeClassHandle> {
        let __cordl_ret: crate::Mono::RuntimeClassHandle = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_ProxyClass",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
}
