#[cfg(feature = "UnitySpecificRandomExtensions")]
#[repr(C)]
#[derive(Debug)]
pub struct UnitySpecificRandomExtensions {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "UnitySpecificRandomExtensions")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::UnitySpecificRandomExtensions {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "UnitySpecificRandomExtensions";
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.class().is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_argument(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
    fn matches_value_parameter(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
}
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
