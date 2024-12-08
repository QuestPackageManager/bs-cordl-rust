#[cfg(feature = "Oculus+Platform+MessageWithDataStoreUnderPrivateUserDataStore")]
#[repr(C)]
#[derive(Debug)]
pub struct MessageWithDataStoreUnderPrivateUserDataStore {
    __cordl_parent: crate::Oculus::Platform::Message_1<
        *mut crate::System::Collections::Generic::Dictionary_2<
            *mut crate::System::String,
            *mut crate::System::String,
        >,
    >,
}
#[cfg(feature = "Oculus+Platform+MessageWithDataStoreUnderPrivateUserDataStore")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Oculus::Platform::MessageWithDataStoreUnderPrivateUserDataStore =>
    "Oculus.Platform"."MessageWithDataStoreUnderPrivateUserDataStore"
);
#[cfg(feature = "Oculus+Platform+MessageWithDataStoreUnderPrivateUserDataStore")]
impl std::ops::Deref
for crate::Oculus::Platform::MessageWithDataStoreUnderPrivateUserDataStore {
    type Target = crate::Oculus::Platform::Message_1<
        *mut crate::System::Collections::Generic::Dictionary_2<
            *mut crate::System::String,
            *mut crate::System::String,
        >,
    >;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Oculus+Platform+MessageWithDataStoreUnderPrivateUserDataStore")]
impl std::ops::DerefMut
for crate::Oculus::Platform::MessageWithDataStoreUnderPrivateUserDataStore {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Oculus+Platform+MessageWithDataStoreUnderPrivateUserDataStore")]
impl crate::Oculus::Platform::MessageWithDataStoreUnderPrivateUserDataStore {
    pub fn GetDataFromMessage(
        &mut self,
        c_message: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Collections::Generic::Dictionary_2<
            *mut crate::System::String,
            *mut crate::System::String,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::Generic::Dictionary_2<
            *mut crate::System::String,
            *mut crate::System::String,
        > = __cordl_object.invoke("GetDataFromMessage", (c_message))?;
        Ok(__cordl_ret)
    }
    pub fn GetDataStore(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Collections::Generic::Dictionary_2<
            *mut crate::System::String,
            *mut crate::System::String,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::Generic::Dictionary_2<
            *mut crate::System::String,
            *mut crate::System::String,
        > = __cordl_object.invoke("GetDataStore", ())?;
        Ok(__cordl_ret)
    }
    pub fn New(
        c_message: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (c_message))?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
        c_message: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (c_message))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "Oculus+Platform+MessageWithDataStoreUnderPrivateUserDataStore")]
impl quest_hook::libil2cpp::ObjectType
for crate::Oculus::Platform::MessageWithDataStoreUnderPrivateUserDataStore {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}