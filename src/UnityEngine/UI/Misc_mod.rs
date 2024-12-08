#[cfg(feature = "UnityEngine+UI+Misc")]
#[repr(C)]
#[derive(Debug)]
pub struct Misc {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "UnityEngine+UI+Misc")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::UI::Misc => "UnityEngine.UI"."Misc"
);
#[cfg(feature = "UnityEngine+UI+Misc")]
impl std::ops::Deref for crate::UnityEngine::UI::Misc {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UI+Misc")]
impl std::ops::DerefMut for crate::UnityEngine::UI::Misc {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UI+Misc")]
impl crate::UnityEngine::UI::Misc {}
#[cfg(feature = "UnityEngine+UI+Misc")]
impl quest_hook::libil2cpp::ObjectType for crate::UnityEngine::UI::Misc {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
