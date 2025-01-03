#[cfg(feature = "UnitySpecificRandomExtensions")]
#[repr(C)]
#[derive(Debug)]
pub struct UnitySpecificRandomExtensions {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "UnitySpecificRandomExtensions")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::UnitySpecificRandomExtensions
    => ""."UnitySpecificRandomExtensions"
);
#[cfg(feature = "UnitySpecificRandomExtensions")]
impl std::ops::Deref for crate::GlobalNamespace::UnitySpecificRandomExtensions {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnitySpecificRandomExtensions")]
impl std::ops::DerefMut for crate::GlobalNamespace::UnitySpecificRandomExtensions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnitySpecificRandomExtensions")]
impl crate::GlobalNamespace::UnitySpecificRandomExtensions {
    pub fn InsideUnitSphere(
        random: quest_hook::libil2cpp::Gc<crate::System::Random>,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector3> {
        let __cordl_ret: crate::UnityEngine::Vector3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("InsideUnitSphere", (random))?;
        Ok(__cordl_ret.into())
    }
    pub fn OnUnitSphere(
        random: quest_hook::libil2cpp::Gc<crate::System::Random>,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector3> {
        let __cordl_ret: crate::UnityEngine::Vector3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("OnUnitSphere", (random))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnitySpecificRandomExtensions")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::UnitySpecificRandomExtensions {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
