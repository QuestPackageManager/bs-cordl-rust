#[cfg(feature = "System+Net+HeaderInfo")]
#[repr(C)]
#[derive(Debug)]
pub struct HeaderInfo {
    __cordl_parent: crate::System::Object,
    pub IsRequestRestricted: bool,
    pub IsResponseRestricted: bool,
    pub Parser: *mut crate::System::Net::HeaderParser,
    pub HeaderName: *mut crate::System::String,
    pub AllowMultiValues: bool,
}
#[cfg(feature = "System+Net+HeaderInfo")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Net::HeaderInfo => "System.Net"
    ."HeaderInfo"
);
#[cfg(feature = "System+Net+HeaderInfo")]
impl std::ops::Deref for crate::System::Net::HeaderInfo {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Net+HeaderInfo")]
impl std::ops::DerefMut for crate::System::Net::HeaderInfo {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Net+HeaderInfo")]
impl crate::System::Net::HeaderInfo {
    pub fn New(
        name: *mut crate::System::String,
        requestRestricted: bool,
        responseRestricted: bool,
        multi: bool,
        p: *mut crate::System::Net::HeaderParser,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (name, requestRestricted, responseRestricted, multi, p),
            )?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
        name: *mut crate::System::String,
        requestRestricted: bool,
        responseRestricted: bool,
        multi: bool,
        p: *mut crate::System::Net::HeaderParser,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (name, requestRestricted, responseRestricted, multi, p))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "System+Net+HeaderInfo")]
impl quest_hook::libil2cpp::ObjectType for crate::System::Net::HeaderInfo {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}