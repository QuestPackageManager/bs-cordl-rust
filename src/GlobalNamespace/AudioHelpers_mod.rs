#[cfg(feature = "AudioHelpers")]
#[repr(C)]
#[derive(Debug)]
pub struct AudioHelpers {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "AudioHelpers")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for AudioHelpers => ""."AudioHelpers"
);
#[cfg(feature = "AudioHelpers")]
impl std::ops::Deref for AudioHelpers {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "AudioHelpers")]
impl std::ops::DerefMut for AudioHelpers {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "AudioHelpers")]
impl AudioHelpers {}
#[cfg(feature = "AudioHelpers")]
impl quest_hook::libil2cpp::ObjectType for AudioHelpers {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
