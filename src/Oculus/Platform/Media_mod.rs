#[cfg(feature = "cordl_class_Oculus+Platform+Media")]
#[repr(C)]
#[derive(Debug)]
pub struct Media {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "cordl_class_Oculus+Platform+Media")]
unsafe impl quest_hook::libil2cpp::Type for crate::Oculus::Platform::Media {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Oculus.Platform";
    const CLASS_NAME: &'static str = "Media";
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
#[cfg(feature = "Oculus+Platform+Media")]
impl std::ops::Deref for crate::Oculus::Platform::Media {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Oculus+Platform+Media")]
impl std::ops::DerefMut for crate::Oculus::Platform::Media {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
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
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            crate::Oculus::Platform::MediaContentType,
                        ),
                        quest_hook::libil2cpp::Gc<
                            crate::Oculus::Platform::Request_1<
                                quest_hook::libil2cpp::Gc<
                                    crate::Oculus::Platform::Models::ShareMediaResult,
                                >,
                            >,
                        >,
                        3usize,
                    >("ShareToFacebook")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ShareToFacebook", 3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Oculus::Platform::Request_1<
                quest_hook::libil2cpp::Gc<
                    crate::Oculus::Platform::Models::ShareMediaResult,
                >,
            >,
        > = unsafe {
            cordl_method_info
                .invoke_unchecked((), (postTextSuggestion, filePath, contentType))?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_Oculus+Platform+Media")]
impl quest_hook::libil2cpp::ObjectType for crate::Oculus::Platform::Media {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
