#[cfg(feature = "UnityEngine+ProBuilder+Changelog")]
#[repr(C)]
#[derive(Debug)]
pub struct Changelog {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub m_Entries: *mut crate::System::Collections::Generic::List_1<
        *mut crate::UnityEngine::ProBuilder::ChangelogEntry,
    >,
}
#[cfg(feature = "UnityEngine+ProBuilder+Changelog")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::ProBuilder::Changelog =>
    "UnityEngine.ProBuilder"."Changelog"
);
#[cfg(feature = "UnityEngine+ProBuilder+Changelog")]
impl std::ops::Deref for crate::UnityEngine::ProBuilder::Changelog {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+ProBuilder+Changelog")]
impl std::ops::DerefMut for crate::UnityEngine::ProBuilder::Changelog {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+ProBuilder+Changelog")]
impl crate::UnityEngine::ProBuilder::Changelog {
    pub const k_ChangelogEntryPattern: &'static str = "(##\\s\\[[0-9]+\\.[0-9]+\\.[0-9]+(\\-[a-zA-Z]+(\\.[0-9]+)*)*\\])";
    pub const k_VersionDatePattern: &'static str = "(?<=##\\s\\[.*\\]\\s-\\s)[0-9-]*";
    pub const k_VersionInfoPattern: &'static str = "(?<=##\\s\\[).*(?=\\])";
    pub fn CreateEntry(
        &mut self,
        version: *mut quest_hook::libil2cpp::Il2CppString,
        contents: *mut quest_hook::libil2cpp::Il2CppString,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::UnityEngine::ProBuilder::ChangelogEntry,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::ProBuilder::ChangelogEntry = __cordl_object
            .invoke("CreateEntry", (version, contents))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        log: *mut quest_hook::libil2cpp::Il2CppString,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (log))?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
        log: *mut quest_hook::libil2cpp::Il2CppString,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (log))?;
        Ok(__cordl_ret)
    }
    pub fn get_entries(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Collections::ObjectModel::ReadOnlyCollection_1<
            *mut crate::UnityEngine::ProBuilder::ChangelogEntry,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::ObjectModel::ReadOnlyCollection_1<
            *mut crate::UnityEngine::ProBuilder::ChangelogEntry,
        > = __cordl_object.invoke("get_entries", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "UnityEngine+ProBuilder+Changelog")]
impl quest_hook::libil2cpp::ObjectType for crate::UnityEngine::ProBuilder::Changelog {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
