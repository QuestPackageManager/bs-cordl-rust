#[cfg(feature = "HSVPanelController")]
#[repr(C)]
#[derive(Debug)]
pub struct HSVPanelController {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
    pub _colorSaturationValueSlider: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::ColorSaturationValueSlider,
    >,
    pub _colorHueSlider: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::ColorHueSlider,
    >,
    pub colorDidChangeEvent: quest_hook::libil2cpp::Gc<
        crate::System::Action_2<
            crate::UnityEngine::Color,
            crate::GlobalNamespace::ColorChangeUIEventType,
        >,
    >,
    pub _hsvColor: crate::UnityEngine::Vector3,
}
#[cfg(feature = "HSVPanelController")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::HSVPanelController => ""
    ."HSVPanelController"
);
#[cfg(feature = "HSVPanelController")]
impl std::ops::Deref for crate::GlobalNamespace::HSVPanelController {
    type Target = crate::UnityEngine::MonoBehaviour;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "HSVPanelController")]
impl std::ops::DerefMut for crate::GlobalNamespace::HSVPanelController {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "HSVPanelController")]
impl crate::GlobalNamespace::HSVPanelController {
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
    pub fn HandleColorHueDidChange(
        &mut self,
        slider: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::ColorHueSlider>,
        hue: f32,
        colorChangeUIEventType: crate::GlobalNamespace::ColorChangeUIEventType,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleColorHueDidChange", (slider, hue, colorChangeUIEventType))?;
        Ok(__cordl_ret.into())
    }
    pub fn HandleColorSaturationOrValueDidChange(
        &mut self,
        slider: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::ColorSaturationValueSlider,
        >,
        colorSaturationAndValue: crate::UnityEngine::Vector2,
        colorChangeUIEventType: crate::GlobalNamespace::ColorChangeUIEventType,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "HandleColorSaturationOrValueDidChange",
                (slider, colorSaturationAndValue, colorChangeUIEventType),
            )?;
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
    pub fn RefreshSlidersColors(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RefreshSlidersColors", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn RefreshSlidersValues(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RefreshSlidersValues", ())?;
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
            crate::System::Action_2<
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
    pub fn get_color(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Color> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Color = __cordl_object
            .invoke("get_color", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn remove_colorDidChangeEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::Action_2<
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
    pub fn set_color(
        &mut self,
        value: crate::UnityEngine::Color,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_color", (value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "HSVPanelController")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::HSVPanelController {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
