#[cfg(feature = "UnityEngine+InputSystem+Utilities+PredictiveParser")]
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct PredictiveParser {
    pub m_Position: i32,
}
#[cfg(feature = "UnityEngine+InputSystem+Utilities+PredictiveParser")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::InputSystem::Utilities::PredictiveParser =>
    "UnityEngine.InputSystem.Utilities"."PredictiveParser"
);
#[cfg(feature = "UnityEngine+InputSystem+Utilities+PredictiveParser")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::InputSystem::Utilities::PredictiveParser {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+Utilities+PredictiveParser")]
impl crate::UnityEngine::InputSystem::Utilities::PredictiveParser {
    pub fn AcceptInt(
        &mut self,
        str: crate::System::ReadOnlySpan_1<char>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "AcceptInt",
            (str),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn AcceptSingleChar(
        &mut self,
        str: crate::System::ReadOnlySpan_1<char>,
        c: char,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "AcceptSingleChar",
            (str, c),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn AcceptString(
        &mut self,
        input: crate::System::ReadOnlySpan_1<char>,
        output: quest_hook::libil2cpp::ByRefMut<crate::System::ReadOnlySpan_1<char>>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "AcceptString",
            (input, output),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn ExpectInt(
        &mut self,
        str: crate::System::ReadOnlySpan_1<char>,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "ExpectInt",
            (str),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn ExpectSingleChar(
        &mut self,
        str: crate::System::ReadOnlySpan_1<char>,
        c: char,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "ExpectSingleChar",
            (str, c),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn ExpectString(
        &mut self,
        str: crate::System::ReadOnlySpan_1<char>,
    ) -> quest_hook::libil2cpp::Result<crate::System::ReadOnlySpan_1<char>> {
        let __cordl_ret: crate::System::ReadOnlySpan_1<char> = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "ExpectString",
            (str),
        )?;
        Ok(__cordl_ret.into())
    }
}
