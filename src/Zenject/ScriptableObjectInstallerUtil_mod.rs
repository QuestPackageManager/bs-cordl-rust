#[cfg(feature = "Zenject+ScriptableObjectInstallerUtil")]
#[repr(C)]
#[derive(Debug)]
pub struct ScriptableObjectInstallerUtil {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "Zenject+ScriptableObjectInstallerUtil")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Zenject::ScriptableObjectInstallerUtil =>
    "Zenject"."ScriptableObjectInstallerUtil"
);
#[cfg(feature = "Zenject+ScriptableObjectInstallerUtil")]
impl std::ops::Deref for crate::Zenject::ScriptableObjectInstallerUtil {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Zenject+ScriptableObjectInstallerUtil")]
impl std::ops::DerefMut for crate::Zenject::ScriptableObjectInstallerUtil {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Zenject+ScriptableObjectInstallerUtil")]
impl crate::Zenject::ScriptableObjectInstallerUtil {
    pub fn CreateInstaller<TInstaller>(
        resourcePath: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        container: quest_hook::libil2cpp::Gc<crate::Zenject::DiContainer>,
    ) -> quest_hook::libil2cpp::Result<TInstaller>
    where
        TInstaller: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: TInstaller = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CreateInstaller", (resourcePath, container))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetDefaultResourcePath<TInstaller>() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    >
    where
        TInstaller: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetDefaultResourcePath", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Zenject+ScriptableObjectInstallerUtil")]
impl quest_hook::libil2cpp::ObjectType
for crate::Zenject::ScriptableObjectInstallerUtil {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
