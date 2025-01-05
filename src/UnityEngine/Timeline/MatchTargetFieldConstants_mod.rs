#[cfg(feature = "UnityEngine+Timeline+MatchTargetFieldConstants")]
#[repr(C)]
#[derive(Debug)]
pub struct MatchTargetFieldConstants {
    __cordl_parent: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
}
#[cfg(feature = "UnityEngine+Timeline+MatchTargetFieldConstants")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::Timeline::MatchTargetFieldConstants
    => "UnityEngine.Timeline"."MatchTargetFieldConstants"
);
#[cfg(feature = "UnityEngine+Timeline+MatchTargetFieldConstants")]
impl std::ops::Deref for crate::UnityEngine::Timeline::MatchTargetFieldConstants {
    type Target = quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Timeline+MatchTargetFieldConstants")]
impl std::ops::DerefMut for crate::UnityEngine::Timeline::MatchTargetFieldConstants {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Timeline+MatchTargetFieldConstants")]
impl crate::UnityEngine::Timeline::MatchTargetFieldConstants {
    pub fn HasAny(
        me: crate::UnityEngine::Timeline::MatchTargetFields,
        fields: crate::UnityEngine::Timeline::MatchTargetFields,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("HasAny", (me, fields))?;
        Ok(__cordl_ret.into())
    }
    pub fn Toggle(
        me: crate::UnityEngine::Timeline::MatchTargetFields,
        flag: crate::UnityEngine::Timeline::MatchTargetFields,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Timeline::MatchTargetFields> {
        let __cordl_ret: crate::UnityEngine::Timeline::MatchTargetFields = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Toggle", (me, flag))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+Timeline+MatchTargetFieldConstants")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::Timeline::MatchTargetFieldConstants {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
