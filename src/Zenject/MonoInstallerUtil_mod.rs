#[cfg(feature = "Zenject+MonoInstallerUtil")]
#[repr(C)]
#[derive(Debug)]
pub struct MonoInstallerUtil {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "Zenject+MonoInstallerUtil")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Zenject::MonoInstallerUtil => "Zenject"
    ."MonoInstallerUtil"
);
#[cfg(feature = "Zenject+MonoInstallerUtil")]
impl std::ops::Deref for crate::Zenject::MonoInstallerUtil {
    type Target = crate::System::Object;
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
impl crate::Zenject::MonoInstallerUtil {}
#[cfg(feature = "Zenject+MonoInstallerUtil")]
impl quest_hook::libil2cpp::ObjectType for crate::Zenject::MonoInstallerUtil {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
