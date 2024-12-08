#[cfg(feature = "UnityEngine+ProBuilder+ChangelogEntry")]
#[repr(C)]
#[derive(Debug)]
pub struct ChangelogEntry {
    __cordl_parent: crate::System::Object,
    pub m_VersionInfo: *mut crate::UnityEngine::ProBuilder::SemVer,
    pub m_ReleaseNotes: *mut crate::System::String,
}
#[cfg(feature = "UnityEngine+ProBuilder+ChangelogEntry")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::ProBuilder::ChangelogEntry =>
    "UnityEngine.ProBuilder"."ChangelogEntry"
);
#[cfg(feature = "UnityEngine+ProBuilder+ChangelogEntry")]
impl std::ops::Deref for crate::UnityEngine::ProBuilder::ChangelogEntry {
    type Target = crate::System::Object;
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
    pub fn _ctor(
        &mut self,
        version: *mut crate::UnityEngine::ProBuilder::SemVer,
        releaseNotes: *mut crate::System::String,
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
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
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
    pub fn ToString(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("ToString", ())?;
        Ok(__cordl_ret)
    }
    pub fn New(
        version: *mut crate::UnityEngine::ProBuilder::SemVer,
        releaseNotes: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (version, releaseNotes))?;
        Ok(__cordl_object)
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
