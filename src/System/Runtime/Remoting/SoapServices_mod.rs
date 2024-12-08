#[cfg(feature = "System+Runtime+Remoting+SoapServices")]
#[repr(C)]
#[derive(Debug)]
pub struct SoapServices {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "System+Runtime+Remoting+SoapServices")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Runtime::Remoting::SoapServices =>
    "System.Runtime.Remoting"."SoapServices"
);
#[cfg(feature = "System+Runtime+Remoting+SoapServices")]
impl std::ops::Deref for crate::System::Runtime::Remoting::SoapServices {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Runtime+Remoting+SoapServices")]
impl std::ops::DerefMut for crate::System::Runtime::Remoting::SoapServices {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Runtime+Remoting+SoapServices")]
impl crate::System::Runtime::Remoting::SoapServices {
    #[cfg(feature = "System+Runtime+Remoting+SoapServices+TypeInfo")]
    pub type TypeInfo = crate::System::Runtime::Remoting::SoapServices_TypeInfo;
}
#[cfg(feature = "System+Runtime+Remoting+SoapServices")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Runtime::Remoting::SoapServices {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "System+Runtime+Remoting+SoapServices+TypeInfo")]
#[repr(C)]
#[derive(Debug)]
pub struct SoapServices_TypeInfo {
    __cordl_parent: crate::System::Object,
    pub Attributes: *mut crate::System::Collections::Hashtable,
    pub Elements: *mut crate::System::Collections::Hashtable,
}
#[cfg(feature = "System+Runtime+Remoting+SoapServices+TypeInfo")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Runtime::Remoting::SoapServices_TypeInfo
    => "System.Runtime.Remoting"."SoapServices/TypeInfo"
);
#[cfg(feature = "System+Runtime+Remoting+SoapServices+TypeInfo")]
impl std::ops::Deref for crate::System::Runtime::Remoting::SoapServices_TypeInfo {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Runtime+Remoting+SoapServices+TypeInfo")]
impl std::ops::DerefMut for crate::System::Runtime::Remoting::SoapServices_TypeInfo {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Runtime+Remoting+SoapServices+TypeInfo")]
impl crate::System::Runtime::Remoting::SoapServices_TypeInfo {
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "System+Runtime+Remoting+SoapServices+TypeInfo")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Runtime::Remoting::SoapServices_TypeInfo {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
