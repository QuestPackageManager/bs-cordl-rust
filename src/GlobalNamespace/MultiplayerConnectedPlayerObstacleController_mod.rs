#[cfg(feature = "MultiplayerConnectedPlayerObstacleController")]
#[repr(C)]
#[derive(Debug)]
pub struct MultiplayerConnectedPlayerObstacleController {
    __cordl_parent: ObstacleController,
    pub _multiplayerConnectedPlayerObstacleClippingController: *mut MultiplayerConnectedPlayerObstacleClippingController,
}
#[cfg(feature = "MultiplayerConnectedPlayerObstacleController")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for MultiplayerConnectedPlayerObstacleController => ""
    ."MultiplayerConnectedPlayerObstacleController"
);
#[cfg(feature = "MultiplayerConnectedPlayerObstacleController")]
impl std::ops::Deref for MultiplayerConnectedPlayerObstacleController {
    type Target = ObstacleController;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "MultiplayerConnectedPlayerObstacleController")]
impl std::ops::DerefMut for MultiplayerConnectedPlayerObstacleController {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "MultiplayerConnectedPlayerObstacleController")]
impl MultiplayerConnectedPlayerObstacleController {
    #[cfg(feature = "MultiplayerConnectedPlayerObstacleController+Pool")]
    pub type Pool = crate::GlobalNamespace::MultiplayerConnectedPlayerObstacleController_Pool;
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
    pub fn Init(
        &mut self,
        obstacleData: *mut ObstacleData,
        worldRotation: f32,
        startPos: crate::UnityEngine::Vector3,
        midPos: crate::UnityEngine::Vector3,
        endPos: crate::UnityEngine::Vector3,
        move1Duration: f32,
        move2Duration: f32,
        singleLineWidth: f32,
        height: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "Init",
                (
                    obstacleData,
                    worldRotation,
                    startPos,
                    midPos,
                    endPos,
                    move1Duration,
                    move2Duration,
                    singleLineWidth,
                    height,
                ),
            )?;
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
#[cfg(feature = "MultiplayerConnectedPlayerObstacleController")]
impl quest_hook::libil2cpp::ObjectType for MultiplayerConnectedPlayerObstacleController {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "MultiplayerConnectedPlayerObstacleController+Pool")]
#[repr(C)]
#[derive(Debug)]
pub struct MultiplayerConnectedPlayerObstacleController_Pool {
    __cordl_parent: crate::Zenject::MonoMemoryPool_1<
        *mut MultiplayerConnectedPlayerObstacleController,
    >,
}
#[cfg(feature = "MultiplayerConnectedPlayerObstacleController+Pool")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::MultiplayerConnectedPlayerObstacleController_Pool => ""
    ."MultiplayerConnectedPlayerObstacleController/Pool"
);
#[cfg(feature = "MultiplayerConnectedPlayerObstacleController+Pool")]
impl std::ops::Deref
for crate::GlobalNamespace::MultiplayerConnectedPlayerObstacleController_Pool {
    type Target = crate::Zenject::MonoMemoryPool_1<
        *mut MultiplayerConnectedPlayerObstacleController,
    >;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "MultiplayerConnectedPlayerObstacleController+Pool")]
impl std::ops::DerefMut
for crate::GlobalNamespace::MultiplayerConnectedPlayerObstacleController_Pool {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "MultiplayerConnectedPlayerObstacleController+Pool")]
impl crate::GlobalNamespace::MultiplayerConnectedPlayerObstacleController_Pool {
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
    pub fn New() -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "MultiplayerConnectedPlayerObstacleController+Pool")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::MultiplayerConnectedPlayerObstacleController_Pool {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
