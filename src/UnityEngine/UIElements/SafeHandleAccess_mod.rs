#[cfg(feature = "UnityEngine+UIElements+SafeHandleAccess")]
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct SafeHandleAccess {
    pub m_Handle: crate::System::IntPtr,
}
#[cfg(feature = "UnityEngine+UIElements+SafeHandleAccess")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::UIElements::SafeHandleAccess =>
    "UnityEngine.UIElements"."SafeHandleAccess"
);
#[cfg(feature = "UnityEngine+UIElements+SafeHandleAccess")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::UIElements::SafeHandleAccess {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+UIElements+SafeHandleAccess")]
impl crate::UnityEngine::UIElements::SafeHandleAccess {
    pub fn IsNull(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "IsNull",
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
    pub fn op_Implicit(
        a: crate::UnityEngine::UIElements::SafeHandleAccess,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        let __cordl_ret: crate::System::IntPtr = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Implicit", (a))?;
        Ok(__cordl_ret.into())
    }
}
