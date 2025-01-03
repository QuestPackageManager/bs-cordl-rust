#[cfg(feature = "Unity+Burst+BurstCompilerOptions")]
#[repr(C)]
#[derive(Debug)]
pub struct BurstCompilerOptions {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _enableBurstCompilation: bool,
    pub _enableBurstCompileSynchronously: bool,
    pub _enableBurstSafetyChecks: bool,
    pub _enableBurstTimings: bool,
    pub _enableBurstDebug: bool,
    pub _forceEnableBurstSafetyChecks: bool,
    pub _IsGlobal_k__BackingField: bool,
    pub _OptionsChanged_k__BackingField: quest_hook::libil2cpp::Gc<
        crate::System::Action,
    >,
}
#[cfg(feature = "Unity+Burst+BurstCompilerOptions")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Unity::Burst::BurstCompilerOptions =>
    "Unity.Burst"."BurstCompilerOptions"
);
#[cfg(feature = "Unity+Burst+BurstCompilerOptions")]
impl std::ops::Deref for crate::Unity::Burst::BurstCompilerOptions {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+Burst+BurstCompilerOptions")]
impl std::ops::DerefMut for crate::Unity::Burst::BurstCompilerOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+Burst+BurstCompilerOptions")]
impl crate::Unity::Burst::BurstCompilerOptions {
    pub const BurstInitializeExternalsName: &'static str = "burst.initialize.externals";
    pub const BurstInitializeName: &'static str = "burst.initialize";
    pub const BurstInitializeStaticsName: &'static str = "burst.initialize.statics";
    pub const CompilerCommandAotCompilation: &'static str = "$aot_compilation";
    pub const CompilerCommandCancel: &'static str = "$cancel";
    pub const CompilerCommandDisableCompiler: &'static str = "$disable_compiler";
    pub const CompilerCommandDomainReload: &'static str = "$domain_reload";
    pub const CompilerCommandEnableCompiler: &'static str = "$enable_compiler";
    pub const CompilerCommandGetTargetCpuFromHost: &'static str = "$get_target_cpu_from_host";
    pub const CompilerCommandILPPCompilation: &'static str = "$ilpp_compilation";
    pub const CompilerCommandInitialiseDebuggerCommmand: &'static str = "$load_debugger_interface";
    pub const CompilerCommandInitialize: &'static str = "$initialize";
    pub const CompilerCommandIsArmTestEnv: &'static str = "$is_arm_test_env";
    pub const CompilerCommandIsCurrentCompilationDone: &'static str = "$is_current_compilation_done";
    pub const CompilerCommandIsNativeApiAvailable: &'static str = "$is_native_api_available";
    pub const CompilerCommandNotifyAssemblyCompilationFinished: &'static str = "$notify_assembly_compilation_finished";
    pub const CompilerCommandNotifyAssemblyCompilationNotRequired: &'static str = "$notify_assembly_compilation_not_required";
    pub const CompilerCommandNotifyCompilationFinished: &'static str = "$notify_compilation_finished";
    pub const CompilerCommandNotifyCompilationStarted: &'static str = "$notify_compilation_started";
    pub const CompilerCommandRequestInitialiseDebuggerCommmand: &'static str = "$request_debug_command";
    pub const CompilerCommandRequestSetProtocolVersionEditor: &'static str = "$request_set_protocol_version_editor";
    pub const CompilerCommandSetDefaultOptions: &'static str = "$set_default_options";
    pub const CompilerCommandSetProfileCallbacks: &'static str = "$set_profile_callbacks";
    pub const CompilerCommandSetProtocolVersionBurst: &'static str = "$set_protocol_version_burst";
    pub const CompilerCommandShutdown: &'static str = "$shutdown";
    pub const CompilerCommandTriggerRecompilation: &'static str = "$trigger_recompilation";
    pub const CompilerCommandTriggerSetupRecompilation: &'static str = "$trigger_setup_recompilation";
    pub const CompilerCommandUnloadBurstNatives: &'static str = "$unload_burst_natives";
    pub const CompilerCommandVersionNotification: &'static str = "$version";
    pub const DefaultLibraryName: &'static str = "lib_burst_generated";
    pub const DisableCompilationArg: &'static str = "--burst-disable-compilation";
    pub const ForceSynchronousCompilationArg: &'static str = "--burst-force-sync-compilation";
    pub const OptionAlwaysCreateOutput: &'static str = "always-create-output=";
    pub const OptionAotAssembly: &'static str = "assembly=";
    pub const OptionAotAssemblyFolder: &'static str = "assembly-folder=";
    pub const OptionAotDecodeFolder: &'static str = "decode-folder=";
    pub const OptionAotEmitLlvmObjects: &'static str = "emit-llvm-objects";
    pub const OptionAotKeepIntermediateFiles: &'static str = "keep-intermediate-files";
    pub const OptionAotKeyFolder: &'static str = "key-folder=";
    pub const OptionAotMethod: &'static str = "method=";
    pub const OptionAotNoLink: &'static str = "nolink";
    pub const OptionAotNoNativeToolchain: &'static str = "no-native-toolchain";
    pub const OptionAotOnlyStaticMethods: &'static str = "only-static-methods";
    pub const OptionAotOutputPath: &'static str = "output=";
    pub const OptionAotPdbSearchPaths: &'static str = "pdb-search-paths=";
    pub const OptionAotType: &'static str = "type=";
    pub const OptionAssemblyDefines: &'static str = "assembly-defines=";
    pub const OptionBackend: &'static str = "backend=";
    pub const OptionBranchProtection: &'static str = "branch-protection=";
    pub const OptionBurstcSwitch: &'static str = "+burstc";
    pub const OptionCacheDirectory: &'static str = "cache-directory=";
    pub const OptionChunkSize: &'static str = "chunk-size=";
    pub const OptionCompilationId: &'static str = "compilation-id=";
    pub const OptionCompilerThreads: &'static str = "threads=";
    pub const OptionDebug: &'static str = "debug=";
    pub const OptionDebugMode: &'static str = "debugMode";
    pub const OptionDebugTrap: &'static str = "debugtrap";
    pub const OptionDisableOpt: &'static str = "disable-opt";
    pub const OptionDisableSafetyChecks: &'static str = "disable-safety-checks";
    pub const OptionDisableStringInterpolationInExceptionMessages: &'static str = "disable-string-interpolation-in-exception-messages";
    pub const OptionDisableVectors: &'static str = "disable-vectors";
    pub const OptionDisableWarnings: &'static str = "disable-warnings=";
    pub const OptionDiscardAssemblies: &'static str = "discard-assemblies=";
    pub const OptionDump: &'static str = "dump=";
    pub const OptionEnableAutoLayoutFallbackCheck: &'static str = "enable-autolayout-fallback-check";
    pub const OptionEnableDirectExternalLinking: &'static str = "enable-direct-external-linking";
    pub const OptionEnableInterpreter: &'static str = "enable-interpreter";
    pub const OptionFastMath: &'static str = "fastmath";
    pub const OptionFloatMode: &'static str = "float-mode=";
    pub const OptionFloatPrecision: &'static str = "float-precision=";
    pub const OptionFormat: &'static str = "format=";
    pub const OptionGenerateLinkXml: &'static str = "generate-link-xml=";
    pub const OptionGlobalSafetyChecksSetting: &'static str = "global-safety-checks-setting=";
    pub const OptionGroup: &'static str = "group";
    pub const OptionIncludeRootAssemblyReferences: &'static str = "include-root-assembly-references=";
    pub const OptionJitCompilationPriority: &'static str = "compilation-priority=";
    pub const OptionJitDisableAssemblyCaching: &'static str = "disable-assembly-caching";
    pub const OptionJitDisableFunctionCaching: &'static str = "disable-function-caching";
    pub const OptionJitEnableAssemblyCachingLogs: &'static str = "enable-assembly-caching-logs";
    pub const OptionJitEnableSynchronousCompilation: &'static str = "enable-synchronous-compilation";
    pub const OptionJitIsForFunctionPointer: &'static str = "is-for-function-pointer";
    pub const OptionJitManagedDelegateHandle: &'static str = "managed-delegate-handle=";
    pub const OptionJitManagedFunctionPointer: &'static str = "managed-function-pointer=";
    pub const OptionJobMarshalling: &'static str = "generate-job-marshalling-methods";
    pub const OptionLibraryOutputMode: &'static str = "library-output-mode=";
    pub const OptionLinkerOptions: &'static str = "linker-options=";
    pub const OptionLogTimings: &'static str = "log-timings";
    pub const OptionMetaDataGeneration: &'static str = "meta-data-generation=";
    pub const OptionMethodPrefix: &'static str = "method-prefix=";
    pub const OptionOptForSize: &'static str = "opt-for-size";
    pub const OptionOptLevel: &'static str = "opt-level=";
    pub const OptionOutputMode: &'static str = "output-mode=";
    pub const OptionPlatform: &'static str = "platform=";
    pub const OptionPlatformConfiguration: &'static str = "platform-configuration=";
    pub const OptionPrintLogOnMissingPInvokeCallbackAttribute: &'static str = "print-monopinvokecallbackmissing-message";
    pub const OptionRootAssembly: &'static str = "root-assembly=";
    pub const OptionSafetyChecks: &'static str = "safety-checks";
    pub const OptionSaveExtraContext: &'static str = "save-extra-context";
    pub const OptionStaticLinkage: &'static str = "generate-static-linkage-methods";
    pub const OptionTarget: &'static str = "target=";
    pub const OptionTargetFramework: &'static str = "target-framework=";
    pub const OptionTempDirectory: &'static str = "temp-folder=";
    pub const OptionValidateExternalToolChain: &'static str = "validate-external-tool-chain";
    pub const OptionVerbose: &'static str = "verbose";
    pub fn AddOption(
        builder: quest_hook::libil2cpp::Gc<crate::System::Text::StringBuilder>,
        option: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("AddOption", (builder, option))?;
        Ok(__cordl_ret.into())
    }
    pub fn CheckIsSecondaryUnityProcess() -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CheckIsSecondaryUnityProcess", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn Clone(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Unity::Burst::BurstCompilerOptions>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Unity::Burst::BurstCompilerOptions,
        > = __cordl_object.invoke("Clone", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn DeserialiseCompilationOptionsSafe(
        from: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        crate::System::ValueTuple_3<
            *mut quest_hook::libil2cpp::Il2CppArray<
                *mut quest_hook::libil2cpp::Il2CppString,
            >,
            *mut quest_hook::libil2cpp::Il2CppArray<
                *mut quest_hook::libil2cpp::Il2CppString,
            >,
            *mut quest_hook::libil2cpp::Il2CppString,
        >,
    > {
        let __cordl_ret: crate::System::ValueTuple_3<
            *mut quest_hook::libil2cpp::Il2CppArray<
                *mut quest_hook::libil2cpp::Il2CppString,
            >,
            *mut quest_hook::libil2cpp::Il2CppArray<
                *mut quest_hook::libil2cpp::Il2CppString,
            >,
            *mut quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("DeserialiseCompilationOptionsSafe", (from))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetBurstCompileAttribute(
        memberInfo: quest_hook::libil2cpp::Gc<crate::System::Reflection::MemberInfo>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Unity::Burst::BurstCompileAttribute>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Unity::Burst::BurstCompileAttribute,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetBurstCompileAttribute", (memberInfo))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetOption(
        optionName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetOption", (optionName, value))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetOptions(
        &mut self,
        attr: quest_hook::libil2cpp::Gc<crate::Unity::Burst::BurstCompileAttribute>,
        isForILPostProcessing: bool,
        isForCompilerClient: bool,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object
            .invoke("GetOptions", (attr, isForILPostProcessing, isForCompilerClient))?;
        Ok(__cordl_ret.into())
    }
    pub fn HasBurstCompileAttribute(
        member: quest_hook::libil2cpp::Gc<crate::System::Reflection::MemberInfo>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("HasBurstCompileAttribute", (member))?;
        Ok(__cordl_ret.into())
    }
    pub fn MaybeTriggerRecompilation(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("MaybeTriggerRecompilation", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn MergeAttributes(
        memberAttribute: quest_hook::libil2cpp::ByRefMut<
            *mut crate::Unity::Burst::BurstCompileAttribute,
        >,
        assemblyAttribute: quest_hook::libil2cpp::ByRefMut<
            *mut crate::Unity::Burst::BurstCompileAttribute,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("MergeAttributes", (memberAttribute, assemblyAttribute))?;
        Ok(__cordl_ret.into())
    }
    pub fn New_0() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn New__cordl_bool1(
        isGlobal: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (isGlobal))?;
        Ok(__cordl_object.into())
    }
    pub fn OnOptionsChanged(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnOptionsChanged", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn SerialiseCompilationOptionsSafe(
        roots: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<*mut quest_hook::libil2cpp::Il2CppString>,
        >,
        folders: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<*mut quest_hook::libil2cpp::Il2CppString>,
        >,
        options: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SerialiseCompilationOptionsSafe", (roots, folders, options))?;
        Ok(__cordl_ret.into())
    }
    pub fn TryGetAttribute_Assembly1(
        assembly: quest_hook::libil2cpp::Gc<crate::System::Reflection::Assembly>,
        attribute: quest_hook::libil2cpp::ByRefMut<
            *mut crate::Unity::Burst::BurstCompileAttribute,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("TryGetAttribute", (assembly, attribute))?;
        Ok(__cordl_ret.into())
    }
    pub fn TryGetAttribute_MemberInfo0(
        member: quest_hook::libil2cpp::Gc<crate::System::Reflection::MemberInfo>,
        attribute: quest_hook::libil2cpp::ByRefMut<
            *mut crate::Unity::Burst::BurstCompileAttribute,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("TryGetAttribute", (member, attribute))?;
        Ok(__cordl_ret.into())
    }
    pub fn TryGetOptions(
        &mut self,
        member: quest_hook::libil2cpp::Gc<crate::System::Reflection::MemberInfo>,
        flagsOut: quest_hook::libil2cpp::ByRefMut<
            *mut quest_hook::libil2cpp::Il2CppString,
        >,
        isForILPostProcessing: bool,
        isForCompilerClient: bool,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke(
                "TryGetOptions",
                (member, flagsOut, isForILPostProcessing, isForCompilerClient),
            )?;
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
    pub fn _ctor__cordl_bool1(
        &mut self,
        isGlobal: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (isGlobal))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_DisableOptimizations(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_DisableOptimizations", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_EnableBurstCompilation(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_EnableBurstCompilation", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_EnableBurstCompileSynchronously(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("get_EnableBurstCompileSynchronously", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_EnableBurstDebug(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_EnableBurstDebug", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_EnableBurstSafetyChecks(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("get_EnableBurstSafetyChecks", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_EnableBurstTimings(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_EnableBurstTimings", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_EnableFastMath(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_EnableFastMath", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_ForceEnableBurstSafetyChecks(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("get_ForceEnableBurstSafetyChecks", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_IsEnabled(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_IsEnabled", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_IsGlobal(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_IsGlobal", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_OptionsChanged(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Action>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Action> = __cordl_object
            .invoke("get_OptionsChanged", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_RequiresSynchronousCompilation(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("get_RequiresSynchronousCompilation", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn set_DisableOptimizations(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_DisableOptimizations", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_EnableBurstCompilation(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_EnableBurstCompilation", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_EnableBurstCompileSynchronously(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_EnableBurstCompileSynchronously", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_EnableBurstDebug(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_EnableBurstDebug", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_EnableBurstSafetyChecks(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_EnableBurstSafetyChecks", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_EnableBurstTimings(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_EnableBurstTimings", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_EnableFastMath(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_EnableFastMath", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_ForceEnableBurstSafetyChecks(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_ForceEnableBurstSafetyChecks", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_OptionsChanged(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::System::Action>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_OptionsChanged", (value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Unity+Burst+BurstCompilerOptions")]
impl quest_hook::libil2cpp::ObjectType for crate::Unity::Burst::BurstCompilerOptions {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
