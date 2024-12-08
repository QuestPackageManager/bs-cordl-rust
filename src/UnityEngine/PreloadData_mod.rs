#[cfg(feature = "UnityEngine+PreloadData")]
#[repr(C)]
#[derive(Debug)]
pub struct PreloadData {
    __cordl_parent: crate::UnityEngine::Object,
}
#[cfg(feature = "UnityEngine+PreloadData")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::PreloadData => "UnityEngine"
    ."PreloadData"
);
#[cfg(feature = "UnityEngine+PreloadData")]
impl std::ops::Deref for crate::UnityEngine::PreloadData {
    type Target = crate::UnityEngine::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+PreloadData")]
impl std::ops::DerefMut for crate::UnityEngine::PreloadData {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+PreloadData")]
impl crate::UnityEngine::PreloadData {
    pub fn PreloadDataDontStripMe(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("PreloadDataDontStripMe", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "UnityEngine+PreloadData")]
impl quest_hook::libil2cpp::ObjectType for crate::UnityEngine::PreloadData {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
