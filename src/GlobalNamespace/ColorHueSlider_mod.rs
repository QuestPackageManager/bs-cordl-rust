#[cfg(feature = "ColorHueSlider")]
#[repr(C)]
#[derive(Debug)]
pub struct ColorHueSlider {
    __cordl_parent: crate::HMUI::CircleSlider,
    pub _darkColor: crate::UnityEngine::Color,
    pub _lightColor: crate::UnityEngine::Color,
    pub colorHueDidChangeEvent: *mut crate::System::Action_3<
        *mut crate::GlobalNamespace::ColorHueSlider,
        f32,
        crate::GlobalNamespace::ColorChangeUIEventType,
    >,
}
#[cfg(feature = "ColorHueSlider")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::ColorHueSlider => ""
    ."ColorHueSlider"
);
#[cfg(feature = "ColorHueSlider")]
impl std::ops::Deref for crate::GlobalNamespace::ColorHueSlider {
    type Target = crate::HMUI::CircleSlider;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "ColorHueSlider")]
impl std::ops::DerefMut for crate::GlobalNamespace::ColorHueSlider {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "ColorHueSlider")]
impl crate::GlobalNamespace::ColorHueSlider {
    pub fn Awake(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Awake", ())?;
        Ok(__cordl_ret)
    }
    pub fn HandleNormalizedValueDidChange(
        &mut self,
        slider: *mut crate::HMUI::CircleSlider,
        normalizedValue: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleNormalizedValueDidChange", (slider, normalizedValue))?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn OnDestroy(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnDestroy", ())?;
        Ok(__cordl_ret)
    }
    pub fn OnPointerUp(
        &mut self,
        eventData: *mut crate::UnityEngine::EventSystems::PointerEventData,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnPointerUp", (eventData))?;
        Ok(__cordl_ret)
    }
    pub fn UpdateVisuals(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UpdateVisuals", ())?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret)
    }
    pub fn add_colorHueDidChangeEvent(
        &mut self,
        value: *mut crate::System::Action_3<
            *mut crate::GlobalNamespace::ColorHueSlider,
            f32,
            crate::GlobalNamespace::ColorChangeUIEventType,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_colorHueDidChangeEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn remove_colorHueDidChangeEvent(
        &mut self,
        value: *mut crate::System::Action_3<
            *mut crate::GlobalNamespace::ColorHueSlider,
            f32,
            crate::GlobalNamespace::ColorChangeUIEventType,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_colorHueDidChangeEvent", (value))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "ColorHueSlider")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::ColorHueSlider {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
