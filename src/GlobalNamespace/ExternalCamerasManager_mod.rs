#[cfg(feature = "ExternalCamerasManager")]
#[repr(C)]
#[derive(Debug)]
pub struct ExternalCamerasManager {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
    pub _oculusMRCManager: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::OculusMRCManager,
    >,
    pub _mrcBackgroundCameraPrefab: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::Camera,
    >,
    pub _mrcForegroundCameraPrefab: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::Camera,
    >,
}
#[cfg(feature = "ExternalCamerasManager")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::ExternalCamerasManager => ""
    ."ExternalCamerasManager"
);
#[cfg(feature = "ExternalCamerasManager")]
impl std::ops::Deref for crate::GlobalNamespace::ExternalCamerasManager {
    type Target = crate::UnityEngine::MonoBehaviour;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "ExternalCamerasManager")]
impl std::ops::DerefMut for crate::GlobalNamespace::ExternalCamerasManager {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "ExternalCamerasManager")]
impl crate::GlobalNamespace::ExternalCamerasManager {
    pub fn InstantiateMixedRealityBackgroundCameraGameObject(
        &mut self,
        mainCameraGameObject: quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject> = __cordl_object
            .invoke(
                "InstantiateMixedRealityBackgroundCameraGameObject",
                (mainCameraGameObject),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn InstantiateMixedRealityForegroundCameraGameObject(
        &mut self,
        mainCameraGameObject: quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject> = __cordl_object
            .invoke(
                "InstantiateMixedRealityForegroundCameraGameObject",
                (mainCameraGameObject),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn OnEnable(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnEnable", ())?;
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
#[cfg(feature = "ExternalCamerasManager")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::ExternalCamerasManager {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
