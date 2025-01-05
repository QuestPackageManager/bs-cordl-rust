#[cfg(feature = "System+Net+NetRes")]
#[repr(C)]
#[derive(Debug)]
pub struct NetRes {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "System+Net+NetRes")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Net::NetRes => "System.Net"."NetRes"
);
#[cfg(feature = "System+Net+NetRes")]
impl std::ops::Deref for crate::System::Net::NetRes {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Net+NetRes")]
impl std::ops::DerefMut for crate::System::Net::NetRes {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Net+NetRes")]
impl crate::System::Net::NetRes {
    pub fn GetWebStatusCodeString(
        statusCode: crate::System::Net::FtpStatusCode,
        statusDescription: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetWebStatusCodeString", (statusCode, statusDescription))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetWebStatusString_Il2CppString_WebExceptionStatus0(
        Res: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        Status: crate::System::Net::WebExceptionStatus,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetWebStatusString", (Res, Status))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetWebStatusString_WebExceptionStatus1(
        Status: crate::System::Net::WebExceptionStatus,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetWebStatusString", (Status))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Net+NetRes")]
impl quest_hook::libil2cpp::ObjectType for crate::System::Net::NetRes {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
