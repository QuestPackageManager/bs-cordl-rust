#[cfg(feature = "Zenject+MonoInstallerUtil")]
#[repr(C)]
#[derive(Debug)]
pub struct MonoInstallerUtil {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "Zenject+MonoInstallerUtil")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Zenject::MonoInstallerUtil => "Zenject"
    ."MonoInstallerUtil"
);
#[cfg(feature = "Zenject+MonoInstallerUtil")]
impl std::ops::Deref for crate::Zenject::MonoInstallerUtil {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Zenject+MonoInstallerUtil")]
impl std::ops::DerefMut for crate::Zenject::MonoInstallerUtil {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Zenject+MonoInstallerUtil")]
impl crate::Zenject::MonoInstallerUtil {
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
#[cfg(feature = "Zenject+MonoInstallerUtil")]
impl quest_hook::libil2cpp::ObjectType for crate::Zenject::MonoInstallerUtil {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
