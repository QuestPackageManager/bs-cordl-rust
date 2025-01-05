#[cfg(
    feature = "Unity+Collections+LowLevel+Unsafe+NativeSetClassTypeToNullOnScheduleAttribute"
)]
#[repr(C)]
#[derive(Debug)]
pub struct NativeSetClassTypeToNullOnScheduleAttribute {
    __cordl_parent: crate::System::Attribute,
}
#[cfg(
    feature = "Unity+Collections+LowLevel+Unsafe+NativeSetClassTypeToNullOnScheduleAttribute"
)]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Unity::Collections::LowLevel::Unsafe::NativeSetClassTypeToNullOnScheduleAttribute
    => "Unity.Collections.LowLevel.Unsafe"."NativeSetClassTypeToNullOnScheduleAttribute"
);
#[cfg(
    feature = "Unity+Collections+LowLevel+Unsafe+NativeSetClassTypeToNullOnScheduleAttribute"
)]
impl std::ops::Deref
for crate::Unity::Collections::LowLevel::Unsafe::NativeSetClassTypeToNullOnScheduleAttribute {
    type Target = crate::System::Attribute;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(
    feature = "Unity+Collections+LowLevel+Unsafe+NativeSetClassTypeToNullOnScheduleAttribute"
)]
impl std::ops::DerefMut
for crate::Unity::Collections::LowLevel::Unsafe::NativeSetClassTypeToNullOnScheduleAttribute {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(
    feature = "Unity+Collections+LowLevel+Unsafe+NativeSetClassTypeToNullOnScheduleAttribute"
)]
impl crate::Unity::Collections::LowLevel::Unsafe::NativeSetClassTypeToNullOnScheduleAttribute {}
#[cfg(
    feature = "Unity+Collections+LowLevel+Unsafe+NativeSetClassTypeToNullOnScheduleAttribute"
)]
impl quest_hook::libil2cpp::ObjectType
for crate::Unity::Collections::LowLevel::Unsafe::NativeSetClassTypeToNullOnScheduleAttribute {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
