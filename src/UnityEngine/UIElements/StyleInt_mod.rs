#[cfg(feature = "UnityEngine+UIElements+StyleInt")]
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct StyleInt {
    pub m_Value: i32,
    pub m_Keyword: crate::UnityEngine::UIElements::StyleKeyword,
}
#[cfg(feature = "UnityEngine+UIElements+StyleInt")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::UIElements::StyleInt =>
    "UnityEngine.UIElements"."StyleInt"
);
#[cfg(feature = "UnityEngine+UIElements+StyleInt")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::UIElements::StyleInt {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+UIElements+StyleInt")]
impl crate::UnityEngine::UIElements::StyleInt {
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
    pub fn Equals_StyleInt0(
        &mut self,
        other: crate::UnityEngine::UIElements::StyleInt,
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
    pub fn _ctor_StyleKeyword0(
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
    pub fn _ctor_i32_StyleKeyword1(
        &mut self,
        v: i32,
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
    pub fn get_value(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_value",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Equality(
        lhs: crate::UnityEngine::UIElements::StyleInt,
        rhs: crate::UnityEngine::UIElements::StyleInt,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Equality", (lhs, rhs))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Implicit(
        keyword: crate::UnityEngine::UIElements::StyleKeyword,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::UIElements::StyleInt> {
        let __cordl_ret: crate::UnityEngine::UIElements::StyleInt = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Implicit", (keyword))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+UIElements+StyleInt")]
impl AsRef<crate::System::IEquatable_1<crate::UnityEngine::UIElements::StyleInt>>
for crate::UnityEngine::UIElements::StyleInt {
    fn as_ref(
        &self,
    ) -> &crate::System::IEquatable_1<crate::UnityEngine::UIElements::StyleInt> {
        todo!()
    }
}
#[cfg(feature = "UnityEngine+UIElements+StyleInt")]
impl AsMut<crate::System::IEquatable_1<crate::UnityEngine::UIElements::StyleInt>>
for crate::UnityEngine::UIElements::StyleInt {
    fn as_mut(
        &mut self,
    ) -> &mut crate::System::IEquatable_1<crate::UnityEngine::UIElements::StyleInt> {
        todo!()
    }
}
#[cfg(feature = "UnityEngine+UIElements+StyleInt")]
impl AsRef<crate::UnityEngine::UIElements::IStyleValue_1<i32>>
for crate::UnityEngine::UIElements::StyleInt {
    fn as_ref(&self) -> &crate::UnityEngine::UIElements::IStyleValue_1<i32> {
        todo!()
    }
}
#[cfg(feature = "UnityEngine+UIElements+StyleInt")]
impl AsMut<crate::UnityEngine::UIElements::IStyleValue_1<i32>>
for crate::UnityEngine::UIElements::StyleInt {
    fn as_mut(&mut self) -> &mut crate::UnityEngine::UIElements::IStyleValue_1<i32> {
        todo!()
    }
}
