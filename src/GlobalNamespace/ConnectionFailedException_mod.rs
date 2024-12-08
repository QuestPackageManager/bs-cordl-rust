#[cfg(feature = "ConnectionFailedException")]
#[repr(C)]
#[derive(Debug)]
pub struct ConnectionFailedException {
    __cordl_parent: crate::System::Exception,
    pub reason: ConnectionFailedReason,
}
#[cfg(feature = "ConnectionFailedException")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for ConnectionFailedException => ""
    ."ConnectionFailedException"
);
#[cfg(feature = "ConnectionFailedException")]
impl std::ops::Deref for ConnectionFailedException {
    type Target = crate::System::Exception;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "ConnectionFailedException")]
impl std::ops::DerefMut for ConnectionFailedException {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "ConnectionFailedException")]
impl ConnectionFailedException {
    pub fn New_ConnectionFailedReason0(
        reason: ConnectionFailedReason,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (reason))?;
        Ok(__cordl_object)
    }
    pub fn New_String1(
        reason: ConnectionFailedReason,
        message: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (reason, message))?;
        Ok(__cordl_object)
    }
    pub fn _ctor_ConnectionFailedReason0(
        &mut self,
        reason: ConnectionFailedReason,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (reason))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_String1(
        &mut self,
        reason: ConnectionFailedReason,
        message: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (reason, message))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "ConnectionFailedException")]
impl quest_hook::libil2cpp::ObjectType for ConnectionFailedException {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
