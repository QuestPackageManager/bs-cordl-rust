#[cfg(feature = "Mono+SafeGPtrArrayHandle")]
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct SafeGPtrArrayHandle {
    pub handle: crate::Mono::RuntimeGPtrArrayHandle,
}
#[cfg(feature = "Mono+SafeGPtrArrayHandle")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::Mono::SafeGPtrArrayHandle => "Mono"
    ."SafeGPtrArrayHandle"
);
#[cfg(feature = "Mono+SafeGPtrArrayHandle")]
unsafe impl quest_hook::libil2cpp::ThisArgument for crate::Mono::SafeGPtrArrayHandle {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "Mono+SafeGPtrArrayHandle")]
impl crate::Mono::SafeGPtrArrayHandle {
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
    pub fn _ctor(
        &mut self,
        ptr: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (ptr),
        )?;
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
    }
    pub fn get_Length(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_Length",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Mono+SafeGPtrArrayHandle")]
impl AsRef<crate::System::IDisposable> for crate::Mono::SafeGPtrArrayHandle {
    fn as_ref(&self) -> &crate::System::IDisposable {
        todo!()
    }
}
#[cfg(feature = "Mono+SafeGPtrArrayHandle")]
impl AsMut<crate::System::IDisposable> for crate::Mono::SafeGPtrArrayHandle {
    fn as_mut(&mut self) -> &mut crate::System::IDisposable {
        todo!()
    }
}
