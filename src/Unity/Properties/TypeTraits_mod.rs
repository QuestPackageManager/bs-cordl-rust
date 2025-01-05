#[cfg(feature = "Unity+Properties+TypeTraits")]
#[repr(C)]
#[derive(Debug)]
pub struct TypeTraits {
    __cordl_parent: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
}
#[cfg(feature = "Unity+Properties+TypeTraits")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Unity::Properties::TypeTraits =>
    "Unity.Properties"."TypeTraits"
);
#[cfg(feature = "Unity+Properties+TypeTraits")]
impl std::ops::Deref for crate::Unity::Properties::TypeTraits {
    type Target = quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+Properties+TypeTraits")]
impl std::ops::DerefMut for crate::Unity::Properties::TypeTraits {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+Properties+TypeTraits")]
impl crate::Unity::Properties::TypeTraits {
    pub fn IsContainer(
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IsContainer", (_cordl_type))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Unity+Properties+TypeTraits")]
impl quest_hook::libil2cpp::ObjectType for crate::Unity::Properties::TypeTraits {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
