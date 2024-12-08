#[cfg(feature = "BeatmapObjectSpawnCenter")]
#[repr(C)]
#[derive(Debug)]
pub struct BeatmapObjectSpawnCenter {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
    pub _distances: *mut quest_hook::libil2cpp::Il2CppArray<
        *mut crate::GlobalNamespace::BeatmapObjectSpawnCenter_PlayerCountToDistance,
    >,
    pub _defaultDistnace: f32,
    pub spawnCenterDistanceWasFoundEvent: *mut crate::System::Action_1<f32>,
    pub _spawnCenterDistanceWasFound: bool,
    pub _spawnCenterDistance: f32,
}
#[cfg(feature = "BeatmapObjectSpawnCenter")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for BeatmapObjectSpawnCenter => ""
    ."BeatmapObjectSpawnCenter"
);
#[cfg(feature = "BeatmapObjectSpawnCenter")]
impl std::ops::Deref for BeatmapObjectSpawnCenter {
    type Target = crate::UnityEngine::MonoBehaviour;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapObjectSpawnCenter")]
impl std::ops::DerefMut for BeatmapObjectSpawnCenter {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapObjectSpawnCenter")]
impl BeatmapObjectSpawnCenter {
    #[cfg(feature = "BeatmapObjectSpawnCenter+PlayerCountToDistance")]
    pub type PlayerCountToDistance = crate::GlobalNamespace::BeatmapObjectSpawnCenter_PlayerCountToDistance;
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
    pub fn get_spawnCenterDistance(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_spawnCenterDistance", ())?;
        Ok(__cordl_ret)
    }
    pub fn add_spawnCenterDistanceWasFoundEvent(
        &mut self,
        value: *mut crate::System::Action_1<f32>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_spawnCenterDistanceWasFoundEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn get_spawnCenterDistanceWasFound(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("get_spawnCenterDistanceWasFound", ())?;
        Ok(__cordl_ret)
    }
    pub fn remove_spawnCenterDistanceWasFoundEvent(
        &mut self,
        value: *mut crate::System::Action_1<f32>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_spawnCenterDistanceWasFoundEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn CalculateSpawnCenterPosition(
        &mut self,
        numberOfPlayers: i32,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object
            .invoke("CalculateSpawnCenterPosition", (numberOfPlayers))?;
        Ok(__cordl_ret)
    }
    pub fn ReportAndSaveSpawnCenterDistance(
        &mut self,
        distance: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ReportAndSaveSpawnCenterDistance", (distance))?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "BeatmapObjectSpawnCenter")]
impl quest_hook::libil2cpp::ObjectType for BeatmapObjectSpawnCenter {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "BeatmapObjectSpawnCenter+PlayerCountToDistance")]
#[repr(C)]
#[derive(Debug)]
pub struct BeatmapObjectSpawnCenter_PlayerCountToDistance {
    __cordl_parent: crate::System::Object,
    pub _playerCount: i32,
    pub _distance: f32,
}
#[cfg(feature = "BeatmapObjectSpawnCenter+PlayerCountToDistance")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::BeatmapObjectSpawnCenter_PlayerCountToDistance => ""
    ."BeatmapObjectSpawnCenter/PlayerCountToDistance"
);
#[cfg(feature = "BeatmapObjectSpawnCenter+PlayerCountToDistance")]
impl std::ops::Deref
for crate::GlobalNamespace::BeatmapObjectSpawnCenter_PlayerCountToDistance {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapObjectSpawnCenter+PlayerCountToDistance")]
impl std::ops::DerefMut
for crate::GlobalNamespace::BeatmapObjectSpawnCenter_PlayerCountToDistance {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapObjectSpawnCenter+PlayerCountToDistance")]
impl crate::GlobalNamespace::BeatmapObjectSpawnCenter_PlayerCountToDistance {
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
    pub fn get_distance(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_distance", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_playerCount(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_playerCount", ())?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "BeatmapObjectSpawnCenter+PlayerCountToDistance")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::BeatmapObjectSpawnCenter_PlayerCountToDistance {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
