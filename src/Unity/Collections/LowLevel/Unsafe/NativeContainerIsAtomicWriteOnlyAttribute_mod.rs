#[cfg(
    feature = "Unity+Collections+LowLevel+Unsafe+NativeContainerIsAtomicWriteOnlyAttribute"
)]
#[repr(C)]
#[derive(Debug)]
pub struct NativeContainerIsAtomicWriteOnlyAttribute {
    __cordl_parent: crate::System::Attribute,
}
#[cfg(
    feature = "Unity+Collections+LowLevel+Unsafe+NativeContainerIsAtomicWriteOnlyAttribute"
)]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Unity::Collections::LowLevel::Unsafe::NativeContainerIsAtomicWriteOnlyAttribute =>
    "Unity.Collections.LowLevel.Unsafe"."NativeContainerIsAtomicWriteOnlyAttribute"
);
#[cfg(
    feature = "Unity+Collections+LowLevel+Unsafe+NativeContainerIsAtomicWriteOnlyAttribute"
)]
impl std::ops::Deref
for crate::Unity::Collections::LowLevel::Unsafe::NativeContainerIsAtomicWriteOnlyAttribute {
    type Target = crate::System::Attribute;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(
    feature = "Unity+Collections+LowLevel+Unsafe+NativeContainerIsAtomicWriteOnlyAttribute"
)]
impl std::ops::DerefMut
for crate::Unity::Collections::LowLevel::Unsafe::NativeContainerIsAtomicWriteOnlyAttribute {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(
    feature = "Unity+Collections+LowLevel+Unsafe+NativeContainerIsAtomicWriteOnlyAttribute"
)]
impl crate::Unity::Collections::LowLevel::Unsafe::NativeContainerIsAtomicWriteOnlyAttribute {}
#[cfg(
    feature = "Unity+Collections+LowLevel+Unsafe+NativeContainerIsAtomicWriteOnlyAttribute"
)]
impl quest_hook::libil2cpp::ObjectType
for crate::Unity::Collections::LowLevel::Unsafe::NativeContainerIsAtomicWriteOnlyAttribute {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
