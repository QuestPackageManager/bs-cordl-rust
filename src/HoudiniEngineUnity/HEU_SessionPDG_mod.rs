#[cfg(feature = "HoudiniEngineUnity+HEU_SessionPDG")]
#[repr(C)]
#[derive(Debug)]
pub struct HEU_SessionPDG {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "HoudiniEngineUnity+HEU_SessionPDG")]
unsafe impl quest_hook::libil2cpp::Type for crate::HoudiniEngineUnity::HEU_SessionPDG {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "HoudiniEngineUnity";
    const CLASS_NAME: &'static str = "HEU_SessionPDG";
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
#[cfg(feature = "HoudiniEngineUnity+HEU_SessionPDG")]
impl std::ops::Deref for crate::HoudiniEngineUnity::HEU_SessionPDG {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "HoudiniEngineUnity+HEU_SessionPDG")]
impl std::ops::DerefMut for crate::HoudiniEngineUnity::HEU_SessionPDG {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "HoudiniEngineUnity+HEU_SessionPDG")]
impl crate::HoudiniEngineUnity::HEU_SessionPDG {}
#[cfg(feature = "HoudiniEngineUnity+HEU_SessionPDG")]
impl quest_hook::libil2cpp::ObjectType for crate::HoudiniEngineUnity::HEU_SessionPDG {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
