#[cfg(feature = "UnityEngine+ResourceManagement+Util+LocationUtils")]
#[repr(C)]
#[derive(Debug)]
pub struct LocationUtils {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "UnityEngine+ResourceManagement+Util+LocationUtils")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::ResourceManagement::Util::LocationUtils =>
    "UnityEngine.ResourceManagement.Util"."LocationUtils"
);
#[cfg(feature = "UnityEngine+ResourceManagement+Util+LocationUtils")]
impl std::ops::Deref for crate::UnityEngine::ResourceManagement::Util::LocationUtils {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+ResourceManagement+Util+LocationUtils")]
impl std::ops::DerefMut for crate::UnityEngine::ResourceManagement::Util::LocationUtils {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+ResourceManagement+Util+LocationUtils")]
impl crate::UnityEngine::ResourceManagement::Util::LocationUtils {
    pub fn DependenciesEqual(
        deps1: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IList_1<
                quest_hook::libil2cpp::Gc<
                    crate::UnityEngine::ResourceManagement::ResourceLocations::IResourceLocation,
                >,
            >,
        >,
        deps2: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IList_1<
                quest_hook::libil2cpp::Gc<
                    crate::UnityEngine::ResourceManagement::ResourceLocations::IResourceLocation,
                >,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("DependenciesEqual", (deps1, deps2))?;
        Ok(__cordl_ret.into())
    }
    pub fn LocationEquals(
        loc1: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::ResourceManagement::ResourceLocations::IResourceLocation,
        >,
        loc2: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::ResourceManagement::ResourceLocations::IResourceLocation,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("LocationEquals", (loc1, loc2))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+ResourceManagement+Util+LocationUtils")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::ResourceManagement::Util::LocationUtils {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
