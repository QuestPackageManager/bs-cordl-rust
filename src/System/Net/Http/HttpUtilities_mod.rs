#[cfg(feature = "System+Net+Http+HttpUtilities")]
#[repr(C)]
#[derive(Debug)]
pub struct HttpUtilities {
    __cordl_parent: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
}
#[cfg(feature = "System+Net+Http+HttpUtilities")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Net::Http::HttpUtilities =>
    "System.Net.Http"."HttpUtilities"
);
#[cfg(feature = "System+Net+Http+HttpUtilities")]
impl std::ops::Deref for crate::System::Net::Http::HttpUtilities {
    type Target = quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Net+Http+HttpUtilities")]
impl std::ops::DerefMut for crate::System::Net::Http::HttpUtilities {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Net+Http+HttpUtilities")]
impl crate::System::Net::Http::HttpUtilities {
    pub fn IsSecureWebSocketScheme(
        scheme: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IsSecureWebSocketScheme", (scheme))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsSupportedSecureScheme(
        scheme: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IsSupportedSecureScheme", (scheme))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Net+Http+HttpUtilities")]
impl quest_hook::libil2cpp::ObjectType for crate::System::Net::Http::HttpUtilities {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
