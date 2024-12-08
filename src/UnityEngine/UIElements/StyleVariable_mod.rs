#[cfg(feature = "UnityEngine+UIElements+StyleVariable")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct StyleVariable {
    pub name: *mut crate::System::String,
    pub sheet: *mut crate::UnityEngine::UIElements::StyleSheet,
    pub handles: *mut quest_hook::libil2cpp::Il2CppArray<
        crate::UnityEngine::UIElements::StyleValueHandle,
    >,
}
#[cfg(feature = "UnityEngine+UIElements+StyleVariable")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::UIElements::StyleVariable =>
    "UnityEngine.UIElements"."StyleVariable"
);
#[cfg(feature = "UnityEngine+UIElements+StyleVariable")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::UIElements::StyleVariable {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+UIElements+StyleVariable")]
impl crate::UnityEngine::UIElements::StyleVariable {
    pub fn GetHashCode(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "GetHashCode",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        name: *mut crate::System::String,
        sheet: *mut crate::UnityEngine::UIElements::StyleSheet,
        handles: *mut quest_hook::libil2cpp::Il2CppArray<
            crate::UnityEngine::UIElements::StyleValueHandle,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (name, sheet, handles),
        )?;
        Ok(__cordl_ret)
    }
}
