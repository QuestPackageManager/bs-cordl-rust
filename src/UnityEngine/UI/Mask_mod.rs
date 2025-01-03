#[cfg(feature = "UnityEngine+UI+Mask")]
#[repr(C)]
#[derive(Debug)]
pub struct Mask {
    __cordl_parent: crate::UnityEngine::EventSystems::UIBehaviour,
    pub m_RectTransform: quest_hook::libil2cpp::Gc<crate::UnityEngine::RectTransform>,
    pub m_ShowMaskGraphic: bool,
    pub m_Graphic: quest_hook::libil2cpp::Gc<crate::UnityEngine::UI::Graphic>,
    pub m_MaskMaterial: quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
    pub m_UnmaskMaterial: quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
}
#[cfg(feature = "UnityEngine+UI+Mask")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::UI::Mask => "UnityEngine.UI"."Mask"
);
#[cfg(feature = "UnityEngine+UI+Mask")]
impl std::ops::Deref for crate::UnityEngine::UI::Mask {
    type Target = crate::UnityEngine::EventSystems::UIBehaviour;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UI+Mask")]
impl std::ops::DerefMut for crate::UnityEngine::UI::Mask {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UI+Mask")]
impl crate::UnityEngine::UI::Mask {
    pub fn GetModifiedMaterial(
        &mut self,
        baseMaterial: quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::Material> = __cordl_object
            .invoke("GetModifiedMaterial", (baseMaterial))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsRaycastLocationValid(
        &mut self,
        sp: crate::UnityEngine::Vector2,
        eventCamera: quest_hook::libil2cpp::Gc<crate::UnityEngine::Camera>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("IsRaycastLocationValid", (sp, eventCamera))?;
        Ok(__cordl_ret.into())
    }
    pub fn MaskEnabled(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("MaskEnabled", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn OnDisable(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnDisable", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn OnEnable(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnEnable", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn OnSiblingGraphicEnabledDisabled(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnSiblingGraphicEnabledDisabled", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_graphic(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::UI::Graphic>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::UI::Graphic> = __cordl_object
            .invoke("get_graphic", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_rectTransform(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::RectTransform>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::RectTransform> = __cordl_object
            .invoke("get_rectTransform", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_showMaskGraphic(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_showMaskGraphic", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn set_showMaskGraphic(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_showMaskGraphic", (value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+UI+Mask")]
impl quest_hook::libil2cpp::ObjectType for crate::UnityEngine::UI::Mask {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "UnityEngine+UI+Mask")]
impl AsRef<crate::UnityEngine::ICanvasRaycastFilter> for crate::UnityEngine::UI::Mask {
    fn as_ref(&self) -> &crate::UnityEngine::ICanvasRaycastFilter {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "UnityEngine+UI+Mask")]
impl AsMut<crate::UnityEngine::ICanvasRaycastFilter> for crate::UnityEngine::UI::Mask {
    fn as_mut(&mut self) -> &mut crate::UnityEngine::ICanvasRaycastFilter {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "UnityEngine+UI+Mask")]
impl AsRef<crate::UnityEngine::UI::IMaterialModifier> for crate::UnityEngine::UI::Mask {
    fn as_ref(&self) -> &crate::UnityEngine::UI::IMaterialModifier {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "UnityEngine+UI+Mask")]
impl AsMut<crate::UnityEngine::UI::IMaterialModifier> for crate::UnityEngine::UI::Mask {
    fn as_mut(&mut self) -> &mut crate::UnityEngine::UI::IMaterialModifier {
        unsafe { std::mem::transmute(self) }
    }
}
