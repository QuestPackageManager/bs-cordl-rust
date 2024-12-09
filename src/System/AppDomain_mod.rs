#[cfg(feature = "System+AppDomain")]
#[repr(C)]
#[derive(Debug)]
pub struct AppDomain {
    __cordl_parent: crate::System::MarshalByRefObject,
    pub _mono_app_domain: crate::System::IntPtr,
    pub _evidence: *mut quest_hook::libil2cpp::Il2CppObject,
    pub _granted: *mut quest_hook::libil2cpp::Il2CppObject,
    pub _principalPolicy: i32,
    pub AssemblyLoad: *mut crate::System::AssemblyLoadEventHandler,
    pub AssemblyResolve: *mut crate::System::ResolveEventHandler,
    pub DomainUnload: *mut crate::System::EventHandler,
    pub ProcessExit: *mut crate::System::EventHandler,
    pub ResourceResolve: *mut crate::System::ResolveEventHandler,
    pub TypeResolve: *mut crate::System::ResolveEventHandler,
    pub UnhandledException: *mut crate::System::UnhandledExceptionEventHandler,
    pub FirstChanceException: *mut crate::System::EventHandler_1<
        *mut crate::System::Runtime::ExceptionServices::FirstChanceExceptionEventArgs,
    >,
    pub _domain_manager: *mut quest_hook::libil2cpp::Il2CppObject,
    pub ReflectionOnlyAssemblyResolve: *mut crate::System::ResolveEventHandler,
    pub _activation: *mut quest_hook::libil2cpp::Il2CppObject,
    pub _applicationIdentity: *mut quest_hook::libil2cpp::Il2CppObject,
    pub compatibility_switch: *mut crate::System::Collections::Generic::List_1<
        *mut quest_hook::libil2cpp::Il2CppString,
    >,
}
#[cfg(feature = "System+AppDomain")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::AppDomain => "System"."AppDomain"
);
#[cfg(feature = "System+AppDomain")]
impl std::ops::Deref for crate::System::AppDomain {
    type Target = crate::System::MarshalByRefObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+AppDomain")]
impl std::ops::DerefMut for crate::System::AppDomain {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+AppDomain")]
impl crate::System::AppDomain {
    pub fn DoAssemblyLoad(
        &mut self,
        assembly: *mut crate::System::Reflection::Assembly,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("DoAssemblyLoad", (assembly))?;
        Ok(__cordl_ret)
    }
    pub fn DoAssemblyResolve(
        &mut self,
        name: *mut quest_hook::libil2cpp::Il2CppString,
        requestingAssembly: *mut crate::System::Reflection::Assembly,
        refonly: bool,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Reflection::Assembly> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Reflection::Assembly = __cordl_object
            .invoke("DoAssemblyResolve", (name, requestingAssembly, refonly))?;
        Ok(__cordl_ret)
    }
    pub fn DoDomainUnload(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("DoDomainUnload", ())?;
        Ok(__cordl_ret)
    }
    pub fn DoTypeResolve(
        &mut self,
        name: *mut quest_hook::libil2cpp::Il2CppString,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Reflection::Assembly> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Reflection::Assembly = __cordl_object
            .invoke("DoTypeResolve", (name))?;
        Ok(__cordl_ret)
    }
    pub fn GetAssemblies_1(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut quest_hook::libil2cpp::Il2CppArray<*mut crate::System::Reflection::Assembly>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::System::Reflection::Assembly,
        > = __cordl_object.invoke("GetAssemblies", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetAssemblies__cordl_bool0(
        &mut self,
        refOnly: bool,
    ) -> quest_hook::libil2cpp::Result<
        *mut quest_hook::libil2cpp::Il2CppArray<*mut crate::System::Reflection::Assembly>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::System::Reflection::Assembly,
        > = __cordl_object.invoke("GetAssemblies", (refOnly))?;
        Ok(__cordl_ret)
    }
    pub fn GetData(
        &mut self,
        name: *mut quest_hook::libil2cpp::Il2CppString,
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppObject> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppObject = __cordl_object
            .invoke("GetData", (name))?;
        Ok(__cordl_ret)
    }
    pub fn GetMarshalledDomainObjRef(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppArray<u8>> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<u8> = __cordl_object
            .invoke("GetMarshalledDomainObjRef", ())?;
        Ok(__cordl_ret)
    }
    pub fn InitializeLifetimeService(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppObject> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppObject = __cordl_object
            .invoke("InitializeLifetimeService", ())?;
        Ok(__cordl_ret)
    }
    pub fn IsFinalizingForUnload(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("IsFinalizingForUnload", ())?;
        Ok(__cordl_ret)
    }
    pub fn LoadAssembly(
        &mut self,
        assemblyRef: *mut quest_hook::libil2cpp::Il2CppString,
        securityEvidence: *mut crate::System::Security::Policy::Evidence,
        refOnly: bool,
        stackMark: quest_hook::libil2cpp::ByRefMut<
            crate::System::Threading::StackCrawlMark,
        >,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Reflection::Assembly> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Reflection::Assembly = __cordl_object
            .invoke(
                "LoadAssembly",
                (assemblyRef, securityEvidence, refOnly, stackMark),
            )?;
        Ok(__cordl_ret)
    }
    pub fn Load_Evidence__cordl_bool_ByRefMut1(
        &mut self,
        assemblyString: *mut quest_hook::libil2cpp::Il2CppString,
        assemblySecurity: *mut crate::System::Security::Policy::Evidence,
        refonly: bool,
        stackMark: quest_hook::libil2cpp::ByRefMut<
            crate::System::Threading::StackCrawlMark,
        >,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Reflection::Assembly> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Reflection::Assembly = __cordl_object
            .invoke("Load", (assemblyString, assemblySecurity, refonly, stackMark))?;
        Ok(__cordl_ret)
    }
    pub fn Load_Il2CppString0(
        &mut self,
        assemblyString: *mut quest_hook::libil2cpp::Il2CppString,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Reflection::Assembly> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Reflection::Assembly = __cordl_object
            .invoke("Load", (assemblyString))?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn ProcessMessageInDomain(
        &mut self,
        arrRequest: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        cadMsg: *mut crate::System::Runtime::Remoting::Messaging::CADMethodCallMessage,
        arrResponse: quest_hook::libil2cpp::ByRefMut<
            *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        >,
        cadMrm: quest_hook::libil2cpp::ByRefMut<
            *mut crate::System::Runtime::Remoting::Messaging::CADMethodReturnMessage,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "ProcessMessageInDomain",
                (arrRequest, cadMsg, arrResponse, cadMrm),
            )?;
        Ok(__cordl_ret)
    }
    pub fn ToString(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppString> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppString = __cordl_object
            .invoke("ToString", ())?;
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
    pub fn add_AssemblyResolve(
        &mut self,
        value: *mut crate::System::ResolveEventHandler,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_AssemblyResolve", (value))?;
        Ok(__cordl_ret)
    }
    pub fn add_DomainUnload(
        &mut self,
        value: *mut crate::System::EventHandler,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_DomainUnload", (value))?;
        Ok(__cordl_ret)
    }
    pub fn add_UnhandledException(
        &mut self,
        value: *mut crate::System::UnhandledExceptionEventHandler,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_UnhandledException", (value))?;
        Ok(__cordl_ret)
    }
    pub fn getDomainID(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("getDomainID", ())?;
        Ok(__cordl_ret)
    }
    pub fn getFriendlyName(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppString> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppString = __cordl_object
            .invoke("getFriendlyName", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_IsFullyTrusted(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_IsFullyTrusted", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_IsHomogenous(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_IsHomogenous", ())?;
        Ok(__cordl_ret)
    }
    pub fn remove_AssemblyResolve(
        &mut self,
        value: *mut crate::System::ResolveEventHandler,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_AssemblyResolve", (value))?;
        Ok(__cordl_ret)
    }
    pub fn remove_DomainUnload(
        &mut self,
        value: *mut crate::System::EventHandler,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_DomainUnload", (value))?;
        Ok(__cordl_ret)
    }
    pub fn remove_UnhandledException(
        &mut self,
        value: *mut crate::System::UnhandledExceptionEventHandler,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_UnhandledException", (value))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "System+AppDomain")]
impl quest_hook::libil2cpp::ObjectType for crate::System::AppDomain {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
