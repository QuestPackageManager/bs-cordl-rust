#[cfg(feature = "UnityEngine+HumanTrait")]
#[repr(C)]
#[derive(Debug)]
pub struct HumanTrait {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "UnityEngine+HumanTrait")]
unsafe impl quest_hook::libil2cpp::Type for crate::UnityEngine::HumanTrait {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine";
    const CLASS_NAME: &'static str = "HumanTrait";
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
#[cfg(feature = "UnityEngine+HumanTrait")]
impl std::ops::Deref for crate::UnityEngine::HumanTrait {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+HumanTrait")]
impl std::ops::DerefMut for crate::UnityEngine::HumanTrait {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+HumanTrait")]
impl crate::UnityEngine::HumanTrait {
    pub fn GetBoneIndexFromMono(humanId: i32) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::HumanTrait as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<(i32), i32, 1usize>("GetBoneIndexFromMono")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::HumanTrait as quest_hook::libil2cpp::Type >
                    ::class(), "GetBoneIndexFromMono", 1usize
                )
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked((), (humanId))? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+HumanTrait")]
impl quest_hook::libil2cpp::ObjectType for crate::UnityEngine::HumanTrait {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
