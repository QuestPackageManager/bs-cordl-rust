#[cfg(feature = "Oculus+Platform+UserDataStore")]
#[repr(C)]
#[derive(Debug)]
pub struct UserDataStore {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "Oculus+Platform+UserDataStore")]
unsafe impl quest_hook::libil2cpp::Type for crate::Oculus::Platform::UserDataStore {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Oculus.Platform";
    const CLASS_NAME: &'static str = "UserDataStore";
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
#[cfg(feature = "Oculus+Platform+UserDataStore")]
impl std::ops::Deref for crate::Oculus::Platform::UserDataStore {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Oculus+Platform+UserDataStore")]
impl std::ops::DerefMut for crate::Oculus::Platform::UserDataStore {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Oculus+Platform+UserDataStore")]
impl crate::Oculus::Platform::UserDataStore {
    pub fn PrivateDeleteEntryByKey(
        userID: u64,
        key: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::Oculus::Platform::Request_1<
                quest_hook::libil2cpp::Gc<
                    crate::Oculus::Platform::Models::UserDataStoreUpdateResponse,
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
                            u64,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                        ),
                        quest_hook::libil2cpp::Gc<
                            crate::Oculus::Platform::Request_1<
                                quest_hook::libil2cpp::Gc<
                                    crate::Oculus::Platform::Models::UserDataStoreUpdateResponse,
                                >,
                            >,
                        >,
                        2usize,
                    >("PrivateDeleteEntryByKey")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "PrivateDeleteEntryByKey", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Oculus::Platform::Request_1<
                quest_hook::libil2cpp::Gc<
                    crate::Oculus::Platform::Models::UserDataStoreUpdateResponse,
                >,
            >,
        > = unsafe { cordl_method_info.invoke_unchecked((), (userID, key))? };
        Ok(__cordl_ret.into())
    }
    pub fn PrivateGetEntries(
        userID: u64,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::Oculus::Platform::Request_1<
                quest_hook::libil2cpp::Gc<
                    crate::System::Collections::Generic::Dictionary_2<
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    >,
                >,
            >,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (u64),
                        quest_hook::libil2cpp::Gc<
                            crate::Oculus::Platform::Request_1<
                                quest_hook::libil2cpp::Gc<
                                    crate::System::Collections::Generic::Dictionary_2<
                                        quest_hook::libil2cpp::Gc<
                                            quest_hook::libil2cpp::Il2CppString,
                                        >,
                                        quest_hook::libil2cpp::Gc<
                                            quest_hook::libil2cpp::Il2CppString,
                                        >,
                                    >,
                                >,
                            >,
                        >,
                        1usize,
                    >("PrivateGetEntries")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "PrivateGetEntries", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Oculus::Platform::Request_1<
                quest_hook::libil2cpp::Gc<
                    crate::System::Collections::Generic::Dictionary_2<
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    >,
                >,
            >,
        > = unsafe { cordl_method_info.invoke_unchecked((), (userID))? };
        Ok(__cordl_ret.into())
    }
    pub fn PrivateGetEntryByKey(
        userID: u64,
        key: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::Oculus::Platform::Request_1<
                quest_hook::libil2cpp::Gc<
                    crate::System::Collections::Generic::Dictionary_2<
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    >,
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
                            u64,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                        ),
                        quest_hook::libil2cpp::Gc<
                            crate::Oculus::Platform::Request_1<
                                quest_hook::libil2cpp::Gc<
                                    crate::System::Collections::Generic::Dictionary_2<
                                        quest_hook::libil2cpp::Gc<
                                            quest_hook::libil2cpp::Il2CppString,
                                        >,
                                        quest_hook::libil2cpp::Gc<
                                            quest_hook::libil2cpp::Il2CppString,
                                        >,
                                    >,
                                >,
                            >,
                        >,
                        2usize,
                    >("PrivateGetEntryByKey")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "PrivateGetEntryByKey", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Oculus::Platform::Request_1<
                quest_hook::libil2cpp::Gc<
                    crate::System::Collections::Generic::Dictionary_2<
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    >,
                >,
            >,
        > = unsafe { cordl_method_info.invoke_unchecked((), (userID, key))? };
        Ok(__cordl_ret.into())
    }
    pub fn PrivateWriteEntry(
        userID: u64,
        key: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::Oculus::Platform::Request_1<
                quest_hook::libil2cpp::Gc<
                    crate::Oculus::Platform::Models::UserDataStoreUpdateResponse,
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
                            u64,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                        ),
                        quest_hook::libil2cpp::Gc<
                            crate::Oculus::Platform::Request_1<
                                quest_hook::libil2cpp::Gc<
                                    crate::Oculus::Platform::Models::UserDataStoreUpdateResponse,
                                >,
                            >,
                        >,
                        3usize,
                    >("PrivateWriteEntry")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "PrivateWriteEntry", 3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Oculus::Platform::Request_1<
                quest_hook::libil2cpp::Gc<
                    crate::Oculus::Platform::Models::UserDataStoreUpdateResponse,
                >,
            >,
        > = unsafe { cordl_method_info.invoke_unchecked((), (userID, key, value))? };
        Ok(__cordl_ret.into())
    }
    pub fn PublicDeleteEntryByKey(
        userID: u64,
        key: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::Oculus::Platform::Request_1<
                quest_hook::libil2cpp::Gc<
                    crate::Oculus::Platform::Models::UserDataStoreUpdateResponse,
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
                            u64,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                        ),
                        quest_hook::libil2cpp::Gc<
                            crate::Oculus::Platform::Request_1<
                                quest_hook::libil2cpp::Gc<
                                    crate::Oculus::Platform::Models::UserDataStoreUpdateResponse,
                                >,
                            >,
                        >,
                        2usize,
                    >("PublicDeleteEntryByKey")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "PublicDeleteEntryByKey", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Oculus::Platform::Request_1<
                quest_hook::libil2cpp::Gc<
                    crate::Oculus::Platform::Models::UserDataStoreUpdateResponse,
                >,
            >,
        > = unsafe { cordl_method_info.invoke_unchecked((), (userID, key))? };
        Ok(__cordl_ret.into())
    }
    pub fn PublicGetEntries(
        userID: u64,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::Oculus::Platform::Request_1<
                quest_hook::libil2cpp::Gc<
                    crate::System::Collections::Generic::Dictionary_2<
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    >,
                >,
            >,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (u64),
                        quest_hook::libil2cpp::Gc<
                            crate::Oculus::Platform::Request_1<
                                quest_hook::libil2cpp::Gc<
                                    crate::System::Collections::Generic::Dictionary_2<
                                        quest_hook::libil2cpp::Gc<
                                            quest_hook::libil2cpp::Il2CppString,
                                        >,
                                        quest_hook::libil2cpp::Gc<
                                            quest_hook::libil2cpp::Il2CppString,
                                        >,
                                    >,
                                >,
                            >,
                        >,
                        1usize,
                    >("PublicGetEntries")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "PublicGetEntries", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Oculus::Platform::Request_1<
                quest_hook::libil2cpp::Gc<
                    crate::System::Collections::Generic::Dictionary_2<
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    >,
                >,
            >,
        > = unsafe { cordl_method_info.invoke_unchecked((), (userID))? };
        Ok(__cordl_ret.into())
    }
    pub fn PublicGetEntryByKey(
        userID: u64,
        key: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::Oculus::Platform::Request_1<
                quest_hook::libil2cpp::Gc<
                    crate::System::Collections::Generic::Dictionary_2<
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    >,
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
                            u64,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                        ),
                        quest_hook::libil2cpp::Gc<
                            crate::Oculus::Platform::Request_1<
                                quest_hook::libil2cpp::Gc<
                                    crate::System::Collections::Generic::Dictionary_2<
                                        quest_hook::libil2cpp::Gc<
                                            quest_hook::libil2cpp::Il2CppString,
                                        >,
                                        quest_hook::libil2cpp::Gc<
                                            quest_hook::libil2cpp::Il2CppString,
                                        >,
                                    >,
                                >,
                            >,
                        >,
                        2usize,
                    >("PublicGetEntryByKey")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "PublicGetEntryByKey", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Oculus::Platform::Request_1<
                quest_hook::libil2cpp::Gc<
                    crate::System::Collections::Generic::Dictionary_2<
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    >,
                >,
            >,
        > = unsafe { cordl_method_info.invoke_unchecked((), (userID, key))? };
        Ok(__cordl_ret.into())
    }
    pub fn PublicWriteEntry(
        userID: u64,
        key: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::Oculus::Platform::Request_1<
                quest_hook::libil2cpp::Gc<
                    crate::Oculus::Platform::Models::UserDataStoreUpdateResponse,
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
                            u64,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                        ),
                        quest_hook::libil2cpp::Gc<
                            crate::Oculus::Platform::Request_1<
                                quest_hook::libil2cpp::Gc<
                                    crate::Oculus::Platform::Models::UserDataStoreUpdateResponse,
                                >,
                            >,
                        >,
                        3usize,
                    >("PublicWriteEntry")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "PublicWriteEntry", 3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Oculus::Platform::Request_1<
                quest_hook::libil2cpp::Gc<
                    crate::Oculus::Platform::Models::UserDataStoreUpdateResponse,
                >,
            >,
        > = unsafe { cordl_method_info.invoke_unchecked((), (userID, key, value))? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Oculus+Platform+UserDataStore")]
impl quest_hook::libil2cpp::ObjectType for crate::Oculus::Platform::UserDataStore {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
