#[cfg(feature = "OVRProfilerScope")]
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct OVRProfilerScope {}
#[cfg(feature = "OVRProfilerScope")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::OVRProfilerScope => ""
    ."OVRProfilerScope"
);
#[cfg(feature = "OVRProfilerScope")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::GlobalNamespace::OVRProfilerScope {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "OVRProfilerScope")]
impl crate::GlobalNamespace::OVRProfilerScope {
    pub fn System_IDisposable_Dispose(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "System.IDisposable.Dispose",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (name),
        )?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "OVRProfilerScope")]
impl AsRef<crate::System::IDisposable> for crate::GlobalNamespace::OVRProfilerScope {
    fn as_ref(&self) -> &crate::System::IDisposable {
        todo!()
    }
}
#[cfg(feature = "OVRProfilerScope")]
impl AsMut<crate::System::IDisposable> for crate::GlobalNamespace::OVRProfilerScope {
    fn as_mut(&mut self) -> &mut crate::System::IDisposable {
        todo!()
    }
}
