#[cfg(feature = "Mono+SafeStringMarshal")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct SafeStringMarshal {
    pub str: *mut quest_hook::libil2cpp::Il2CppString,
    pub marshaled_string: crate::System::IntPtr,
}
#[cfg(feature = "Mono+SafeStringMarshal")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::Mono::SafeStringMarshal => "Mono"
    ."SafeStringMarshal"
);
#[cfg(feature = "Mono+SafeStringMarshal")]
unsafe impl quest_hook::libil2cpp::ThisArgument for crate::Mono::SafeStringMarshal {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "Mono+SafeStringMarshal")]
impl crate::Mono::SafeStringMarshal {
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
        str: *mut quest_hook::libil2cpp::Il2CppString,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (str),
        )?;
        Ok(__cordl_ret)
    }
    pub fn get_Value(&mut self) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        let __cordl_ret: crate::System::IntPtr = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_Value",
            (),
        )?;
        Ok(__cordl_ret)
    }
}
