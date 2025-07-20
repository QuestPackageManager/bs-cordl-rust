#[cfg(feature = "BGLib+Polyglot+LocalizationAsset")]
#[repr(C)]
#[derive(Debug)]
pub struct LocalizationAsset {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub textAsset: quest_hook::libil2cpp::Gc<crate::UnityEngine::TextAsset>,
    pub format: crate::BGLib::Polyglot::GoogleDriveDownloadFormat,
}
#[cfg(feature = "BGLib+Polyglot+LocalizationAsset")]
unsafe impl quest_hook::libil2cpp::Type for crate::BGLib::Polyglot::LocalizationAsset {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "BGLib.Polyglot";
    const CLASS_NAME: &'static str = "LocalizationAsset";
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.class().is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_argument(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
    fn matches_value_parameter(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
}
#[cfg(feature = "BGLib+Polyglot+LocalizationAsset")]
impl std::ops::Deref for crate::BGLib::Polyglot::LocalizationAsset {
    type Target = quest_hook::libil2cpp::Il2CppObject;
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
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::BGLib::Polyglot::LocalizationAsset as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<crate::UnityEngine::TextAsset>,
                    crate::BGLib::Polyglot::GoogleDriveDownloadFormat,
                ),
                quest_hook::libil2cpp::Void,
                2usize,
            >(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::BGLib::Polyglot::LocalizationAsset as
                    quest_hook::libil2cpp::Type > ::class(), ".ctor", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (textAsset, format))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_Format(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::BGLib::Polyglot::GoogleDriveDownloadFormat,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::BGLib::Polyglot::LocalizationAsset as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                crate::BGLib::Polyglot::GoogleDriveDownloadFormat,
                0usize,
            >("get_Format")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::BGLib::Polyglot::LocalizationAsset as
                    quest_hook::libil2cpp::Type > ::class(), "get_Format", 0usize
                )
            });
        let __cordl_ret: crate::BGLib::Polyglot::GoogleDriveDownloadFormat = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_TextAsset(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::TextAsset>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::BGLib::Polyglot::LocalizationAsset as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Gc<crate::UnityEngine::TextAsset>,
                0usize,
            >("get_TextAsset")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::BGLib::Polyglot::LocalizationAsset as
                    quest_hook::libil2cpp::Type > ::class(), "get_TextAsset", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::TextAsset> = unsafe {
            method.invoke_unchecked(self, ())?
        };
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
