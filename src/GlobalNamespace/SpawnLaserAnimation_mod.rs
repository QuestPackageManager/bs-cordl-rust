#[cfg(feature = "SpawnLaserAnimation")]
#[repr(C)]
#[derive(Debug)]
pub struct SpawnLaserAnimation {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
    pub _centerThresholdTransform: *mut crate::UnityEngine::Transform,
    pub _horizontalLasersTransform: *mut crate::UnityEngine::Transform,
    pub _leftHorizontalLaser: *mut TubeBloomPrePassLight,
    pub _rightHorizontalLaser: *mut TubeBloomPrePassLight,
    pub _normalizedDistance: f32,
    pub _alphaMultiplier: f32,
    pub _laserLength: f32,
    pub _centerDistance: f32,
    pub _initialized: bool,
}
#[cfg(feature = "SpawnLaserAnimation")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for SpawnLaserAnimation => ""."SpawnLaserAnimation"
);
#[cfg(feature = "SpawnLaserAnimation")]
impl std::ops::Deref for SpawnLaserAnimation {
    type Target = crate::UnityEngine::MonoBehaviour;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "SpawnLaserAnimation")]
impl std::ops::DerefMut for SpawnLaserAnimation {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "SpawnLaserAnimation")]
impl SpawnLaserAnimation {
    pub fn InitIfNeeded(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("InitIfNeeded", ())?;
        Ok(__cordl_ret)
    }
    pub fn LateUpdate(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("LateUpdate", ())?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
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
}
#[cfg(feature = "SpawnLaserAnimation")]
impl quest_hook::libil2cpp::ObjectType for SpawnLaserAnimation {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
