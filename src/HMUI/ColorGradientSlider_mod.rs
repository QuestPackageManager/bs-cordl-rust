#[cfg(feature = "HMUI+ColorGradientSlider")]
#[repr(C)]
#[derive(Debug)]
pub struct ColorGradientSlider {
    __cordl_parent: crate::HMUI::TextSlider,
    pub _textPrefix: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub _color0: crate::UnityEngine::Color,
    pub _color1: crate::UnityEngine::Color,
    pub _gradientImages: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<
            quest_hook::libil2cpp::Gc<crate::HMUI::ImageViewBase>,
        >,
    >,
    pub _darkColor: crate::UnityEngine::Color,
    pub _lightColor: crate::UnityEngine::Color,
    pub colorDidChangeEvent: quest_hook::libil2cpp::Gc<
        crate::System::Action_3<
            quest_hook::libil2cpp::Gc<crate::HMUI::ColorGradientSlider>,
            crate::UnityEngine::Color,
            crate::GlobalNamespace::ColorChangeUIEventType,
        >,
    >,
}
#[cfg(feature = "HMUI+ColorGradientSlider")]
unsafe impl quest_hook::libil2cpp::Type for crate::HMUI::ColorGradientSlider {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "HMUI";
    const CLASS_NAME: &'static str = "ColorGradientSlider";
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.class().is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_argument(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
    fn matches_value_parameter(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
}
#[cfg(feature = "HMUI+ColorGradientSlider")]
impl std::ops::Deref for crate::HMUI::ColorGradientSlider {
    type Target = crate::HMUI::TextSlider;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "HMUI+ColorGradientSlider")]
impl std::ops::DerefMut for crate::HMUI::ColorGradientSlider {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "HMUI+ColorGradientSlider")]
impl crate::HMUI::ColorGradientSlider {
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
        slider: quest_hook::libil2cpp::Gc<crate::HMUI::TextSlider>,
        normalizedValue: f32,
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
    pub fn SetColors(
        &mut self,
        color0: crate::UnityEngine::Color,
        color1: crate::UnityEngine::Color,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetColors", (color0, color1))?;
        Ok(__cordl_ret.into())
    }
    pub fn TextForNormalizedValue(
        &mut self,
        normalizedValue: f32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("TextForNormalizedValue", (normalizedValue))?;
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
    pub fn add_colorDidChangeEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::Action_3<
                quest_hook::libil2cpp::Gc<crate::HMUI::ColorGradientSlider>,
                crate::UnityEngine::Color,
                crate::GlobalNamespace::ColorChangeUIEventType,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_colorDidChangeEvent", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn remove_colorDidChangeEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::Action_3<
                quest_hook::libil2cpp::Gc<crate::HMUI::ColorGradientSlider>,
                crate::UnityEngine::Color,
                crate::GlobalNamespace::ColorChangeUIEventType,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_colorDidChangeEvent", (value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "HMUI+ColorGradientSlider")]
impl quest_hook::libil2cpp::ObjectType for crate::HMUI::ColorGradientSlider {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "HMUI+ColorGradientSlider")]
impl AsRef<crate::UnityEngine::EventSystems::IEventSystemHandler>
for crate::HMUI::ColorGradientSlider {
    fn as_ref(&self) -> &crate::UnityEngine::EventSystems::IEventSystemHandler {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "HMUI+ColorGradientSlider")]
impl AsMut<crate::UnityEngine::EventSystems::IEventSystemHandler>
for crate::HMUI::ColorGradientSlider {
    fn as_mut(&mut self) -> &mut crate::UnityEngine::EventSystems::IEventSystemHandler {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "HMUI+ColorGradientSlider")]
impl AsRef<crate::UnityEngine::EventSystems::IPointerUpHandler>
for crate::HMUI::ColorGradientSlider {
    fn as_ref(&self) -> &crate::UnityEngine::EventSystems::IPointerUpHandler {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "HMUI+ColorGradientSlider")]
impl AsMut<crate::UnityEngine::EventSystems::IPointerUpHandler>
for crate::HMUI::ColorGradientSlider {
    fn as_mut(&mut self) -> &mut crate::UnityEngine::EventSystems::IPointerUpHandler {
        unsafe { std::mem::transmute(self) }
    }
}
