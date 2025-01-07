#[cfg(feature = "BeatmapObjectSpawnCenter")]
#[repr(C)]
#[derive(Debug)]
pub struct BeatmapObjectSpawnCenter {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
    pub _distances: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<
            quest_hook::libil2cpp::Gc<
                crate::GlobalNamespace::BeatmapObjectSpawnCenter_PlayerCountToDistance,
            >,
        >,
    >,
    pub _defaultDistnace: f32,
    pub spawnCenterDistanceWasFoundEvent: quest_hook::libil2cpp::Gc<
        crate::System::Action_1<f32>,
    >,
    pub _spawnCenterDistanceWasFound: bool,
    pub _spawnCenterDistance: f32,
}
#[cfg(feature = "BeatmapObjectSpawnCenter")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::BeatmapObjectSpawnCenter {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "BeatmapObjectSpawnCenter";
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
#[cfg(feature = "BeatmapObjectSpawnCenter")]
impl std::ops::Deref for crate::GlobalNamespace::BeatmapObjectSpawnCenter {
    type Target = crate::UnityEngine::MonoBehaviour;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapObjectSpawnCenter")]
impl std::ops::DerefMut for crate::GlobalNamespace::BeatmapObjectSpawnCenter {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapObjectSpawnCenter")]
impl crate::GlobalNamespace::BeatmapObjectSpawnCenter {
    #[cfg(feature = "BeatmapObjectSpawnCenter+PlayerCountToDistance")]
    pub type PlayerCountToDistance = crate::GlobalNamespace::BeatmapObjectSpawnCenter_PlayerCountToDistance;
    pub fn CalculateSpawnCenterPosition(
        &mut self,
        numberOfPlayers: i32,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object
            .invoke("CalculateSpawnCenterPosition", (numberOfPlayers))?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
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
    pub fn add_spawnCenterDistanceWasFoundEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::System::Action_1<f32>>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_spawnCenterDistanceWasFoundEvent", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_spawnCenterDistance(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_spawnCenterDistance", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_spawnCenterDistanceWasFound(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("get_spawnCenterDistanceWasFound", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn remove_spawnCenterDistanceWasFoundEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::System::Action_1<f32>>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_spawnCenterDistanceWasFoundEvent", (value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "BeatmapObjectSpawnCenter")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::BeatmapObjectSpawnCenter {
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
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _playerCount: i32,
    pub _distance: f32,
}
#[cfg(feature = "BeatmapObjectSpawnCenter+PlayerCountToDistance")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::BeatmapObjectSpawnCenter_PlayerCountToDistance {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "PlayerCountToDistance";
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
#[cfg(feature = "BeatmapObjectSpawnCenter+PlayerCountToDistance")]
impl std::ops::Deref
for crate::GlobalNamespace::BeatmapObjectSpawnCenter_PlayerCountToDistance {
    type Target = quest_hook::libil2cpp::Il2CppObject;
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
    pub fn get_distance(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_distance", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_playerCount(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_playerCount", ())?;
        Ok(__cordl_ret.into())
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
