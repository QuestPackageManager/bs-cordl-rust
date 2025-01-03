#[cfg(feature = "ColorSaturationValueSlider")]
#[repr(C)]
#[derive(Debug)]
pub struct ColorSaturationValueSlider {
    __cordl_parent: crate::HMUI::Slider2D,
    pub _hue: f32,
    pub _graphics: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<*mut crate::UnityEngine::UI::Graphic>,
    >,
    pub _darkColor: crate::UnityEngine::Color,
    pub _lightColor: crate::UnityEngine::Color,
    pub colorSaturationOrValueDidChangeEvent: quest_hook::libil2cpp::Gc<
        crate::System::Action_3<
            *mut crate::GlobalNamespace::ColorSaturationValueSlider,
            crate::UnityEngine::Vector2,
            crate::GlobalNamespace::ColorChangeUIEventType,
        >,
    >,
}
#[cfg(feature = "ColorSaturationValueSlider")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::ColorSaturationValueSlider =>
    ""."ColorSaturationValueSlider"
);
#[cfg(feature = "ColorSaturationValueSlider")]
impl std::ops::Deref for crate::GlobalNamespace::ColorSaturationValueSlider {
    type Target = crate::HMUI::Slider2D;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "ColorSaturationValueSlider")]
impl std::ops::DerefMut for crate::GlobalNamespace::ColorSaturationValueSlider {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "ColorSaturationValueSlider")]
impl crate::GlobalNamespace::ColorSaturationValueSlider {
    pub fn Awake(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Awake", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn HandleNormalizedValueDidChange(
        &mut self,
        slider: quest_hook::libil2cpp::Gc<crate::HMUI::Slider2D>,
        normalizedValue: crate::UnityEngine::Vector2,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleNormalizedValueDidChange", (slider, normalizedValue))?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn OnDestroy(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnDestroy", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn OnPointerUp(
        &mut self,
        eventData: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::EventSystems::PointerEventData,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnPointerUp", (eventData))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetHue(
        &mut self,
        hue: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetHue", (hue))?;
        Ok(__cordl_ret.into())
    }
    pub fn UpdateVisuals(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UpdateVisuals", ())?;
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
    pub fn add_colorSaturationOrValueDidChangeEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::Action_3<
                *mut crate::GlobalNamespace::ColorSaturationValueSlider,
                crate::UnityEngine::Vector2,
                crate::GlobalNamespace::ColorChangeUIEventType,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_colorSaturationOrValueDidChangeEvent", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn remove_colorSaturationOrValueDidChangeEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::Action_3<
                *mut crate::GlobalNamespace::ColorSaturationValueSlider,
                crate::UnityEngine::Vector2,
                crate::GlobalNamespace::ColorChangeUIEventType,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_colorSaturationOrValueDidChangeEvent", (value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "ColorSaturationValueSlider")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::ColorSaturationValueSlider {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "ColorSaturationValueSlider")]
impl AsRef<crate::UnityEngine::EventSystems::IEventSystemHandler>
for crate::GlobalNamespace::ColorSaturationValueSlider {
    fn as_ref(&self) -> &crate::UnityEngine::EventSystems::IEventSystemHandler {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "ColorSaturationValueSlider")]
impl AsMut<crate::UnityEngine::EventSystems::IEventSystemHandler>
for crate::GlobalNamespace::ColorSaturationValueSlider {
    fn as_mut(&mut self) -> &mut crate::UnityEngine::EventSystems::IEventSystemHandler {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "ColorSaturationValueSlider")]
impl AsRef<crate::UnityEngine::EventSystems::IPointerUpHandler>
for crate::GlobalNamespace::ColorSaturationValueSlider {
    fn as_ref(&self) -> &crate::UnityEngine::EventSystems::IPointerUpHandler {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "ColorSaturationValueSlider")]
impl AsMut<crate::UnityEngine::EventSystems::IPointerUpHandler>
for crate::GlobalNamespace::ColorSaturationValueSlider {
    fn as_mut(&mut self) -> &mut crate::UnityEngine::EventSystems::IPointerUpHandler {
        unsafe { std::mem::transmute(self) }
    }
}
