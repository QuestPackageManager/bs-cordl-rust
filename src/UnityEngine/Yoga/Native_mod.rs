#[cfg(feature = "UnityEngine+Yoga+Native")]
#[repr(C)]
#[derive(Debug)]
pub struct Native {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "UnityEngine+Yoga+Native")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::Yoga::Native => "UnityEngine.Yoga"
    ."Native"
);
#[cfg(feature = "UnityEngine+Yoga+Native")]
impl std::ops::Deref for crate::UnityEngine::Yoga::Native {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Yoga+Native")]
impl std::ops::DerefMut for crate::UnityEngine::Yoga::Native {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Yoga+Native")]
impl crate::UnityEngine::Yoga::Native {}
#[cfg(feature = "UnityEngine+Yoga+Native")]
impl quest_hook::libil2cpp::ObjectType for crate::UnityEngine::Yoga::Native {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
