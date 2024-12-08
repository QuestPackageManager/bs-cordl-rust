#[cfg(feature = "System+Runtime+Remoting+Proxies+RealProxy")]
#[repr(C)]
#[derive(Debug)]
pub struct RealProxy {
    __cordl_parent: crate::System::Object,
    pub class_to_proxy: *mut crate::System::Type,
    pub _targetContext: *mut crate::System::Runtime::Remoting::Contexts::Context,
    pub _server: *mut crate::System::MarshalByRefObject,
    pub _targetDomainId: i32,
    pub _targetUri: *mut crate::System::String,
    pub _objectIdentity: *mut crate::System::Runtime::Remoting::Identity,
    pub _objTP: *mut crate::System::Object,
    pub _stubData: *mut crate::System::Object,
}
#[cfg(feature = "System+Runtime+Remoting+Proxies+RealProxy")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Runtime::Remoting::Proxies::RealProxy =>
    "System.Runtime.Remoting.Proxies"."RealProxy"
);
#[cfg(feature = "System+Runtime+Remoting+Proxies+RealProxy")]
impl std::ops::Deref for crate::System::Runtime::Remoting::Proxies::RealProxy {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Runtime+Remoting+Proxies+RealProxy")]
impl std::ops::DerefMut for crate::System::Runtime::Remoting::Proxies::RealProxy {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Runtime+Remoting+Proxies+RealProxy")]
impl crate::System::Runtime::Remoting::Proxies::RealProxy {
    pub fn AttachServer(
        &mut self,
        s: *mut crate::System::MarshalByRefObject,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AttachServer", (s))?;
        Ok(__cordl_ret)
    }
    pub fn GetAppDomainTarget(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Object> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Object = __cordl_object
            .invoke("GetAppDomainTarget", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetObjectData(
        &mut self,
        info: *mut crate::System::Runtime::Serialization::SerializationInfo,
        context: crate::System::Runtime::Serialization::StreamingContext,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("GetObjectData", (info, context))?;
        Ok(__cordl_ret)
    }
    pub fn GetProxiedType(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Type> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Type = __cordl_object
            .invoke("GetProxiedType", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetTransparentProxy(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Object> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Object = __cordl_object
            .invoke("GetTransparentProxy", ())?;
        Ok(__cordl_ret)
    }
    pub fn InternalGetTransparentProxy(
        &mut self,
        className: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Object> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Object = __cordl_object
            .invoke("InternalGetTransparentProxy", (className))?;
        Ok(__cordl_ret)
    }
    pub fn Invoke(
        &mut self,
        msg: *mut crate::System::Runtime::Remoting::Messaging::IMessage,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Runtime::Remoting::Messaging::IMessage,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Runtime::Remoting::Messaging::IMessage = __cordl_object
            .invoke("Invoke", (msg))?;
        Ok(__cordl_ret)
    }
    pub fn New_0() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn New_Type1(
        classToProxy: *mut crate::System::Type,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (classToProxy))?;
        Ok(__cordl_object)
    }
    pub fn New_Type_ClientIdentity2(
        classToProxy: *mut crate::System::Type,
        identity: *mut crate::System::Runtime::Remoting::ClientIdentity,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (classToProxy, identity))?;
        Ok(__cordl_object)
    }
    pub fn New_Type_IntPtr_Object3(
        classToProxy: *mut crate::System::Type,
        stub: crate::System::IntPtr,
        stubData: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (classToProxy, stub, stubData))?;
        Ok(__cordl_object)
    }
    pub fn SetTargetDomain(
        &mut self,
        domainId: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetTargetDomain", (domainId))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_Type1(
        &mut self,
        classToProxy: *mut crate::System::Type,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (classToProxy))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_Type_ClientIdentity2(
        &mut self,
        classToProxy: *mut crate::System::Type,
        identity: *mut crate::System::Runtime::Remoting::ClientIdentity,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (classToProxy, identity))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_Type_IntPtr_Object3(
        &mut self,
        classToProxy: *mut crate::System::Type,
        stub: crate::System::IntPtr,
        stubData: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (classToProxy, stub, stubData))?;
        Ok(__cordl_ret)
    }
    pub fn get_ObjectIdentity(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Runtime::Remoting::Identity> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Runtime::Remoting::Identity = __cordl_object
            .invoke("get_ObjectIdentity", ())?;
        Ok(__cordl_ret)
    }
    pub fn set_ObjectIdentity(
        &mut self,
        value: *mut crate::System::Runtime::Remoting::Identity,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_ObjectIdentity", (value))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "System+Runtime+Remoting+Proxies+RealProxy")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Runtime::Remoting::Proxies::RealProxy {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
