#[cfg(feature = "cordl_class_UnityEngine+Animations+NotKeyableAttribute")]
#[repr(C)]
#[derive(Debug)]
pub struct NotKeyableAttribute {
    __cordl_parent: crate::System::Attribute,
}
#[cfg(feature = "cordl_class_UnityEngine+Animations+NotKeyableAttribute")]
unsafe impl quest_hook::libil2cpp::Type for crate::UnityEngine::Animations::NotKeyableAttribute {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.Animations";
    const CLASS_NAME: &'static str = "NotKeyableAttribute";
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
#[cfg(feature = "UnityEngine+Animations+NotKeyableAttribute")]
impl std::ops::Deref for crate::UnityEngine::Animations::NotKeyableAttribute {
    type Target = crate::System::Attribute;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Animations+NotKeyableAttribute")]
impl std::ops::DerefMut for crate::UnityEngine::Animations::NotKeyableAttribute {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Animations+NotKeyableAttribute")]
impl crate::UnityEngine::Animations::NotKeyableAttribute {}
#[cfg(feature = "cordl_class_UnityEngine+Animations+NotKeyableAttribute")]
impl quest_hook::libil2cpp::ObjectType for crate::UnityEngine::Animations::NotKeyableAttribute {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
