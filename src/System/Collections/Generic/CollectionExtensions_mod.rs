#[cfg(feature = "System+Collections+Generic+CollectionExtensions")]
#[repr(C)]
#[derive(Debug)]
pub struct CollectionExtensions {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "System+Collections+Generic+CollectionExtensions")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::Collections::Generic::CollectionExtensions => "System.Collections.Generic"
    ."CollectionExtensions"
);
#[cfg(feature = "System+Collections+Generic+CollectionExtensions")]
impl std::ops::Deref for crate::System::Collections::Generic::CollectionExtensions {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Collections+Generic+CollectionExtensions")]
impl std::ops::DerefMut for crate::System::Collections::Generic::CollectionExtensions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Collections+Generic+CollectionExtensions")]
impl crate::System::Collections::Generic::CollectionExtensions {}
#[cfg(feature = "System+Collections+Generic+CollectionExtensions")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Collections::Generic::CollectionExtensions {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
