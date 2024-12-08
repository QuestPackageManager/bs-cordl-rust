#[cfg(feature = "Zenject+ScriptableObjectInstallerUtil")]
#[repr(C)]
#[derive(Debug)]
pub struct ScriptableObjectInstallerUtil {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "Zenject+ScriptableObjectInstallerUtil")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Zenject::ScriptableObjectInstallerUtil =>
    "Zenject"."ScriptableObjectInstallerUtil"
);
#[cfg(feature = "Zenject+ScriptableObjectInstallerUtil")]
impl std::ops::Deref for crate::Zenject::ScriptableObjectInstallerUtil {
    type Target = crate::System::Object;
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
impl crate::Zenject::ScriptableObjectInstallerUtil {}
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
