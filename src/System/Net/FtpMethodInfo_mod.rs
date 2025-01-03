#[cfg(feature = "System+Net+FtpMethodInfo")]
#[repr(C)]
#[derive(Debug)]
pub struct FtpMethodInfo {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub Method: *mut quest_hook::libil2cpp::Il2CppString,
    pub Operation: crate::System::Net::FtpOperation,
    pub Flags: crate::System::Net::FtpMethodFlags,
    pub HttpCommand: *mut quest_hook::libil2cpp::Il2CppString,
}
#[cfg(feature = "System+Net+FtpMethodInfo")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Net::FtpMethodInfo => "System.Net"
    ."FtpMethodInfo"
);
#[cfg(feature = "System+Net+FtpMethodInfo")]
impl std::ops::Deref for crate::System::Net::FtpMethodInfo {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Net+FtpMethodInfo")]
impl std::ops::DerefMut for crate::System::Net::FtpMethodInfo {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Net+FtpMethodInfo")]
impl crate::System::Net::FtpMethodInfo {
    pub fn GetMethodInfo(
        method: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Net::FtpMethodInfo>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Net::FtpMethodInfo> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetMethodInfo", (method))?;
        Ok(__cordl_ret.into())
    }
    pub fn HasFlag(
        &mut self,
        flags: crate::System::Net::FtpMethodFlags,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("HasFlag", (flags))?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        method: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        operation: crate::System::Net::FtpOperation,
        flags: crate::System::Net::FtpMethodFlags,
        httpCommand: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (method, operation, flags, httpCommand))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        method: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        operation: crate::System::Net::FtpOperation,
        flags: crate::System::Net::FtpMethodFlags,
        httpCommand: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (method, operation, flags, httpCommand))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_IsCommandOnly(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_IsCommandOnly", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_IsDownload(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_IsDownload", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_IsUpload(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_IsUpload", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_ShouldParseForResponseUri(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("get_ShouldParseForResponseUri", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Net+FtpMethodInfo")]
impl quest_hook::libil2cpp::ObjectType for crate::System::Net::FtpMethodInfo {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
