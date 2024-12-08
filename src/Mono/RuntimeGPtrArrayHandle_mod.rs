#[cfg(feature = "Mono+RuntimeGPtrArrayHandle")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct RuntimeGPtrArrayHandle {
    pub value: *mut quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "Mono+RuntimeGPtrArrayHandle")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::Mono::RuntimeGPtrArrayHandle => "Mono"
    ."RuntimeGPtrArrayHandle"
);
#[cfg(feature = "Mono+RuntimeGPtrArrayHandle")]
unsafe impl quest_hook::libil2cpp::ThisArgument for crate::Mono::RuntimeGPtrArrayHandle {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "Mono+RuntimeGPtrArrayHandle")]
impl crate::Mono::RuntimeGPtrArrayHandle {
    pub fn Lookup(
        &mut self,
        i: i32,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        let __cordl_ret: crate::System::IntPtr = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Lookup",
            (i),
        )?;
        Ok(__cordl_ret)
    }
    pub fn get_Length(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_Length",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn get_Item(
        &mut self,
        i: i32,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        let __cordl_ret: crate::System::IntPtr = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_Item",
            (i),
        )?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        ptr: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (ptr),
        )?;
        Ok(__cordl_ret)
    }
}
