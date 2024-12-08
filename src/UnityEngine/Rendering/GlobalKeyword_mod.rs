#[cfg(feature = "UnityEngine+Rendering+GlobalKeyword")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct GlobalKeyword {
    pub m_Name: *mut crate::System::String,
    pub m_Index: u32,
}
#[cfg(feature = "UnityEngine+Rendering+GlobalKeyword")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::Rendering::GlobalKeyword =>
    "UnityEngine.Rendering"."GlobalKeyword"
);
#[cfg(feature = "UnityEngine+Rendering+GlobalKeyword")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::Rendering::GlobalKeyword {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+Rendering+GlobalKeyword")]
impl crate::UnityEngine::Rendering::GlobalKeyword {
    pub fn ToString(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_ret: *mut crate::System::String = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "ToString",
            (),
        )?;
        Ok(__cordl_ret)
    }
}
