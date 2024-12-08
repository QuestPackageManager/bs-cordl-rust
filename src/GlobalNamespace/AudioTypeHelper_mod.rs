#[cfg(feature = "AudioTypeHelper")]
#[repr(C)]
#[derive(Debug)]
pub struct AudioTypeHelper {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "AudioTypeHelper")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for AudioTypeHelper => ""."AudioTypeHelper"
);
#[cfg(feature = "AudioTypeHelper")]
impl std::ops::Deref for AudioTypeHelper {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "AudioTypeHelper")]
impl std::ops::DerefMut for AudioTypeHelper {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "AudioTypeHelper")]
impl AudioTypeHelper {}
#[cfg(feature = "AudioTypeHelper")]
impl quest_hook::libil2cpp::ObjectType for AudioTypeHelper {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
