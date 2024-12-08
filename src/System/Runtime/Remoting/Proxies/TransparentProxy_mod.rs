#[cfg(feature = "System+Runtime+Remoting+Proxies+TransparentProxy")]
#[repr(C)]
#[derive(Debug)]
pub struct TransparentProxy {
    __cordl_parent: crate::System::Object,
    pub _rp: *mut crate::System::Runtime::Remoting::Proxies::RealProxy,
    pub _class: crate::Mono::RuntimeRemoteClassHandle,
    pub _custom_type_info: bool,
}
#[cfg(feature = "System+Runtime+Remoting+Proxies+TransparentProxy")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::Runtime::Remoting::Proxies::TransparentProxy =>
    "System.Runtime.Remoting.Proxies"."TransparentProxy"
);
#[cfg(feature = "System+Runtime+Remoting+Proxies+TransparentProxy")]
impl std::ops::Deref for crate::System::Runtime::Remoting::Proxies::TransparentProxy {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Runtime+Remoting+Proxies+TransparentProxy")]
impl std::ops::DerefMut for crate::System::Runtime::Remoting::Proxies::TransparentProxy {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Runtime+Remoting+Proxies+TransparentProxy")]
impl crate::System::Runtime::Remoting::Proxies::TransparentProxy {
    pub fn GetProxyType(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::RuntimeType> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::RuntimeType = __cordl_object
            .invoke("GetProxyType", ())?;
        Ok(__cordl_ret)
    }
    pub fn StoreRemoteField(
        &mut self,
        classPtr: crate::System::IntPtr,
        fieldPtr: crate::System::IntPtr,
        arg: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("StoreRemoteField", (classPtr, fieldPtr, arg))?;
        Ok(__cordl_ret)
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
    pub fn get_IsContextBoundObject(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_IsContextBoundObject", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_TargetContext(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Runtime::Remoting::Contexts::Context,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Runtime::Remoting::Contexts::Context = __cordl_object
            .invoke("get_TargetContext", ())?;
        Ok(__cordl_ret)
    }
    pub fn LoadRemoteFieldNew(
        &mut self,
        classPtr: crate::System::IntPtr,
        fieldPtr: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Object> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Object = __cordl_object
            .invoke("LoadRemoteFieldNew", (classPtr, fieldPtr))?;
        Ok(__cordl_ret)
    }
    pub fn InCurrentContext(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("InCurrentContext", ())?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "System+Runtime+Remoting+Proxies+TransparentProxy")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Runtime::Remoting::Proxies::TransparentProxy {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
