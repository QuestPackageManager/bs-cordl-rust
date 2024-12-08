#[cfg(feature = "PlayerVRControllersManager")]
#[repr(C)]
#[derive(Debug)]
pub struct PlayerVRControllersManager {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
    pub _leftHandVRController: *mut crate::GlobalNamespace::VRController,
    pub _rightHandVRController: *mut crate::GlobalNamespace::VRController,
}
#[cfg(feature = "PlayerVRControllersManager")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::PlayerVRControllersManager =>
    ""."PlayerVRControllersManager"
);
#[cfg(feature = "PlayerVRControllersManager")]
impl std::ops::Deref for crate::GlobalNamespace::PlayerVRControllersManager {
    type Target = crate::UnityEngine::MonoBehaviour;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "PlayerVRControllersManager")]
impl std::ops::DerefMut for crate::GlobalNamespace::PlayerVRControllersManager {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "PlayerVRControllersManager")]
impl crate::GlobalNamespace::PlayerVRControllersManager {
    pub fn DisableAllVRControllers(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("DisableAllVRControllers", ())?;
        Ok(__cordl_ret)
    }
    pub fn EnableAllVRControllers(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("EnableAllVRControllers", ())?;
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
    pub fn get_leftHandVRController(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::GlobalNamespace::VRController> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::GlobalNamespace::VRController = __cordl_object
            .invoke("get_leftHandVRController", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_rightHandVRController(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::GlobalNamespace::VRController> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::GlobalNamespace::VRController = __cordl_object
            .invoke("get_rightHandVRController", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "PlayerVRControllersManager")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::PlayerVRControllersManager {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
