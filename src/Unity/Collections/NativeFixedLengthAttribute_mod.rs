#[cfg(feature = "cordl_class_Unity+Collections+NativeFixedLengthAttribute")]
#[repr(C)]
#[cfg_attr(feature = "derive_Debug", derive(Debug))]
pub struct NativeFixedLengthAttribute {
    __cordl_parent: crate::System::Attribute,
}
#[cfg(feature = "cordl_class_Unity+Collections+NativeFixedLengthAttribute")]
unsafe impl quest_hook::libil2cpp::Type for crate::Unity::Collections::NativeFixedLengthAttribute {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Unity.Collections";
    const CLASS_NAME: &'static str = "NativeFixedLengthAttribute";
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
#[cfg(feature = "Unity+Collections+NativeFixedLengthAttribute")]
impl std::ops::Deref for crate::Unity::Collections::NativeFixedLengthAttribute {
    type Target = crate::System::Attribute;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+Collections+NativeFixedLengthAttribute")]
impl std::ops::DerefMut for crate::Unity::Collections::NativeFixedLengthAttribute {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+Collections+NativeFixedLengthAttribute")]
impl crate::Unity::Collections::NativeFixedLengthAttribute {}
#[cfg(feature = "cordl_class_Unity+Collections+NativeFixedLengthAttribute")]
impl quest_hook::libil2cpp::ObjectType for crate::Unity::Collections::NativeFixedLengthAttribute {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
