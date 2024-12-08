#[cfg(feature = "Saber")]
#[repr(C)]
#[derive(Debug)]
pub struct Saber {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
    pub _saberBladeTopTransform: *mut crate::UnityEngine::Transform,
    pub _saberBladeBottomTransform: *mut crate::UnityEngine::Transform,
    pub _handleTransform: *mut crate::UnityEngine::Transform,
    pub _saberType: *mut crate::GlobalNamespace::SaberTypeObject,
    pub _movementData: *mut crate::GlobalNamespace::SaberMovementData,
    pub _saberBladeTopPos: crate::UnityEngine::Vector3,
    pub _saberBladeBottomPos: crate::UnityEngine::Vector3,
    pub _handlePos: crate::UnityEngine::Vector3,
    pub _handleRot: crate::UnityEngine::Quaternion,
}
#[cfg(feature = "Saber")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::Saber => ""."Saber"
);
#[cfg(feature = "Saber")]
impl std::ops::Deref for crate::GlobalNamespace::Saber {
    type Target = crate::UnityEngine::MonoBehaviour;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Saber")]
impl std::ops::DerefMut for crate::GlobalNamespace::Saber {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Saber")]
impl crate::GlobalNamespace::Saber {
    pub fn ManualUpdate(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ManualUpdate", ())?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn OverridePositionAndRotation(
        &mut self,
        pos: crate::UnityEngine::Vector3,
        rot: crate::UnityEngine::Quaternion,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OverridePositionAndRotation", (pos, rot))?;
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
    pub fn get_bladeSpeed(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_bladeSpeed", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_handlePos(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector3> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Vector3 = __cordl_object
            .invoke("get_handlePos", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_handleRot(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Quaternion> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Quaternion = __cordl_object
            .invoke("get_handleRot", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_movementData(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::GlobalNamespace::SaberMovementData> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::GlobalNamespace::SaberMovementData = __cordl_object
            .invoke("get_movementData", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_saberBladeBottomPos(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector3> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Vector3 = __cordl_object
            .invoke("get_saberBladeBottomPos", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_saberBladeTopPos(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector3> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Vector3 = __cordl_object
            .invoke("get_saberBladeTopPos", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_saberType(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::SaberType> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::GlobalNamespace::SaberType = __cordl_object
            .invoke("get_saberType", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "Saber")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::Saber {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
