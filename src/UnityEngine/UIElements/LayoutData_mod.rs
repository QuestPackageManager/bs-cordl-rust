#[cfg(feature = "UnityEngine+UIElements+LayoutData")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct LayoutData {
    pub alignContent: crate::UnityEngine::UIElements::Align,
    pub alignItems: crate::UnityEngine::UIElements::Align,
    pub alignSelf: crate::UnityEngine::UIElements::Align,
    pub borderBottomWidth: f32,
    pub borderLeftWidth: f32,
    pub borderRightWidth: f32,
    pub borderTopWidth: f32,
    pub bottom: crate::UnityEngine::UIElements::Length,
    pub display: crate::UnityEngine::UIElements::DisplayStyle,
    pub flexBasis: crate::UnityEngine::UIElements::Length,
    pub flexDirection: crate::UnityEngine::UIElements::FlexDirection,
    pub flexGrow: f32,
    pub flexShrink: f32,
    pub flexWrap: crate::UnityEngine::UIElements::Wrap,
    pub height: crate::UnityEngine::UIElements::Length,
    pub justifyContent: crate::UnityEngine::UIElements::Justify,
    pub left: crate::UnityEngine::UIElements::Length,
    pub marginBottom: crate::UnityEngine::UIElements::Length,
    pub marginLeft: crate::UnityEngine::UIElements::Length,
    pub marginRight: crate::UnityEngine::UIElements::Length,
    pub marginTop: crate::UnityEngine::UIElements::Length,
    pub maxHeight: crate::UnityEngine::UIElements::Length,
    pub maxWidth: crate::UnityEngine::UIElements::Length,
    pub minHeight: crate::UnityEngine::UIElements::Length,
    pub minWidth: crate::UnityEngine::UIElements::Length,
    pub paddingBottom: crate::UnityEngine::UIElements::Length,
    pub paddingLeft: crate::UnityEngine::UIElements::Length,
    pub paddingRight: crate::UnityEngine::UIElements::Length,
    pub paddingTop: crate::UnityEngine::UIElements::Length,
    pub position: crate::UnityEngine::UIElements::Position,
    pub right: crate::UnityEngine::UIElements::Length,
    pub top: crate::UnityEngine::UIElements::Length,
    pub width: crate::UnityEngine::UIElements::Length,
}
#[cfg(feature = "UnityEngine+UIElements+LayoutData")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::UIElements::LayoutData =>
    "UnityEngine.UIElements"."LayoutData"
);
#[cfg(feature = "UnityEngine+UIElements+LayoutData")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::UIElements::LayoutData {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+UIElements+LayoutData")]
impl crate::UnityEngine::UIElements::LayoutData {
    pub fn Copy(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::UIElements::LayoutData> {
        let __cordl_ret: crate::UnityEngine::UIElements::LayoutData = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Copy",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn CopyFrom(
        &mut self,
        other: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::UIElements::LayoutData,
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
    pub fn Equals_LayoutData0(
        &mut self,
        other: crate::UnityEngine::UIElements::LayoutData,
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
}
#[cfg(feature = "UnityEngine+UIElements+LayoutData")]
impl AsRef<crate::System::IEquatable_1<crate::UnityEngine::UIElements::LayoutData>>
for crate::UnityEngine::UIElements::LayoutData {
    fn as_ref(
        &self,
    ) -> &crate::System::IEquatable_1<crate::UnityEngine::UIElements::LayoutData> {
        todo!()
    }
}
#[cfg(feature = "UnityEngine+UIElements+LayoutData")]
impl AsMut<crate::System::IEquatable_1<crate::UnityEngine::UIElements::LayoutData>>
for crate::UnityEngine::UIElements::LayoutData {
    fn as_mut(
        &mut self,
    ) -> &mut crate::System::IEquatable_1<crate::UnityEngine::UIElements::LayoutData> {
        todo!()
    }
}
#[cfg(feature = "UnityEngine+UIElements+LayoutData")]
impl AsRef<
    crate::UnityEngine::UIElements::IStyleDataGroup_1<
        crate::UnityEngine::UIElements::LayoutData,
    >,
> for crate::UnityEngine::UIElements::LayoutData {
    fn as_ref(
        &self,
    ) -> &crate::UnityEngine::UIElements::IStyleDataGroup_1<
        crate::UnityEngine::UIElements::LayoutData,
    > {
        todo!()
    }
}
#[cfg(feature = "UnityEngine+UIElements+LayoutData")]
impl AsMut<
    crate::UnityEngine::UIElements::IStyleDataGroup_1<
        crate::UnityEngine::UIElements::LayoutData,
    >,
> for crate::UnityEngine::UIElements::LayoutData {
    fn as_mut(
        &mut self,
    ) -> &mut crate::UnityEngine::UIElements::IStyleDataGroup_1<
        crate::UnityEngine::UIElements::LayoutData,
    > {
        todo!()
    }
}
