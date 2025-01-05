#[cfg(feature = "Oculus+Platform+Media")]
#[repr(C)]
#[derive(Debug)]
pub struct Media {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "Oculus+Platform+Media")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Oculus::Platform::Media => "Oculus.Platform"
    ."Media"
);
#[cfg(feature = "Oculus+Platform+Media")]
impl std::ops::Deref for crate::Oculus::Platform::Media {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Oculus+Platform+Media")]
impl std::ops::DerefMut for crate::Oculus::Platform::Media {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Oculus+Platform+Media")]
impl crate::Oculus::Platform::Media {
    pub fn ShareToFacebook(
        postTextSuggestion: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        >,
        filePath: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        contentType: crate::Oculus::Platform::MediaContentType,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::Oculus::Platform::Request_1<
                quest_hook::libil2cpp::Gc<
                    crate::Oculus::Platform::Models::ShareMediaResult,
                >,
            >,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Oculus::Platform::Request_1<
                quest_hook::libil2cpp::Gc<
                    crate::Oculus::Platform::Models::ShareMediaResult,
                >,
            >,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ShareToFacebook", (postTextSuggestion, filePath, contentType))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Oculus+Platform+Media")]
impl quest_hook::libil2cpp::ObjectType for crate::Oculus::Platform::Media {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
