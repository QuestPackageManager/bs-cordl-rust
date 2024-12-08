#[cfg(feature = "Mono+Net+Security+AsyncReadRequest")]
#[repr(C)]
#[derive(Debug)]
pub struct AsyncReadRequest {
    __cordl_parent: crate::Mono::Net::Security::AsyncReadOrWriteRequest,
}
#[cfg(feature = "Mono+Net+Security+AsyncReadRequest")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Mono::Net::Security::AsyncReadRequest =>
    "Mono.Net.Security"."AsyncReadRequest"
);
#[cfg(feature = "Mono+Net+Security+AsyncReadRequest")]
impl std::ops::Deref for crate::Mono::Net::Security::AsyncReadRequest {
    type Target = crate::Mono::Net::Security::AsyncReadOrWriteRequest;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Mono+Net+Security+AsyncReadRequest")]
impl std::ops::DerefMut for crate::Mono::Net::Security::AsyncReadRequest {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Mono+Net+Security+AsyncReadRequest")]
impl crate::Mono::Net::Security::AsyncReadRequest {
    pub fn _ctor(
        &mut self,
        parent: *mut crate::Mono::Net::Security::MobileAuthenticatedStream,
        sync: bool,
        buffer: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        offset: i32,
        _cordl_size: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (parent, sync, buffer, offset, _cordl_size))?;
        Ok(__cordl_ret)
    }
    pub fn Run(
        &mut self,
        status: crate::Mono::Net::Security::AsyncOperationStatus,
    ) -> quest_hook::libil2cpp::Result<
        crate::Mono::Net::Security::AsyncOperationStatus,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::Mono::Net::Security::AsyncOperationStatus = __cordl_object
            .invoke("Run", (status))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        parent: *mut crate::Mono::Net::Security::MobileAuthenticatedStream,
        sync: bool,
        buffer: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        offset: i32,
        _cordl_size: i32,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (parent, sync, buffer, offset, _cordl_size))?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "Mono+Net+Security+AsyncReadRequest")]
impl quest_hook::libil2cpp::ObjectType for crate::Mono::Net::Security::AsyncReadRequest {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
