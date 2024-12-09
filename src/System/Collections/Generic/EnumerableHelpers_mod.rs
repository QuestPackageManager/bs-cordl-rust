#[cfg(feature = "System+Collections+Generic+EnumerableHelpers")]
#[repr(C)]
#[derive(Debug)]
pub struct EnumerableHelpers {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "System+Collections+Generic+EnumerableHelpers")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Collections::Generic::EnumerableHelpers
    => "System.Collections.Generic"."EnumerableHelpers"
);
#[cfg(feature = "System+Collections+Generic+EnumerableHelpers")]
impl std::ops::Deref for crate::System::Collections::Generic::EnumerableHelpers {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Collections+Generic+EnumerableHelpers")]
impl std::ops::DerefMut for crate::System::Collections::Generic::EnumerableHelpers {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Collections+Generic+EnumerableHelpers")]
impl crate::System::Collections::Generic::EnumerableHelpers {}
#[cfg(feature = "System+Collections+Generic+EnumerableHelpers")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Collections::Generic::EnumerableHelpers {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
