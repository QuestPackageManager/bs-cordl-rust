#[cfg(feature = "OffsetDirectionExtensions")]
#[repr(C)]
#[derive(Debug)]
pub struct OffsetDirectionExtensions {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "OffsetDirectionExtensions")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for OffsetDirectionExtensions => ""
    ."OffsetDirectionExtensions"
);
#[cfg(feature = "OffsetDirectionExtensions")]
impl std::ops::Deref for OffsetDirectionExtensions {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OffsetDirectionExtensions")]
impl std::ops::DerefMut for OffsetDirectionExtensions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OffsetDirectionExtensions")]
impl OffsetDirectionExtensions {}
#[cfg(feature = "OffsetDirectionExtensions")]
impl quest_hook::libil2cpp::ObjectType for OffsetDirectionExtensions {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
