#[cfg(feature = "UnityEngine+SharedBetweenAnimatorsAttribute")]
#[repr(C)]
#[derive(Debug)]
pub struct SharedBetweenAnimatorsAttribute {
    __cordl_parent: crate::System::Attribute,
}
#[cfg(feature = "UnityEngine+SharedBetweenAnimatorsAttribute")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::SharedBetweenAnimatorsAttribute {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine";
    const CLASS_NAME: &'static str = "SharedBetweenAnimatorsAttribute";
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
#[cfg(feature = "UnityEngine+SharedBetweenAnimatorsAttribute")]
impl std::ops::Deref for crate::UnityEngine::SharedBetweenAnimatorsAttribute {
    type Target = crate::System::Attribute;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+SharedBetweenAnimatorsAttribute")]
impl std::ops::DerefMut for crate::UnityEngine::SharedBetweenAnimatorsAttribute {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+SharedBetweenAnimatorsAttribute")]
impl crate::UnityEngine::SharedBetweenAnimatorsAttribute {}
#[cfg(feature = "UnityEngine+SharedBetweenAnimatorsAttribute")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::SharedBetweenAnimatorsAttribute {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
