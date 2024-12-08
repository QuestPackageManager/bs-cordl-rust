#[cfg(feature = "Unity+Collections+LowLevel+Unsafe+NativeSetThreadIndexAttribute")]
#[repr(C)]
#[derive(Debug)]
pub struct NativeSetThreadIndexAttribute {
    __cordl_parent: crate::System::Attribute,
}
#[cfg(feature = "Unity+Collections+LowLevel+Unsafe+NativeSetThreadIndexAttribute")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Unity::Collections::LowLevel::Unsafe::NativeSetThreadIndexAttribute =>
    "Unity.Collections.LowLevel.Unsafe"."NativeSetThreadIndexAttribute"
);
#[cfg(feature = "Unity+Collections+LowLevel+Unsafe+NativeSetThreadIndexAttribute")]
impl std::ops::Deref
for crate::Unity::Collections::LowLevel::Unsafe::NativeSetThreadIndexAttribute {
    type Target = crate::System::Attribute;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+Collections+LowLevel+Unsafe+NativeSetThreadIndexAttribute")]
impl std::ops::DerefMut
for crate::Unity::Collections::LowLevel::Unsafe::NativeSetThreadIndexAttribute {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+Collections+LowLevel+Unsafe+NativeSetThreadIndexAttribute")]
impl crate::Unity::Collections::LowLevel::Unsafe::NativeSetThreadIndexAttribute {}
#[cfg(feature = "Unity+Collections+LowLevel+Unsafe+NativeSetThreadIndexAttribute")]
impl quest_hook::libil2cpp::ObjectType
for crate::Unity::Collections::LowLevel::Unsafe::NativeSetThreadIndexAttribute {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
