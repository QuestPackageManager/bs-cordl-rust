#[cfg(feature = "PseudoHDREncoding")]
#[repr(C)]
#[derive(Debug)]
pub struct PseudoHDREncoding {
    __cordl_parent: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
}
#[cfg(feature = "PseudoHDREncoding")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::PseudoHDREncoding => ""
    ."PseudoHDREncoding"
);
#[cfg(feature = "PseudoHDREncoding")]
impl std::ops::Deref for crate::GlobalNamespace::PseudoHDREncoding {
    type Target = quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "PseudoHDREncoding")]
impl std::ops::DerefMut for crate::GlobalNamespace::PseudoHDREncoding {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "PseudoHDREncoding")]
impl crate::GlobalNamespace::PseudoHDREncoding {
    pub const kPseudoHDREncodingShaderName: &'static str = "Hidden/PseudoHDREncoding";
    pub fn CreatePseudoHDREncodedTexture(
        src: quest_hook::libil2cpp::Gc<crate::UnityEngine::RenderTexture>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::RenderTexture>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::RenderTexture> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CreatePseudoHDREncodedTexture", (src))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "PseudoHDREncoding")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::PseudoHDREncoding {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
