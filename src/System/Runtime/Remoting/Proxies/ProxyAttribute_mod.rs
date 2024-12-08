#[cfg(feature = "System+Runtime+Remoting+Proxies+ProxyAttribute")]
#[repr(C)]
#[derive(Debug)]
pub struct ProxyAttribute {
    __cordl_parent: crate::System::Attribute,
}
#[cfg(feature = "System+Runtime+Remoting+Proxies+ProxyAttribute")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::Runtime::Remoting::Proxies::ProxyAttribute =>
    "System.Runtime.Remoting.Proxies"."ProxyAttribute"
);
#[cfg(feature = "System+Runtime+Remoting+Proxies+ProxyAttribute")]
impl std::ops::Deref for crate::System::Runtime::Remoting::Proxies::ProxyAttribute {
    type Target = crate::System::Attribute;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Runtime+Remoting+Proxies+ProxyAttribute")]
impl std::ops::DerefMut for crate::System::Runtime::Remoting::Proxies::ProxyAttribute {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Runtime+Remoting+Proxies+ProxyAttribute")]
impl crate::System::Runtime::Remoting::Proxies::ProxyAttribute {
    pub fn CreateInstance(
        &mut self,
        serverType: *mut crate::System::Type,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::MarshalByRefObject> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::MarshalByRefObject = __cordl_object
            .invoke("CreateInstance", (serverType))?;
        Ok(__cordl_ret)
    }
    pub fn CreateProxy(
        &mut self,
        objRef: *mut crate::System::Runtime::Remoting::ObjRef,
        serverType: *mut crate::System::Type,
        serverObject: *mut crate::System::Object,
        serverContext: *mut crate::System::Runtime::Remoting::Contexts::Context,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Runtime::Remoting::Proxies::RealProxy,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Runtime::Remoting::Proxies::RealProxy = __cordl_object
            .invoke("CreateProxy", (objRef, serverType, serverObject, serverContext))?;
        Ok(__cordl_ret)
    }
    pub fn GetPropertiesForNewContext(
        &mut self,
        msg: *mut crate::System::Runtime::Remoting::Activation::IConstructionCallMessage,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("GetPropertiesForNewContext", (msg))?;
        Ok(__cordl_ret)
    }
    pub fn IsContextOK(
        &mut self,
        ctx: *mut crate::System::Runtime::Remoting::Contexts::Context,
        msg: *mut crate::System::Runtime::Remoting::Activation::IConstructionCallMessage,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("IsContextOK", (ctx, msg))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "System+Runtime+Remoting+Proxies+ProxyAttribute")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Runtime::Remoting::Proxies::ProxyAttribute {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
