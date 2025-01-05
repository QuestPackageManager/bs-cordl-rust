#[cfg(feature = "TMPro+TMPro_EventManager")]
#[repr(C)]
#[derive(Debug)]
pub struct TMPro_EventManager {
    __cordl_parent: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
}
#[cfg(feature = "TMPro+TMPro_EventManager")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::TMPro::TMPro_EventManager => "TMPro"
    ."TMPro_EventManager"
);
#[cfg(feature = "TMPro+TMPro_EventManager")]
impl std::ops::Deref for crate::TMPro::TMPro_EventManager {
    type Target = quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "TMPro+TMPro_EventManager")]
impl std::ops::DerefMut for crate::TMPro::TMPro_EventManager {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "TMPro+TMPro_EventManager")]
impl crate::TMPro::TMPro_EventManager {
    pub fn ON_COLOR_GRADIENT_PROPERTY_CHANGED(
        obj: quest_hook::libil2cpp::Gc<crate::UnityEngine::Object>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ON_COLOR_GRADIENT_PROPERTY_CHANGED", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn ON_COMPUTE_DT_EVENT(
        Sender: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        e: quest_hook::libil2cpp::Gc<crate::TMPro::Compute_DT_EventArgs>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ON_COMPUTE_DT_EVENT", (Sender, e))?;
        Ok(__cordl_ret.into())
    }
    pub fn ON_DRAG_AND_DROP_MATERIAL_CHANGED(
        sender: quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
        currentMaterial: quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
        newMaterial: quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "ON_DRAG_AND_DROP_MATERIAL_CHANGED",
                (sender, currentMaterial, newMaterial),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn ON_FONT_PROPERTY_CHANGED(
        isChanged: bool,
        obj: quest_hook::libil2cpp::Gc<crate::UnityEngine::Object>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ON_FONT_PROPERTY_CHANGED", (isChanged, obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn ON_MATERIAL_PROPERTY_CHANGED(
        isChanged: bool,
        mat: quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ON_MATERIAL_PROPERTY_CHANGED", (isChanged, mat))?;
        Ok(__cordl_ret.into())
    }
    pub fn ON_RESOURCES_LOADED() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Void,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ON_RESOURCES_LOADED", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ON_SPRITE_ASSET_PROPERTY_CHANGED(
        isChanged: bool,
        obj: quest_hook::libil2cpp::Gc<crate::UnityEngine::Object>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ON_SPRITE_ASSET_PROPERTY_CHANGED", (isChanged, obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn ON_TEXTMESHPRO_PROPERTY_CHANGED(
        isChanged: bool,
        obj: quest_hook::libil2cpp::Gc<crate::UnityEngine::Object>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ON_TEXTMESHPRO_PROPERTY_CHANGED", (isChanged, obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn ON_TEXTMESHPRO_UGUI_PROPERTY_CHANGED(
        isChanged: bool,
        obj: quest_hook::libil2cpp::Gc<crate::UnityEngine::Object>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ON_TEXTMESHPRO_UGUI_PROPERTY_CHANGED", (isChanged, obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn ON_TEXT_CHANGED(
        obj: quest_hook::libil2cpp::Gc<crate::UnityEngine::Object>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ON_TEXT_CHANGED", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn ON_TEXT_STYLE_PROPERTY_CHANGED(
        isChanged: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ON_TEXT_STYLE_PROPERTY_CHANGED", (isChanged))?;
        Ok(__cordl_ret.into())
    }
    pub fn ON_TMP_SETTINGS_CHANGED() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Void,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ON_TMP_SETTINGS_CHANGED", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "TMPro+TMPro_EventManager")]
impl quest_hook::libil2cpp::ObjectType for crate::TMPro::TMPro_EventManager {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
