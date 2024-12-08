#[cfg(feature = "UnityEngine+TextMesh")]
#[repr(C)]
#[derive(Debug)]
pub struct TextMesh {
    __cordl_parent: crate::UnityEngine::Component,
}
#[cfg(feature = "UnityEngine+TextMesh")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::TextMesh => "UnityEngine"
    ."TextMesh"
);
#[cfg(feature = "UnityEngine+TextMesh")]
impl std::ops::Deref for crate::UnityEngine::TextMesh {
    type Target = crate::UnityEngine::Component;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+TextMesh")]
impl std::ops::DerefMut for crate::UnityEngine::TextMesh {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+TextMesh")]
impl crate::UnityEngine::TextMesh {
    pub fn get_alignment(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::TextAlignment> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::TextAlignment = __cordl_object
            .invoke("get_alignment", ())?;
        Ok(__cordl_ret)
    }
    pub fn set_alignment(
        &mut self,
        value: crate::UnityEngine::TextAlignment,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_alignment", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_text(
        &mut self,
        value: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_text", (value))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "UnityEngine+TextMesh")]
impl quest_hook::libil2cpp::ObjectType for crate::UnityEngine::TextMesh {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
