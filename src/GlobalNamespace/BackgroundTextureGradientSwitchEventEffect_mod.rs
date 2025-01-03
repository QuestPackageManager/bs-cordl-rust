#[cfg(feature = "BackgroundTextureGradientSwitchEventEffect")]
#[repr(C)]
#[derive(Debug)]
pub struct BackgroundTextureGradientSwitchEventEffect {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
    pub _defaultTextureGradient: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::BloomPrePassBackgroundTextureGradient,
    >,
    pub _boostTextureGradient: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::BloomPrePassBackgroundTextureGradient,
    >,
    pub _beatmapCallbacksController: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::BeatmapCallbacksController,
    >,
    pub _beatmapDataCallbackWrapper: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::BeatmapDataCallbackWrapper,
    >,
}
#[cfg(feature = "BackgroundTextureGradientSwitchEventEffect")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::BackgroundTextureGradientSwitchEventEffect => ""
    ."BackgroundTextureGradientSwitchEventEffect"
);
#[cfg(feature = "BackgroundTextureGradientSwitchEventEffect")]
impl std::ops::Deref
for crate::GlobalNamespace::BackgroundTextureGradientSwitchEventEffect {
    type Target = crate::UnityEngine::MonoBehaviour;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BackgroundTextureGradientSwitchEventEffect")]
impl std::ops::DerefMut
for crate::GlobalNamespace::BackgroundTextureGradientSwitchEventEffect {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BackgroundTextureGradientSwitchEventEffect")]
impl crate::GlobalNamespace::BackgroundTextureGradientSwitchEventEffect {
    pub fn HandleBeatmapEvent(
        &mut self,
        eventData: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::ColorBoostBeatmapEventData,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleBeatmapEvent", (eventData))?;
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
    pub fn Start(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Start", ())?;
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
#[cfg(feature = "BackgroundTextureGradientSwitchEventEffect")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::BackgroundTextureGradientSwitchEventEffect {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
