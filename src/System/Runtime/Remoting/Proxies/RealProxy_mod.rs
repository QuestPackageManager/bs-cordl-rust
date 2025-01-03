#[cfg(feature = "System+Runtime+Remoting+Proxies+RealProxy")]
#[repr(C)]
#[derive(Debug)]
pub struct RealProxy {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub class_to_proxy: *mut crate::System::Type,
    pub _targetContext: *mut crate::System::Runtime::Remoting::Contexts::Context,
    pub _server: *mut crate::System::MarshalByRefObject,
    pub _targetDomainId: i32,
    pub _targetUri: *mut quest_hook::libil2cpp::Il2CppString,
    pub _objectIdentity: *mut crate::System::Runtime::Remoting::Identity,
    pub _objTP: *mut quest_hook::libil2cpp::Il2CppObject,
    pub _stubData: *mut quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "System+Runtime+Remoting+Proxies+RealProxy")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Runtime::Remoting::Proxies::RealProxy =>
    "System.Runtime.Remoting.Proxies"."RealProxy"
);
#[cfg(feature = "System+Runtime+Remoting+Proxies+RealProxy")]
impl std::ops::Deref for crate::System::Runtime::Remoting::Proxies::RealProxy {
    type Target = quest_hook::libil2cpp::Il2CppObject;
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
        s: quest_hook::libil2cpp::Gc<crate::System::MarshalByRefObject>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AttachServer", (s))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetAppDomainTarget(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = __cordl_object.invoke("GetAppDomainTarget", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetObjectData(
        &mut self,
        info: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::SerializationInfo,
        >,
        context: crate::System::Runtime::Serialization::StreamingContext,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("GetObjectData", (info, context))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetProxiedType(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<crate::System::Type>> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Type> = __cordl_object
            .invoke("GetProxiedType", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetTransparentProxy(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = __cordl_object.invoke("GetTransparentProxy", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn InternalGetProxyType(
        transparentProxy: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<crate::System::Type>> {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Type> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("InternalGetProxyType", (transparentProxy))?;
        Ok(__cordl_ret.into())
    }
    pub fn InternalGetTransparentProxy(
        &mut self,
        className: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = __cordl_object.invoke("InternalGetTransparentProxy", (className))?;
        Ok(__cordl_ret.into())
    }
    pub fn Invoke(
        &mut self,
        msg: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Remoting::Messaging::IMessage,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Runtime::Remoting::Messaging::IMessage>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Remoting::Messaging::IMessage,
        > = __cordl_object.invoke("Invoke", (msg))?;
        Ok(__cordl_ret.into())
    }
    pub fn New_0() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn New_Type1(
        classToProxy: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (classToProxy))?;
        Ok(__cordl_object.into())
    }
    pub fn New_Type_ClientIdentity2(
        classToProxy: quest_hook::libil2cpp::Gc<crate::System::Type>,
        identity: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Remoting::ClientIdentity,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (classToProxy, identity))?;
        Ok(__cordl_object.into())
    }
    pub fn New_Type_IntPtr_Il2CppObject3(
        classToProxy: quest_hook::libil2cpp::Gc<crate::System::Type>,
        stub: crate::System::IntPtr,
        stubData: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (classToProxy, stub, stubData))?;
        Ok(__cordl_object.into())
    }
    pub fn PrivateInvoke(
        rp: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Remoting::Proxies::RealProxy,
        >,
        msg: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Remoting::Messaging::IMessage,
        >,
        exc: quest_hook::libil2cpp::ByRefMut<*mut crate::System::Exception>,
        out_args: quest_hook::libil2cpp::ByRefMut<
            *mut quest_hook::libil2cpp::Il2CppArray<
                *mut quest_hook::libil2cpp::Il2CppObject,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("PrivateInvoke", (rp, msg, exc, out_args))?;
        Ok(__cordl_ret.into())
    }
    pub fn ProcessResponse(
        mrm: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Remoting::Messaging::IMethodReturnMessage,
        >,
        call: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Remoting::Messaging::MonoMethodMessage,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<*mut quest_hook::libil2cpp::Il2CppObject>,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<*mut quest_hook::libil2cpp::Il2CppObject>,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ProcessResponse", (mrm, call))?;
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_Type1(
        &mut self,
        classToProxy: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (classToProxy))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_Type_ClientIdentity2(
        &mut self,
        classToProxy: quest_hook::libil2cpp::Gc<crate::System::Type>,
        identity: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Remoting::ClientIdentity,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (classToProxy, identity))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_Type_IntPtr_Il2CppObject3(
        &mut self,
        classToProxy: quest_hook::libil2cpp::Gc<crate::System::Type>,
        stub: crate::System::IntPtr,
        stubData: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (classToProxy, stub, stubData))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_ObjectIdentity(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Runtime::Remoting::Identity>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Remoting::Identity,
        > = __cordl_object.invoke("get_ObjectIdentity", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn set_ObjectIdentity(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::System::Runtime::Remoting::Identity>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_ObjectIdentity", (value))?;
        Ok(__cordl_ret.into())
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
