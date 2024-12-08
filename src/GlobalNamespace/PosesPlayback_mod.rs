#[cfg(feature = "PosesPlayback")]
#[repr(C)]
#[derive(Debug)]
pub struct PosesPlayback {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
    pub _audioTimeSyncController: *mut crate::GlobalNamespace::AudioTimeSyncController,
    pub _logger: *mut crate::GlobalNamespace::IBeatSaberLogger,
    pub _transforms: *mut quest_hook::libil2cpp::Il2CppArray<
        *mut crate::UnityEngine::Transform,
    >,
    pub _data: *mut crate::GlobalNamespace::PosesRecordingData,
    pub _keyframeIndex: i32,
}
#[cfg(feature = "PosesPlayback")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::PosesPlayback => ""
    ."PosesPlayback"
);
#[cfg(feature = "PosesPlayback")]
impl std::ops::Deref for crate::GlobalNamespace::PosesPlayback {
    type Target = crate::UnityEngine::MonoBehaviour;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "PosesPlayback")]
impl std::ops::DerefMut for crate::GlobalNamespace::PosesPlayback {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "PosesPlayback")]
impl crate::GlobalNamespace::PosesPlayback {
    pub fn Init(
        &mut self,
        poseObjects: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::GlobalNamespace::PoseObject,
        >,
        data: *mut crate::GlobalNamespace::PosesRecordingData,
        logger: *mut crate::GlobalNamespace::IBeatSaberLogger,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Init", (poseObjects, data, logger))?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn PlaybackTick(
        &mut self,
        _cordl_time: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("PlaybackTick", (_cordl_time))?;
        Ok(__cordl_ret)
    }
    pub fn StartPlayback(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("StartPlayback", ())?;
        Ok(__cordl_ret)
    }
    pub fn StopPlayback(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("StopPlayback", ())?;
        Ok(__cordl_ret)
    }
    pub fn Update(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Update", ())?;
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
}
#[cfg(feature = "PosesPlayback")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::PosesPlayback {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
