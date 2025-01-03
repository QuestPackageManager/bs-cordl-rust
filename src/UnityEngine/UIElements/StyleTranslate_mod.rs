#[cfg(feature = "UnityEngine+UIElements+StyleTranslate")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct StyleTranslate {
    pub m_Value: crate::UnityEngine::UIElements::Translate,
    pub m_Keyword: crate::UnityEngine::UIElements::StyleKeyword,
}
#[cfg(feature = "UnityEngine+UIElements+StyleTranslate")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::UIElements::StyleTranslate =>
    "UnityEngine.UIElements"."StyleTranslate"
);
#[cfg(feature = "UnityEngine+UIElements+StyleTranslate")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::UIElements::StyleTranslate {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+UIElements+StyleTranslate")]
impl crate::UnityEngine::UIElements::StyleTranslate {
    pub fn Equals_Il2CppObject1(
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
    pub fn Equals_StyleTranslate0(
        &mut self,
        other: crate::UnityEngine::UIElements::StyleTranslate,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Equals",
            (other),
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
    pub fn ToString(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = quest_hook::libil2cpp::ValueTypeExt::invoke(self, "ToString", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_StyleKeyword1(
        &mut self,
        keyword: crate::UnityEngine::UIElements::StyleKeyword,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (keyword),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_Translate0(
        &mut self,
        v: crate::UnityEngine::UIElements::Translate,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (v),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_Translate_StyleKeyword2(
        &mut self,
        v: crate::UnityEngine::UIElements::Translate,
        keyword: crate::UnityEngine::UIElements::StyleKeyword,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (v, keyword),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_keyword(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::UIElements::StyleKeyword> {
        let __cordl_ret: crate::UnityEngine::UIElements::StyleKeyword = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_keyword",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_value(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::UIElements::Translate> {
        let __cordl_ret: crate::UnityEngine::UIElements::Translate = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_value",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Equality(
        lhs: crate::UnityEngine::UIElements::StyleTranslate,
        rhs: crate::UnityEngine::UIElements::StyleTranslate,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Equality", (lhs, rhs))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Implicit_StyleKeyword0(
        keyword: crate::UnityEngine::UIElements::StyleKeyword,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::UIElements::StyleTranslate> {
        let __cordl_ret: crate::UnityEngine::UIElements::StyleTranslate = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Implicit", (keyword))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Implicit_Translate1(
        v: crate::UnityEngine::UIElements::Translate,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::UIElements::StyleTranslate> {
        let __cordl_ret: crate::UnityEngine::UIElements::StyleTranslate = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Implicit", (v))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+UIElements+StyleTranslate")]
impl AsRef<crate::System::IEquatable_1<crate::UnityEngine::UIElements::StyleTranslate>>
for crate::UnityEngine::UIElements::StyleTranslate {
    fn as_ref(
        &self,
    ) -> &crate::System::IEquatable_1<crate::UnityEngine::UIElements::StyleTranslate> {
        todo!()
    }
}
#[cfg(feature = "UnityEngine+UIElements+StyleTranslate")]
impl AsMut<crate::System::IEquatable_1<crate::UnityEngine::UIElements::StyleTranslate>>
for crate::UnityEngine::UIElements::StyleTranslate {
    fn as_mut(
        &mut self,
    ) -> &mut crate::System::IEquatable_1<
        crate::UnityEngine::UIElements::StyleTranslate,
    > {
        todo!()
    }
}
#[cfg(feature = "UnityEngine+UIElements+StyleTranslate")]
impl AsRef<
    crate::UnityEngine::UIElements::IStyleValue_1<
        crate::UnityEngine::UIElements::Translate,
    >,
> for crate::UnityEngine::UIElements::StyleTranslate {
    fn as_ref(
        &self,
    ) -> &crate::UnityEngine::UIElements::IStyleValue_1<
        crate::UnityEngine::UIElements::Translate,
    > {
        todo!()
    }
}
#[cfg(feature = "UnityEngine+UIElements+StyleTranslate")]
impl AsMut<
    crate::UnityEngine::UIElements::IStyleValue_1<
        crate::UnityEngine::UIElements::Translate,
    >,
> for crate::UnityEngine::UIElements::StyleTranslate {
    fn as_mut(
        &mut self,
    ) -> &mut crate::UnityEngine::UIElements::IStyleValue_1<
        crate::UnityEngine::UIElements::Translate,
    > {
        todo!()
    }
}
