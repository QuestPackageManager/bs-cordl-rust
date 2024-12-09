#[cfg(feature = "System+ParseNumbers")]
#[repr(C)]
#[derive(Debug)]
pub struct ParseNumbers {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "System+ParseNumbers")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::ParseNumbers => "System"."ParseNumbers"
);
#[cfg(feature = "System+ParseNumbers")]
impl std::ops::Deref for crate::System::ParseNumbers {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+ParseNumbers")]
impl std::ops::DerefMut for crate::System::ParseNumbers {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+ParseNumbers")]
impl crate::System::ParseNumbers {}
#[cfg(feature = "System+ParseNumbers")]
impl quest_hook::libil2cpp::ObjectType for crate::System::ParseNumbers {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
