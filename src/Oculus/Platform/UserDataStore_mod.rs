#[cfg(feature = "Oculus+Platform+UserDataStore")]
#[repr(C)]
#[derive(Debug)]
pub struct UserDataStore {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "Oculus+Platform+UserDataStore")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Oculus::Platform::UserDataStore =>
    "Oculus.Platform"."UserDataStore"
);
#[cfg(feature = "Oculus+Platform+UserDataStore")]
impl std::ops::Deref for crate::Oculus::Platform::UserDataStore {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Oculus+Platform+UserDataStore")]
impl std::ops::DerefMut for crate::Oculus::Platform::UserDataStore {
    fn deref_mut(&mut self) -> &mut Self::Target {
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
                *mut crate::Oculus::Platform::Models::UserDataStoreUpdateResponse,
            >,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Oculus::Platform::Request_1<
                *mut crate::Oculus::Platform::Models::UserDataStoreUpdateResponse,
            >,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("PrivateDeleteEntryByKey", (userID, key))?;
        Ok(__cordl_ret.into())
    }
    pub fn PrivateGetEntries(
        userID: u64,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::Oculus::Platform::Request_1<
                *mut crate::System::Collections::Generic::Dictionary_2<
                    *mut quest_hook::libil2cpp::Il2CppString,
                    *mut quest_hook::libil2cpp::Il2CppString,
                >,
            >,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Oculus::Platform::Request_1<
                *mut crate::System::Collections::Generic::Dictionary_2<
                    *mut quest_hook::libil2cpp::Il2CppString,
                    *mut quest_hook::libil2cpp::Il2CppString,
                >,
            >,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("PrivateGetEntries", (userID))?;
        Ok(__cordl_ret.into())
    }
    pub fn PrivateGetEntryByKey(
        userID: u64,
        key: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::Oculus::Platform::Request_1<
                *mut crate::System::Collections::Generic::Dictionary_2<
                    *mut quest_hook::libil2cpp::Il2CppString,
                    *mut quest_hook::libil2cpp::Il2CppString,
                >,
            >,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Oculus::Platform::Request_1<
                *mut crate::System::Collections::Generic::Dictionary_2<
                    *mut quest_hook::libil2cpp::Il2CppString,
                    *mut quest_hook::libil2cpp::Il2CppString,
                >,
            >,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("PrivateGetEntryByKey", (userID, key))?;
        Ok(__cordl_ret.into())
    }
    pub fn PrivateWriteEntry(
        userID: u64,
        key: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::Oculus::Platform::Request_1<
                *mut crate::Oculus::Platform::Models::UserDataStoreUpdateResponse,
            >,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Oculus::Platform::Request_1<
                *mut crate::Oculus::Platform::Models::UserDataStoreUpdateResponse,
            >,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("PrivateWriteEntry", (userID, key, value))?;
        Ok(__cordl_ret.into())
    }
    pub fn PublicDeleteEntryByKey(
        userID: u64,
        key: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::Oculus::Platform::Request_1<
                *mut crate::Oculus::Platform::Models::UserDataStoreUpdateResponse,
            >,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Oculus::Platform::Request_1<
                *mut crate::Oculus::Platform::Models::UserDataStoreUpdateResponse,
            >,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("PublicDeleteEntryByKey", (userID, key))?;
        Ok(__cordl_ret.into())
    }
    pub fn PublicGetEntries(
        userID: u64,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::Oculus::Platform::Request_1<
                *mut crate::System::Collections::Generic::Dictionary_2<
                    *mut quest_hook::libil2cpp::Il2CppString,
                    *mut quest_hook::libil2cpp::Il2CppString,
                >,
            >,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Oculus::Platform::Request_1<
                *mut crate::System::Collections::Generic::Dictionary_2<
                    *mut quest_hook::libil2cpp::Il2CppString,
                    *mut quest_hook::libil2cpp::Il2CppString,
                >,
            >,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("PublicGetEntries", (userID))?;
        Ok(__cordl_ret.into())
    }
    pub fn PublicGetEntryByKey(
        userID: u64,
        key: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::Oculus::Platform::Request_1<
                *mut crate::System::Collections::Generic::Dictionary_2<
                    *mut quest_hook::libil2cpp::Il2CppString,
                    *mut quest_hook::libil2cpp::Il2CppString,
                >,
            >,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Oculus::Platform::Request_1<
                *mut crate::System::Collections::Generic::Dictionary_2<
                    *mut quest_hook::libil2cpp::Il2CppString,
                    *mut quest_hook::libil2cpp::Il2CppString,
                >,
            >,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("PublicGetEntryByKey", (userID, key))?;
        Ok(__cordl_ret.into())
    }
    pub fn PublicWriteEntry(
        userID: u64,
        key: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::Oculus::Platform::Request_1<
                *mut crate::Oculus::Platform::Models::UserDataStoreUpdateResponse,
            >,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Oculus::Platform::Request_1<
                *mut crate::Oculus::Platform::Models::UserDataStoreUpdateResponse,
            >,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("PublicWriteEntry", (userID, key, value))?;
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
