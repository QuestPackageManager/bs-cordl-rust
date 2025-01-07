#[cfg(feature = "UnityEngine+WaitForFixedUpdate")]
#[repr(C)]
#[derive(Debug)]
pub struct WaitForFixedUpdate {
    __cordl_parent: crate::UnityEngine::YieldInstruction,
}
#[cfg(feature = "UnityEngine+WaitForFixedUpdate")]
unsafe impl quest_hook::libil2cpp::Type for crate::UnityEngine::WaitForFixedUpdate {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine";
    const CLASS_NAME: &'static str = "WaitForFixedUpdate";
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
#[cfg(feature = "UnityEngine+WaitForFixedUpdate")]
impl std::ops::Deref for crate::UnityEngine::WaitForFixedUpdate {
    type Target = crate::UnityEngine::YieldInstruction;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+WaitForFixedUpdate")]
impl std::ops::DerefMut for crate::UnityEngine::WaitForFixedUpdate {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+WaitForFixedUpdate")]
impl crate::UnityEngine::WaitForFixedUpdate {}
#[cfg(feature = "UnityEngine+WaitForFixedUpdate")]
impl quest_hook::libil2cpp::ObjectType for crate::UnityEngine::WaitForFixedUpdate {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
