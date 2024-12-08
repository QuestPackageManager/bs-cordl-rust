#[cfg(feature = "MultiplayerConnectedPlayerObstacleClippingController")]
#[repr(C)]
#[derive(Debug)]
pub struct MultiplayerConnectedPlayerObstacleClippingController {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
    pub _materialPropertyBlockControllers: *mut quest_hook::libil2cpp::Il2CppArray<
        *mut MaterialPropertyBlockController,
    >,
}
#[cfg(feature = "MultiplayerConnectedPlayerObstacleClippingController")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for MultiplayerConnectedPlayerObstacleClippingController =>
    ""."MultiplayerConnectedPlayerObstacleClippingController"
);
#[cfg(feature = "MultiplayerConnectedPlayerObstacleClippingController")]
impl std::ops::Deref for MultiplayerConnectedPlayerObstacleClippingController {
    type Target = crate::UnityEngine::MonoBehaviour;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "MultiplayerConnectedPlayerObstacleClippingController")]
impl std::ops::DerefMut for MultiplayerConnectedPlayerObstacleClippingController {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "MultiplayerConnectedPlayerObstacleClippingController")]
impl MultiplayerConnectedPlayerObstacleClippingController {
    pub fn SetClippingParams(
        &mut self,
        position: crate::UnityEngine::Vector3,
        normal: crate::UnityEngine::Vector3,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetClippingParams", (position, normal))?;
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
    pub fn New() -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "MultiplayerConnectedPlayerObstacleClippingController")]
impl quest_hook::libil2cpp::ObjectType
for MultiplayerConnectedPlayerObstacleClippingController {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
