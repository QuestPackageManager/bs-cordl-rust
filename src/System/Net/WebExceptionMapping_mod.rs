#[cfg(feature = "System+Net+WebExceptionMapping")]
#[repr(C)]
#[derive(Debug)]
pub struct WebExceptionMapping {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "System+Net+WebExceptionMapping")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Net::WebExceptionMapping => "System.Net"
    ."WebExceptionMapping"
);
#[cfg(feature = "System+Net+WebExceptionMapping")]
impl std::ops::Deref for crate::System::Net::WebExceptionMapping {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Net+WebExceptionMapping")]
impl std::ops::DerefMut for crate::System::Net::WebExceptionMapping {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Net+WebExceptionMapping")]
impl crate::System::Net::WebExceptionMapping {
    pub fn GetWebStatusString(
        status: crate::System::Net::WebExceptionStatus,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetWebStatusString", (status))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Net+WebExceptionMapping")]
impl quest_hook::libil2cpp::ObjectType for crate::System::Net::WebExceptionMapping {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
