#[cfg(feature = "System+Linq+Expressions+Interpreter+LightCompiler")]
#[repr(C)]
#[derive(Debug)]
pub struct LightCompiler {
    __cordl_parent: crate::System::Object,
    pub _instructions: *mut crate::System::Linq::Expressions::Interpreter::InstructionList,
    pub _locals: *mut crate::System::Linq::Expressions::Interpreter::LocalVariables,
    pub _debugInfos: *mut crate::System::Collections::Generic::List_1<
        *mut crate::System::Linq::Expressions::Interpreter::DebugInfo,
    >,
    pub _treeLabels: *mut crate::System::Linq::Expressions::Interpreter::HybridReferenceDictionary_2<
        *mut crate::System::Linq::Expressions::LabelTarget,
        *mut crate::System::Linq::Expressions::Interpreter::LabelInfo,
    >,
    pub _labelBlock: *mut crate::System::Linq::Expressions::Interpreter::LabelScopeInfo,
    pub _exceptionForRethrowStack: *mut crate::System::Collections::Generic::Stack_1<
        *mut crate::System::Linq::Expressions::ParameterExpression,
    >,
    pub _parent: *mut crate::System::Linq::Expressions::Interpreter::LightCompiler,
    pub _guard: *mut crate::System::Linq::Expressions::StackGuard,
}
#[cfg(feature = "System+Linq+Expressions+Interpreter+LightCompiler")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::Linq::Expressions::Interpreter::LightCompiler =>
    "System.Linq.Expressions.Interpreter"."LightCompiler"
);
#[cfg(feature = "System+Linq+Expressions+Interpreter+LightCompiler")]
impl std::ops::Deref for crate::System::Linq::Expressions::Interpreter::LightCompiler {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Linq+Expressions+Interpreter+LightCompiler")]
impl std::ops::DerefMut
for crate::System::Linq::Expressions::Interpreter::LightCompiler {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Linq+Expressions+Interpreter+LightCompiler")]
impl crate::System::Linq::Expressions::Interpreter::LightCompiler {
    #[cfg(feature = "System+Linq+Expressions+Interpreter+LightCompiler+QuoteVisitor")]
    pub type QuoteVisitor = crate::System::Linq::Expressions::Interpreter::LightCompiler_QuoteVisitor;
    #[cfg(feature = "System+Linq+Expressions+Interpreter+LightCompiler+__c")]
    pub type __c = crate::System::Linq::Expressions::Interpreter::LightCompiler___c;
    pub fn CompileTypeEqualExpression(
        &mut self,
        expr: *mut crate::System::Linq::Expressions::Expression,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CompileTypeEqualExpression", (expr))?;
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
    pub fn _ctor_LightCompiler1(
        &mut self,
        parent: *mut crate::System::Linq::Expressions::Interpreter::LightCompiler,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (parent))?;
        Ok(__cordl_ret)
    }
    pub fn EmitUnaryMethodCall(
        &mut self,
        node: *mut crate::System::Linq::Expressions::UnaryExpression,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("EmitUnaryMethodCall", (node))?;
        Ok(__cordl_ret)
    }
    pub fn CompileCoalesceBinaryExpression(
        &mut self,
        expr: *mut crate::System::Linq::Expressions::Expression,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CompileCoalesceBinaryExpression", (expr))?;
        Ok(__cordl_ret)
    }
    pub fn EnsureAvailableForClosure(
        &mut self,
        expr: *mut crate::System::Linq::Expressions::ParameterExpression,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Linq::Expressions::Interpreter::LocalVariable,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Linq::Expressions::Interpreter::LocalVariable = __cordl_object
            .invoke("EnsureAvailableForClosure", (expr))?;
        Ok(__cordl_ret)
    }
    pub fn CompileGotoExpression(
        &mut self,
        expr: *mut crate::System::Linq::Expressions::Expression,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CompileGotoExpression", (expr))?;
        Ok(__cordl_ret)
    }
    pub fn EnsureLabel(
        &mut self,
        node: *mut crate::System::Linq::Expressions::LabelTarget,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Linq::Expressions::Interpreter::LabelInfo,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Linq::Expressions::Interpreter::LabelInfo = __cordl_object
            .invoke("EnsureLabel", (node))?;
        Ok(__cordl_ret)
    }
    pub fn CompileTryExpression(
        &mut self,
        expr: *mut crate::System::Linq::Expressions::Expression,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CompileTryExpression", (expr))?;
        Ok(__cordl_ret)
    }
    pub fn CompileIndexAssignment(
        &mut self,
        node: *mut crate::System::Linq::Expressions::BinaryExpression,
        asVoid: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CompileIndexAssignment", (node, asVoid))?;
        Ok(__cordl_ret)
    }
    pub fn CompileMethodCallExpression_Expression0(
        &mut self,
        expr: *mut crate::System::Linq::Expressions::Expression,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CompileMethodCallExpression", (expr))?;
        Ok(__cordl_ret)
    }
    pub fn CompileMethodCallExpression_MethodInfo_IArgumentProvider1(
        &mut self,
        object: *mut crate::System::Linq::Expressions::Expression,
        method: *mut crate::System::Reflection::MethodInfo,
        arguments: *mut crate::System::Linq::Expressions::IArgumentProvider,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CompileMethodCallExpression", (object, method, arguments))?;
        Ok(__cordl_ret)
    }
    pub fn CompileConstantExpression(
        &mut self,
        expr: *mut crate::System::Linq::Expressions::Expression,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CompileConstantExpression", (expr))?;
        Ok(__cordl_ret)
    }
    pub fn CompileComparison(
        &mut self,
        node: *mut crate::System::Linq::Expressions::BinaryExpression,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CompileComparison", (node))?;
        Ok(__cordl_ret)
    }
    pub fn CheckRethrow(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CheckRethrow", ())?;
        Ok(__cordl_ret)
    }
    pub fn CompileTop(
        &mut self,
        node: *mut crate::System::Linq::Expressions::LambdaExpression,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Linq::Expressions::Interpreter::LightDelegateCreator,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Linq::Expressions::Interpreter::LightDelegateCreator = __cordl_object
            .invoke("CompileTop", (node))?;
        Ok(__cordl_ret)
    }
    pub fn CompileLogicalBinaryExpression(
        &mut self,
        b: *mut crate::System::Linq::Expressions::BinaryExpression,
        andAlso: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CompileLogicalBinaryExpression", (b, andAlso))?;
        Ok(__cordl_ret)
    }
    pub fn CompileConditionalExpression(
        &mut self,
        expr: *mut crate::System::Linq::Expressions::Expression,
        asVoid: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CompileConditionalExpression", (expr, asVoid))?;
        Ok(__cordl_ret)
    }
    pub fn CompileIndexExpression(
        &mut self,
        expr: *mut crate::System::Linq::Expressions::Expression,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CompileIndexExpression", (expr))?;
        Ok(__cordl_ret)
    }
    pub fn CompileMethodLogicalBinaryExpression(
        &mut self,
        expr: *mut crate::System::Linq::Expressions::BinaryExpression,
        andAlso: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CompileMethodLogicalBinaryExpression", (expr, andAlso))?;
        Ok(__cordl_ret)
    }
    pub fn CompileLabelExpression(
        &mut self,
        expr: *mut crate::System::Linq::Expressions::Expression,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CompileLabelExpression", (expr))?;
        Ok(__cordl_ret)
    }
    pub fn ResolveLocal(
        &mut self,
        variable: *mut crate::System::Linq::Expressions::ParameterExpression,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Linq::Expressions::Interpreter::LocalVariable,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Linq::Expressions::Interpreter::LocalVariable = __cordl_object
            .invoke("ResolveLocal", (variable))?;
        Ok(__cordl_ret)
    }
    pub fn CompileMemberExpression(
        &mut self,
        expr: *mut crate::System::Linq::Expressions::Expression,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CompileMemberExpression", (expr))?;
        Ok(__cordl_ret)
    }
    pub fn Compile__cordl_bool0(
        &mut self,
        expr: *mut crate::System::Linq::Expressions::Expression,
        asVoid: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Compile", (expr, asVoid))?;
        Ok(__cordl_ret)
    }
    pub fn Compile_Expression1(
        &mut self,
        expr: *mut crate::System::Linq::Expressions::Expression,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Compile", (expr))?;
        Ok(__cordl_ret)
    }
    pub fn CompileDefaultExpression_Expression0(
        &mut self,
        expr: *mut crate::System::Linq::Expressions::Expression,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CompileDefaultExpression", (expr))?;
        Ok(__cordl_ret)
    }
    pub fn CompileDefaultExpression_Type1(
        &mut self,
        _cordl_type: *mut crate::System::Type,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CompileDefaultExpression", (_cordl_type))?;
        Ok(__cordl_ret)
    }
    pub fn CompileArrayIndexAddress(
        &mut self,
        array: *mut crate::System::Linq::Expressions::Expression,
        index: *mut crate::System::Linq::Expressions::Expression,
        argumentIndex: i32,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Linq::Expressions::Interpreter::ByRefUpdater,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Linq::Expressions::Interpreter::ByRefUpdater = __cordl_object
            .invoke("CompileArrayIndexAddress", (array, index, argumentIndex))?;
        Ok(__cordl_ret)
    }
    pub fn PopLabelBlock(
        &mut self,
        kind: crate::System::Linq::Expressions::Interpreter::LabelScopeKind,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("PopLabelBlock", (kind))?;
        Ok(__cordl_ret)
    }
    pub fn CompileNewExpression(
        &mut self,
        expr: *mut crate::System::Linq::Expressions::Expression,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CompileNewExpression", (expr))?;
        Ok(__cordl_ret)
    }
    pub fn CompileMember(
        &mut self,
        from: *mut crate::System::Linq::Expressions::Expression,
        member: *mut crate::System::Reflection::MemberInfo,
        forBinding: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CompileMember", (from, member, forBinding))?;
        Ok(__cordl_ret)
    }
    pub fn CompileSwitchExpression(
        &mut self,
        expr: *mut crate::System::Linq::Expressions::Expression,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CompileSwitchExpression", (expr))?;
        Ok(__cordl_ret)
    }
    pub fn CompileArithmetic(
        &mut self,
        nodeType: crate::System::Linq::Expressions::ExpressionType,
        left: *mut crate::System::Linq::Expressions::Expression,
        right: *mut crate::System::Linq::Expressions::Expression,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CompileArithmetic", (nodeType, left, right))?;
        Ok(__cordl_ret)
    }
    pub fn CompileMultiDimArrayAccess(
        &mut self,
        array: *mut crate::System::Linq::Expressions::Expression,
        arguments: *mut crate::System::Linq::Expressions::IArgumentProvider,
        index: i32,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Linq::Expressions::Interpreter::ByRefUpdater,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Linq::Expressions::Interpreter::ByRefUpdater = __cordl_object
            .invoke("CompileMultiDimArrayAccess", (array, arguments, index))?;
        Ok(__cordl_ret)
    }
    pub fn CompileAsVoid(
        &mut self,
        expr: *mut crate::System::Linq::Expressions::Expression,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CompileAsVoid", (expr))?;
        Ok(__cordl_ret)
    }
    pub fn LoadLocalNoValueTypeCopy(
        &mut self,
        variable: *mut crate::System::Linq::Expressions::ParameterExpression,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("LoadLocalNoValueTypeCopy", (variable))?;
        Ok(__cordl_ret)
    }
    pub fn CompileNoLabelPush(
        &mut self,
        expr: *mut crate::System::Linq::Expressions::Expression,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CompileNoLabelPush", (expr))?;
        Ok(__cordl_ret)
    }
    pub fn EmitCopyValueType(
        &mut self,
        valueType: *mut crate::System::Type,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("EmitCopyValueType", (valueType))?;
        Ok(__cordl_ret)
    }
    pub fn get_Instructions(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Linq::Expressions::Interpreter::InstructionList,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Linq::Expressions::Interpreter::InstructionList = __cordl_object
            .invoke("get_Instructions", ())?;
        Ok(__cordl_ret)
    }
    pub fn CompileMemberInitExpression(
        &mut self,
        expr: *mut crate::System::Linq::Expressions::Expression,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CompileMemberInitExpression", (expr))?;
        Ok(__cordl_ret)
    }
    pub fn CompileBinaryExpression(
        &mut self,
        expr: *mut crate::System::Linq::Expressions::Expression,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CompileBinaryExpression", (expr))?;
        Ok(__cordl_ret)
    }
    pub fn CompileLiftedLogicalBinaryExpression(
        &mut self,
        node: *mut crate::System::Linq::Expressions::BinaryExpression,
        andAlso: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CompileLiftedLogicalBinaryExpression", (node, andAlso))?;
        Ok(__cordl_ret)
    }
    pub fn CompileGetVariable(
        &mut self,
        variable: *mut crate::System::Linq::Expressions::ParameterExpression,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CompileGetVariable", (variable))?;
        Ok(__cordl_ret)
    }
    pub fn CompileTypeIsExpression(
        &mut self,
        expr: *mut crate::System::Linq::Expressions::Expression,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CompileTypeIsExpression", (expr))?;
        Ok(__cordl_ret)
    }
    pub fn PushLabelBlock(
        &mut self,
        _cordl_type: crate::System::Linq::Expressions::Interpreter::LabelScopeKind,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("PushLabelBlock", (_cordl_type))?;
        Ok(__cordl_ret)
    }
    pub fn DefineBlockLabels(
        &mut self,
        node: *mut crate::System::Linq::Expressions::Expression,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("DefineBlockLabels", (node))?;
        Ok(__cordl_ret)
    }
    pub fn CompileSetVariable(
        &mut self,
        variable: *mut crate::System::Linq::Expressions::ParameterExpression,
        isVoid: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CompileSetVariable", (variable, isVoid))?;
        Ok(__cordl_ret)
    }
    pub fn CompileNotExpression(
        &mut self,
        node: *mut crate::System::Linq::Expressions::UnaryExpression,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CompileNotExpression", (node))?;
        Ok(__cordl_ret)
    }
    pub fn CompileUnaryExpression(
        &mut self,
        expr: *mut crate::System::Linq::Expressions::Expression,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CompileUnaryExpression", (expr))?;
        Ok(__cordl_ret)
    }
    pub fn TryPushLabelBlock(
        &mut self,
        node: *mut crate::System::Linq::Expressions::Expression,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("TryPushLabelBlock", (node))?;
        Ok(__cordl_ret)
    }
    pub fn CompileVariableAssignment(
        &mut self,
        node: *mut crate::System::Linq::Expressions::BinaryExpression,
        asVoid: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CompileVariableAssignment", (node, asVoid))?;
        Ok(__cordl_ret)
    }
    pub fn MaybeMutableValueType(
        &mut self,
        _cordl_type: *mut crate::System::Type,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("MaybeMutableValueType", (_cordl_type))?;
        Ok(__cordl_ret)
    }
    pub fn CompileEqual(
        &mut self,
        left: *mut crate::System::Linq::Expressions::Expression,
        right: *mut crate::System::Linq::Expressions::Expression,
        liftedToNull: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CompileEqual", (left, right, liftedToNull))?;
        Ok(__cordl_ret)
    }
    pub fn CompileBlockEnd(
        &mut self,
        locals: *mut quest_hook::libil2cpp::Il2CppArray<
            crate::System::Linq::Expressions::Interpreter::LocalDefinition,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CompileBlockEnd", (locals))?;
        Ok(__cordl_ret)
    }
    pub fn CompileAddress(
        &mut self,
        node: *mut crate::System::Linq::Expressions::Expression,
        index: i32,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Linq::Expressions::Interpreter::ByRefUpdater,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Linq::Expressions::Interpreter::ByRefUpdater = __cordl_object
            .invoke("CompileAddress", (node, index))?;
        Ok(__cordl_ret)
    }
    pub fn CompileNewArrayExpression(
        &mut self,
        expr: *mut crate::System::Linq::Expressions::Expression,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CompileNewArrayExpression", (expr))?;
        Ok(__cordl_ret)
    }
    pub fn CompileListInitExpression(
        &mut self,
        expr: *mut crate::System::Linq::Expressions::Expression,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CompileListInitExpression", (expr))?;
        Ok(__cordl_ret)
    }
    pub fn CompileUnboxUnaryExpression(
        &mut self,
        expr: *mut crate::System::Linq::Expressions::Expression,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CompileUnboxUnaryExpression", (expr))?;
        Ok(__cordl_ret)
    }
    pub fn DefineLabel(
        &mut self,
        node: *mut crate::System::Linq::Expressions::LabelTarget,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Linq::Expressions::Interpreter::LabelInfo,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Linq::Expressions::Interpreter::LabelInfo = __cordl_object
            .invoke("DefineLabel", (node))?;
        Ok(__cordl_ret)
    }
    pub fn CompileAndAlsoBinaryExpression(
        &mut self,
        expr: *mut crate::System::Linq::Expressions::Expression,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CompileAndAlsoBinaryExpression", (expr))?;
        Ok(__cordl_ret)
    }
    pub fn CompileTypeAsExpression(
        &mut self,
        node: *mut crate::System::Linq::Expressions::UnaryExpression,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CompileTypeAsExpression", (node))?;
        Ok(__cordl_ret)
    }
    pub fn CompileLambdaExpression(
        &mut self,
        expr: *mut crate::System::Linq::Expressions::Expression,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CompileLambdaExpression", (expr))?;
        Ok(__cordl_ret)
    }
    pub fn EmitIndexGet(
        &mut self,
        index: *mut crate::System::Linq::Expressions::IndexExpression,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("EmitIndexGet", (index))?;
        Ok(__cordl_ret)
    }
    pub fn CompileOrElseBinaryExpression(
        &mut self,
        expr: *mut crate::System::Linq::Expressions::Expression,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CompileOrElseBinaryExpression", (expr))?;
        Ok(__cordl_ret)
    }
    pub fn ReferenceLabel(
        &mut self,
        node: *mut crate::System::Linq::Expressions::LabelTarget,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Linq::Expressions::Interpreter::LabelInfo,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Linq::Expressions::Interpreter::LabelInfo = __cordl_object
            .invoke("ReferenceLabel", (node))?;
        Ok(__cordl_ret)
    }
    pub fn CompileLoopExpression(
        &mut self,
        expr: *mut crate::System::Linq::Expressions::Expression,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CompileLoopExpression", (expr))?;
        Ok(__cordl_ret)
    }
    pub fn CompileStringSwitchExpression(
        &mut self,
        node: *mut crate::System::Linq::Expressions::SwitchExpression,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CompileStringSwitchExpression", (node))?;
        Ok(__cordl_ret)
    }
    pub fn CompileUnliftedLogicalBinaryExpression(
        &mut self,
        expr: *mut crate::System::Linq::Expressions::BinaryExpression,
        andAlso: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CompileUnliftedLogicalBinaryExpression", (expr, andAlso))?;
        Ok(__cordl_ret)
    }
    pub fn CompileConvertToType(
        &mut self,
        typeFrom: *mut crate::System::Type,
        typeTo: *mut crate::System::Type,
        isChecked: bool,
        isLiftedToNull: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "CompileConvertToType",
                (typeFrom, typeTo, isChecked, isLiftedToNull),
            )?;
        Ok(__cordl_ret)
    }
    pub fn CompileGetBoxedVariable(
        &mut self,
        variable: *mut crate::System::Linq::Expressions::ParameterExpression,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CompileGetBoxedVariable", (variable))?;
        Ok(__cordl_ret)
    }
    pub fn CompileTryFaultExpression(
        &mut self,
        expr: *mut crate::System::Linq::Expressions::TryExpression,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CompileTryFaultExpression", (expr))?;
        Ok(__cordl_ret)
    }
    pub fn CompileRuntimeVariablesExpression(
        &mut self,
        expr: *mut crate::System::Linq::Expressions::Expression,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CompileRuntimeVariablesExpression", (expr))?;
        Ok(__cordl_ret)
    }
    pub fn CompileQuoteUnaryExpression(
        &mut self,
        expr: *mut crate::System::Linq::Expressions::Expression,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CompileQuoteUnaryExpression", (expr))?;
        Ok(__cordl_ret)
    }
    pub fn CompileBlockExpression(
        &mut self,
        expr: *mut crate::System::Linq::Expressions::Expression,
        asVoid: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CompileBlockExpression", (expr, asVoid))?;
        Ok(__cordl_ret)
    }
    pub fn EmitUnaryBoolCheck(
        &mut self,
        node: *mut crate::System::Linq::Expressions::UnaryExpression,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("EmitUnaryBoolCheck", (node))?;
        Ok(__cordl_ret)
    }
    pub fn CompileThrowUnaryExpression(
        &mut self,
        expr: *mut crate::System::Linq::Expressions::Expression,
        asVoid: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CompileThrowUnaryExpression", (expr, asVoid))?;
        Ok(__cordl_ret)
    }
    pub fn CompileConvertUnaryExpression(
        &mut self,
        expr: *mut crate::System::Linq::Expressions::Expression,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CompileConvertUnaryExpression", (expr))?;
        Ok(__cordl_ret)
    }
    pub fn EmitThisForMethodCall(
        &mut self,
        node: *mut crate::System::Linq::Expressions::Expression,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("EmitThisForMethodCall", (node))?;
        Ok(__cordl_ret)
    }
    pub fn CompileDebugInfoExpression(
        &mut self,
        expr: *mut crate::System::Linq::Expressions::Expression,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CompileDebugInfoExpression", (expr))?;
        Ok(__cordl_ret)
    }
    pub fn CompileIntSwitchExpression<T>(
        &mut self,
        node: *mut crate::System::Linq::Expressions::SwitchExpression,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CompileIntSwitchExpression", (node))?;
        Ok(__cordl_ret)
    }
    pub fn CompileListInit(
        &mut self,
        initializers: *mut crate::System::Collections::ObjectModel::ReadOnlyCollection_1<
            *mut crate::System::Linq::Expressions::ElementInit,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CompileListInit", (initializers))?;
        Ok(__cordl_ret)
    }
    pub fn CompileMemberAssignment_BinaryExpression__cordl_bool0(
        &mut self,
        node: *mut crate::System::Linq::Expressions::BinaryExpression,
        asVoid: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CompileMemberAssignment", (node, asVoid))?;
        Ok(__cordl_ret)
    }
    pub fn CompileMemberAssignment__cordl_bool_MemberInfo_Expression__cordl_bool1(
        &mut self,
        asVoid: bool,
        refMember: *mut crate::System::Reflection::MemberInfo,
        value: *mut crate::System::Linq::Expressions::Expression,
        forBinding: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CompileMemberAssignment", (asVoid, refMember, value, forBinding))?;
        Ok(__cordl_ret)
    }
    pub fn CompileParameterExpression(
        &mut self,
        expr: *mut crate::System::Linq::Expressions::Expression,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CompileParameterExpression", (expr))?;
        Ok(__cordl_ret)
    }
    pub fn CompileAssignBinaryExpression(
        &mut self,
        expr: *mut crate::System::Linq::Expressions::Expression,
        asVoid: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CompileAssignBinaryExpression", (expr, asVoid))?;
        Ok(__cordl_ret)
    }
    pub fn CompileMemberInit(
        &mut self,
        bindings: *mut crate::System::Collections::ObjectModel::ReadOnlyCollection_1<
            *mut crate::System::Linq::Expressions::MemberBinding,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CompileMemberInit", (bindings))?;
        Ok(__cordl_ret)
    }
    pub fn MakeInterpreter(
        &mut self,
        lambdaName: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Linq::Expressions::Interpreter::Interpreter,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Linq::Expressions::Interpreter::Interpreter = __cordl_object
            .invoke("MakeInterpreter", (lambdaName))?;
        Ok(__cordl_ret)
    }
    pub fn CompileBlockStart(
        &mut self,
        node: *mut crate::System::Linq::Expressions::BlockExpression,
    ) -> quest_hook::libil2cpp::Result<
        *mut quest_hook::libil2cpp::Il2CppArray<
            crate::System::Linq::Expressions::Interpreter::LocalDefinition,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<
            crate::System::Linq::Expressions::Interpreter::LocalDefinition,
        > = __cordl_object.invoke("CompileBlockStart", (node))?;
        Ok(__cordl_ret)
    }
    pub fn CompileInvocationExpression(
        &mut self,
        expr: *mut crate::System::Linq::Expressions::Expression,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CompileInvocationExpression", (expr))?;
        Ok(__cordl_ret)
    }
    pub fn CompileNotEqual(
        &mut self,
        left: *mut crate::System::Linq::Expressions::Expression,
        right: *mut crate::System::Linq::Expressions::Expression,
        liftedToNull: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CompileNotEqual", (left, right, liftedToNull))?;
        Ok(__cordl_ret)
    }
    pub fn New_0() -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn New_LightCompiler1(
        parent: *mut crate::System::Linq::Expressions::Interpreter::LightCompiler,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (parent))?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "System+Linq+Expressions+Interpreter+LightCompiler")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Linq::Expressions::Interpreter::LightCompiler {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "System+Linq+Expressions+Interpreter+LightCompiler+QuoteVisitor")]
#[repr(C)]
#[derive(Debug)]
pub struct LightCompiler_QuoteVisitor {
    __cordl_parent: crate::System::Linq::Expressions::ExpressionVisitor,
    pub _definedParameters: *mut crate::System::Collections::Generic::Dictionary_2<
        *mut crate::System::Linq::Expressions::ParameterExpression,
        i32,
    >,
    pub _hoistedParameters: *mut crate::System::Collections::Generic::HashSet_1<
        *mut crate::System::Linq::Expressions::ParameterExpression,
    >,
}
#[cfg(feature = "System+Linq+Expressions+Interpreter+LightCompiler+QuoteVisitor")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::Linq::Expressions::Interpreter::LightCompiler_QuoteVisitor =>
    "System.Linq.Expressions.Interpreter"."LightCompiler/QuoteVisitor"
);
#[cfg(feature = "System+Linq+Expressions+Interpreter+LightCompiler+QuoteVisitor")]
impl std::ops::Deref
for crate::System::Linq::Expressions::Interpreter::LightCompiler_QuoteVisitor {
    type Target = crate::System::Linq::Expressions::ExpressionVisitor;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Linq+Expressions+Interpreter+LightCompiler+QuoteVisitor")]
impl std::ops::DerefMut
for crate::System::Linq::Expressions::Interpreter::LightCompiler_QuoteVisitor {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Linq+Expressions+Interpreter+LightCompiler+QuoteVisitor")]
impl crate::System::Linq::Expressions::Interpreter::LightCompiler_QuoteVisitor {
    pub fn PushParameters(
        &mut self,
        parameters: *mut crate::System::Collections::Generic::IEnumerable_1<
            *mut crate::System::Linq::Expressions::ParameterExpression,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("PushParameters", (parameters))?;
        Ok(__cordl_ret)
    }
    pub fn VisitBlock(
        &mut self,
        node: *mut crate::System::Linq::Expressions::BlockExpression,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Linq::Expressions::Expression,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Linq::Expressions::Expression = __cordl_object
            .invoke("VisitBlock", (node))?;
        Ok(__cordl_ret)
    }
    pub fn VisitLambda<T>(
        &mut self,
        node: *mut crate::System::Linq::Expressions::Expression_1<T>,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Linq::Expressions::Expression>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Linq::Expressions::Expression = __cordl_object
            .invoke("VisitLambda", (node))?;
        Ok(__cordl_ret)
    }
    pub fn VisitParameter(
        &mut self,
        node: *mut crate::System::Linq::Expressions::ParameterExpression,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Linq::Expressions::Expression,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Linq::Expressions::Expression = __cordl_object
            .invoke("VisitParameter", (node))?;
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
    pub fn PopParameters(
        &mut self,
        parameters: *mut crate::System::Collections::Generic::IEnumerable_1<
            *mut crate::System::Linq::Expressions::ParameterExpression,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("PopParameters", (parameters))?;
        Ok(__cordl_ret)
    }
    pub fn VisitCatchBlock(
        &mut self,
        node: *mut crate::System::Linq::Expressions::CatchBlock,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Linq::Expressions::CatchBlock,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Linq::Expressions::CatchBlock = __cordl_object
            .invoke("VisitCatchBlock", (node))?;
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
#[cfg(feature = "System+Linq+Expressions+Interpreter+LightCompiler+QuoteVisitor")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Linq::Expressions::Interpreter::LightCompiler_QuoteVisitor {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
