#[cfg(feature = "UnityEngine+UIElements+BackgroundPosition")]
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct BackgroundPosition {
    pub keyword: crate::UnityEngine::UIElements::BackgroundPositionKeyword,
    pub offset: crate::UnityEngine::UIElements::Length,
}
#[cfg(feature = "UnityEngine+UIElements+BackgroundPosition")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::UIElements::BackgroundPosition =>
    "UnityEngine.UIElements"."BackgroundPosition"
);
#[cfg(feature = "UnityEngine+UIElements+BackgroundPosition")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::UIElements::BackgroundPosition {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+UIElements+BackgroundPosition")]
impl crate::UnityEngine::UIElements::BackgroundPosition {
    pub fn Equals_BackgroundPosition1(
        &mut self,
        other: crate::UnityEngine::UIElements::BackgroundPosition,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Equals",
            (other),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn Equals_Il2CppObject0(
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
    pub fn Initial() -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::UIElements::BackgroundPosition,
    > {
        let __cordl_ret: crate::UnityEngine::UIElements::BackgroundPosition = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Initial", ())?;
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
    pub fn _ctor_BackgroundPositionKeyword0(
        &mut self,
        keyword: crate::UnityEngine::UIElements::BackgroundPositionKeyword,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (keyword),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_Length1(
        &mut self,
        keyword: crate::UnityEngine::UIElements::BackgroundPositionKeyword,
        offset: crate::UnityEngine::UIElements::Length,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (keyword, offset),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Equality(
        style1: crate::UnityEngine::UIElements::BackgroundPosition,
        style2: crate::UnityEngine::UIElements::BackgroundPosition,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Equality", (style1, style2))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Inequality(
        style1: crate::UnityEngine::UIElements::BackgroundPosition,
        style2: crate::UnityEngine::UIElements::BackgroundPosition,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Inequality", (style1, style2))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+UIElements+BackgroundPosition")]
impl AsRef<
    crate::System::IEquatable_1<crate::UnityEngine::UIElements::BackgroundPosition>,
> for crate::UnityEngine::UIElements::BackgroundPosition {
    fn as_ref(
        &self,
    ) -> &crate::System::IEquatable_1<
        crate::UnityEngine::UIElements::BackgroundPosition,
    > {
        todo!()
    }
}
#[cfg(feature = "UnityEngine+UIElements+BackgroundPosition")]
impl AsMut<
    crate::System::IEquatable_1<crate::UnityEngine::UIElements::BackgroundPosition>,
> for crate::UnityEngine::UIElements::BackgroundPosition {
    fn as_mut(
        &mut self,
    ) -> &mut crate::System::IEquatable_1<
        crate::UnityEngine::UIElements::BackgroundPosition,
    > {
        todo!()
    }
}
