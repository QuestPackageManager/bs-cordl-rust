#[cfg(feature = "UnityEngine+TextCore+Text+HighlightState")]
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct HighlightState {
    pub color: crate::UnityEngine::Color32,
    pub padding: crate::UnityEngine::TextCore::Text::Offset,
}
#[cfg(feature = "UnityEngine+TextCore+Text+HighlightState")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::TextCore::Text::HighlightState =>
    "UnityEngine.TextCore.Text"."HighlightState"
);
#[cfg(feature = "UnityEngine+TextCore+Text+HighlightState")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::TextCore::Text::HighlightState {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+TextCore+Text+HighlightState")]
impl crate::UnityEngine::TextCore::Text::HighlightState {
    pub fn Equals(
        &mut self,
        obj: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Equals",
            (obj),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn GetHashCode(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "GetHashCode",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        color: crate::UnityEngine::Color32,
        padding: crate::UnityEngine::TextCore::Text::Offset,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (color, padding),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Equality(
        lhs: crate::UnityEngine::TextCore::Text::HighlightState,
        rhs: crate::UnityEngine::TextCore::Text::HighlightState,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Equality", (lhs, rhs))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Inequality(
        lhs: crate::UnityEngine::TextCore::Text::HighlightState,
        rhs: crate::UnityEngine::TextCore::Text::HighlightState,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Inequality", (lhs, rhs))?;
        Ok(__cordl_ret.into())
    }
}
