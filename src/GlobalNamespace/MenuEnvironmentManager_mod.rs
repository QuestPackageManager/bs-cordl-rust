#[cfg(feature = "MenuEnvironmentManager")]
#[repr(C)]
#[derive(Debug)]
pub struct MenuEnvironmentManager {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
    pub _data: *mut quest_hook::libil2cpp::Il2CppArray<
        *mut crate::GlobalNamespace::MenuEnvironmentManager_MenuEnvironmentObjects,
    >,
    pub _prevMenuEnvironmentType: crate::GlobalNamespace::MenuEnvironmentManager_MenuEnvironmentType,
}
#[cfg(feature = "MenuEnvironmentManager")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::MenuEnvironmentManager => ""
    ."MenuEnvironmentManager"
);
#[cfg(feature = "MenuEnvironmentManager")]
impl std::ops::Deref for crate::GlobalNamespace::MenuEnvironmentManager {
    type Target = crate::UnityEngine::MonoBehaviour;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "MenuEnvironmentManager")]
impl std::ops::DerefMut for crate::GlobalNamespace::MenuEnvironmentManager {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "MenuEnvironmentManager")]
impl crate::GlobalNamespace::MenuEnvironmentManager {
    #[cfg(feature = "MenuEnvironmentManager+MenuEnvironmentObjects")]
    pub type MenuEnvironmentObjects = crate::GlobalNamespace::MenuEnvironmentManager_MenuEnvironmentObjects;
    #[cfg(feature = "MenuEnvironmentManager+MenuEnvironmentType")]
    pub type MenuEnvironmentType = crate::GlobalNamespace::MenuEnvironmentManager_MenuEnvironmentType;
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn ShowEnvironmentType(
        &mut self,
        menuEnvironmentType: crate::GlobalNamespace::MenuEnvironmentManager_MenuEnvironmentType,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ShowEnvironmentType", (menuEnvironmentType))?;
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
#[cfg(feature = "MenuEnvironmentManager")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::MenuEnvironmentManager {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "MenuEnvironmentManager+MenuEnvironmentObjects")]
#[repr(C)]
#[derive(Debug)]
pub struct MenuEnvironmentManager_MenuEnvironmentObjects {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _menuEnvironmentType: crate::GlobalNamespace::MenuEnvironmentManager_MenuEnvironmentType,
    pub _wrapper: *mut crate::UnityEngine::GameObject,
}
#[cfg(feature = "MenuEnvironmentManager+MenuEnvironmentObjects")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::MenuEnvironmentManager_MenuEnvironmentObjects => ""
    ."MenuEnvironmentManager/MenuEnvironmentObjects"
);
#[cfg(feature = "MenuEnvironmentManager+MenuEnvironmentObjects")]
impl std::ops::Deref
for crate::GlobalNamespace::MenuEnvironmentManager_MenuEnvironmentObjects {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "MenuEnvironmentManager+MenuEnvironmentObjects")]
impl std::ops::DerefMut
for crate::GlobalNamespace::MenuEnvironmentManager_MenuEnvironmentObjects {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "MenuEnvironmentManager+MenuEnvironmentObjects")]
impl crate::GlobalNamespace::MenuEnvironmentManager_MenuEnvironmentObjects {
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
    pub fn get_menuEnvironmentType(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::GlobalNamespace::MenuEnvironmentManager_MenuEnvironmentType,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::GlobalNamespace::MenuEnvironmentManager_MenuEnvironmentType = __cordl_object
            .invoke("get_menuEnvironmentType", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_wrapper(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::UnityEngine::GameObject> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::GameObject = __cordl_object
            .invoke("get_wrapper", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "MenuEnvironmentManager+MenuEnvironmentObjects")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::MenuEnvironmentManager_MenuEnvironmentObjects {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "MenuEnvironmentManager+MenuEnvironmentType")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MenuEnvironmentManager_MenuEnvironmentType {
    Default = 1i32,
    Lobby = 2i32,
    None = 0i32,
}
#[cfg(feature = "MenuEnvironmentManager+MenuEnvironmentType")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::MenuEnvironmentManager_MenuEnvironmentType => ""
    ."MenuEnvironmentManager/MenuEnvironmentType"
);
