#[cfg(feature = "UnityEngine+RuntimeAnimatorController")]
#[repr(C)]
#[derive(Debug)]
pub struct RuntimeAnimatorController {
    __cordl_parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::Object>,
}
#[cfg(feature = "UnityEngine+RuntimeAnimatorController")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::RuntimeAnimatorController =>
    "UnityEngine"."RuntimeAnimatorController"
);
#[cfg(feature = "UnityEngine+RuntimeAnimatorController")]
impl std::ops::Deref for crate::UnityEngine::RuntimeAnimatorController {
    type Target = quest_hook::libil2cpp::Gc<crate::UnityEngine::Object>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+RuntimeAnimatorController")]
impl std::ops::DerefMut for crate::UnityEngine::RuntimeAnimatorController {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+RuntimeAnimatorController")]
impl crate::UnityEngine::RuntimeAnimatorController {
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_animationClips(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<crate::UnityEngine::AnimationClip>,
            >,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<crate::UnityEngine::AnimationClip>,
            >,
        > = __cordl_object.invoke("get_animationClips", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+RuntimeAnimatorController")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::RuntimeAnimatorController {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
