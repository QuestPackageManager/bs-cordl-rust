#[cfg(feature = "OffsetByBeatmapObjectSpawnCenter")]
#[repr(C)]
#[derive(Debug)]
pub struct OffsetByBeatmapObjectSpawnCenter {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
    pub _spawnCenter: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::BeatmapObjectSpawnCenter,
    >,
}
#[cfg(feature = "OffsetByBeatmapObjectSpawnCenter")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::OffsetByBeatmapObjectSpawnCenter => ""
    ."OffsetByBeatmapObjectSpawnCenter"
);
#[cfg(feature = "OffsetByBeatmapObjectSpawnCenter")]
impl std::ops::Deref for crate::GlobalNamespace::OffsetByBeatmapObjectSpawnCenter {
    type Target = crate::UnityEngine::MonoBehaviour;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OffsetByBeatmapObjectSpawnCenter")]
impl std::ops::DerefMut for crate::GlobalNamespace::OffsetByBeatmapObjectSpawnCenter {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OffsetByBeatmapObjectSpawnCenter")]
impl crate::GlobalNamespace::OffsetByBeatmapObjectSpawnCenter {
    pub fn HandleSpawnCenterDistanceWasFound(
        &mut self,
        distance: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleSpawnCenterDistanceWasFound", (distance))?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
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
#[cfg(feature = "OffsetByBeatmapObjectSpawnCenter")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::OffsetByBeatmapObjectSpawnCenter {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
