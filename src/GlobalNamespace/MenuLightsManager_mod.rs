#[cfg(feature = "MenuLightsManager")]
#[repr(C)]
#[derive(Debug)]
pub struct MenuLightsManager {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
    pub _defaultPreset: *mut crate::GlobalNamespace::MenuLightsPresetSO,
    pub _lightManager: *mut crate::GlobalNamespace::LightWithIdManager,
    pub _tweeningManager: *mut crate::Tweening::TimeTweeningManager,
    pub _preset: *mut crate::GlobalNamespace::MenuLightsPresetSO,
    pub _originalColors: *mut crate::System::Collections::Generic::Dictionary_2<
        i32,
        crate::UnityEngine::Color,
    >,
    pub _animationTween: *mut crate::Tweening::FloatTween,
    pub _alphaMultiplier: f32,
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
    pub const kDefaultAnimationDuration: f32 = 0.5f32;
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
    pub fn GetLightForIndex(
        &mut self,
        index: i32,
    ) -> quest_hook::libil2cpp::Result<
        crate::System::ValueTuple_2<i32, crate::UnityEngine::Color>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::ValueTuple_2<i32, crate::UnityEngine::Color> = __cordl_object
            .invoke("GetLightForIndex", (index))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetLightsCount(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("GetLightsCount", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn HandleTweenFinished(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleTweenFinished", ())?;
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
    pub fn ResetColorPresetToDefault(
        &mut self,
        animated: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ResetColorPresetToDefault", (animated))?;
        Ok(__cordl_ret.into())
    }
    pub fn SaveOriginalColors(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SaveOriginalColors", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn SetAlphaMultiplier(
        &mut self,
        alphaMultiplier: f32,
        animated: bool,
        duration: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetAlphaMultiplier", (alphaMultiplier, animated, duration))?;
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
        duration: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetColorPreset", (preset, animated, duration))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetDefaultPreset(
        &mut self,
        preset: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::MenuLightsPresetSO>,
        animated: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetDefaultPreset", (preset, animated))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetTargetColors(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetTargetColors", ())?;
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
    pub fn StartLightAnimation(
        &mut self,
        duration: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("StartLightAnimation", (duration))?;
        Ok(__cordl_ret.into())
    }
    pub fn UpdateColors(
        &mut self,
        interpolationFactor: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UpdateColors", (interpolationFactor))?;
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
