#[cfg(feature = "System+Net+Logging")]
#[repr(C)]
#[derive(Debug)]
pub struct Logging {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "System+Net+Logging")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Net::Logging => "System.Net"."Logging"
);
#[cfg(feature = "System+Net+Logging")]
impl std::ops::Deref for crate::System::Net::Logging {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Net+Logging")]
impl std::ops::DerefMut for crate::System::Net::Logging {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Net+Logging")]
impl crate::System::Net::Logging {
    pub fn get_On() -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_On", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Net+Logging")]
impl quest_hook::libil2cpp::ObjectType for crate::System::Net::Logging {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
