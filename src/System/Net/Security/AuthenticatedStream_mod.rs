#[cfg(feature = "System+Net+Security+AuthenticatedStream")]
#[repr(C)]
#[derive(Debug)]
pub struct AuthenticatedStream {
    __cordl_parent: crate::System::IO::Stream,
    pub _InnerStream: *mut crate::System::IO::Stream,
    pub _LeaveStreamOpen: bool,
}
#[cfg(feature = "System+Net+Security+AuthenticatedStream")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Net::Security::AuthenticatedStream =>
    "System.Net.Security"."AuthenticatedStream"
);
#[cfg(feature = "System+Net+Security+AuthenticatedStream")]
impl std::ops::Deref for crate::System::Net::Security::AuthenticatedStream {
    type Target = crate::System::IO::Stream;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Net+Security+AuthenticatedStream")]
impl std::ops::DerefMut for crate::System::Net::Security::AuthenticatedStream {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Net+Security+AuthenticatedStream")]
impl crate::System::Net::Security::AuthenticatedStream {
    pub fn Dispose(
        &mut self,
        disposing: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Dispose", (disposing))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        innerStream: *mut crate::System::IO::Stream,
        leaveInnerStreamOpen: bool,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (innerStream, leaveInnerStreamOpen))?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
        innerStream: *mut crate::System::IO::Stream,
        leaveInnerStreamOpen: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (innerStream, leaveInnerStreamOpen))?;
        Ok(__cordl_ret)
    }
    pub fn get_InnerStream(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::IO::Stream> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::IO::Stream = __cordl_object
            .invoke("get_InnerStream", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_IsAuthenticated(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_IsAuthenticated", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "System+Net+Security+AuthenticatedStream")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Net::Security::AuthenticatedStream {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}