#[cfg(feature = "UnityEngine+UIElements+StyleFont")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct StyleFont {
    pub m_Value: *mut crate::UnityEngine::Font,
    pub m_Keyword: crate::UnityEngine::UIElements::StyleKeyword,
}
#[cfg(feature = "UnityEngine+UIElements+StyleFont")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::UIElements::StyleFont =>
    "UnityEngine.UIElements"."StyleFont"
);
#[cfg(feature = "UnityEngine+UIElements+StyleFont")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::UIElements::StyleFont {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+UIElements+StyleFont")]
impl crate::UnityEngine::UIElements::StyleFont {
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
    pub fn Equals_StyleFont0(
        &mut self,
        other: crate::UnityEngine::UIElements::StyleFont,
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
    pub fn _ctor_Font0(
        &mut self,
        v: quest_hook::libil2cpp::Gc<crate::UnityEngine::Font>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (v),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_StyleKeyword1(
        &mut self,
        v: quest_hook::libil2cpp::Gc<crate::UnityEngine::Font>,
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
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Font>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::Font> = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_value",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Equality(
        lhs: crate::UnityEngine::UIElements::StyleFont,
        rhs: crate::UnityEngine::UIElements::StyleFont,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Equality", (lhs, rhs))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Implicit(
        v: quest_hook::libil2cpp::Gc<crate::UnityEngine::Font>,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::UIElements::StyleFont> {
        let __cordl_ret: crate::UnityEngine::UIElements::StyleFont = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Implicit", (v))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+UIElements+StyleFont")]
impl AsRef<crate::System::IEquatable_1<crate::UnityEngine::UIElements::StyleFont>>
for crate::UnityEngine::UIElements::StyleFont {
    fn as_ref(
        &self,
    ) -> &crate::System::IEquatable_1<crate::UnityEngine::UIElements::StyleFont> {
        todo!()
    }
}
#[cfg(feature = "UnityEngine+UIElements+StyleFont")]
impl AsMut<crate::System::IEquatable_1<crate::UnityEngine::UIElements::StyleFont>>
for crate::UnityEngine::UIElements::StyleFont {
    fn as_mut(
        &mut self,
    ) -> &mut crate::System::IEquatable_1<crate::UnityEngine::UIElements::StyleFont> {
        todo!()
    }
}
#[cfg(feature = "UnityEngine+UIElements+StyleFont")]
impl AsRef<crate::UnityEngine::UIElements::IStyleValue_1<*mut crate::UnityEngine::Font>>
for crate::UnityEngine::UIElements::StyleFont {
    fn as_ref(
        &self,
    ) -> &crate::UnityEngine::UIElements::IStyleValue_1<*mut crate::UnityEngine::Font> {
        todo!()
    }
}
#[cfg(feature = "UnityEngine+UIElements+StyleFont")]
impl AsMut<crate::UnityEngine::UIElements::IStyleValue_1<*mut crate::UnityEngine::Font>>
for crate::UnityEngine::UIElements::StyleFont {
    fn as_mut(
        &mut self,
    ) -> &mut crate::UnityEngine::UIElements::IStyleValue_1<
        *mut crate::UnityEngine::Font,
    > {
        todo!()
    }
}
