#[cfg(feature = "BGLib+DotnetExtension+EnvironmentVariableHelper")]
#[repr(C)]
#[derive(Debug)]
pub struct EnvironmentVariableHelper {
    __cordl_parent: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
}
#[cfg(feature = "BGLib+DotnetExtension+EnvironmentVariableHelper")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::BGLib::DotnetExtension::EnvironmentVariableHelper => "BGLib.DotnetExtension"
    ."EnvironmentVariableHelper"
);
#[cfg(feature = "BGLib+DotnetExtension+EnvironmentVariableHelper")]
impl std::ops::Deref for crate::BGLib::DotnetExtension::EnvironmentVariableHelper {
    type Target = quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>;
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
impl crate::BGLib::DotnetExtension::EnvironmentVariableHelper {
    pub fn GetDirectoryPath(
        variableName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetDirectoryPath", (variableName))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetDirectoryPath(
        variableName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        path: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SetDirectoryPath", (variableName, path))?;
        Ok(__cordl_ret.into())
    }
}
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
