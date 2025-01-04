#[cfg(feature = "AppInitScenesTransitionSetupDataSO")]
#[repr(C)]
#[derive(Debug)]
pub struct AppInitScenesTransitionSetupDataSO {
    __cordl_parent: crate::GlobalNamespace::SingleFixedSceneScenesTransitionSetupDataSO,
}
#[cfg(feature = "AppInitScenesTransitionSetupDataSO")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::AppInitScenesTransitionSetupDataSO => ""
    ."AppInitScenesTransitionSetupDataSO"
);
#[cfg(feature = "AppInitScenesTransitionSetupDataSO")]
impl std::ops::Deref for crate::GlobalNamespace::AppInitScenesTransitionSetupDataSO {
    type Target = crate::GlobalNamespace::SingleFixedSceneScenesTransitionSetupDataSO;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "AppInitScenesTransitionSetupDataSO")]
impl std::ops::DerefMut for crate::GlobalNamespace::AppInitScenesTransitionSetupDataSO {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "AppInitScenesTransitionSetupDataSO")]
impl crate::GlobalNamespace::AppInitScenesTransitionSetupDataSO {
    #[cfg(feature = "AppInitScenesTransitionSetupDataSO+AppInitOverrideStartType")]
    pub type AppInitOverrideStartType = crate::GlobalNamespace::AppInitScenesTransitionSetupDataSO_AppInitOverrideStartType;
    #[cfg(feature = "AppInitScenesTransitionSetupDataSO+AppInitSceneSetupData")]
    pub type AppInitSceneSetupData = crate::GlobalNamespace::AppInitScenesTransitionSetupDataSO_AppInitSceneSetupData;
    pub fn Init(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Init", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn InitAsAppStart(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("InitAsAppStart", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn __Init(
        &mut self,
        appInitOverrideStartType: crate::GlobalNamespace::AppInitScenesTransitionSetupDataSO_AppInitOverrideStartType,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("__Init", (appInitOverrideStartType))?;
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
#[cfg(feature = "AppInitScenesTransitionSetupDataSO")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::AppInitScenesTransitionSetupDataSO {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "AppInitScenesTransitionSetupDataSO+AppInitOverrideStartType")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum AppInitScenesTransitionSetupDataSO_AppInitOverrideStartType {
    #[default]
    AppRestart = 2i32,
    AppStart = 1i32,
    DoNotOverride = 0i32,
    MultiSceneEditor = 3i32,
}
#[cfg(feature = "AppInitScenesTransitionSetupDataSO+AppInitOverrideStartType")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::AppInitScenesTransitionSetupDataSO_AppInitOverrideStartType => ""
    ."AppInitScenesTransitionSetupDataSO/AppInitOverrideStartType"
);
#[cfg(feature = "AppInitScenesTransitionSetupDataSO+AppInitSceneSetupData")]
#[repr(C)]
#[derive(Debug)]
pub struct AppInitScenesTransitionSetupDataSO_AppInitSceneSetupData {
    __cordl_parent: crate::GlobalNamespace::SceneSetupData,
    pub _appInitOverrideStartType_k__BackingField: crate::GlobalNamespace::AppInitScenesTransitionSetupDataSO_AppInitOverrideStartType,
}
#[cfg(feature = "AppInitScenesTransitionSetupDataSO+AppInitSceneSetupData")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::AppInitScenesTransitionSetupDataSO_AppInitSceneSetupData => ""
    ."AppInitScenesTransitionSetupDataSO/AppInitSceneSetupData"
);
#[cfg(feature = "AppInitScenesTransitionSetupDataSO+AppInitSceneSetupData")]
impl std::ops::Deref
for crate::GlobalNamespace::AppInitScenesTransitionSetupDataSO_AppInitSceneSetupData {
    type Target = crate::GlobalNamespace::SceneSetupData;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "AppInitScenesTransitionSetupDataSO+AppInitSceneSetupData")]
impl std::ops::DerefMut
for crate::GlobalNamespace::AppInitScenesTransitionSetupDataSO_AppInitSceneSetupData {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "AppInitScenesTransitionSetupDataSO+AppInitSceneSetupData")]
impl crate::GlobalNamespace::AppInitScenesTransitionSetupDataSO_AppInitSceneSetupData {
    pub fn New(
        appInitOverrideStartType: crate::GlobalNamespace::AppInitScenesTransitionSetupDataSO_AppInitOverrideStartType,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (appInitOverrideStartType))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        appInitOverrideStartType: crate::GlobalNamespace::AppInitScenesTransitionSetupDataSO_AppInitOverrideStartType,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (appInitOverrideStartType))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_appInitOverrideStartType(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::GlobalNamespace::AppInitScenesTransitionSetupDataSO_AppInitOverrideStartType,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::GlobalNamespace::AppInitScenesTransitionSetupDataSO_AppInitOverrideStartType = __cordl_object
            .invoke("get_appInitOverrideStartType", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn set_appInitOverrideStartType(
        &mut self,
        value: crate::GlobalNamespace::AppInitScenesTransitionSetupDataSO_AppInitOverrideStartType,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_appInitOverrideStartType", (value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "AppInitScenesTransitionSetupDataSO+AppInitSceneSetupData")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::AppInitScenesTransitionSetupDataSO_AppInitSceneSetupData {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
