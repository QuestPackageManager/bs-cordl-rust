#[cfg(feature = "MultiplayerLocalActiveCenterRingLightsController")]
#[repr(C)]
#[derive(Debug)]
pub struct MultiplayerLocalActiveCenterRingLightsController {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
    pub _verticalLinePositions: *mut quest_hook::libil2cpp::Il2CppArray<f32>,
    pub _horizontalLines: *mut quest_hook::libil2cpp::Il2CppArray<
        *mut crate::UnityEngine::Transform,
    >,
    pub _centerResizeController: *mut crate::GlobalNamespace::MultiplayerCenterResizeController,
    pub _layoutProvider: *mut crate::GlobalNamespace::MultiplayerLayoutProvider,
    pub _beatmapObjectSpawnCenter: *mut crate::GlobalNamespace::BeatmapObjectSpawnCenter,
    pub _edgeDistanceFromCenterCalculated: bool,
    pub _spawnCenterDistanceFound: bool,
}
#[cfg(feature = "MultiplayerLocalActiveCenterRingLightsController")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::MultiplayerLocalActiveCenterRingLightsController => ""
    ."MultiplayerLocalActiveCenterRingLightsController"
);
#[cfg(feature = "MultiplayerLocalActiveCenterRingLightsController")]
impl std::ops::Deref
for crate::GlobalNamespace::MultiplayerLocalActiveCenterRingLightsController {
    type Target = crate::UnityEngine::MonoBehaviour;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "MultiplayerLocalActiveCenterRingLightsController")]
impl std::ops::DerefMut
for crate::GlobalNamespace::MultiplayerLocalActiveCenterRingLightsController {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "MultiplayerLocalActiveCenterRingLightsController")]
impl crate::GlobalNamespace::MultiplayerLocalActiveCenterRingLightsController {
    pub fn HandleEdgeDistanceFromCenterWasCalculated(
        &mut self,
        constructEdgeDistanceFromCenter: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "HandleEdgeDistanceFromCenterWasCalculated",
                (constructEdgeDistanceFromCenter),
            )?;
        Ok(__cordl_ret)
    }
    pub fn HandleSpawnCenterDistanceWasFound(
        &mut self,
        spawnCenterDistance: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleSpawnCenterDistanceWasFound", (spawnCenterDistance))?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn OnDestroy(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnDestroy", ())?;
        Ok(__cordl_ret)
    }
    pub fn Resize(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Resize", ())?;
        Ok(__cordl_ret)
    }
    pub fn Start(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Start", ())?;
        Ok(__cordl_ret)
    }
    pub fn TryResize(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("TryResize", ())?;
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
#[cfg(feature = "MultiplayerLocalActiveCenterRingLightsController")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::MultiplayerLocalActiveCenterRingLightsController {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
