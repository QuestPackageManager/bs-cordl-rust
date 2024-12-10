#[cfg(feature = "MainCameraCullingMask")]
#[repr(C)]
#[derive(Debug)]
pub struct MainCameraCullingMask {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
    pub _camera: *mut crate::UnityEngine::Camera,
    pub _initData: *mut crate::GlobalNamespace::MainCameraCullingMask_InitData,
}
#[cfg(feature = "MainCameraCullingMask")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::MainCameraCullingMask => ""
    ."MainCameraCullingMask"
);
#[cfg(feature = "MainCameraCullingMask")]
impl std::ops::Deref for crate::GlobalNamespace::MainCameraCullingMask {
    type Target = crate::UnityEngine::MonoBehaviour;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "MainCameraCullingMask")]
impl std::ops::DerefMut for crate::GlobalNamespace::MainCameraCullingMask {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "MainCameraCullingMask")]
impl crate::GlobalNamespace::MainCameraCullingMask {
    #[cfg(feature = "MainCameraCullingMask+InitData")]
    pub type InitData = crate::GlobalNamespace::MainCameraCullingMask_InitData;
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn Start(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Start", ())?;
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
#[cfg(feature = "MainCameraCullingMask")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::MainCameraCullingMask {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "MainCameraCullingMask+InitData")]
#[repr(C)]
#[derive(Debug)]
pub struct MainCameraCullingMask_InitData {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub showDebris: bool,
}
#[cfg(feature = "MainCameraCullingMask+InitData")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::MainCameraCullingMask_InitData
    => ""."MainCameraCullingMask/InitData"
);
#[cfg(feature = "MainCameraCullingMask+InitData")]
impl std::ops::Deref for crate::GlobalNamespace::MainCameraCullingMask_InitData {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "MainCameraCullingMask+InitData")]
impl std::ops::DerefMut for crate::GlobalNamespace::MainCameraCullingMask_InitData {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "MainCameraCullingMask+InitData")]
impl crate::GlobalNamespace::MainCameraCullingMask_InitData {
    pub fn New(
        showDebris: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (showDebris))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        showDebris: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (showDebris))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "MainCameraCullingMask+InitData")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::MainCameraCullingMask_InitData {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
