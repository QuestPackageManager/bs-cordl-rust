#[cfg(feature = "UnityEngine+Android+AndroidAssetPackInfo")]
#[repr(C)]
#[derive(Debug)]
pub struct AndroidAssetPackInfo {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _name_k__BackingField: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppString,
    >,
    pub _status_k__BackingField: crate::UnityEngine::Android::AndroidAssetPackStatus,
    pub _size_k__BackingField: u64,
    pub _bytesDownloaded_k__BackingField: u64,
    pub _transferProgress_k__BackingField: f32,
    pub _error_k__BackingField: crate::UnityEngine::Android::AndroidAssetPackError,
}
#[cfg(feature = "UnityEngine+Android+AndroidAssetPackInfo")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::Android::AndroidAssetPackInfo {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.Android";
    const CLASS_NAME: &'static str = "AndroidAssetPackInfo";
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
#[cfg(feature = "UnityEngine+Android+AndroidAssetPackInfo")]
impl std::ops::Deref for crate::UnityEngine::Android::AndroidAssetPackInfo {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Android+AndroidAssetPackInfo")]
impl std::ops::DerefMut for crate::UnityEngine::Android::AndroidAssetPackInfo {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Android+AndroidAssetPackInfo")]
impl crate::UnityEngine::Android::AndroidAssetPackInfo {
    pub fn New(
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        status: crate::UnityEngine::Android::AndroidAssetPackStatus,
        _cordl_size: u64,
        bytesDownloaded: u64,
        transferProgress: f32,
        error: crate::UnityEngine::Android::AndroidAssetPackError,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (name, status, _cordl_size, bytesDownloaded, transferProgress, error),
            )?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        status: crate::UnityEngine::Android::AndroidAssetPackStatus,
        _cordl_size: u64,
        bytesDownloaded: u64,
        transferProgress: f32,
        error: crate::UnityEngine::Android::AndroidAssetPackError,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            crate::UnityEngine::Android::AndroidAssetPackStatus,
                            u64,
                            u64,
                            f32,
                            crate::UnityEngine::Android::AndroidAssetPackError,
                        ),
                        quest_hook::libil2cpp::Void,
                        6usize,
                    >(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), ".ctor",
                            6usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    self,
                    (name, status, _cordl_size, bytesDownloaded, transferProgress, error),
                )?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+Android+AndroidAssetPackInfo")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::Android::AndroidAssetPackInfo {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
