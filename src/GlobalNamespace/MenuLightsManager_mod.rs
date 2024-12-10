#[cfg(feature = "MenuLightsManager")]
#[repr(C)]
#[derive(Debug)]
pub struct MenuLightsManager {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
    pub _defaultPreset: *mut crate::GlobalNamespace::MenuLightsPresetSO,
    pub _smooth: f32,
    pub _lightManager: *mut crate::GlobalNamespace::LightWithIdManager,
    pub _preset: *mut crate::GlobalNamespace::MenuLightsPresetSO,
}
#[cfg(feature = "MenuLightsManager")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::MenuLightsManager => ""
    ."MenuLightsManager"
);
#[cfg(feature = "MenuLightsManager")]
impl std::ops::Deref for crate::GlobalNamespace::MenuLightsManager {
    type Target = crate::UnityEngine::MonoBehaviour;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "MenuLightsManager")]
impl std::ops::DerefMut for crate::GlobalNamespace::MenuLightsManager {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "MenuLightsManager")]
impl crate::GlobalNamespace::MenuLightsManager {
    #[cfg(feature = "MenuLightsManager+_Start_d__4")]
    pub type _Start_d__4 = crate::GlobalNamespace::MenuLightsManager__Start_d__4;
    pub fn CurrentColorForID(
        &mut self,
        lightId: i32,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Color> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Color = __cordl_object
            .invoke("CurrentColorForID", (lightId))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsColorVeryCloseToColor(
        &mut self,
        color0: crate::UnityEngine::Color,
        color1: crate::UnityEngine::Color,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("IsColorVeryCloseToColor", (color0, color1))?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn RefreshColors(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RefreshColors", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn RefreshLightsDictForPreset(
        &mut self,
        preset: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::MenuLightsPresetSO>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RefreshLightsDictForPreset", (preset))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetColor(
        &mut self,
        lightId: i32,
        color: crate::UnityEngine::Color,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetColor", (lightId, color))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetColorPreset(
        &mut self,
        preset: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::MenuLightsPresetSO>,
        animated: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetColorPreset", (preset, animated))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetColorsFromPreset(
        &mut self,
        preset: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::MenuLightsPresetSO>,
        interpolationFactor: f32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("SetColorsFromPreset", (preset, interpolationFactor))?;
        Ok(__cordl_ret.into())
    }
    pub fn Start(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Collections::IEnumerator>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::IEnumerator,
        > = __cordl_object.invoke("Start", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn Update(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Update", ())?;
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
}
#[cfg(feature = "MenuLightsManager")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::MenuLightsManager {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
