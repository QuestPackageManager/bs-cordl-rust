#[cfg(feature = "System+Runtime+InteropServices+HandleRef")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct HandleRef {
    pub _wrapper: *mut quest_hook::libil2cpp::Il2CppObject,
    pub _handle: crate::System::IntPtr,
}
#[cfg(feature = "System+Runtime+InteropServices+HandleRef")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::System::Runtime::InteropServices::HandleRef =>
    "System.Runtime.InteropServices"."HandleRef"
);
#[cfg(feature = "System+Runtime+InteropServices+HandleRef")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::System::Runtime::InteropServices::HandleRef {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "System+Runtime+InteropServices+HandleRef")]
impl crate::System::Runtime::InteropServices::HandleRef {
    pub fn _ctor(
        &mut self,
        wrapper: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        handle: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (wrapper, handle),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Handle(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        let __cordl_ret: crate::System::IntPtr = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_Handle",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
}
