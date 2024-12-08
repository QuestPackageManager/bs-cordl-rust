#[cfg(feature = "System+Net+AuthenticationSchemeSelector")]
#[repr(C)]
#[derive(Debug)]
pub struct AuthenticationSchemeSelector {
    __cordl_parent: crate::System::MulticastDelegate,
}
#[cfg(feature = "System+Net+AuthenticationSchemeSelector")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Net::AuthenticationSchemeSelector =>
    "System.Net"."AuthenticationSchemeSelector"
);
#[cfg(feature = "System+Net+AuthenticationSchemeSelector")]
impl std::ops::Deref for crate::System::Net::AuthenticationSchemeSelector {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Net+AuthenticationSchemeSelector")]
impl std::ops::DerefMut for crate::System::Net::AuthenticationSchemeSelector {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Net+AuthenticationSchemeSelector")]
impl crate::System::Net::AuthenticationSchemeSelector {
    pub fn Invoke(
        &mut self,
        httpRequest: *mut crate::System::Net::HttpListenerRequest,
    ) -> quest_hook::libil2cpp::Result<crate::System::Net::AuthenticationSchemes> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::Net::AuthenticationSchemes = __cordl_object
            .invoke("Invoke", (httpRequest))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        object: *mut crate::System::Object,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (object, method))?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
        object: *mut crate::System::Object,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (object, method))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "System+Net+AuthenticationSchemeSelector")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Net::AuthenticationSchemeSelector {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
