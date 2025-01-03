#[cfg(feature = "UnityEngine+UIElements+InheritedData")]
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct InheritedData {
    pub color: crate::UnityEngine::Color,
    pub fontSize: crate::UnityEngine::UIElements::Length,
    pub letterSpacing: crate::UnityEngine::UIElements::Length,
    pub textShadow: crate::UnityEngine::UIElements::TextShadow,
    pub unityFont: quest_hook::libil2cpp::Gc<crate::UnityEngine::Font>,
    pub unityFontDefinition: crate::UnityEngine::UIElements::FontDefinition,
    pub unityFontStyleAndWeight: crate::UnityEngine::FontStyle,
    pub unityParagraphSpacing: crate::UnityEngine::UIElements::Length,
    pub unityTextAlign: crate::UnityEngine::TextAnchor,
    pub unityTextOutlineColor: crate::UnityEngine::Color,
    pub unityTextOutlineWidth: f32,
    pub visibility: crate::UnityEngine::UIElements::Visibility,
    pub whiteSpace: crate::UnityEngine::UIElements::WhiteSpace,
    pub wordSpacing: crate::UnityEngine::UIElements::Length,
}
#[cfg(feature = "UnityEngine+UIElements+InheritedData")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::UIElements::InheritedData =>
    "UnityEngine.UIElements"."InheritedData"
);
#[cfg(feature = "UnityEngine+UIElements+InheritedData")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::UIElements::InheritedData {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+UIElements+InheritedData")]
impl crate::UnityEngine::UIElements::InheritedData {
    pub fn Copy(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::UIElements::InheritedData> {
        let __cordl_ret: crate::UnityEngine::UIElements::InheritedData = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Copy",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn CopyFrom(
        &mut self,
        other: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::UIElements::InheritedData,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "CopyFrom",
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
    pub fn Equals_InheritedData0(
        &mut self,
        other: crate::UnityEngine::UIElements::InheritedData,
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
    pub fn op_Equality(
        lhs: crate::UnityEngine::UIElements::InheritedData,
        rhs: crate::UnityEngine::UIElements::InheritedData,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Equality", (lhs, rhs))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+UIElements+InheritedData")]
impl AsRef<crate::System::IEquatable_1<crate::UnityEngine::UIElements::InheritedData>>
for crate::UnityEngine::UIElements::InheritedData {
    fn as_ref(
        &self,
    ) -> &crate::System::IEquatable_1<crate::UnityEngine::UIElements::InheritedData> {
        todo!()
    }
}
#[cfg(feature = "UnityEngine+UIElements+InheritedData")]
impl AsMut<crate::System::IEquatable_1<crate::UnityEngine::UIElements::InheritedData>>
for crate::UnityEngine::UIElements::InheritedData {
    fn as_mut(
        &mut self,
    ) -> &mut crate::System::IEquatable_1<
        crate::UnityEngine::UIElements::InheritedData,
    > {
        todo!()
    }
}
#[cfg(feature = "UnityEngine+UIElements+InheritedData")]
impl AsRef<
    crate::UnityEngine::UIElements::IStyleDataGroup_1<
        crate::UnityEngine::UIElements::InheritedData,
    >,
> for crate::UnityEngine::UIElements::InheritedData {
    fn as_ref(
        &self,
    ) -> &crate::UnityEngine::UIElements::IStyleDataGroup_1<
        crate::UnityEngine::UIElements::InheritedData,
    > {
        todo!()
    }
}
#[cfg(feature = "UnityEngine+UIElements+InheritedData")]
impl AsMut<
    crate::UnityEngine::UIElements::IStyleDataGroup_1<
        crate::UnityEngine::UIElements::InheritedData,
    >,
> for crate::UnityEngine::UIElements::InheritedData {
    fn as_mut(
        &mut self,
    ) -> &mut crate::UnityEngine::UIElements::IStyleDataGroup_1<
        crate::UnityEngine::UIElements::InheritedData,
    > {
        todo!()
    }
}
