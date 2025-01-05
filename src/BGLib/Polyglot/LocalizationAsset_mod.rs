#[cfg(feature = "BGLib+Polyglot+LocalizationAsset")]
#[repr(C)]
#[derive(Debug)]
pub struct LocalizationAsset {
    __cordl_parent: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    pub textAsset: quest_hook::libil2cpp::Gc<crate::UnityEngine::TextAsset>,
    pub format: crate::BGLib::Polyglot::GoogleDriveDownloadFormat,
}
#[cfg(feature = "BGLib+Polyglot+LocalizationAsset")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::BGLib::Polyglot::LocalizationAsset =>
    "BGLib.Polyglot"."LocalizationAsset"
);
#[cfg(feature = "BGLib+Polyglot+LocalizationAsset")]
impl std::ops::Deref for crate::BGLib::Polyglot::LocalizationAsset {
    type Target = quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BGLib+Polyglot+LocalizationAsset")]
impl std::ops::DerefMut for crate::BGLib::Polyglot::LocalizationAsset {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BGLib+Polyglot+LocalizationAsset")]
impl crate::BGLib::Polyglot::LocalizationAsset {
    pub fn New(
        textAsset: quest_hook::libil2cpp::Gc<crate::UnityEngine::TextAsset>,
        format: crate::BGLib::Polyglot::GoogleDriveDownloadFormat,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (textAsset, format))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        textAsset: quest_hook::libil2cpp::Gc<crate::UnityEngine::TextAsset>,
        format: crate::BGLib::Polyglot::GoogleDriveDownloadFormat,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (textAsset, format))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Format(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::BGLib::Polyglot::GoogleDriveDownloadFormat,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::BGLib::Polyglot::GoogleDriveDownloadFormat = __cordl_object
            .invoke("get_Format", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_TextAsset(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::TextAsset>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::TextAsset> = __cordl_object
            .invoke("get_TextAsset", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "BGLib+Polyglot+LocalizationAsset")]
impl quest_hook::libil2cpp::ObjectType for crate::BGLib::Polyglot::LocalizationAsset {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
