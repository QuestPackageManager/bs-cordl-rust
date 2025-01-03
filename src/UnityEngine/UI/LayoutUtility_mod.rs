#[cfg(feature = "UnityEngine+UI+LayoutUtility")]
#[repr(C)]
#[derive(Debug)]
pub struct LayoutUtility {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "UnityEngine+UI+LayoutUtility")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::UI::LayoutUtility =>
    "UnityEngine.UI"."LayoutUtility"
);
#[cfg(feature = "UnityEngine+UI+LayoutUtility")]
impl std::ops::Deref for crate::UnityEngine::UI::LayoutUtility {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UI+LayoutUtility")]
impl std::ops::DerefMut for crate::UnityEngine::UI::LayoutUtility {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UI+LayoutUtility")]
impl crate::UnityEngine::UI::LayoutUtility {
    pub fn GetFlexibleHeight(
        rect: quest_hook::libil2cpp::Gc<crate::UnityEngine::RectTransform>,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetFlexibleHeight", (rect))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetFlexibleSize(
        rect: quest_hook::libil2cpp::Gc<crate::UnityEngine::RectTransform>,
        axis: i32,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetFlexibleSize", (rect, axis))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetFlexibleWidth(
        rect: quest_hook::libil2cpp::Gc<crate::UnityEngine::RectTransform>,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetFlexibleWidth", (rect))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetLayoutProperty_ByRefMut1(
        rect: quest_hook::libil2cpp::Gc<crate::UnityEngine::RectTransform>,
        property: quest_hook::libil2cpp::Gc<
            crate::System::Func_2<*mut crate::UnityEngine::UI::ILayoutElement, f32>,
        >,
        defaultValue: f32,
        source: quest_hook::libil2cpp::ByRefMut<
            *mut crate::UnityEngine::UI::ILayoutElement,
        >,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetLayoutProperty", (rect, property, defaultValue, source))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetLayoutProperty_RectTransform_Func_2_f32_0(
        rect: quest_hook::libil2cpp::Gc<crate::UnityEngine::RectTransform>,
        property: quest_hook::libil2cpp::Gc<
            crate::System::Func_2<*mut crate::UnityEngine::UI::ILayoutElement, f32>,
        >,
        defaultValue: f32,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetLayoutProperty", (rect, property, defaultValue))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetMinHeight(
        rect: quest_hook::libil2cpp::Gc<crate::UnityEngine::RectTransform>,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetMinHeight", (rect))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetMinSize(
        rect: quest_hook::libil2cpp::Gc<crate::UnityEngine::RectTransform>,
        axis: i32,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetMinSize", (rect, axis))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetMinWidth(
        rect: quest_hook::libil2cpp::Gc<crate::UnityEngine::RectTransform>,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetMinWidth", (rect))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetPreferredHeight(
        rect: quest_hook::libil2cpp::Gc<crate::UnityEngine::RectTransform>,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetPreferredHeight", (rect))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetPreferredSize(
        rect: quest_hook::libil2cpp::Gc<crate::UnityEngine::RectTransform>,
        axis: i32,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetPreferredSize", (rect, axis))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetPreferredWidth(
        rect: quest_hook::libil2cpp::Gc<crate::UnityEngine::RectTransform>,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetPreferredWidth", (rect))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+UI+LayoutUtility")]
impl quest_hook::libil2cpp::ObjectType for crate::UnityEngine::UI::LayoutUtility {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
