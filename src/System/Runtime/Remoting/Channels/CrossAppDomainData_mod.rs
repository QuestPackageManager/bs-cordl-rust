#[cfg(feature = "System+Runtime+Remoting+Channels+CrossAppDomainData")]
#[repr(C)]
#[derive(Debug)]
pub struct CrossAppDomainData {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _ContextID: *mut quest_hook::libil2cpp::Il2CppObject,
    pub _DomainID: i32,
    pub _processGuid: *mut quest_hook::libil2cpp::Il2CppString,
}
#[cfg(feature = "System+Runtime+Remoting+Channels+CrossAppDomainData")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::Runtime::Remoting::Channels::CrossAppDomainData =>
    "System.Runtime.Remoting.Channels"."CrossAppDomainData"
);
#[cfg(feature = "System+Runtime+Remoting+Channels+CrossAppDomainData")]
impl std::ops::Deref for crate::System::Runtime::Remoting::Channels::CrossAppDomainData {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Runtime+Remoting+Channels+CrossAppDomainData")]
impl std::ops::DerefMut
for crate::System::Runtime::Remoting::Channels::CrossAppDomainData {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Runtime+Remoting+Channels+CrossAppDomainData")]
impl crate::System::Runtime::Remoting::Channels::CrossAppDomainData {
    pub fn New(
        domainId: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (domainId))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        domainId: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (domainId))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_DomainID(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_DomainID", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_ProcessID(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("get_ProcessID", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Runtime+Remoting+Channels+CrossAppDomainData")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Runtime::Remoting::Channels::CrossAppDomainData {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
