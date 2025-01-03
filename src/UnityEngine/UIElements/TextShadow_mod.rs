#[cfg(feature = "UnityEngine+UIElements+TextShadow")]
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct TextShadow {
    pub offset: crate::UnityEngine::Vector2,
    pub blurRadius: f32,
    pub color: crate::UnityEngine::Color,
}
#[cfg(feature = "UnityEngine+UIElements+TextShadow")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::UIElements::TextShadow =>
    "UnityEngine.UIElements"."TextShadow"
);
#[cfg(feature = "UnityEngine+UIElements+TextShadow")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::UIElements::TextShadow {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+UIElements+TextShadow")]
impl crate::UnityEngine::UIElements::TextShadow {
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
    pub fn Equals_TextShadow1(
        &mut self,
        other: crate::UnityEngine::UIElements::TextShadow,
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
    pub fn LerpUnclamped(
        a: crate::UnityEngine::UIElements::TextShadow,
        b: crate::UnityEngine::UIElements::TextShadow,
        t: f32,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::UIElements::TextShadow> {
        let __cordl_ret: crate::UnityEngine::UIElements::TextShadow = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("LerpUnclamped", (a, b, t))?;
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
    pub fn op_Equality(
        style1: crate::UnityEngine::UIElements::TextShadow,
        style2: crate::UnityEngine::UIElements::TextShadow,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Equality", (style1, style2))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Inequality(
        style1: crate::UnityEngine::UIElements::TextShadow,
        style2: crate::UnityEngine::UIElements::TextShadow,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Inequality", (style1, style2))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+UIElements+TextShadow")]
impl AsRef<crate::System::IEquatable_1<crate::UnityEngine::UIElements::TextShadow>>
for crate::UnityEngine::UIElements::TextShadow {
    fn as_ref(
        &self,
    ) -> &crate::System::IEquatable_1<crate::UnityEngine::UIElements::TextShadow> {
        todo!()
    }
}
#[cfg(feature = "UnityEngine+UIElements+TextShadow")]
impl AsMut<crate::System::IEquatable_1<crate::UnityEngine::UIElements::TextShadow>>
for crate::UnityEngine::UIElements::TextShadow {
    fn as_mut(
        &mut self,
    ) -> &mut crate::System::IEquatable_1<crate::UnityEngine::UIElements::TextShadow> {
        todo!()
    }
}
