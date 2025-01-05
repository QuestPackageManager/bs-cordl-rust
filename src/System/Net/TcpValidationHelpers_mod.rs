#[cfg(feature = "System+Net+TcpValidationHelpers")]
#[repr(C)]
#[derive(Debug)]
pub struct TcpValidationHelpers {
    __cordl_parent: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
}
#[cfg(feature = "System+Net+TcpValidationHelpers")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Net::TcpValidationHelpers =>
    "System.Net"."TcpValidationHelpers"
);
#[cfg(feature = "System+Net+TcpValidationHelpers")]
impl std::ops::Deref for crate::System::Net::TcpValidationHelpers {
    type Target = quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Net+TcpValidationHelpers")]
impl std::ops::DerefMut for crate::System::Net::TcpValidationHelpers {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Net+TcpValidationHelpers")]
impl crate::System::Net::TcpValidationHelpers {
    pub fn ValidatePortNumber(port: i32) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ValidatePortNumber", (port))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Net+TcpValidationHelpers")]
impl quest_hook::libil2cpp::ObjectType for crate::System::Net::TcpValidationHelpers {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
