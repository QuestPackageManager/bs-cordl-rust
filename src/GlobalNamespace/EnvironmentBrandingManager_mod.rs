#[cfg(feature = "EnvironmentBrandingManager")]
#[repr(C)]
#[derive(Debug)]
pub struct EnvironmentBrandingManager {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
    pub _brandingObjects: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<*mut crate::UnityEngine::GameObject>,
    >,
    pub _replacementBrandingObjects: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<*mut crate::UnityEngine::GameObject>,
    >,
    pub _initData: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::EnvironmentBrandingManager_InitData,
    >,
}
#[cfg(feature = "EnvironmentBrandingManager")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::EnvironmentBrandingManager =>
    ""."EnvironmentBrandingManager"
);
#[cfg(feature = "EnvironmentBrandingManager")]
impl std::ops::Deref for crate::GlobalNamespace::EnvironmentBrandingManager {
    type Target = crate::UnityEngine::MonoBehaviour;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "EnvironmentBrandingManager")]
impl std::ops::DerefMut for crate::GlobalNamespace::EnvironmentBrandingManager {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "EnvironmentBrandingManager")]
impl crate::GlobalNamespace::EnvironmentBrandingManager {
    #[cfg(feature = "EnvironmentBrandingManager+InitData")]
    pub type InitData = crate::GlobalNamespace::EnvironmentBrandingManager_InitData;
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
#[cfg(feature = "EnvironmentBrandingManager")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::EnvironmentBrandingManager {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "EnvironmentBrandingManager+InitData")]
#[repr(C)]
#[derive(Debug)]
pub struct EnvironmentBrandingManager_InitData {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub hideBranding: bool,
}
#[cfg(feature = "EnvironmentBrandingManager+InitData")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::EnvironmentBrandingManager_InitData => ""
    ."EnvironmentBrandingManager/InitData"
);
#[cfg(feature = "EnvironmentBrandingManager+InitData")]
impl std::ops::Deref for crate::GlobalNamespace::EnvironmentBrandingManager_InitData {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "EnvironmentBrandingManager+InitData")]
impl std::ops::DerefMut for crate::GlobalNamespace::EnvironmentBrandingManager_InitData {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "EnvironmentBrandingManager+InitData")]
impl crate::GlobalNamespace::EnvironmentBrandingManager_InitData {
    pub fn New(
        hideBranding: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (hideBranding))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        hideBranding: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (hideBranding))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "EnvironmentBrandingManager+InitData")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::EnvironmentBrandingManager_InitData {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
