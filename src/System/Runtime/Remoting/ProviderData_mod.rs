#[cfg(feature = "System+Runtime+Remoting+ProviderData")]
#[repr(C)]
#[derive(Debug)]
pub struct ProviderData {
    __cordl_parent: crate::System::Object,
    pub Ref: *mut crate::System::String,
    pub Type: *mut crate::System::String,
    pub Id: *mut crate::System::String,
    pub CustomProperties: *mut crate::System::Collections::Hashtable,
    pub CustomData: *mut crate::System::Collections::IList,
}
#[cfg(feature = "System+Runtime+Remoting+ProviderData")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Runtime::Remoting::ProviderData =>
    "System.Runtime.Remoting"."ProviderData"
);
#[cfg(feature = "System+Runtime+Remoting+ProviderData")]
impl std::ops::Deref for crate::System::Runtime::Remoting::ProviderData {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Runtime+Remoting+ProviderData")]
impl std::ops::DerefMut for crate::System::Runtime::Remoting::ProviderData {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Runtime+Remoting+ProviderData")]
impl crate::System::Runtime::Remoting::ProviderData {
    pub fn CopyFrom(
        &mut self,
        other: *mut crate::System::Runtime::Remoting::ProviderData,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CopyFrom", (other))?;
        Ok(__cordl_ret)
    }
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
#[cfg(feature = "System+Runtime+Remoting+ProviderData")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Runtime::Remoting::ProviderData {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
