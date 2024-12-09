#[cfg(
    feature = "BGLib+AppFlow+Initialization+CustomizableEnvironmentCommandLineArgsProvider"
)]
#[repr(C)]
#[derive(Debug)]
pub struct CustomizableEnvironmentCommandLineArgsProvider {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(
    feature = "BGLib+AppFlow+Initialization+CustomizableEnvironmentCommandLineArgsProvider"
)]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::BGLib::AppFlow::Initialization::CustomizableEnvironmentCommandLineArgsProvider =>
    "BGLib.AppFlow.Initialization"."CustomizableEnvironmentCommandLineArgsProvider"
);
#[cfg(
    feature = "BGLib+AppFlow+Initialization+CustomizableEnvironmentCommandLineArgsProvider"
)]
impl std::ops::Deref
for crate::BGLib::AppFlow::Initialization::CustomizableEnvironmentCommandLineArgsProvider {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(
    feature = "BGLib+AppFlow+Initialization+CustomizableEnvironmentCommandLineArgsProvider"
)]
impl std::ops::DerefMut
for crate::BGLib::AppFlow::Initialization::CustomizableEnvironmentCommandLineArgsProvider {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(
    feature = "BGLib+AppFlow+Initialization+CustomizableEnvironmentCommandLineArgsProvider"
)]
impl crate::BGLib::AppFlow::Initialization::CustomizableEnvironmentCommandLineArgsProvider {}
#[cfg(
    feature = "BGLib+AppFlow+Initialization+CustomizableEnvironmentCommandLineArgsProvider"
)]
impl quest_hook::libil2cpp::ObjectType
for crate::BGLib::AppFlow::Initialization::CustomizableEnvironmentCommandLineArgsProvider {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
