#[cfg(feature = "BGLib+DotnetExtension+EnvironmentVariableHelper")]
#[repr(C)]
#[derive(Debug)]
pub struct EnvironmentVariableHelper {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "BGLib+DotnetExtension+EnvironmentVariableHelper")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::BGLib::DotnetExtension::EnvironmentVariableHelper => "BGLib.DotnetExtension"
    ."EnvironmentVariableHelper"
);
#[cfg(feature = "BGLib+DotnetExtension+EnvironmentVariableHelper")]
impl std::ops::Deref for crate::BGLib::DotnetExtension::EnvironmentVariableHelper {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BGLib+DotnetExtension+EnvironmentVariableHelper")]
impl std::ops::DerefMut for crate::BGLib::DotnetExtension::EnvironmentVariableHelper {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BGLib+DotnetExtension+EnvironmentVariableHelper")]
impl crate::BGLib::DotnetExtension::EnvironmentVariableHelper {}
#[cfg(feature = "BGLib+DotnetExtension+EnvironmentVariableHelper")]
impl quest_hook::libil2cpp::ObjectType
for crate::BGLib::DotnetExtension::EnvironmentVariableHelper {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
