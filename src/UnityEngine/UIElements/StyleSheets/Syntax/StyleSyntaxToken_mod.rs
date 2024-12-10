#[cfg(feature = "UnityEngine+UIElements+StyleSheets+Syntax+StyleSyntaxToken")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct StyleSyntaxToken {
    pub _cordl_type: crate::UnityEngine::UIElements::StyleSheets::Syntax::StyleSyntaxTokenType,
    pub text: *mut quest_hook::libil2cpp::Il2CppString,
    pub number: i32,
}
#[cfg(feature = "UnityEngine+UIElements+StyleSheets+Syntax+StyleSyntaxToken")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::UIElements::StyleSheets::Syntax::StyleSyntaxToken =>
    "UnityEngine.UIElements.StyleSheets.Syntax"."StyleSyntaxToken"
);
#[cfg(feature = "UnityEngine+UIElements+StyleSheets+Syntax+StyleSyntaxToken")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::UIElements::StyleSheets::Syntax::StyleSyntaxToken {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+UIElements+StyleSheets+Syntax+StyleSyntaxToken")]
impl crate::UnityEngine::UIElements::StyleSheets::Syntax::StyleSyntaxToken {
    pub fn _ctor_Il2CppString1(
        &mut self,
        _cordl_type: crate::UnityEngine::UIElements::StyleSheets::Syntax::StyleSyntaxTokenType,
        text: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (_cordl_type, text),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_StyleSyntaxTokenType0(
        &mut self,
        t: crate::UnityEngine::UIElements::StyleSheets::Syntax::StyleSyntaxTokenType,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (t),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_i32_2(
        &mut self,
        _cordl_type: crate::UnityEngine::UIElements::StyleSheets::Syntax::StyleSyntaxTokenType,
        number: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (_cordl_type, number),
        )?;
        Ok(__cordl_ret.into())
    }
}
