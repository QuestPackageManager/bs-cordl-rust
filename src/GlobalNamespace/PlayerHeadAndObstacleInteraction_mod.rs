#[cfg(feature = "PlayerHeadAndObstacleInteraction")]
#[repr(C)]
#[derive(Debug)]
pub struct PlayerHeadAndObstacleInteraction {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
    pub _playerTransforms: *mut crate::GlobalNamespace::PlayerTransforms,
    pub _beatmapObjectManager: *mut crate::GlobalNamespace::BeatmapObjectManager,
    pub headDidEnterObstaclesEvent: *mut crate::System::Action,
    pub headDidEnterObstacleEvent: *mut crate::System::Action_1<
        *mut crate::GlobalNamespace::ObstacleController,
    >,
    pub _lastFrameNumCheck: i32,
    pub _intersectingObstacles: *mut crate::System::Collections::Generic::HashSet_1<
        *mut crate::GlobalNamespace::ObstacleController,
    >,
    pub _prevFrameNumberOfIntersectingObstaclesCount: i32,
}
#[cfg(feature = "PlayerHeadAndObstacleInteraction")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::PlayerHeadAndObstacleInteraction => ""
    ."PlayerHeadAndObstacleInteraction"
);
#[cfg(feature = "PlayerHeadAndObstacleInteraction")]
impl std::ops::Deref for crate::GlobalNamespace::PlayerHeadAndObstacleInteraction {
    type Target = crate::UnityEngine::MonoBehaviour;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "PlayerHeadAndObstacleInteraction")]
impl std::ops::DerefMut for crate::GlobalNamespace::PlayerHeadAndObstacleInteraction {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "PlayerHeadAndObstacleInteraction")]
impl crate::GlobalNamespace::PlayerHeadAndObstacleInteraction {
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn RefreshIntersectingObstacles(
        &mut self,
        worldPos: crate::UnityEngine::Vector3,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RefreshIntersectingObstacles", (worldPos))?;
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
    pub fn add_headDidEnterObstacleEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<*mut crate::GlobalNamespace::ObstacleController>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_headDidEnterObstacleEvent", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn add_headDidEnterObstaclesEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::System::Action>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_headDidEnterObstaclesEvent", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_playerHeadIsInObstacle(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_playerHeadIsInObstacle", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn remove_headDidEnterObstacleEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<*mut crate::GlobalNamespace::ObstacleController>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_headDidEnterObstacleEvent", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn remove_headDidEnterObstaclesEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::System::Action>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_headDidEnterObstaclesEvent", (value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "PlayerHeadAndObstacleInteraction")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::PlayerHeadAndObstacleInteraction {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
