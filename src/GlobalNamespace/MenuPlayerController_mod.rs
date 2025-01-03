#[cfg(feature = "MenuPlayerController")]
#[repr(C)]
#[derive(Debug)]
pub struct MenuPlayerController {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
    pub _leftController: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::VRController>,
    pub _rightController: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::VRController,
    >,
    pub _headTransform: quest_hook::libil2cpp::Gc<crate::UnityEngine::Transform>,
}
#[cfg(feature = "MenuPlayerController")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::MenuPlayerController => ""
    ."MenuPlayerController"
);
#[cfg(feature = "MenuPlayerController")]
impl std::ops::Deref for crate::GlobalNamespace::MenuPlayerController {
    type Target = crate::UnityEngine::MonoBehaviour;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "MenuPlayerController")]
impl std::ops::DerefMut for crate::GlobalNamespace::MenuPlayerController {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "MenuPlayerController")]
impl crate::GlobalNamespace::MenuPlayerController {
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
    pub fn get_headPos(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector3> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Vector3 = __cordl_object
            .invoke("get_headPos", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_headRot(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Quaternion> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Quaternion = __cordl_object
            .invoke("get_headRot", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_leftController(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::VRController>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::VRController,
        > = __cordl_object.invoke("get_leftController", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_rightController(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::VRController>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::VRController,
        > = __cordl_object.invoke("get_rightController", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "MenuPlayerController")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::MenuPlayerController {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
