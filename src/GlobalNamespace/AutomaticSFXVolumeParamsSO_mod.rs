#[cfg(feature = "AutomaticSFXVolumeParamsSO")]
#[repr(C)]
#[derive(Debug)]
pub struct AutomaticSFXVolumeParamsSO {
    __cordl_parent: crate::UnityEngine::ScriptableObject,
    pub _musicVolumeMultiplier: f32,
    pub _threshold: f32,
    pub _impact: f32,
    pub _attackTime: f32,
    pub _releaseTime: f32,
    pub _minVolume: f32,
    pub _maxVolume: f32,
    pub _volumeSmooth: f32,
}
#[cfg(feature = "AutomaticSFXVolumeParamsSO")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::AutomaticSFXVolumeParamsSO =>
    ""."AutomaticSFXVolumeParamsSO"
);
#[cfg(feature = "AutomaticSFXVolumeParamsSO")]
impl std::ops::Deref for crate::GlobalNamespace::AutomaticSFXVolumeParamsSO {
    type Target = crate::UnityEngine::ScriptableObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "AutomaticSFXVolumeParamsSO")]
impl std::ops::DerefMut for crate::GlobalNamespace::AutomaticSFXVolumeParamsSO {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "AutomaticSFXVolumeParamsSO")]
impl crate::GlobalNamespace::AutomaticSFXVolumeParamsSO {
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
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
    pub fn get_attackTime(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_attackTime", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_impact(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_impact", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_maxVolume(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_maxVolume", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_minVolume(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_minVolume", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_musicVolumeMultiplier(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_musicVolumeMultiplier", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_releaseTime(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_releaseTime", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_threshold(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_threshold", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_volumeSmooth(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_volumeSmooth", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "AutomaticSFXVolumeParamsSO")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::AutomaticSFXVolumeParamsSO {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
