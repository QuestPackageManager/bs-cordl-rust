#[cfg(feature = "System+Buffers+MemoryHandle")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct MemoryHandle {
    pub _pointer: *mut quest_hook::libil2cpp::Il2CppObject,
    pub _handle: crate::System::Runtime::InteropServices::GCHandle,
    pub _pinnable: *mut crate::System::Buffers::IPinnable,
}
#[cfg(feature = "System+Buffers+MemoryHandle")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::System::Buffers::MemoryHandle =>
    "System.Buffers"."MemoryHandle"
);
#[cfg(feature = "System+Buffers+MemoryHandle")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::System::Buffers::MemoryHandle {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "System+Buffers+MemoryHandle")]
impl crate::System::Buffers::MemoryHandle {
    pub fn Dispose(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Dispose",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        pointer: *mut quest_hook::libil2cpp::Il2CppObject,
        handle: crate::System::Runtime::InteropServices::GCHandle,
        pinnable: *mut crate::System::Buffers::IPinnable,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (pointer, handle, pinnable),
        )?;
        Ok(__cordl_ret)
    }
    pub fn get_Pointer(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppObject> {
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_Pointer",
            (),
        )?;
        Ok(__cordl_ret)
    }
}
