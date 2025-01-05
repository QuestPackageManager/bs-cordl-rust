#[cfg(feature = "UnityEngine+HumanTrait")]
#[repr(C)]
#[derive(Debug)]
pub struct HumanTrait {
    __cordl_parent: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
}
#[cfg(feature = "UnityEngine+HumanTrait")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::HumanTrait => "UnityEngine"
    ."HumanTrait"
);
#[cfg(feature = "UnityEngine+HumanTrait")]
impl std::ops::Deref for crate::UnityEngine::HumanTrait {
    type Target = quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>;
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
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetBoneIndexFromMono", (humanId))?;
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
