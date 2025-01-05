#[cfg(feature = "Unity+Collections+NativeFixedLengthAttribute")]
#[repr(C)]
#[derive(Debug)]
pub struct NativeFixedLengthAttribute {
    __cordl_parent: quest_hook::libil2cpp::Gc<crate::System::Attribute>,
}
#[cfg(feature = "Unity+Collections+NativeFixedLengthAttribute")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Unity::Collections::NativeFixedLengthAttribute
    => "Unity.Collections"."NativeFixedLengthAttribute"
);
#[cfg(feature = "Unity+Collections+NativeFixedLengthAttribute")]
impl std::ops::Deref for crate::Unity::Collections::NativeFixedLengthAttribute {
    type Target = quest_hook::libil2cpp::Gc<crate::System::Attribute>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+Collections+NativeFixedLengthAttribute")]
impl std::ops::DerefMut for crate::Unity::Collections::NativeFixedLengthAttribute {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+Collections+NativeFixedLengthAttribute")]
impl crate::Unity::Collections::NativeFixedLengthAttribute {}
#[cfg(feature = "Unity+Collections+NativeFixedLengthAttribute")]
impl quest_hook::libil2cpp::ObjectType
for crate::Unity::Collections::NativeFixedLengthAttribute {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
