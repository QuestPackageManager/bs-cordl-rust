#[cfg(feature = "System+AppDomain")]
#[repr(C)]
#[derive(Debug)]
pub struct AppDomain {
    __cordl_parent: crate::System::MarshalByRefObject,
    pub _mono_app_domain: crate::System::IntPtr,
    pub _evidence: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    pub _granted: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    pub _principalPolicy: i32,
    pub AssemblyLoad: quest_hook::libil2cpp::Gc<crate::System::AssemblyLoadEventHandler>,
    pub AssemblyResolve: quest_hook::libil2cpp::Gc<crate::System::ResolveEventHandler>,
    pub DomainUnload: quest_hook::libil2cpp::Gc<crate::System::EventHandler>,
    pub ProcessExit: quest_hook::libil2cpp::Gc<crate::System::EventHandler>,
    pub ResourceResolve: quest_hook::libil2cpp::Gc<crate::System::ResolveEventHandler>,
    pub TypeResolve: quest_hook::libil2cpp::Gc<crate::System::ResolveEventHandler>,
    pub UnhandledException: quest_hook::libil2cpp::Gc<
        crate::System::UnhandledExceptionEventHandler,
    >,
    pub FirstChanceException: quest_hook::libil2cpp::Gc<
        crate::System::EventHandler_1<
            quest_hook::libil2cpp::Gc<
                crate::System::Runtime::ExceptionServices::FirstChanceExceptionEventArgs,
            >,
        >,
    >,
    pub _domain_manager: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    pub ReflectionOnlyAssemblyResolve: quest_hook::libil2cpp::Gc<
        crate::System::ResolveEventHandler,
    >,
    pub _activation: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    pub _applicationIdentity: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppObject,
    >,
    pub compatibility_switch: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::List_1<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        >,
    >,
}
#[cfg(feature = "System+AppDomain")]
unsafe impl quest_hook::libil2cpp::Type for crate::System::AppDomain {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "System";
    const CLASS_NAME: &'static str = "AppDomain";
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.class().is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_argument(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
    fn matches_value_parameter(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
}
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
        assembly: quest_hook::libil2cpp::Gc<crate::System::Reflection::Assembly>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("DoAssemblyLoad", (assembly))?;
        Ok(__cordl_ret.into())
    }
    pub fn DoAssemblyResolve(
        &mut self,
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        requestingAssembly: quest_hook::libil2cpp::Gc<
            crate::System::Reflection::Assembly,
        >,
        refonly: bool,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Reflection::Assembly>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Reflection::Assembly,
        > = __cordl_object
            .invoke("DoAssemblyResolve", (name, requestingAssembly, refonly))?;
        Ok(__cordl_ret.into())
    }
    pub fn DoDomainUnload(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("DoDomainUnload", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn DoTypeResolve(
        &mut self,
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Reflection::Assembly>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Reflection::Assembly,
        > = __cordl_object.invoke("DoTypeResolve", (name))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetAssemblies_1(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<crate::System::Reflection::Assembly>,
            >,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<crate::System::Reflection::Assembly>,
            >,
        > = __cordl_object.invoke("GetAssemblies", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetAssemblies__cordl_bool0(
        &mut self,
        refOnly: bool,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<crate::System::Reflection::Assembly>,
            >,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<crate::System::Reflection::Assembly>,
            >,
        > = __cordl_object.invoke("GetAssemblies", (refOnly))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetData(
        &mut self,
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = __cordl_object.invoke("GetData", (name))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetMarshalledDomainObjRef(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<u8>,
        > = __cordl_object.invoke("GetMarshalledDomainObjRef", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetProcessGuid() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class().invoke("GetProcessGuid", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn InitializeLifetimeService(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = __cordl_object.invoke("InitializeLifetimeService", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn InternalGetContext() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Runtime::Remoting::Contexts::Context>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Remoting::Contexts::Context,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("InternalGetContext", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn InternalGetDefaultContext() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Runtime::Remoting::Contexts::Context>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Remoting::Contexts::Context,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("InternalGetDefaultContext", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn InternalGetProcessGuid(
        newguid: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("InternalGetProcessGuid", (newguid))?;
        Ok(__cordl_ret.into())
    }
    pub fn InternalIsFinalizingForUnload(
        domain_id: i32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("InternalIsFinalizingForUnload", (domain_id))?;
        Ok(__cordl_ret.into())
    }
    pub fn InternalPopDomainRef() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Void,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("InternalPopDomainRef", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn InternalPushDomainRefByID(
        domain_id: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("InternalPushDomainRefByID", (domain_id))?;
        Ok(__cordl_ret.into())
    }
    pub fn InternalSetContext(
        context: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Remoting::Contexts::Context,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Runtime::Remoting::Contexts::Context>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Remoting::Contexts::Context,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("InternalSetContext", (context))?;
        Ok(__cordl_ret.into())
    }
    pub fn InternalSetDomain(
        context: quest_hook::libil2cpp::Gc<crate::System::AppDomain>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::AppDomain>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::AppDomain> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("InternalSetDomain", (context))?;
        Ok(__cordl_ret.into())
    }
    pub fn InternalSetDomainByID(
        domain_id: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::AppDomain>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::AppDomain> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("InternalSetDomainByID", (domain_id))?;
        Ok(__cordl_ret.into())
    }
    pub fn InvokeInDomainByID(
        domain_id: i32,
        method: quest_hook::libil2cpp::Gc<crate::System::Reflection::MethodInfo>,
        obj: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        args: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("InvokeInDomainByID", (domain_id, method, obj, args))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsAppXModel() -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IsAppXModel", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn IsFinalizingForUnload(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("IsFinalizingForUnload", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn LoadAssembly(
        &mut self,
        assemblyRef: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        securityEvidence: quest_hook::libil2cpp::Gc<
            crate::System::Security::Policy::Evidence,
        >,
        refOnly: bool,
        stackMark: quest_hook::libil2cpp::ByRefMut<
            crate::System::Threading::StackCrawlMark,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Reflection::Assembly>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Reflection::Assembly,
        > = __cordl_object
            .invoke(
                "LoadAssembly",
                (assemblyRef, securityEvidence, refOnly, stackMark),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn Load_Evidence__cordl_bool_ByRefMut1(
        &mut self,
        assemblyString: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        assemblySecurity: quest_hook::libil2cpp::Gc<
            crate::System::Security::Policy::Evidence,
        >,
        refonly: bool,
        stackMark: quest_hook::libil2cpp::ByRefMut<
            crate::System::Threading::StackCrawlMark,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Reflection::Assembly>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Reflection::Assembly,
        > = __cordl_object
            .invoke("Load", (assemblyString, assemblySecurity, refonly, stackMark))?;
        Ok(__cordl_ret.into())
    }
    pub fn Load_Il2CppString0(
        &mut self,
        assemblyString: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Reflection::Assembly>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Reflection::Assembly,
        > = __cordl_object.invoke("Load", (assemblyString))?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn ProcessMessageInDomain(
        &mut self,
        arrRequest: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        cadMsg: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Remoting::Messaging::CADMethodCallMessage,
        >,
        arrResponse: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        >,
        cadMrm: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<
                crate::System::Runtime::Remoting::Messaging::CADMethodReturnMessage,
            >,
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
        Ok(__cordl_ret.into())
    }
    pub fn ToString(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("ToString", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn add_AssemblyResolve(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::System::ResolveEventHandler>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_AssemblyResolve", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn add_DomainUnload(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::System::EventHandler>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_DomainUnload", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn add_UnhandledException(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::System::UnhandledExceptionEventHandler>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_UnhandledException", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn getCurDomain() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::AppDomain>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::AppDomain> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("getCurDomain", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn getDomainID(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("getDomainID", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn getFriendlyName(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("getFriendlyName", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_CurrentDomain() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::AppDomain>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::AppDomain> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_CurrentDomain", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_IsFullyTrusted(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_IsFullyTrusted", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_IsHomogenous(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_IsHomogenous", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn remove_AssemblyResolve(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::System::ResolveEventHandler>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_AssemblyResolve", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn remove_DomainUnload(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::System::EventHandler>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_DomainUnload", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn remove_UnhandledException(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::System::UnhandledExceptionEventHandler>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_UnhandledException", (value))?;
        Ok(__cordl_ret.into())
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
