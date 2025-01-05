#[cfg(feature = "UnityEngine+ColorUtility")]
#[repr(C)]
#[derive(Debug)]
pub struct ColorUtility {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "UnityEngine+ColorUtility")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::ColorUtility => "UnityEngine"
    ."ColorUtility"
);
#[cfg(feature = "UnityEngine+ColorUtility")]
impl std::ops::Deref for crate::UnityEngine::ColorUtility {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+ColorUtility")]
impl std::ops::DerefMut for crate::UnityEngine::ColorUtility {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+ColorUtility")]
impl crate::UnityEngine::ColorUtility {
    pub fn ToHtmlStringRGB(
        color: crate::UnityEngine::Color,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ToHtmlStringRGB", (color))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+ColorUtility")]
impl quest_hook::libil2cpp::ObjectType for crate::UnityEngine::ColorUtility {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
