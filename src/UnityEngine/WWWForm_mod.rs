#[cfg(feature = "UnityEngine+WWWForm")]
#[repr(C)]
#[derive(Debug)]
pub struct WWWForm {
    __cordl_parent: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
}
#[cfg(feature = "UnityEngine+WWWForm")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::WWWForm => "UnityEngine"."WWWForm"
);
#[cfg(feature = "UnityEngine+WWWForm")]
impl std::ops::Deref for crate::UnityEngine::WWWForm {
    type Target = quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+WWWForm")]
impl std::ops::DerefMut for crate::UnityEngine::WWWForm {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+WWWForm")]
impl crate::UnityEngine::WWWForm {
    pub fn get_DefaultEncoding() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Text::Encoding>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Text::Encoding> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_DefaultEncoding", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+WWWForm")]
impl quest_hook::libil2cpp::ObjectType for crate::UnityEngine::WWWForm {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
