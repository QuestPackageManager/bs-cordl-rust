#[cfg(feature = "UnityEngine+UIElements+StyleColor")]
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct StyleColor {
    pub m_Value: crate::UnityEngine::Color,
    pub m_Keyword: crate::UnityEngine::UIElements::StyleKeyword,
}
#[cfg(feature = "UnityEngine+UIElements+StyleColor")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::UIElements::StyleColor =>
    "UnityEngine.UIElements"."StyleColor"
);
#[cfg(feature = "UnityEngine+UIElements+StyleColor")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::UIElements::StyleColor {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+UIElements+StyleColor")]
impl crate::UnityEngine::UIElements::StyleColor {
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
    pub fn Equals_StyleColor0(
        &mut self,
        other: crate::UnityEngine::UIElements::StyleColor,
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
    pub fn _ctor_Color0(
        &mut self,
        v: crate::UnityEngine::Color,
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
        v: crate::UnityEngine::Color,
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
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Color> {
        let __cordl_ret: crate::UnityEngine::Color = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_value",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Equality(
        lhs: crate::UnityEngine::UIElements::StyleColor,
        rhs: crate::UnityEngine::UIElements::StyleColor,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Equality", (lhs, rhs))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Implicit(
        v: crate::UnityEngine::Color,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::UIElements::StyleColor> {
        let __cordl_ret: crate::UnityEngine::UIElements::StyleColor = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Implicit", (v))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+UIElements+StyleColor")]
impl AsRef<crate::System::IEquatable_1<crate::UnityEngine::UIElements::StyleColor>>
for crate::UnityEngine::UIElements::StyleColor {
    fn as_ref(
        &self,
    ) -> &crate::System::IEquatable_1<crate::UnityEngine::UIElements::StyleColor> {
        todo!()
    }
}
#[cfg(feature = "UnityEngine+UIElements+StyleColor")]
impl AsMut<crate::System::IEquatable_1<crate::UnityEngine::UIElements::StyleColor>>
for crate::UnityEngine::UIElements::StyleColor {
    fn as_mut(
        &mut self,
    ) -> &mut crate::System::IEquatable_1<crate::UnityEngine::UIElements::StyleColor> {
        todo!()
    }
}
#[cfg(feature = "UnityEngine+UIElements+StyleColor")]
impl AsRef<crate::UnityEngine::UIElements::IStyleValue_1<crate::UnityEngine::Color>>
for crate::UnityEngine::UIElements::StyleColor {
    fn as_ref(
        &self,
    ) -> &crate::UnityEngine::UIElements::IStyleValue_1<crate::UnityEngine::Color> {
        todo!()
    }
}
#[cfg(feature = "UnityEngine+UIElements+StyleColor")]
impl AsMut<crate::UnityEngine::UIElements::IStyleValue_1<crate::UnityEngine::Color>>
for crate::UnityEngine::UIElements::StyleColor {
    fn as_mut(
        &mut self,
    ) -> &mut crate::UnityEngine::UIElements::IStyleValue_1<crate::UnityEngine::Color> {
        todo!()
    }
}
