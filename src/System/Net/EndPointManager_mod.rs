#[cfg(feature = "System+Net+EndPointManager")]
#[repr(C)]
#[derive(Debug)]
pub struct EndPointManager {
    __cordl_parent: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
}
#[cfg(feature = "System+Net+EndPointManager")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Net::EndPointManager => "System.Net"
    ."EndPointManager"
);
#[cfg(feature = "System+Net+EndPointManager")]
impl std::ops::Deref for crate::System::Net::EndPointManager {
    type Target = quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Net+EndPointManager")]
impl std::ops::DerefMut for crate::System::Net::EndPointManager {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Net+EndPointManager")]
impl crate::System::Net::EndPointManager {
    pub fn AddListener(
        listener: quest_hook::libil2cpp::Gc<crate::System::Net::HttpListener>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("AddListener", (listener))?;
        Ok(__cordl_ret.into())
    }
    pub fn AddPrefix(
        prefix: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        listener: quest_hook::libil2cpp::Gc<crate::System::Net::HttpListener>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("AddPrefix", (prefix, listener))?;
        Ok(__cordl_ret.into())
    }
    pub fn AddPrefixInternal(
        p: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        listener: quest_hook::libil2cpp::Gc<crate::System::Net::HttpListener>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("AddPrefixInternal", (p, listener))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetEPListener(
        host: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        port: i32,
        listener: quest_hook::libil2cpp::Gc<crate::System::Net::HttpListener>,
        secure: bool,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Net::EndPointListener>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Net::EndPointListener,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetEPListener", (host, port, listener, secure))?;
        Ok(__cordl_ret.into())
    }
    pub fn RemoveEndPoint(
        epl: quest_hook::libil2cpp::Gc<crate::System::Net::EndPointListener>,
        ep: quest_hook::libil2cpp::Gc<crate::System::Net::IPEndPoint>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("RemoveEndPoint", (epl, ep))?;
        Ok(__cordl_ret.into())
    }
    pub fn RemoveListener(
        listener: quest_hook::libil2cpp::Gc<crate::System::Net::HttpListener>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("RemoveListener", (listener))?;
        Ok(__cordl_ret.into())
    }
    pub fn RemovePrefix(
        prefix: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        listener: quest_hook::libil2cpp::Gc<crate::System::Net::HttpListener>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("RemovePrefix", (prefix, listener))?;
        Ok(__cordl_ret.into())
    }
    pub fn RemovePrefixInternal(
        prefix: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        listener: quest_hook::libil2cpp::Gc<crate::System::Net::HttpListener>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("RemovePrefixInternal", (prefix, listener))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Net+EndPointManager")]
impl quest_hook::libil2cpp::ObjectType for crate::System::Net::EndPointManager {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
