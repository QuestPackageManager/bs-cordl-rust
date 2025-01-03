#[cfg(feature = "Unity+Burst+BurstCompiler")]
#[repr(C)]
#[derive(Debug)]
pub struct BurstCompiler {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "Unity+Burst+BurstCompiler")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Unity::Burst::BurstCompiler => "Unity.Burst"
    ."BurstCompiler"
);
#[cfg(feature = "Unity+Burst+BurstCompiler")]
impl std::ops::Deref for crate::Unity::Burst::BurstCompiler {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+Burst+BurstCompiler")]
impl std::ops::DerefMut for crate::Unity::Burst::BurstCompiler {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+Burst+BurstCompiler")]
impl crate::Unity::Burst::BurstCompiler {
    #[cfg(feature = "Unity+Burst+BurstCompiler+BurstCompilerHelper")]
    pub type BurstCompilerHelper = crate::Unity::Burst::BurstCompiler_BurstCompilerHelper;
    #[cfg(feature = "Unity+Burst+BurstCompiler+CommandBuilder")]
    pub type CommandBuilder = crate::Unity::Burst::BurstCompiler_CommandBuilder;
    #[cfg(feature = "Unity+Burst+BurstCompiler+FakeDelegate")]
    pub type FakeDelegate = crate::Unity::Burst::BurstCompiler_FakeDelegate;
    #[cfg(feature = "Unity+Burst+BurstCompiler+StaticTypeReinitAttribute")]
    pub type StaticTypeReinitAttribute = crate::Unity::Burst::BurstCompiler_StaticTypeReinitAttribute;
    #[cfg(feature = "Unity+Burst+BurstCompiler+__c")]
    pub type __c = crate::Unity::Burst::BurstCompiler___c;
    pub fn AotCompilation(
        assemblyFolders: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<*mut quest_hook::libil2cpp::Il2CppString>,
        >,
        assemblyRoots: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<*mut quest_hook::libil2cpp::Il2CppString>,
        >,
        options: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("AotCompilation", (assemblyFolders, assemblyRoots, options))?;
        Ok(__cordl_ret.into())
    }
    pub fn BeginCompilerCommand(
        cmd: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Unity::Burst::BurstCompiler_CommandBuilder>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Unity::Burst::BurstCompiler_CommandBuilder,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("BeginCompilerCommand", (cmd))?;
        Ok(__cordl_ret.into())
    }
    pub fn Cancel() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Cancel", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn CompileDelegate<T>(delegateMethod: T) -> quest_hook::libil2cpp::Result<T>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: T = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CompileDelegate", (delegateMethod))?;
        Ok(__cordl_ret.into())
    }
    pub fn CompileFunctionPointer<T>(
        delegateMethod: T,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::FunctionPointer_1<T>>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: crate::Unity::Burst::FunctionPointer_1<T> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CompileFunctionPointer", (delegateMethod))?;
        Ok(__cordl_ret.into())
    }
    pub fn CompileILPPMethod(
        burstMethodHandle: crate::System::RuntimeMethodHandle,
        managedMethodHandle: crate::System::RuntimeMethodHandle,
        delegateTypeHandle: crate::System::RuntimeTypeHandle,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        let __cordl_ret: crate::System::IntPtr = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "CompileILPPMethod",
                (burstMethodHandle, managedMethodHandle, delegateTypeHandle),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn CompileILPPMethod2(
        burstMethodHandle: crate::System::RuntimeMethodHandle,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        let __cordl_ret: crate::System::IntPtr = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CompileILPPMethod2", (burstMethodHandle))?;
        Ok(__cordl_ret.into())
    }
    pub fn CompileUnsafeStaticMethod(
        handle: crate::System::RuntimeMethodHandle,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CompileUnsafeStaticMethod", (handle))?;
        Ok(__cordl_ret.into())
    }
    pub fn Compile_MethodInfo__cordl_bool__cordl_bool1(
        delegateObj: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        methodInfo: quest_hook::libil2cpp::Gc<crate::System::Reflection::MethodInfo>,
        isFunctionPointer: bool,
        isILPostProcessing: bool,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "Compile",
                (delegateObj, methodInfo, isFunctionPointer, isILPostProcessing),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn Compile__cordl_bool0(
        delegateObj: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        isFunctionPointer: bool,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Compile", (delegateObj, isFunctionPointer))?;
        Ok(__cordl_ret.into())
    }
    pub fn Disable() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Disable", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn DummyMethod() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("DummyMethod", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn Enable() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Enable", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetExecutionMode() -> quest_hook::libil2cpp::Result<
        crate::Unity::Burst::BurstExecutionEnvironment,
    > {
        let __cordl_ret: crate::Unity::Burst::BurstExecutionEnvironment = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetExecutionMode", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetILPPMethodFunctionPointer(
        ilppMethod: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetILPPMethodFunctionPointer", (ilppMethod))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetILPPMethodFunctionPointer2(
        ilppMethod: crate::System::IntPtr,
        managedMethodHandle: crate::System::RuntimeMethodHandle,
        delegateTypeHandle: crate::System::RuntimeTypeHandle,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "GetILPPMethodFunctionPointer2",
                (ilppMethod, managedMethodHandle, delegateTypeHandle),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn InitialiseDebuggerHooks() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Void,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("InitialiseDebuggerHooks", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn Initialize(
        assemblyFolders: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<*mut quest_hook::libil2cpp::Il2CppString>,
        >,
        ignoreAssemblies: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<*mut quest_hook::libil2cpp::Il2CppString>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Initialize", (assemblyFolders, ignoreAssemblies))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsApiAvailable(
        apiName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IsApiAvailable", (apiName))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsCurrentCompilationDone() -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IsCurrentCompilationDone", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn IsHostEditorArm() -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IsHostEditorArm", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn IsLoadAdditionalLibrarySupported() -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IsLoadAdditionalLibrarySupported", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn NotifyAssemblyCompilationFinished(
        assemblyName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        defines: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<*mut quest_hook::libil2cpp::Il2CppString>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("NotifyAssemblyCompilationFinished", (assemblyName, defines))?;
        Ok(__cordl_ret.into())
    }
    pub fn NotifyAssemblyCompilationNotRequired(
        assemblyName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("NotifyAssemblyCompilationNotRequired", (assemblyName))?;
        Ok(__cordl_ret.into())
    }
    pub fn NotifyCompilationFinished() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Void,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("NotifyCompilationFinished", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn NotifyCompilationStarted(
        assemblyFolders: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<*mut quest_hook::libil2cpp::Il2CppString>,
        >,
        ignoreAssemblies: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<*mut quest_hook::libil2cpp::Il2CppString>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("NotifyCompilationStarted", (assemblyFolders, ignoreAssemblies))?;
        Ok(__cordl_ret.into())
    }
    pub fn RequestSetProtocolVersion(
        version: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("RequestSetProtocolVersion", (version))?;
        Ok(__cordl_ret.into())
    }
    pub fn SendCommandToCompiler(
        commandName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        commandArgs: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SendCommandToCompiler", (commandName, commandArgs))?;
        Ok(__cordl_ret.into())
    }
    pub fn SendRawCommandToCompiler(
        command: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SendRawCommandToCompiler", (command))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetExecutionMode(
        mode: crate::Unity::Burst::BurstExecutionEnvironment,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SetExecutionMode", (mode))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetProfilerCallbacks() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Void,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SetProfilerCallbacks", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn Shutdown() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Shutdown", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn TriggerRecompilation() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Void,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("TriggerRecompilation", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn TriggerUnsafeStaticMethodRecompilation() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Void,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("TriggerUnsafeStaticMethodRecompilation", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn UnloadAdditionalLibraries() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Void,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("UnloadAdditionalLibraries", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn VerifyDelegateHasCorrectUnmanagedFunctionPointerAttribute<T>(
        delegateMethod: T,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "VerifyDelegateHasCorrectUnmanagedFunctionPointerAttribute",
                (delegateMethod),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn VerifyDelegateIsNotMulticast<T>(
        delegateMethod: T,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("VerifyDelegateIsNotMulticast", (delegateMethod))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_IsEnabled() -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_IsEnabled", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Unity+Burst+BurstCompiler")]
impl quest_hook::libil2cpp::ObjectType for crate::Unity::Burst::BurstCompiler {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Unity+Burst+BurstCompiler+BurstCompilerHelper+IsBurstEnabledDelegate")]
#[repr(C)]
#[derive(Debug)]
pub struct BurstCompilerHelper_BurstCompiler_IsBurstEnabledDelegate {
    __cordl_parent: crate::System::MulticastDelegate,
}
#[cfg(feature = "Unity+Burst+BurstCompiler+BurstCompilerHelper+IsBurstEnabledDelegate")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Unity::Burst::BurstCompilerHelper_BurstCompiler_IsBurstEnabledDelegate =>
    "Unity.Burst"."BurstCompiler/BurstCompilerHelper/IsBurstEnabledDelegate"
);
#[cfg(feature = "Unity+Burst+BurstCompiler+BurstCompilerHelper+IsBurstEnabledDelegate")]
impl std::ops::Deref
for crate::Unity::Burst::BurstCompilerHelper_BurstCompiler_IsBurstEnabledDelegate {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+Burst+BurstCompiler+BurstCompilerHelper+IsBurstEnabledDelegate")]
impl std::ops::DerefMut
for crate::Unity::Burst::BurstCompilerHelper_BurstCompiler_IsBurstEnabledDelegate {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+Burst+BurstCompiler+BurstCompilerHelper+IsBurstEnabledDelegate")]
impl crate::Unity::Burst::BurstCompilerHelper_BurstCompiler_IsBurstEnabledDelegate {
    pub fn BeginInvoke(
        &mut self,
        callback: quest_hook::libil2cpp::Gc<crate::System::AsyncCallback>,
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult> = __cordl_object
            .invoke("BeginInvoke", (callback, object))?;
        Ok(__cordl_ret.into())
    }
    pub fn EndInvoke(
        &mut self,
        result: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("EndInvoke", (result))?;
        Ok(__cordl_ret.into())
    }
    pub fn Invoke(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("Invoke", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (object, method))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (object, method))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Unity+Burst+BurstCompiler+BurstCompilerHelper+IsBurstEnabledDelegate")]
impl quest_hook::libil2cpp::ObjectType
for crate::Unity::Burst::BurstCompilerHelper_BurstCompiler_IsBurstEnabledDelegate {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Unity+Burst+BurstCompiler+BurstCompilerHelper")]
#[repr(C)]
#[derive(Debug)]
pub struct BurstCompiler_BurstCompilerHelper {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "Unity+Burst+BurstCompiler+BurstCompilerHelper")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Unity::Burst::BurstCompiler_BurstCompilerHelper
    => "Unity.Burst"."BurstCompiler/BurstCompilerHelper"
);
#[cfg(feature = "Unity+Burst+BurstCompiler+BurstCompilerHelper")]
impl std::ops::Deref for crate::Unity::Burst::BurstCompiler_BurstCompilerHelper {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+Burst+BurstCompiler+BurstCompilerHelper")]
impl std::ops::DerefMut for crate::Unity::Burst::BurstCompiler_BurstCompilerHelper {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+Burst+BurstCompiler+BurstCompilerHelper")]
impl crate::Unity::Burst::BurstCompiler_BurstCompilerHelper {
    #[cfg(
        feature = "Unity+Burst+BurstCompiler+BurstCompilerHelper+IsBurstEnabledDelegate"
    )]
    pub type IsBurstEnabledDelegate = crate::Unity::Burst::BurstCompilerHelper_BurstCompiler_IsBurstEnabledDelegate;
    pub fn DiscardedMethod(
        value: quest_hook::libil2cpp::ByRefMut<bool>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("DiscardedMethod", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsBurstEnabled() -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IsBurstEnabled", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn IsCompiledByBurst(
        del: quest_hook::libil2cpp::Gc<crate::System::Delegate>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IsCompiledByBurst", (del))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Unity+Burst+BurstCompiler+BurstCompilerHelper")]
impl quest_hook::libil2cpp::ObjectType
for crate::Unity::Burst::BurstCompiler_BurstCompilerHelper {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Unity+Burst+BurstCompiler+CommandBuilder")]
#[repr(C)]
#[derive(Debug)]
pub struct BurstCompiler_CommandBuilder {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _builder: *mut crate::System::Text::StringBuilder,
    pub _hasArgs: bool,
}
#[cfg(feature = "Unity+Burst+BurstCompiler+CommandBuilder")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Unity::Burst::BurstCompiler_CommandBuilder =>
    "Unity.Burst"."BurstCompiler/CommandBuilder"
);
#[cfg(feature = "Unity+Burst+BurstCompiler+CommandBuilder")]
impl std::ops::Deref for crate::Unity::Burst::BurstCompiler_CommandBuilder {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+Burst+BurstCompiler+CommandBuilder")]
impl std::ops::DerefMut for crate::Unity::Burst::BurstCompiler_CommandBuilder {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+Burst+BurstCompiler+CommandBuilder")]
impl crate::Unity::Burst::BurstCompiler_CommandBuilder {
    pub fn And(
        &mut self,
        sep: char,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Unity::Burst::BurstCompiler_CommandBuilder>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Unity::Burst::BurstCompiler_CommandBuilder,
        > = __cordl_object.invoke("And", (sep))?;
        Ok(__cordl_ret.into())
    }
    pub fn Begin(
        &mut self,
        cmd: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Unity::Burst::BurstCompiler_CommandBuilder>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Unity::Burst::BurstCompiler_CommandBuilder,
        > = __cordl_object.invoke("Begin", (cmd))?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn SendToCompiler(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("SendToCompiler", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn With_Il2CppString0(
        &mut self,
        arg: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Unity::Burst::BurstCompiler_CommandBuilder>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Unity::Burst::BurstCompiler_CommandBuilder,
        > = __cordl_object.invoke("With", (arg))?;
        Ok(__cordl_ret.into())
    }
    pub fn With_IntPtr1(
        &mut self,
        arg: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Unity::Burst::BurstCompiler_CommandBuilder>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Unity::Burst::BurstCompiler_CommandBuilder,
        > = __cordl_object.invoke("With", (arg))?;
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
}
#[cfg(feature = "Unity+Burst+BurstCompiler+CommandBuilder")]
impl quest_hook::libil2cpp::ObjectType
for crate::Unity::Burst::BurstCompiler_CommandBuilder {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Unity+Burst+BurstCompiler+FakeDelegate")]
#[repr(C)]
#[derive(Debug)]
pub struct BurstCompiler_FakeDelegate {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _Method_k__BackingField: *mut crate::System::Reflection::MethodInfo,
}
#[cfg(feature = "Unity+Burst+BurstCompiler+FakeDelegate")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Unity::Burst::BurstCompiler_FakeDelegate =>
    "Unity.Burst"."BurstCompiler/FakeDelegate"
);
#[cfg(feature = "Unity+Burst+BurstCompiler+FakeDelegate")]
impl std::ops::Deref for crate::Unity::Burst::BurstCompiler_FakeDelegate {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+Burst+BurstCompiler+FakeDelegate")]
impl std::ops::DerefMut for crate::Unity::Burst::BurstCompiler_FakeDelegate {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+Burst+BurstCompiler+FakeDelegate")]
impl crate::Unity::Burst::BurstCompiler_FakeDelegate {
    pub fn New(
        method: quest_hook::libil2cpp::Gc<crate::System::Reflection::MethodInfo>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (method))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        method: quest_hook::libil2cpp::Gc<crate::System::Reflection::MethodInfo>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (method))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Method(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Reflection::MethodInfo>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Reflection::MethodInfo,
        > = __cordl_object.invoke("get_Method", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Unity+Burst+BurstCompiler+FakeDelegate")]
impl quest_hook::libil2cpp::ObjectType
for crate::Unity::Burst::BurstCompiler_FakeDelegate {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Unity+Burst+BurstCompiler+StaticTypeReinitAttribute")]
#[repr(C)]
#[derive(Debug)]
pub struct BurstCompiler_StaticTypeReinitAttribute {
    __cordl_parent: crate::System::Attribute,
    pub reinitType: *mut crate::System::Type,
}
#[cfg(feature = "Unity+Burst+BurstCompiler+StaticTypeReinitAttribute")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Unity::Burst::BurstCompiler_StaticTypeReinitAttribute => "Unity.Burst"
    ."BurstCompiler/StaticTypeReinitAttribute"
);
#[cfg(feature = "Unity+Burst+BurstCompiler+StaticTypeReinitAttribute")]
impl std::ops::Deref for crate::Unity::Burst::BurstCompiler_StaticTypeReinitAttribute {
    type Target = crate::System::Attribute;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+Burst+BurstCompiler+StaticTypeReinitAttribute")]
impl std::ops::DerefMut
for crate::Unity::Burst::BurstCompiler_StaticTypeReinitAttribute {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+Burst+BurstCompiler+StaticTypeReinitAttribute")]
impl crate::Unity::Burst::BurstCompiler_StaticTypeReinitAttribute {
    pub fn New(
        toReinit: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (toReinit))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        toReinit: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (toReinit))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Unity+Burst+BurstCompiler+StaticTypeReinitAttribute")]
impl quest_hook::libil2cpp::ObjectType
for crate::Unity::Burst::BurstCompiler_StaticTypeReinitAttribute {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
