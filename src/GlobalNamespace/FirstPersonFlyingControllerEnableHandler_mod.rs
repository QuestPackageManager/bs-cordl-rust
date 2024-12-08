#[cfg(feature = "FirstPersonFlyingControllerEnableHandler")]
#[repr(C)]
#[derive(Debug)]
pub struct FirstPersonFlyingControllerEnableHandler {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
    pub _flyingController: *mut crate::GlobalNamespace::FirstPersonFlyingController,
    pub _commandLineParserResult: crate::BGLib::DotnetExtension::CommandLine::CommandLineParserResult,
}
#[cfg(feature = "FirstPersonFlyingControllerEnableHandler")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::FirstPersonFlyingControllerEnableHandler => ""
    ."FirstPersonFlyingControllerEnableHandler"
);
#[cfg(feature = "FirstPersonFlyingControllerEnableHandler")]
impl std::ops::Deref
for crate::GlobalNamespace::FirstPersonFlyingControllerEnableHandler {
    type Target = crate::UnityEngine::MonoBehaviour;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "FirstPersonFlyingControllerEnableHandler")]
impl std::ops::DerefMut
for crate::GlobalNamespace::FirstPersonFlyingControllerEnableHandler {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "FirstPersonFlyingControllerEnableHandler")]
impl crate::GlobalNamespace::FirstPersonFlyingControllerEnableHandler {
    pub fn InstallDependencies(
        &mut self,
        commandLineParserResult: crate::BGLib::DotnetExtension::CommandLine::CommandLineParserResult,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("InstallDependencies", (commandLineParserResult))?;
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
    pub fn get_flyingControllerEnabled(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("get_flyingControllerEnabled", ())?;
        Ok(__cordl_ret)
    }
    pub fn set_flyingControllerEnabled(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_flyingControllerEnabled", (value))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "FirstPersonFlyingControllerEnableHandler")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::FirstPersonFlyingControllerEnableHandler {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
