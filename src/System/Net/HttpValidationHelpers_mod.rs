#[cfg(feature = "System+Net+HttpValidationHelpers")]
#[repr(C)]
#[derive(Debug)]
pub struct HttpValidationHelpers {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "System+Net+HttpValidationHelpers")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Net::HttpValidationHelpers =>
    "System.Net"."HttpValidationHelpers"
);
#[cfg(feature = "System+Net+HttpValidationHelpers")]
impl std::ops::Deref for crate::System::Net::HttpValidationHelpers {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Net+HttpValidationHelpers")]
impl std::ops::DerefMut for crate::System::Net::HttpValidationHelpers {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Net+HttpValidationHelpers")]
impl crate::System::Net::HttpValidationHelpers {
    pub fn IsInvalidMethodOrHeaderString(
        stringValue: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IsInvalidMethodOrHeaderString", (stringValue))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Net+HttpValidationHelpers")]
impl quest_hook::libil2cpp::ObjectType for crate::System::Net::HttpValidationHelpers {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
