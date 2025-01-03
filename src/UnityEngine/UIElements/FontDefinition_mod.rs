#[cfg(feature = "UnityEngine+UIElements+FontDefinition")]
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct FontDefinition {
    pub m_Font: quest_hook::libil2cpp::Gc<crate::UnityEngine::Font>,
    pub m_FontAsset: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::TextCore::Text::FontAsset,
    >,
}
#[cfg(feature = "UnityEngine+UIElements+FontDefinition")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::UIElements::FontDefinition =>
    "UnityEngine.UIElements"."FontDefinition"
);
#[cfg(feature = "UnityEngine+UIElements+FontDefinition")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::UIElements::FontDefinition {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+UIElements+FontDefinition")]
impl crate::UnityEngine::UIElements::FontDefinition {
    pub fn Equals_FontDefinition0(
        &mut self,
        other: crate::UnityEngine::UIElements::FontDefinition,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Equals",
            (other),
        )?;
        Ok(__cordl_ret.into())
    }
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
    pub fn FromFont(
        f: quest_hook::libil2cpp::Gc<crate::UnityEngine::Font>,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::UIElements::FontDefinition> {
        let __cordl_ret: crate::UnityEngine::UIElements::FontDefinition = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("FromFont", (f))?;
        Ok(__cordl_ret.into())
    }
    pub fn FromObject(
        obj: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::UIElements::FontDefinition> {
        let __cordl_ret: crate::UnityEngine::UIElements::FontDefinition = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("FromObject", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn FromSDFFont(
        f: quest_hook::libil2cpp::Gc<crate::UnityEngine::TextCore::Text::FontAsset>,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::UIElements::FontDefinition> {
        let __cordl_ret: crate::UnityEngine::UIElements::FontDefinition = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("FromSDFFont", (f))?;
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
    pub fn IsEmpty(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "IsEmpty",
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
    pub fn get_font(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Font>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::Font> = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_font",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_fontAsset(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::TextCore::Text::FontAsset>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::TextCore::Text::FontAsset,
        > = quest_hook::libil2cpp::ValueTypeExt::invoke(self, "get_fontAsset", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Equality(
        left: crate::UnityEngine::UIElements::FontDefinition,
        right: crate::UnityEngine::UIElements::FontDefinition,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Equality", (left, right))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Inequality(
        left: crate::UnityEngine::UIElements::FontDefinition,
        right: crate::UnityEngine::UIElements::FontDefinition,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Inequality", (left, right))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+UIElements+FontDefinition")]
impl AsRef<crate::System::IEquatable_1<crate::UnityEngine::UIElements::FontDefinition>>
for crate::UnityEngine::UIElements::FontDefinition {
    fn as_ref(
        &self,
    ) -> &crate::System::IEquatable_1<crate::UnityEngine::UIElements::FontDefinition> {
        todo!()
    }
}
#[cfg(feature = "UnityEngine+UIElements+FontDefinition")]
impl AsMut<crate::System::IEquatable_1<crate::UnityEngine::UIElements::FontDefinition>>
for crate::UnityEngine::UIElements::FontDefinition {
    fn as_mut(
        &mut self,
    ) -> &mut crate::System::IEquatable_1<
        crate::UnityEngine::UIElements::FontDefinition,
    > {
        todo!()
    }
}
