#[cfg(
    feature = "cordl_class_Unity+Collections+LowLevel+Unsafe+NativeSetClassTypeToNullOnScheduleAttribute"
)]
#[repr(C)]
#[derive(Debug)]
pub struct NativeSetClassTypeToNullOnScheduleAttribute {
    __cordl_parent: crate::System::Attribute,
}
#[cfg(
    feature = "cordl_class_Unity+Collections+LowLevel+Unsafe+NativeSetClassTypeToNullOnScheduleAttribute"
)]
unsafe impl quest_hook::libil2cpp::Type
    for crate::Unity::Collections::LowLevel::Unsafe::NativeSetClassTypeToNullOnScheduleAttribute
{
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Unity.Collections.LowLevel.Unsafe";
    const CLASS_NAME: &'static str = "NativeSetClassTypeToNullOnScheduleAttribute";
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.class()
            .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_argument(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
    fn matches_value_parameter(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
}
#[cfg(feature = "Unity+Collections+LowLevel+Unsafe+NativeSetClassTypeToNullOnScheduleAttribute")]
impl std::ops::Deref
    for crate::Unity::Collections::LowLevel::Unsafe::NativeSetClassTypeToNullOnScheduleAttribute
{
    type Target = crate::System::Attribute;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+Collections+LowLevel+Unsafe+NativeSetClassTypeToNullOnScheduleAttribute")]
impl std::ops::DerefMut
    for crate::Unity::Collections::LowLevel::Unsafe::NativeSetClassTypeToNullOnScheduleAttribute
{
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+Collections+LowLevel+Unsafe+NativeSetClassTypeToNullOnScheduleAttribute")]
impl crate::Unity::Collections::LowLevel::Unsafe::NativeSetClassTypeToNullOnScheduleAttribute {}
#[cfg(
    feature = "cordl_class_Unity+Collections+LowLevel+Unsafe+NativeSetClassTypeToNullOnScheduleAttribute"
)]
impl quest_hook::libil2cpp::ObjectType
    for crate::Unity::Collections::LowLevel::Unsafe::NativeSetClassTypeToNullOnScheduleAttribute
{
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
