#[cfg(feature = "cordl_class_UnityEngine+Android+AndroidAssetPacks")]
#[repr(C)]
#[cfg_attr(feature = "derive_Debug", derive(Debug))]
pub struct AndroidAssetPacks {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "cordl_class_UnityEngine+Android+AndroidAssetPacks")]
unsafe impl quest_hook::libil2cpp::Type for crate::UnityEngine::Android::AndroidAssetPacks {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.Android";
    const CLASS_NAME: &'static str = "AndroidAssetPacks";
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.class()
            .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
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
#[cfg(feature = "UnityEngine+Android+AndroidAssetPacks")]
impl std::ops::Deref for crate::UnityEngine::Android::AndroidAssetPacks {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Android+AndroidAssetPacks")]
impl std::ops::DerefMut for crate::UnityEngine::Android::AndroidAssetPacks {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Android+AndroidAssetPacks")]
impl crate::UnityEngine::Android::AndroidAssetPacks {
    #[cfg(feature = "UnityEngine+Android+AndroidAssetPacks+AssetPackManagerDownloadStatusCallback")]
    pub type AssetPackManagerDownloadStatusCallback =
        crate::UnityEngine::Android::AndroidAssetPacks_AssetPackManagerDownloadStatusCallback;
    #[cfg(
        feature = "UnityEngine+Android+AndroidAssetPacks+AssetPackManagerMobileDataConfirmationCallback"
    )]
    pub type AssetPackManagerMobileDataConfirmationCallback = crate::UnityEngine::Android::AndroidAssetPacks_AssetPackManagerMobileDataConfirmationCallback;
    #[cfg(feature = "UnityEngine+Android+AndroidAssetPacks+AssetPackManagerStatusQueryCallback")]
    pub type AssetPackManagerStatusQueryCallback =
        crate::UnityEngine::Android::AndroidAssetPacks_AssetPackManagerStatusQueryCallback;
    pub fn GetAssetPackManager() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::AndroidJavaObject>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (),
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::AndroidJavaObject>,
                        0usize,
                    >("GetAssetPackManager")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetAssetPackManager", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::AndroidJavaObject> =
            unsafe { cordl_method_info.invoke_unchecked((), ())? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_UnityEngine+Android+AndroidAssetPacks")]
impl quest_hook::libil2cpp::ObjectType for crate::UnityEngine::Android::AndroidAssetPacks {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(
    feature = "cordl_class_UnityEngine+Android+AndroidAssetPacks+AssetPackManagerDownloadStatusCallback"
)]
#[repr(C)]
#[cfg_attr(feature = "derive_Debug", derive(Debug))]
pub struct AndroidAssetPacks_AssetPackManagerDownloadStatusCallback {
    __cordl_parent: crate::UnityEngine::AndroidJavaProxy,
    pub m_Callback: quest_hook::libil2cpp::Gc<
        crate::System::Action_1<
            quest_hook::libil2cpp::Gc<crate::UnityEngine::Android::AndroidAssetPackInfo>,
        >,
    >,
    pub m_AssetPacks: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::Dictionary_2<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
            crate::UnityEngine::Android::AndroidAssetPackStatus,
        >,
    >,
}
#[cfg(
    feature = "cordl_class_UnityEngine+Android+AndroidAssetPacks+AssetPackManagerDownloadStatusCallback"
)]
unsafe impl quest_hook::libil2cpp::Type
    for crate::UnityEngine::Android::AndroidAssetPacks_AssetPackManagerDownloadStatusCallback
{
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.Android";
    const CLASS_NAME: &'static str = "AndroidAssetPacks/AssetPackManagerDownloadStatusCallback";
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.class()
            .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
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
#[cfg(feature = "UnityEngine+Android+AndroidAssetPacks+AssetPackManagerDownloadStatusCallback")]
impl std::ops::Deref
    for crate::UnityEngine::Android::AndroidAssetPacks_AssetPackManagerDownloadStatusCallback
{
    type Target = crate::UnityEngine::AndroidJavaProxy;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Android+AndroidAssetPacks+AssetPackManagerDownloadStatusCallback")]
impl std::ops::DerefMut
    for crate::UnityEngine::Android::AndroidAssetPacks_AssetPackManagerDownloadStatusCallback
{
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Android+AndroidAssetPacks+AssetPackManagerDownloadStatusCallback")]
impl crate::UnityEngine::Android::AndroidAssetPacks_AssetPackManagerDownloadStatusCallback {
    pub fn New(
        callback: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<
                quest_hook::libil2cpp::Gc<crate::UnityEngine::Android::AndroidAssetPackInfo>,
            >,
        >,
        assetPacks: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self =
            <Self as quest_hook::libil2cpp::Type>::class().instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (callback, assetPacks))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        callback: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<
                quest_hook::libil2cpp::Gc<crate::UnityEngine::Android::AndroidAssetPackInfo>,
            >,
        >,
        assetPacks: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<
                            crate::System::Action_1<
                                quest_hook::libil2cpp::Gc<
                                    crate::UnityEngine::Android::AndroidAssetPackInfo,
                                >,
                            >,
                        >,
                        quest_hook::libil2cpp::Gc<
                            quest_hook::libil2cpp::Il2CppArray<
                                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                            >,
                        >,
                    ), quest_hook::libil2cpp::Void, 2usize>(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            ".ctor",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (callback, assetPacks))? };
        Ok(__cordl_ret.into())
    }
    pub fn onStatusUpdate(
        &mut self,
        assetPackName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        assetPackStatus: i32,
        assetPackSize: i64,
        assetPackBytesDownloaded: i64,
        assetPackTransferProgress: i32,
        assetPackErrorCode: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        i32,
                        i64,
                        i64,
                        i32,
                        i32,
                    ), quest_hook::libil2cpp::Void, 6usize>("onStatusUpdate")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "onStatusUpdate",
                            6usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(
                self,
                (
                    assetPackName,
                    assetPackStatus,
                    assetPackSize,
                    assetPackBytesDownloaded,
                    assetPackTransferProgress,
                    assetPackErrorCode,
                ),
            )?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(
    feature = "cordl_class_UnityEngine+Android+AndroidAssetPacks+AssetPackManagerDownloadStatusCallback"
)]
impl quest_hook::libil2cpp::ObjectType
    for crate::UnityEngine::Android::AndroidAssetPacks_AssetPackManagerDownloadStatusCallback
{
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(
    feature = "cordl_class_UnityEngine+Android+AndroidAssetPacks+AssetPackManagerMobileDataConfirmationCallback"
)]
#[repr(C)]
#[cfg_attr(feature = "derive_Debug", derive(Debug))]
pub struct AndroidAssetPacks_AssetPackManagerMobileDataConfirmationCallback {
    __cordl_parent: crate::UnityEngine::AndroidJavaProxy,
    pub m_Callback: quest_hook::libil2cpp::Gc<
        crate::System::Action_1<
            quest_hook::libil2cpp::Gc<
                crate::UnityEngine::Android::AndroidAssetPackUseMobileDataRequestResult,
            >,
        >,
    >,
}
#[cfg(
    feature = "cordl_class_UnityEngine+Android+AndroidAssetPacks+AssetPackManagerMobileDataConfirmationCallback"
)]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::Android::AndroidAssetPacks_AssetPackManagerMobileDataConfirmationCallback {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.Android";
    const CLASS_NAME: &'static str = "AndroidAssetPacks/AssetPackManagerMobileDataConfirmationCallback";
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
#[cfg(
    feature = "UnityEngine+Android+AndroidAssetPacks+AssetPackManagerMobileDataConfirmationCallback"
)]
impl std::ops::Deref
for crate::UnityEngine::Android::AndroidAssetPacks_AssetPackManagerMobileDataConfirmationCallback {
    type Target = crate::UnityEngine::AndroidJavaProxy;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(
    feature = "UnityEngine+Android+AndroidAssetPacks+AssetPackManagerMobileDataConfirmationCallback"
)]
impl std::ops::DerefMut
for crate::UnityEngine::Android::AndroidAssetPacks_AssetPackManagerMobileDataConfirmationCallback {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(
    feature = "UnityEngine+Android+AndroidAssetPacks+AssetPackManagerMobileDataConfirmationCallback"
)]
impl crate::UnityEngine::Android::AndroidAssetPacks_AssetPackManagerMobileDataConfirmationCallback {
    pub fn New(
        callback: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<
                quest_hook::libil2cpp::Gc<
                    crate::UnityEngine::Android::AndroidAssetPackUseMobileDataRequestResult,
                >,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self =
            <Self as quest_hook::libil2cpp::Type>::class().instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (callback))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        callback: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<
                quest_hook::libil2cpp::Gc<
                    crate::UnityEngine::Android::AndroidAssetPackUseMobileDataRequestResult,
                >,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::System::Action_1<
                                quest_hook::libil2cpp::Gc<
                                    crate::UnityEngine::Android::AndroidAssetPackUseMobileDataRequestResult,
                                >,
                            >,
                        >),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), ".ctor",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (callback))? };
        Ok(__cordl_ret.into())
    }
    pub fn onMobileDataConfirmationResult(
        &mut self,
        allowed: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(bool), quest_hook::libil2cpp::Void, 1usize>(
                        "onMobileDataConfirmationResult",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "onMobileDataConfirmationResult",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (allowed))? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(
    feature = "cordl_class_UnityEngine+Android+AndroidAssetPacks+AssetPackManagerMobileDataConfirmationCallback"
)]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::Android::AndroidAssetPacks_AssetPackManagerMobileDataConfirmationCallback {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(
    feature = "cordl_class_UnityEngine+Android+AndroidAssetPacks+AssetPackManagerStatusQueryCallback"
)]
#[repr(C)]
#[cfg_attr(feature = "derive_Debug", derive(Debug))]
pub struct AndroidAssetPacks_AssetPackManagerStatusQueryCallback {
    __cordl_parent: crate::UnityEngine::AndroidJavaProxy,
    pub m_Callback: quest_hook::libil2cpp::Gc<
        crate::System::Action_2<
            u64,
            quest_hook::libil2cpp::Gc<
                quest_hook::libil2cpp::Il2CppArray<
                    quest_hook::libil2cpp::Gc<crate::UnityEngine::Android::AndroidAssetPackState>,
                >,
            >,
        >,
    >,
    pub m_AssetPackNames: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::List_1<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        >,
    >,
    pub m_States: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::List_1<
            quest_hook::libil2cpp::Gc<crate::UnityEngine::Android::AndroidAssetPackState>,
        >,
    >,
    pub m_Size: i64,
}
#[cfg(
    feature = "cordl_class_UnityEngine+Android+AndroidAssetPacks+AssetPackManagerStatusQueryCallback"
)]
unsafe impl quest_hook::libil2cpp::Type
    for crate::UnityEngine::Android::AndroidAssetPacks_AssetPackManagerStatusQueryCallback
{
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.Android";
    const CLASS_NAME: &'static str = "AndroidAssetPacks/AssetPackManagerStatusQueryCallback";
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.class()
            .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
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
#[cfg(feature = "UnityEngine+Android+AndroidAssetPacks+AssetPackManagerStatusQueryCallback")]
impl std::ops::Deref
    for crate::UnityEngine::Android::AndroidAssetPacks_AssetPackManagerStatusQueryCallback
{
    type Target = crate::UnityEngine::AndroidJavaProxy;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Android+AndroidAssetPacks+AssetPackManagerStatusQueryCallback")]
impl std::ops::DerefMut
    for crate::UnityEngine::Android::AndroidAssetPacks_AssetPackManagerStatusQueryCallback
{
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Android+AndroidAssetPacks+AssetPackManagerStatusQueryCallback")]
impl crate::UnityEngine::Android::AndroidAssetPacks_AssetPackManagerStatusQueryCallback {
    pub fn New(
        callback: quest_hook::libil2cpp::Gc<
            crate::System::Action_2<
                u64,
                quest_hook::libil2cpp::Gc<
                    quest_hook::libil2cpp::Il2CppArray<
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::Android::AndroidAssetPackState,
                        >,
                    >,
                >,
            >,
        >,
        assetPacks: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self =
            <Self as quest_hook::libil2cpp::Type>::class().instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (callback, assetPacks))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        callback: quest_hook::libil2cpp::Gc<
            crate::System::Action_2<
                u64,
                quest_hook::libil2cpp::Gc<
                    quest_hook::libil2cpp::Il2CppArray<
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::Android::AndroidAssetPackState,
                        >,
                    >,
                >,
            >,
        >,
        assetPacks: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<
                            crate::System::Action_2<
                                u64,
                                quest_hook::libil2cpp::Gc<
                                    quest_hook::libil2cpp::Il2CppArray<
                                        quest_hook::libil2cpp::Gc<
                                            crate::UnityEngine::Android::AndroidAssetPackState,
                                        >,
                                    >,
                                >,
                            >,
                        >,
                        quest_hook::libil2cpp::Gc<
                            quest_hook::libil2cpp::Il2CppArray<
                                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                            >,
                        >,
                    ), quest_hook::libil2cpp::Void, 2usize>(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            ".ctor",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (callback, assetPacks))? };
        Ok(__cordl_ret.into())
    }
    pub fn onStatusResult(
        &mut self,
        totalBytes: i64,
        assetPackNames: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
            >,
        >,
        assetPackStatuses: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<i32>>,
        assetPackErrorCodes: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<i32>>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        i64,
                        quest_hook::libil2cpp::Gc<
                            quest_hook::libil2cpp::Il2CppArray<
                                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                            >,
                        >,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<i32>>,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<i32>>,
                    ), quest_hook::libil2cpp::Void, 4usize>("onStatusResult")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "onStatusResult",
                            4usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(
                self,
                (
                    totalBytes,
                    assetPackNames,
                    assetPackStatuses,
                    assetPackErrorCodes,
                ),
            )?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(
    feature = "cordl_class_UnityEngine+Android+AndroidAssetPacks+AssetPackManagerStatusQueryCallback"
)]
impl quest_hook::libil2cpp::ObjectType
    for crate::UnityEngine::Android::AndroidAssetPacks_AssetPackManagerStatusQueryCallback
{
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
