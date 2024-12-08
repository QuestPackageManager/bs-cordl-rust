#[cfg(feature = "System+Linq+Expressions+Interpreter+InstructionList+DebugView")]
#[repr(C)]
#[derive(Debug)]
pub struct InstructionList_DebugView {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "System+Linq+Expressions+Interpreter+InstructionList+DebugView")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::Linq::Expressions::Interpreter::InstructionList_DebugView =>
    "System.Linq.Expressions.Interpreter"."InstructionList/DebugView"
);
#[cfg(feature = "System+Linq+Expressions+Interpreter+InstructionList+DebugView")]
impl std::ops::Deref
for crate::System::Linq::Expressions::Interpreter::InstructionList_DebugView {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Linq+Expressions+Interpreter+InstructionList+DebugView")]
impl std::ops::DerefMut
for crate::System::Linq::Expressions::Interpreter::InstructionList_DebugView {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Linq+Expressions+Interpreter+InstructionList+DebugView")]
impl crate::System::Linq::Expressions::Interpreter::InstructionList_DebugView {
    #[cfg(
        feature = "System+Linq+Expressions+Interpreter+InstructionList+DebugView+InstructionView"
    )]
    pub type InstructionView = crate::System::Linq::Expressions::Interpreter::DebugView_InstructionView;
}
#[cfg(feature = "System+Linq+Expressions+Interpreter+InstructionList+DebugView")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Linq::Expressions::Interpreter::InstructionList_DebugView {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "System+Linq+Expressions+Interpreter+InstructionList")]
#[repr(C)]
#[derive(Debug)]
pub struct InstructionList {
    __cordl_parent: crate::System::Object,
    pub _instructions: *mut crate::System::Collections::Generic::List_1<
        *mut crate::System::Linq::Expressions::Interpreter::Instruction,
    >,
    pub _objects: *mut crate::System::Collections::Generic::List_1<
        *mut crate::System::Object,
    >,
    pub _currentStackDepth: i32,
    pub _maxStackDepth: i32,
    pub _currentContinuationsDepth: i32,
    pub _maxContinuationDepth: i32,
    pub _runtimeLabelCount: i32,
    pub _labels: *mut crate::System::Collections::Generic::List_1<
        *mut crate::System::Linq::Expressions::Interpreter::BranchLabel,
    >,
    pub _debugCookies: *mut crate::System::Collections::Generic::List_1<
        crate::System::Collections::Generic::KeyValuePair_2<
            i32,
            *mut crate::System::Object,
        >,
    >,
}
#[cfg(feature = "System+Linq+Expressions+Interpreter+InstructionList")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::Linq::Expressions::Interpreter::InstructionList =>
    "System.Linq.Expressions.Interpreter"."InstructionList"
);
#[cfg(feature = "System+Linq+Expressions+Interpreter+InstructionList")]
impl std::ops::Deref for crate::System::Linq::Expressions::Interpreter::InstructionList {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Linq+Expressions+Interpreter+InstructionList")]
impl std::ops::DerefMut
for crate::System::Linq::Expressions::Interpreter::InstructionList {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Linq+Expressions+Interpreter+InstructionList")]
impl crate::System::Linq::Expressions::Interpreter::InstructionList {
    #[cfg(feature = "System+Linq+Expressions+Interpreter+InstructionList+DebugView")]
    pub type DebugView = crate::System::Linq::Expressions::Interpreter::InstructionList_DebugView;
    pub fn BuildRuntimeLabels(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut quest_hook::libil2cpp::Il2CppArray<
            crate::System::Linq::Expressions::Interpreter::RuntimeLabel,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<
            crate::System::Linq::Expressions::Interpreter::RuntimeLabel,
        > = __cordl_object.invoke("BuildRuntimeLabels", ())?;
        Ok(__cordl_ret)
    }
    pub fn Emit(
        &mut self,
        instruction: *mut crate::System::Linq::Expressions::Interpreter::Instruction,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Emit", (instruction))?;
        Ok(__cordl_ret)
    }
    pub fn EmitAdd(
        &mut self,
        _cordl_type: *mut crate::System::Type,
        checked: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("EmitAdd", (_cordl_type, checked))?;
        Ok(__cordl_ret)
    }
    pub fn EmitAnd(
        &mut self,
        _cordl_type: *mut crate::System::Type,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("EmitAnd", (_cordl_type))?;
        Ok(__cordl_ret)
    }
    pub fn EmitArrayLength(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("EmitArrayLength", ())?;
        Ok(__cordl_ret)
    }
    pub fn EmitAssignLocal(
        &mut self,
        index: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("EmitAssignLocal", (index))?;
        Ok(__cordl_ret)
    }
    pub fn EmitAssignLocalBoxed(
        &mut self,
        index: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("EmitAssignLocalBoxed", (index))?;
        Ok(__cordl_ret)
    }
    pub fn EmitAssignLocalToClosure(
        &mut self,
        index: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("EmitAssignLocalToClosure", (index))?;
        Ok(__cordl_ret)
    }
    pub fn EmitBranchFalse(
        &mut self,
        elseLabel: *mut crate::System::Linq::Expressions::Interpreter::BranchLabel,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("EmitBranchFalse", (elseLabel))?;
        Ok(__cordl_ret)
    }
    pub fn EmitBranchTrue(
        &mut self,
        elseLabel: *mut crate::System::Linq::Expressions::Interpreter::BranchLabel,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("EmitBranchTrue", (elseLabel))?;
        Ok(__cordl_ret)
    }
    pub fn EmitBranch_BranchLabel1(
        &mut self,
        label: *mut crate::System::Linq::Expressions::Interpreter::BranchLabel,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("EmitBranch", (label))?;
        Ok(__cordl_ret)
    }
    pub fn EmitBranch_BranchLabel__cordl_bool__cordl_bool2(
        &mut self,
        label: *mut crate::System::Linq::Expressions::Interpreter::BranchLabel,
        hasResult: bool,
        hasValue: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("EmitBranch", (label, hasResult, hasValue))?;
        Ok(__cordl_ret)
    }
    pub fn EmitBranch_OffsetInstruction_BranchLabel0(
        &mut self,
        instruction: *mut crate::System::Linq::Expressions::Interpreter::OffsetInstruction,
        label: *mut crate::System::Linq::Expressions::Interpreter::BranchLabel,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("EmitBranch", (instruction, label))?;
        Ok(__cordl_ret)
    }
    pub fn EmitByRefCall(
        &mut self,
        method: *mut crate::System::Reflection::MethodInfo,
        parameters: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::System::Reflection::ParameterInfo,
        >,
        byrefArgs: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::System::Linq::Expressions::Interpreter::ByRefUpdater,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("EmitByRefCall", (method, parameters, byrefArgs))?;
        Ok(__cordl_ret)
    }
    pub fn EmitByRefNew(
        &mut self,
        constructorInfo: *mut crate::System::Reflection::ConstructorInfo,
        parameters: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::System::Reflection::ParameterInfo,
        >,
        updaters: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::System::Linq::Expressions::Interpreter::ByRefUpdater,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("EmitByRefNew", (constructorInfo, parameters, updaters))?;
        Ok(__cordl_ret)
    }
    pub fn EmitCall_Il2CppArray1(
        &mut self,
        method: *mut crate::System::Reflection::MethodInfo,
        parameters: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::System::Reflection::ParameterInfo,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("EmitCall", (method, parameters))?;
        Ok(__cordl_ret)
    }
    pub fn EmitCall_MethodInfo0(
        &mut self,
        method: *mut crate::System::Reflection::MethodInfo,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("EmitCall", (method))?;
        Ok(__cordl_ret)
    }
    pub fn EmitCast(
        &mut self,
        toType: *mut crate::System::Type,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("EmitCast", (toType))?;
        Ok(__cordl_ret)
    }
    pub fn EmitCastReferenceToEnum(
        &mut self,
        toType: *mut crate::System::Type,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("EmitCastReferenceToEnum", (toType))?;
        Ok(__cordl_ret)
    }
    pub fn EmitCastToEnum(
        &mut self,
        toType: *mut crate::System::Type,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("EmitCastToEnum", (toType))?;
        Ok(__cordl_ret)
    }
    pub fn EmitCoalescingBranch(
        &mut self,
        leftNotNull: *mut crate::System::Linq::Expressions::Interpreter::BranchLabel,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("EmitCoalescingBranch", (leftNotNull))?;
        Ok(__cordl_ret)
    }
    pub fn EmitConvertToUnderlying(
        &mut self,
        to: crate::System::TypeCode,
        isLiftedToNull: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("EmitConvertToUnderlying", (to, isLiftedToNull))?;
        Ok(__cordl_ret)
    }
    pub fn EmitCreateDelegate(
        &mut self,
        creator: *mut crate::System::Linq::Expressions::Interpreter::LightDelegateCreator,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("EmitCreateDelegate", (creator))?;
        Ok(__cordl_ret)
    }
    pub fn EmitDecrement(
        &mut self,
        _cordl_type: *mut crate::System::Type,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("EmitDecrement", (_cordl_type))?;
        Ok(__cordl_ret)
    }
    pub fn EmitDefaultValue(
        &mut self,
        _cordl_type: *mut crate::System::Type,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("EmitDefaultValue", (_cordl_type))?;
        Ok(__cordl_ret)
    }
    pub fn EmitDiv(
        &mut self,
        _cordl_type: *mut crate::System::Type,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("EmitDiv", (_cordl_type))?;
        Ok(__cordl_ret)
    }
    pub fn EmitDup(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("EmitDup", ())?;
        Ok(__cordl_ret)
    }
    pub fn EmitEnterExceptionFilter(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("EmitEnterExceptionFilter", ())?;
        Ok(__cordl_ret)
    }
    pub fn EmitEnterExceptionHandlerNonVoid(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("EmitEnterExceptionHandlerNonVoid", ())?;
        Ok(__cordl_ret)
    }
    pub fn EmitEnterExceptionHandlerVoid(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("EmitEnterExceptionHandlerVoid", ())?;
        Ok(__cordl_ret)
    }
    pub fn EmitEnterFault(
        &mut self,
        faultStartLabel: *mut crate::System::Linq::Expressions::Interpreter::BranchLabel,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("EmitEnterFault", (faultStartLabel))?;
        Ok(__cordl_ret)
    }
    pub fn EmitEnterFinally(
        &mut self,
        finallyStartLabel: *mut crate::System::Linq::Expressions::Interpreter::BranchLabel,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("EmitEnterFinally", (finallyStartLabel))?;
        Ok(__cordl_ret)
    }
    pub fn EmitEnterTryCatch(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("EmitEnterTryCatch", ())?;
        Ok(__cordl_ret)
    }
    pub fn EmitEnterTryFault(
        &mut self,
        tryEnd: *mut crate::System::Linq::Expressions::Interpreter::BranchLabel,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Linq::Expressions::Interpreter::EnterTryFaultInstruction,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Linq::Expressions::Interpreter::EnterTryFaultInstruction = __cordl_object
            .invoke("EmitEnterTryFault", (tryEnd))?;
        Ok(__cordl_ret)
    }
    pub fn EmitEnterTryFinally(
        &mut self,
        finallyStartLabel: *mut crate::System::Linq::Expressions::Interpreter::BranchLabel,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("EmitEnterTryFinally", (finallyStartLabel))?;
        Ok(__cordl_ret)
    }
    pub fn EmitEqual(
        &mut self,
        _cordl_type: *mut crate::System::Type,
        liftedToNull: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("EmitEqual", (_cordl_type, liftedToNull))?;
        Ok(__cordl_ret)
    }
    pub fn EmitExclusiveOr(
        &mut self,
        _cordl_type: *mut crate::System::Type,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("EmitExclusiveOr", (_cordl_type))?;
        Ok(__cordl_ret)
    }
    pub fn EmitGetArrayItem(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("EmitGetArrayItem", ())?;
        Ok(__cordl_ret)
    }
    pub fn EmitGoto(
        &mut self,
        label: *mut crate::System::Linq::Expressions::Interpreter::BranchLabel,
        hasResult: bool,
        hasValue: bool,
        labelTargetGetsValue: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("EmitGoto", (label, hasResult, hasValue, labelTargetGetsValue))?;
        Ok(__cordl_ret)
    }
    pub fn EmitGreaterThan(
        &mut self,
        _cordl_type: *mut crate::System::Type,
        liftedToNull: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("EmitGreaterThan", (_cordl_type, liftedToNull))?;
        Ok(__cordl_ret)
    }
    pub fn EmitGreaterThanOrEqual(
        &mut self,
        _cordl_type: *mut crate::System::Type,
        liftedToNull: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("EmitGreaterThanOrEqual", (_cordl_type, liftedToNull))?;
        Ok(__cordl_ret)
    }
    pub fn EmitIncrement(
        &mut self,
        _cordl_type: *mut crate::System::Type,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("EmitIncrement", (_cordl_type))?;
        Ok(__cordl_ret)
    }
    pub fn EmitInitializeLocal(
        &mut self,
        index: i32,
        _cordl_type: *mut crate::System::Type,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("EmitInitializeLocal", (index, _cordl_type))?;
        Ok(__cordl_ret)
    }
    pub fn EmitInitializeParameter(
        &mut self,
        index: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("EmitInitializeParameter", (index))?;
        Ok(__cordl_ret)
    }
    pub fn EmitIntSwitch<T>(
        &mut self,
        cases: *mut crate::System::Collections::Generic::Dictionary_2<T, i32>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("EmitIntSwitch", (cases))?;
        Ok(__cordl_ret)
    }
    pub fn EmitLeaveExceptionFilter(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("EmitLeaveExceptionFilter", ())?;
        Ok(__cordl_ret)
    }
    pub fn EmitLeaveExceptionHandler(
        &mut self,
        hasValue: bool,
        tryExpressionEndLabel: *mut crate::System::Linq::Expressions::Interpreter::BranchLabel,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("EmitLeaveExceptionHandler", (hasValue, tryExpressionEndLabel))?;
        Ok(__cordl_ret)
    }
    pub fn EmitLeaveFault(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("EmitLeaveFault", ())?;
        Ok(__cordl_ret)
    }
    pub fn EmitLeaveFinally(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("EmitLeaveFinally", ())?;
        Ok(__cordl_ret)
    }
    pub fn EmitLeftShift(
        &mut self,
        _cordl_type: *mut crate::System::Type,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("EmitLeftShift", (_cordl_type))?;
        Ok(__cordl_ret)
    }
    pub fn EmitLessThan(
        &mut self,
        _cordl_type: *mut crate::System::Type,
        liftedToNull: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("EmitLessThan", (_cordl_type, liftedToNull))?;
        Ok(__cordl_ret)
    }
    pub fn EmitLessThanOrEqual(
        &mut self,
        _cordl_type: *mut crate::System::Type,
        liftedToNull: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("EmitLessThanOrEqual", (_cordl_type, liftedToNull))?;
        Ok(__cordl_ret)
    }
    pub fn EmitLoadField(
        &mut self,
        field: *mut crate::System::Reflection::FieldInfo,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("EmitLoadField", (field))?;
        Ok(__cordl_ret)
    }
    pub fn EmitLoadLocal(
        &mut self,
        index: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("EmitLoadLocal", (index))?;
        Ok(__cordl_ret)
    }
    pub fn EmitLoadLocalBoxed(
        &mut self,
        index: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("EmitLoadLocalBoxed", (index))?;
        Ok(__cordl_ret)
    }
    pub fn EmitLoadLocalFromClosure(
        &mut self,
        index: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("EmitLoadLocalFromClosure", (index))?;
        Ok(__cordl_ret)
    }
    pub fn EmitLoadLocalFromClosureBoxed(
        &mut self,
        index: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("EmitLoadLocalFromClosureBoxed", (index))?;
        Ok(__cordl_ret)
    }
    pub fn EmitLoad_Object0(
        &mut self,
        value: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("EmitLoad", (value))?;
        Ok(__cordl_ret)
    }
    pub fn EmitLoad_Object_Type2(
        &mut self,
        value: *mut crate::System::Object,
        _cordl_type: *mut crate::System::Type,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("EmitLoad", (value, _cordl_type))?;
        Ok(__cordl_ret)
    }
    pub fn EmitLoad__cordl_bool1(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("EmitLoad", (value))?;
        Ok(__cordl_ret)
    }
    pub fn EmitModulo(
        &mut self,
        _cordl_type: *mut crate::System::Type,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("EmitModulo", (_cordl_type))?;
        Ok(__cordl_ret)
    }
    pub fn EmitMul(
        &mut self,
        _cordl_type: *mut crate::System::Type,
        checked: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("EmitMul", (_cordl_type, checked))?;
        Ok(__cordl_ret)
    }
    pub fn EmitNegate(
        &mut self,
        _cordl_type: *mut crate::System::Type,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("EmitNegate", (_cordl_type))?;
        Ok(__cordl_ret)
    }
    pub fn EmitNegateChecked(
        &mut self,
        _cordl_type: *mut crate::System::Type,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("EmitNegateChecked", (_cordl_type))?;
        Ok(__cordl_ret)
    }
    pub fn EmitNew(
        &mut self,
        constructorInfo: *mut crate::System::Reflection::ConstructorInfo,
        parameters: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::System::Reflection::ParameterInfo,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("EmitNew", (constructorInfo, parameters))?;
        Ok(__cordl_ret)
    }
    pub fn EmitNewArray(
        &mut self,
        elementType: *mut crate::System::Type,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("EmitNewArray", (elementType))?;
        Ok(__cordl_ret)
    }
    pub fn EmitNewArrayBounds(
        &mut self,
        elementType: *mut crate::System::Type,
        rank: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("EmitNewArrayBounds", (elementType, rank))?;
        Ok(__cordl_ret)
    }
    pub fn EmitNewArrayInit(
        &mut self,
        elementType: *mut crate::System::Type,
        elementCount: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("EmitNewArrayInit", (elementType, elementCount))?;
        Ok(__cordl_ret)
    }
    pub fn EmitNewRuntimeVariables(
        &mut self,
        count: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("EmitNewRuntimeVariables", (count))?;
        Ok(__cordl_ret)
    }
    pub fn EmitNot(
        &mut self,
        _cordl_type: *mut crate::System::Type,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("EmitNot", (_cordl_type))?;
        Ok(__cordl_ret)
    }
    pub fn EmitNotEqual(
        &mut self,
        _cordl_type: *mut crate::System::Type,
        liftedToNull: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("EmitNotEqual", (_cordl_type, liftedToNull))?;
        Ok(__cordl_ret)
    }
    pub fn EmitNullableCall(
        &mut self,
        method: *mut crate::System::Reflection::MethodInfo,
        parameters: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::System::Reflection::ParameterInfo,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("EmitNullableCall", (method, parameters))?;
        Ok(__cordl_ret)
    }
    pub fn EmitNumericConvertChecked(
        &mut self,
        from: crate::System::TypeCode,
        to: crate::System::TypeCode,
        isLiftedToNull: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("EmitNumericConvertChecked", (from, to, isLiftedToNull))?;
        Ok(__cordl_ret)
    }
    pub fn EmitNumericConvertUnchecked(
        &mut self,
        from: crate::System::TypeCode,
        to: crate::System::TypeCode,
        isLiftedToNull: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("EmitNumericConvertUnchecked", (from, to, isLiftedToNull))?;
        Ok(__cordl_ret)
    }
    pub fn EmitOr(
        &mut self,
        _cordl_type: *mut crate::System::Type,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("EmitOr", (_cordl_type))?;
        Ok(__cordl_ret)
    }
    pub fn EmitPop(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("EmitPop", ())?;
        Ok(__cordl_ret)
    }
    pub fn EmitRethrow(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("EmitRethrow", ())?;
        Ok(__cordl_ret)
    }
    pub fn EmitRethrowVoid(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("EmitRethrowVoid", ())?;
        Ok(__cordl_ret)
    }
    pub fn EmitRightShift(
        &mut self,
        _cordl_type: *mut crate::System::Type,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("EmitRightShift", (_cordl_type))?;
        Ok(__cordl_ret)
    }
    pub fn EmitSetArrayItem(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("EmitSetArrayItem", ())?;
        Ok(__cordl_ret)
    }
    pub fn EmitStoreField(
        &mut self,
        field: *mut crate::System::Reflection::FieldInfo,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("EmitStoreField", (field))?;
        Ok(__cordl_ret)
    }
    pub fn EmitStoreLocal(
        &mut self,
        index: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("EmitStoreLocal", (index))?;
        Ok(__cordl_ret)
    }
    pub fn EmitStoreLocalBoxed(
        &mut self,
        index: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("EmitStoreLocalBoxed", (index))?;
        Ok(__cordl_ret)
    }
    pub fn EmitStoreLocalToClosure(
        &mut self,
        index: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("EmitStoreLocalToClosure", (index))?;
        Ok(__cordl_ret)
    }
    pub fn EmitStringSwitch(
        &mut self,
        cases: *mut crate::System::Collections::Generic::Dictionary_2<
            *mut crate::System::String,
            i32,
        >,
        nullCase: *mut crate::System::Runtime::CompilerServices::StrongBox_1<i32>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("EmitStringSwitch", (cases, nullCase))?;
        Ok(__cordl_ret)
    }
    pub fn EmitSub(
        &mut self,
        _cordl_type: *mut crate::System::Type,
        checked: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("EmitSub", (_cordl_type, checked))?;
        Ok(__cordl_ret)
    }
    pub fn EmitThrow(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("EmitThrow", ())?;
        Ok(__cordl_ret)
    }
    pub fn EmitThrowVoid(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("EmitThrowVoid", ())?;
        Ok(__cordl_ret)
    }
    pub fn EmitTypeAs(
        &mut self,
        _cordl_type: *mut crate::System::Type,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("EmitTypeAs", (_cordl_type))?;
        Ok(__cordl_ret)
    }
    pub fn EmitTypeEquals(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("EmitTypeEquals", ())?;
        Ok(__cordl_ret)
    }
    pub fn EmitTypeIs(
        &mut self,
        _cordl_type: *mut crate::System::Type,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("EmitTypeIs", (_cordl_type))?;
        Ok(__cordl_ret)
    }
    pub fn EnsureLabelIndex(
        &mut self,
        label: *mut crate::System::Linq::Expressions::Interpreter::BranchLabel,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("EnsureLabelIndex", (label))?;
        Ok(__cordl_ret)
    }
    pub fn FixupBranch(
        &mut self,
        branchIndex: i32,
        offset: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("FixupBranch", (branchIndex, offset))?;
        Ok(__cordl_ret)
    }
    pub fn GetInstruction(
        &mut self,
        index: i32,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Linq::Expressions::Interpreter::Instruction,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Linq::Expressions::Interpreter::Instruction = __cordl_object
            .invoke("GetInstruction", (index))?;
        Ok(__cordl_ret)
    }
    pub fn GetLoadField(
        &mut self,
        field: *mut crate::System::Reflection::FieldInfo,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Linq::Expressions::Interpreter::Instruction,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Linq::Expressions::Interpreter::Instruction = __cordl_object
            .invoke("GetLoadField", (field))?;
        Ok(__cordl_ret)
    }
    pub fn MakeLabel(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Linq::Expressions::Interpreter::BranchLabel,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Linq::Expressions::Interpreter::BranchLabel = __cordl_object
            .invoke("MakeLabel", ())?;
        Ok(__cordl_ret)
    }
    pub fn MarkLabel(
        &mut self,
        label: *mut crate::System::Linq::Expressions::Interpreter::BranchLabel,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("MarkLabel", (label))?;
        Ok(__cordl_ret)
    }
    pub fn MarkRuntimeLabel(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("MarkRuntimeLabel", ())?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn SwitchToBoxed(
        &mut self,
        index: i32,
        instructionIndex: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SwitchToBoxed", (index, instructionIndex))?;
        Ok(__cordl_ret)
    }
    pub fn ToArray(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::System::Linq::Expressions::Interpreter::InstructionArray,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::Linq::Expressions::Interpreter::InstructionArray = __cordl_object
            .invoke("ToArray", ())?;
        Ok(__cordl_ret)
    }
    pub fn UnEmit(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UnEmit", ())?;
        Ok(__cordl_ret)
    }
    pub fn UpdateStackDepth(
        &mut self,
        instruction: *mut crate::System::Linq::Expressions::Interpreter::Instruction,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UpdateStackDepth", (instruction))?;
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
    pub fn get_Count(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_Count", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_CurrentContinuationsDepth(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke("get_CurrentContinuationsDepth", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_CurrentStackDepth(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_CurrentStackDepth", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "System+Linq+Expressions+Interpreter+InstructionList")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Linq::Expressions::Interpreter::InstructionList {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(
    feature = "System+Linq+Expressions+Interpreter+InstructionList+DebugView+InstructionView"
)]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct DebugView_InstructionView {
    pub _index: i32,
    pub _stackDepth: i32,
    pub _continuationsDepth: i32,
    pub _name: *mut crate::System::String,
    pub _instruction: *mut crate::System::Linq::Expressions::Interpreter::Instruction,
}
#[cfg(
    feature = "System+Linq+Expressions+Interpreter+InstructionList+DebugView+InstructionView"
)]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::System::Linq::Expressions::Interpreter::DebugView_InstructionView =>
    "System.Linq.Expressions.Interpreter"."InstructionList/DebugView/InstructionView"
);
#[cfg(
    feature = "System+Linq+Expressions+Interpreter+InstructionList+DebugView+InstructionView"
)]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::System::Linq::Expressions::Interpreter::DebugView_InstructionView {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(
    feature = "System+Linq+Expressions+Interpreter+InstructionList+DebugView+InstructionView"
)]
impl crate::System::Linq::Expressions::Interpreter::DebugView_InstructionView {
    pub fn GetValue(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_ret: *mut crate::System::String = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "GetValue",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        instruction: *mut crate::System::Linq::Expressions::Interpreter::Instruction,
        name: *mut crate::System::String,
        index: i32,
        stackDepth: i32,
        continuationsDepth: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (instruction, name, index, stackDepth, continuationsDepth),
        )?;
        Ok(__cordl_ret)
    }
}
