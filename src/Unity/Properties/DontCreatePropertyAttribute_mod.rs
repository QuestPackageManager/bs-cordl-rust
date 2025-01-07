#[cfg(feature = "Unity+Properties+DontCreatePropertyAttribute")]
#[repr(C)]
#[derive(Debug)]
pub struct DontCreatePropertyAttribute {
    __cordl_parent: crate::System::Attribute,
}
#[cfg(feature = "Unity+Properties+DontCreatePropertyAttribute")]
unsafe impl quest_hook::libil2cpp::Type
for crate::Unity::Properties::DontCreatePropertyAttribute {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Unity.Properties";
    const CLASS_NAME: &'static str = "DontCreatePropertyAttribute";
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.class().is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
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
#[cfg(feature = "Unity+Properties+DontCreatePropertyAttribute")]
impl std::ops::Deref for crate::Unity::Properties::DontCreatePropertyAttribute {
    type Target = crate::System::Attribute;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+Properties+DontCreatePropertyAttribute")]
impl std::ops::DerefMut for crate::Unity::Properties::DontCreatePropertyAttribute {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+Properties+DontCreatePropertyAttribute")]
impl crate::Unity::Properties::DontCreatePropertyAttribute {}
#[cfg(feature = "Unity+Properties+DontCreatePropertyAttribute")]
impl quest_hook::libil2cpp::ObjectType
for crate::Unity::Properties::DontCreatePropertyAttribute {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
