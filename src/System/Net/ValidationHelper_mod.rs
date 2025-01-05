#[cfg(feature = "System+Net+ValidationHelper")]
#[repr(C)]
#[derive(Debug)]
pub struct ValidationHelper {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "System+Net+ValidationHelper")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Net::ValidationHelper => "System.Net"
    ."ValidationHelper"
);
#[cfg(feature = "System+Net+ValidationHelper")]
impl std::ops::Deref for crate::System::Net::ValidationHelper {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Net+ValidationHelper")]
impl std::ops::DerefMut for crate::System::Net::ValidationHelper {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Net+ValidationHelper")]
impl crate::System::Net::ValidationHelper {
    pub fn IsBlankString(
        stringValue: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IsBlankString", (stringValue))?;
        Ok(__cordl_ret.into())
    }
    pub fn MakeStringNull(
        stringValue: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("MakeStringNull", (stringValue))?;
        Ok(__cordl_ret.into())
    }
    pub fn ValidateTcpPort(port: i32) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ValidateTcpPort", (port))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Net+ValidationHelper")]
impl quest_hook::libil2cpp::ObjectType for crate::System::Net::ValidationHelper {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
