#[cfg(feature = "UnityEngine+ProBuilder+Log")]
#[repr(C)]
#[derive(Debug)]
pub struct Log {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "UnityEngine+ProBuilder+Log")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::ProBuilder::Log =>
    "UnityEngine.ProBuilder"."Log"
);
#[cfg(feature = "UnityEngine+ProBuilder+Log")]
impl std::ops::Deref for crate::UnityEngine::ProBuilder::Log {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+ProBuilder+Log")]
impl std::ops::DerefMut for crate::UnityEngine::ProBuilder::Log {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+ProBuilder+Log")]
impl crate::UnityEngine::ProBuilder::Log {
    pub const k_ProBuilderLogFileName: &'static str = "ProBuilderLog.txt";
}
#[cfg(feature = "UnityEngine+ProBuilder+Log")]
impl quest_hook::libil2cpp::ObjectType for crate::UnityEngine::ProBuilder::Log {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
