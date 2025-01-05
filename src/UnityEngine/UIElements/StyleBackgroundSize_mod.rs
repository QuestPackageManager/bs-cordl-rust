#[cfg(feature = "UnityEngine+UIElements+StyleBackgroundSize")]
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct StyleBackgroundSize {
    pub m_Value: crate::UnityEngine::UIElements::BackgroundSize,
    pub m_Keyword: crate::UnityEngine::UIElements::StyleKeyword,
}
#[cfg(feature = "UnityEngine+UIElements+StyleBackgroundSize")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::UIElements::StyleBackgroundSize =>
    "UnityEngine.UIElements"."StyleBackgroundSize"
);
#[cfg(feature = "UnityEngine+UIElements+StyleBackgroundSize")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::UIElements::StyleBackgroundSize {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+UIElements+StyleBackgroundSize")]
impl crate::UnityEngine::UIElements::StyleBackgroundSize {
    pub fn Equals_Gc1(
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
    pub fn Equals_StyleBackgroundSize0(
        &mut self,
        other: crate::UnityEngine::UIElements::StyleBackgroundSize,
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
    pub fn _ctor_BackgroundSize_StyleKeyword1(
        &mut self,
        v: crate::UnityEngine::UIElements::BackgroundSize,
        keyword: crate::UnityEngine::UIElements::StyleKeyword,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (v, keyword),
        )?;
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
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::UIElements::BackgroundSize> {
        let __cordl_ret: crate::UnityEngine::UIElements::BackgroundSize = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_value",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Equality(
        lhs: crate::UnityEngine::UIElements::StyleBackgroundSize,
        rhs: crate::UnityEngine::UIElements::StyleBackgroundSize,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Equality", (lhs, rhs))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Implicit(
        keyword: crate::UnityEngine::UIElements::StyleKeyword,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::UIElements::StyleBackgroundSize,
    > {
        let __cordl_ret: crate::UnityEngine::UIElements::StyleBackgroundSize = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Implicit", (keyword))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+UIElements+StyleBackgroundSize")]
impl AsRef<quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::BackgroundSize>>
for crate::UnityEngine::UIElements::StyleBackgroundSize {
    fn as_ref(
        &self,
    ) -> &quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::BackgroundSize> {
        todo!()
    }
}
#[cfg(feature = "UnityEngine+UIElements+StyleBackgroundSize")]
impl AsMut<quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::BackgroundSize>>
for crate::UnityEngine::UIElements::StyleBackgroundSize {
    fn as_mut(
        &mut self,
    ) -> &mut quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::BackgroundSize> {
        todo!()
    }
}
#[cfg(feature = "UnityEngine+UIElements+StyleBackgroundSize")]
impl AsRef<
    quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::StyleBackgroundSize>,
> for crate::UnityEngine::UIElements::StyleBackgroundSize {
    fn as_ref(
        &self,
    ) -> &quest_hook::libil2cpp::Gc<
        crate::UnityEngine::UIElements::StyleBackgroundSize,
    > {
        todo!()
    }
}
#[cfg(feature = "UnityEngine+UIElements+StyleBackgroundSize")]
impl AsMut<
    quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::StyleBackgroundSize>,
> for crate::UnityEngine::UIElements::StyleBackgroundSize {
    fn as_mut(
        &mut self,
    ) -> &mut quest_hook::libil2cpp::Gc<
        crate::UnityEngine::UIElements::StyleBackgroundSize,
    > {
        todo!()
    }
}
