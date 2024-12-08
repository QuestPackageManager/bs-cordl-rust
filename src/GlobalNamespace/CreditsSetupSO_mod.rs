#[cfg(feature = "CreditsSetupSO")]
#[repr(C)]
#[derive(Debug)]
pub struct CreditsSetupSO {
    __cordl_parent: crate::UnityEngine::ScriptableObject,
    pub normalTextPrefab: *mut crate::UnityEngine::GameObject,
    pub normalLocalizedTextPrefab: *mut crate::UnityEngine::GameObject,
    pub titleTextPrefab: *mut crate::UnityEngine::GameObject,
    pub titleLocalizedTextPrefab: *mut crate::UnityEngine::GameObject,
    pub headerTextPrefab: *mut crate::UnityEngine::GameObject,
    pub headerLocalizedTextPrefab: *mut crate::UnityEngine::GameObject,
    pub columnCount: i32,
    pub spaceHeight: f32,
}
#[cfg(feature = "CreditsSetupSO")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for CreditsSetupSO => ""."CreditsSetupSO"
);
#[cfg(feature = "CreditsSetupSO")]
impl std::ops::Deref for CreditsSetupSO {
    type Target = crate::UnityEngine::ScriptableObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "CreditsSetupSO")]
impl std::ops::DerefMut for CreditsSetupSO {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "CreditsSetupSO")]
impl CreditsSetupSO {
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "CreditsSetupSO")]
impl quest_hook::libil2cpp::ObjectType for CreditsSetupSO {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
