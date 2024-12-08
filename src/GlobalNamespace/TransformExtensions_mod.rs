#[cfg(feature = "TransformExtensions")]
#[repr(C)]
#[derive(Debug)]
pub struct TransformExtensions {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "TransformExtensions")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for TransformExtensions => ""."TransformExtensions"
);
#[cfg(feature = "TransformExtensions")]
impl std::ops::Deref for TransformExtensions {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "TransformExtensions")]
impl std::ops::DerefMut for TransformExtensions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "TransformExtensions")]
impl TransformExtensions {}
#[cfg(feature = "TransformExtensions")]
impl quest_hook::libil2cpp::ObjectType for TransformExtensions {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
