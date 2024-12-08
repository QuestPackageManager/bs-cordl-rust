#[cfg(feature = "System+Net+HttpListenerBasicIdentity")]
#[repr(C)]
#[derive(Debug)]
pub struct HttpListenerBasicIdentity {
    __cordl_parent: crate::System::Security::Principal::GenericIdentity,
    pub password: *mut crate::System::String,
}
#[cfg(feature = "System+Net+HttpListenerBasicIdentity")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Net::HttpListenerBasicIdentity =>
    "System.Net"."HttpListenerBasicIdentity"
);
#[cfg(feature = "System+Net+HttpListenerBasicIdentity")]
impl std::ops::Deref for crate::System::Net::HttpListenerBasicIdentity {
    type Target = crate::System::Security::Principal::GenericIdentity;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Net+HttpListenerBasicIdentity")]
impl std::ops::DerefMut for crate::System::Net::HttpListenerBasicIdentity {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Net+HttpListenerBasicIdentity")]
impl crate::System::Net::HttpListenerBasicIdentity {
    pub fn _ctor(
        &mut self,
        username: *mut crate::System::String,
        password: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (username, password))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        username: *mut crate::System::String,
        password: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (username, password))?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "System+Net+HttpListenerBasicIdentity")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Net::HttpListenerBasicIdentity {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
