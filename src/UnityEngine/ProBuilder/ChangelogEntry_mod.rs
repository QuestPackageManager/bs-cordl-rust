#[cfg(feature = "UnityEngine+ProBuilder+ChangelogEntry")]
#[repr(C)]
#[derive(Debug)]
pub struct ChangelogEntry {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub m_VersionInfo: *mut crate::UnityEngine::ProBuilder::SemVer,
    pub m_ReleaseNotes: *mut quest_hook::libil2cpp::Il2CppString,
}
#[cfg(feature = "UnityEngine+ProBuilder+ChangelogEntry")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::ProBuilder::ChangelogEntry =>
    "UnityEngine.ProBuilder"."ChangelogEntry"
);
#[cfg(feature = "UnityEngine+ProBuilder+ChangelogEntry")]
impl std::ops::Deref for crate::UnityEngine::ProBuilder::ChangelogEntry {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+ProBuilder+ChangelogEntry")]
impl std::ops::DerefMut for crate::UnityEngine::ProBuilder::ChangelogEntry {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+ProBuilder+ChangelogEntry")]
impl crate::UnityEngine::ProBuilder::ChangelogEntry {
    pub fn New(
        version: *mut crate::UnityEngine::ProBuilder::SemVer,
        releaseNotes: *mut quest_hook::libil2cpp::Il2CppString,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (version, releaseNotes))?;
        Ok(__cordl_object)
    }
    pub fn ToString(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppString> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppString = __cordl_object
            .invoke("ToString", ())?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        version: *mut crate::UnityEngine::ProBuilder::SemVer,
        releaseNotes: *mut quest_hook::libil2cpp::Il2CppString,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (version, releaseNotes))?;
        Ok(__cordl_ret)
    }
    pub fn get_releaseNotes(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppString> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppString = __cordl_object
            .invoke("get_releaseNotes", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_versionInfo(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::UnityEngine::ProBuilder::SemVer> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::ProBuilder::SemVer = __cordl_object
            .invoke("get_versionInfo", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "UnityEngine+ProBuilder+ChangelogEntry")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::ProBuilder::ChangelogEntry {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
