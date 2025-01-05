#[cfg(feature = "Oculus+Platform+LanguagePack")]
#[repr(C)]
#[derive(Debug)]
pub struct LanguagePack {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "Oculus+Platform+LanguagePack")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Oculus::Platform::LanguagePack =>
    "Oculus.Platform"."LanguagePack"
);
#[cfg(feature = "Oculus+Platform+LanguagePack")]
impl std::ops::Deref for crate::Oculus::Platform::LanguagePack {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Oculus+Platform+LanguagePack")]
impl std::ops::DerefMut for crate::Oculus::Platform::LanguagePack {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Oculus+Platform+LanguagePack")]
impl crate::Oculus::Platform::LanguagePack {
    pub fn GetCurrent() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::Oculus::Platform::Request_1<
                quest_hook::libil2cpp::Gc<crate::Oculus::Platform::Models::AssetDetails>,
            >,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Oculus::Platform::Request_1<
                quest_hook::libil2cpp::Gc<crate::Oculus::Platform::Models::AssetDetails>,
            >,
        > = <Self as quest_hook::libil2cpp::Type>::class().invoke("GetCurrent", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn SetCurrent(
        tag: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::Oculus::Platform::Request_1<
                quest_hook::libil2cpp::Gc<
                    crate::Oculus::Platform::Models::AssetFileDownloadResult,
                >,
            >,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Oculus::Platform::Request_1<
                quest_hook::libil2cpp::Gc<
                    crate::Oculus::Platform::Models::AssetFileDownloadResult,
                >,
            >,
        > = <Self as quest_hook::libil2cpp::Type>::class().invoke("SetCurrent", (tag))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Oculus+Platform+LanguagePack")]
impl quest_hook::libil2cpp::ObjectType for crate::Oculus::Platform::LanguagePack {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
