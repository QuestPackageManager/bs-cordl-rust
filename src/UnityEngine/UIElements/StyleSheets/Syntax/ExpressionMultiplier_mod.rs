#[cfg(feature = "UnityEngine+UIElements+StyleSheets+Syntax+ExpressionMultiplier")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct ExpressionMultiplier {
    pub m_Type: crate::UnityEngine::UIElements::StyleSheets::Syntax::ExpressionMultiplierType,
    pub min: i32,
    pub max: i32,
}
#[cfg(feature = "UnityEngine+UIElements+StyleSheets+Syntax+ExpressionMultiplier")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::UIElements::StyleSheets::Syntax::ExpressionMultiplier =>
    "UnityEngine.UIElements.StyleSheets.Syntax"."ExpressionMultiplier"
);
#[cfg(feature = "UnityEngine+UIElements+StyleSheets+Syntax+ExpressionMultiplier")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::UIElements::StyleSheets::Syntax::ExpressionMultiplier {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+UIElements+StyleSheets+Syntax+ExpressionMultiplier")]
impl crate::UnityEngine::UIElements::StyleSheets::Syntax::ExpressionMultiplier {
    pub const Infinity: i32 = 100i32;
    pub fn SetType(
        &mut self,
        value: crate::UnityEngine::UIElements::StyleSheets::Syntax::ExpressionMultiplierType,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "SetType",
            (value),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        _cordl_type: crate::UnityEngine::UIElements::StyleSheets::Syntax::ExpressionMultiplierType,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (_cordl_type),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_type(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::UIElements::StyleSheets::Syntax::ExpressionMultiplierType,
    > {
        let __cordl_ret: crate::UnityEngine::UIElements::StyleSheets::Syntax::ExpressionMultiplierType = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_type",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn set_type(
        &mut self,
        value: crate::UnityEngine::UIElements::StyleSheets::Syntax::ExpressionMultiplierType,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_type",
            (value),
        )?;
        Ok(__cordl_ret.into())
    }
}
