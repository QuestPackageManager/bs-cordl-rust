#[cfg(feature = "Unity+Burst+DiagnosticId")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum DiagnosticId {
    #[default]
    ERR_AccessingManagedArrayNotSupported = 1022i32,
    ERR_AccessingNestedManagedArrayNotSupported = 1380i32,
    ERR_AssertArgTypesDiffer = 1330i32,
    ERR_AssertTypeNotSupported = 1071i32,
    ERR_AssumeRangeTypeMustBeInteger = 1201i32,
    ERR_AssumeRangeTypeMustBeSameSign = 1202i32,
    ERR_BinaryOperationNotSupported = 1097i32,
    ERR_CalliNonCCallingConventionNotSupported = 1099i32,
    ERR_CalliWithThisNotSupported = 1098i32,
    ERR_CallingBurstDiscardMethodWithReturnValueNotSupported = 1015i32,
    ERR_CallingManagedMethodNotSupported = 1016i32,
    ERR_CatchAndFilterConstructionNotSupported = 1007i32,
    ERR_CatchConstructionNotSupported = 1006i32,
    ERR_CircularStaticConstructorUsage = 1090i32,
    ERR_ConstantExpressionRequired = 1304i32,
    ERR_ConstructorNotSupported = 1056i32,
    ERR_DebugLogNotSupported = 1346i32,
    ERR_ErrorWhileProcessingVariable = 1053i32,
    ERR_ExternalInternalCallsInStaticConstructorsNotSupported = 1091i32,
    ERR_FunctionPointerMethodAndTypeMissingBurstCompileAttribute = 1059i32,
    ERR_FunctionPointerMethodMissingBurstCompileAttribute = 1057i32,
    ERR_FunctionPointerTypeMissingBurstCompileAttribute = 1058i32,
    ERR_InitModuleVerificationError = 1093i32,
    ERR_InstructionBoxNotSupported = 1020i32,
    ERR_InstructionCastclassNotSupported = 1030i32,
    ERR_InstructionEndfilterNotSupported = 1035i32,
    ERR_InstructionEndfinallyNotSupported = 1036i32,
    ERR_InstructionLdftnNotSupported = 1032i32,
    ERR_InstructionLdlenNonConstantLengthNotSupported = 1044i32,
    ERR_InstructionLdstrNotSupported = 1033i32,
    ERR_InstructionLdtokenFieldNotSupported = 1023i32,
    ERR_InstructionLdtokenMethodNotSupported = 1024i32,
    ERR_InstructionLdtokenNotSupported = 1026i32,
    ERR_InstructionLdtokenTypeNotSupported = 1025i32,
    ERR_InstructionLdvirtftnNotSupported = 1027i32,
    ERR_InstructionLeaveNotSupported = 1037i32,
    ERR_InstructionNewarrNotSupported = 1028i32,
    ERR_InstructionNewobjWithManagedTypeNotSupported = 1021i32,
    ERR_InstructionNotSupported = 1038i32,
    ERR_InstructionRethrowNotSupported = 1029i32,
    ERR_InstructionStfldToManagedObjectNotSupported = 1043i32,
    ERR_InstructionStsfldNotSupported = 1034i32,
    ERR_InstructionTargetCpuFeatureNotAllowedInThisBlock = 1200i32,
    ERR_InstructionUnboxNotSupported = 1019i32,
    ERR_InternalCompilerErrorInBackend = 100i32,
    ERR_InternalCompilerErrorInFunction = 101i32,
    ERR_InternalCompilerErrorInInstruction = 102i32,
    ERR_LdfldaWithFixedArrayExpected = 1008i32,
    ERR_LdftnNonCCallingConventionNotSupported = 1101i32,
    ERR_LdobjFromANonPointerNonReference = 1381i32,
    ERR_LoadingArgumentWithManagedTypeNotSupported = 1012i32,
    ERR_LoadingFieldFromManagedObjectNotSupported = 1010i32,
    ERR_LoadingFieldWithManagedTypeNotSupported = 1011i32,
    ERR_LoadingFromManagedNonReadonlyStaticFieldNotSupported = 1042i32,
    ERR_LoadingFromManagedStaticFieldNotSupported = 1041i32,
    ERR_LoadingFromNonReadonlyStaticFieldNotSupported = 1040i32,
    ERR_LoadingFromStaticFieldNotSupported = 1039i32,
    ERR_LoopIntrinsicMustBeCalledInsideLoop = 1320i32,
    ERR_LoopUnexpectedAutoVectorization = 1321i32,
    ERR_ManagedArgumentsNotSupported = 1004i32,
    ERR_ManagedStaticConstructor = 1360i32,
    ERR_MarshalAsNativeTypeOnReturnTypeNotSupported = 1069i32,
    ERR_MarshalAsOnFieldNotSupported = 1050i32,
    ERR_MarshalAsOnParameterNotSupported = 1061i32,
    ERR_MarshalAsOnReturnTypeNotSupported = 1062i32,
    ERR_MethodNotSupported = 1302i32,
    ERR_MissingExternBindings = 1068i32,
    ERR_ModuleVerificationError = 1094i32,
    ERR_MultiDimensionalArrayUnsupported = 1383i32,
    ERR_NonBlittableAndNonManagedSequentialStructNotSupported = 1384i32,
    ERR_OnlyStaticMethodsAllowed = 1000i32,
    ERR_PlatformNotSupported = 1092i32,
    ERR_PointerArgumentsUnexpectedAliasing = 1310i32,
    ERR_PointerExpected = 1009i32,
    ERR_RequiredTypeModifierNotSupported = 1052i32,
    ERR_StaticConstantArrayInStaticConstructor = 1361i32,
    ERR_StoringToFieldInReadOnlyParameterNotAllowed = 1073i32,
    ERR_StoringToReadOnlyFieldNotAllowed = 1072i32,
    ERR_StoringToReadOnlyParameterNotAllowed = 1074i32,
    ERR_StringArgumentIndexOutOfRange = 1344i32,
    ERR_StringArrayInvalidArrayCreation = 1350i32,
    ERR_StringArrayInvalidArrayIndex = 1353i32,
    ERR_StringArrayInvalidArrayIndexOutOfRange = 1354i32,
    ERR_StringArrayInvalidArraySize = 1351i32,
    ERR_StringArrayInvalidControlFlow = 1352i32,
    ERR_StringInternalCompilerFixedStringTooManyUsers = 1340i32,
    ERR_StringInvalidArgument = 1349i32,
    ERR_StringInvalidArgumentType = 1345i32,
    ERR_StringInvalidFormatForArgument = 1343i32,
    ERR_StringInvalidFormatMissingClosingBrace = 1341i32,
    ERR_StringInvalidIntegerForArgumentIndex = 1342i32,
    ERR_StringInvalidNonLiteralFormat = 1347i32,
    ERR_StringInvalidStringFormatMethod = 1348i32,
    ERR_StringLiteralRequired = 1382i32,
    ERR_StringLiteralTooBig = 1100i32,
    ERR_StructByValueNotSupported = 1064i32,
    ERR_StructSizeNotSupported = 1048i32,
    ERR_StructWithAutoLayoutNotSupported = 1045i32,
    ERR_StructWithGenericParametersAndExplicitLayoutNotSupported = 1047i32,
    ERR_StructZeroSizeNotSupported = 1049i32,
    ERR_StructsWithNonUnicodeCharsNotSupported = 1066i32,
    ERR_TypeManagerStaticFieldNotCompatible = 1075i32,
    ERR_TypeNotBlittableForFunctionPointer = 1063i32,
    ERR_TypeNotSupported = 1051i32,
    ERR_UnableToAccessManagedMethod = 1001i32,
    ERR_UnableToCallMethodOnInterfaceObject = 1102i32,
    ERR_UnableToFindFieldForTypeManager = 1077i32,
    ERR_UnableToFindInterfaceMethod = 1002i32,
    ERR_UnableToFindTypeIndexForTypeManagerType = 1076i32,
    ERR_UnableToFindTypeRequiredForTypeManager = 1095i32,
    ERR_UnableToResolveMethod = 1055i32,
    ERR_UnableToResolveType = 1054i32,
    ERR_UnexpectedEmptyMethodBody = 1003i32,
    ERR_UnexpectedIntegerTypesForBinaryOperation = 1096i32,
    ERR_UnmanagedStringMethodInvalid = 1356i32,
    ERR_UnmanagedStringMethodMissing = 1355i32,
    ERR_UnsupportedCpuDependentBranch = 1199i32,
    ERR_UnsupportedSpillTransform = 1300i32,
    ERR_UnsupportedSpillTransformTooManyUsers = 1301i32,
    ERR_VarArgFunctionNotSupported = 1385i32,
    ERR_VectorsByValueNotSupported = 1067i32,
    ERR_VectorsLoadFieldIsAddress = 1303i32,
    INF_FunctionPointerMethodAndTypeMissingMonoPInvokeCallbackAttribute = 10590i32,
    WRN_ACallToMethodHasBeenDiscarded = 1371i32,
    WRN_ExceptionThrownInNonSafetyCheckGuardedFunction = 1370i32,
    WRN_LoopIntrinsicCalledButLoopOptimizedAway = 1322i32,
}
#[cfg(feature = "Unity+Burst+DiagnosticId")]
unsafe impl quest_hook::libil2cpp::Type for crate::Unity::Burst::DiagnosticId {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "Unity.Burst";
    const CLASS_NAME: &'static str = "DiagnosticId";
    fn matches_value_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
}
#[cfg(feature = "Unity+Burst+DiagnosticId")]
unsafe impl quest_hook::libil2cpp::Argument for crate::Unity::Burst::DiagnosticId {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "Unity+Burst+DiagnosticId")]
unsafe impl quest_hook::libil2cpp::Parameter for crate::Unity::Burst::DiagnosticId {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_parameter(ty)
    }
    fn from_actual(actual: <Self as quest_hook::libil2cpp::Parameter>::Actual) -> Self {
        actual
    }
    fn into_actual(self) -> <Self as quest_hook::libil2cpp::Parameter>::Actual {
        self
    }
}
#[cfg(feature = "Unity+Burst+DiagnosticId")]
unsafe impl quest_hook::libil2cpp::Returned for crate::Unity::Burst::DiagnosticId {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_returned(ty)
    }
    fn from_object(object: Option<&mut quest_hook::libil2cpp::Il2CppObject>) -> Self {
        unsafe {
            quest_hook::libil2cpp::raw::unbox(
                quest_hook::libil2cpp::WrapRaw::raw(object.unwrap()),
            )
        }
    }
}
#[cfg(feature = "Unity+Burst+DiagnosticId")]
unsafe impl quest_hook::libil2cpp::Return for crate::Unity::Burst::DiagnosticId {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_return(ty)
    }
    fn into_actual(self) -> <Self as quest_hook::libil2cpp::Return>::Actual {
        self
    }
    fn from_actual(actual: <Self as quest_hook::libil2cpp::Return>::Actual) -> Self {
        actual
    }
}
