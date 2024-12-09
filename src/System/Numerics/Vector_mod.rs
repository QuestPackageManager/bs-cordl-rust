#[cfg(feature = "System+Numerics+Vector")]
#[repr(C)]
#[derive(Debug)]
pub struct Vector {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "System+Numerics+Vector")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Numerics::Vector => "System.Numerics"
    ."Vector"
);
#[cfg(feature = "System+Numerics+Vector")]
impl std::ops::Deref for crate::System::Numerics::Vector {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Numerics+Vector")]
impl std::ops::DerefMut for crate::System::Numerics::Vector {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Numerics+Vector")]
impl crate::System::Numerics::Vector {}
#[cfg(feature = "System+Numerics+Vector")]
impl quest_hook::libil2cpp::ObjectType for crate::System::Numerics::Vector {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
