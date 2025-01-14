#[cfg(feature = "System+Linq+Expressions+Interpreter+LightCompiler")]
#[repr(C)]
#[derive(Debug)]
pub struct LightCompiler {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _instructions: quest_hook::libil2cpp::Gc<
        crate::System::Linq::Expressions::Interpreter::InstructionList,
    >,
    pub _locals: quest_hook::libil2cpp::Gc<
        crate::System::Linq::Expressions::Interpreter::LocalVariables,
    >,
    pub _debugInfos: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::List_1<
            quest_hook::libil2cpp::Gc<
                crate::System::Linq::Expressions::Interpreter::DebugInfo,
            >,
        >,
    >,
    pub _treeLabels: quest_hook::libil2cpp::Gc<
        crate::System::Linq::Expressions::Interpreter::HybridReferenceDictionary_2<
            quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::LabelTarget>,
            quest_hook::libil2cpp::Gc<
                crate::System::Linq::Expressions::Interpreter::LabelInfo,
            >,
        >,
    >,
    pub _labelBlock: quest_hook::libil2cpp::Gc<
        crate::System::Linq::Expressions::Interpreter::LabelScopeInfo,
    >,
    pub _exceptionForRethrowStack: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::Stack_1<
            quest_hook::libil2cpp::Gc<
                crate::System::Linq::Expressions::ParameterExpression,
            >,
        >,
    >,
    pub _parent: quest_hook::libil2cpp::Gc<
        crate::System::Linq::Expressions::Interpreter::LightCompiler,
    >,
    pub _guard: quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::StackGuard>,
}
#[cfg(feature = "System+Linq+Expressions+Interpreter+LightCompiler")]
unsafe impl quest_hook::libil2cpp::Type
for crate::System::Linq::Expressions::Interpreter::LightCompiler {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "System.Linq.Expressions.Interpreter";
    const CLASS_NAME: &'static str = "LightCompiler";
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
#[cfg(feature = "System+Linq+Expressions+Interpreter+LightCompiler")]
impl std::ops::Deref for crate::System::Linq::Expressions::Interpreter::LightCompiler {
    type Target = quest_hook::libil2cpp::Il2CppObject;
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
    pub fn CheckRethrow(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("CheckRethrow")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "CheckRethrow", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())
        };
        Ok(__cordl_ret.into())
    }
    pub fn CompileAddress(
        &mut self,
        node: quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::Expression>,
        index: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Linq::Expressions::Interpreter::ByRefUpdater,
        >,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        crate::System::Linq::Expressions::Expression,
                    >,
                    i32,
                ),
                quest_hook::libil2cpp::Gc<
                    crate::System::Linq::Expressions::Interpreter::ByRefUpdater,
                >,
                2usize,
            >("CompileAddress")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "CompileAddress", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Linq::Expressions::Interpreter::ByRefUpdater,
        > = unsafe { method.invoke_unchecked(self, (node, index)) };
        Ok(__cordl_ret.into())
    }
    pub fn CompileAndAlsoBinaryExpression(
        &mut self,
        expr: quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::Expression>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::System::Linq::Expressions::Expression,
                >),
                quest_hook::libil2cpp::Void,
                1usize,
            >("CompileAndAlsoBinaryExpression")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "CompileAndAlsoBinaryExpression", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (expr))
        };
        Ok(__cordl_ret.into())
    }
    pub fn CompileArithmetic(
        &mut self,
        nodeType: crate::System::Linq::Expressions::ExpressionType,
        left: quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::Expression>,
        right: quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::Expression>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    crate::System::Linq::Expressions::ExpressionType,
                    quest_hook::libil2cpp::Gc<
                        crate::System::Linq::Expressions::Expression,
                    >,
                    quest_hook::libil2cpp::Gc<
                        crate::System::Linq::Expressions::Expression,
                    >,
                ),
                quest_hook::libil2cpp::Void,
                3usize,
            >("CompileArithmetic")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "CompileArithmetic", 3usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (nodeType, left, right))
        };
        Ok(__cordl_ret.into())
    }
    pub fn CompileArrayIndexAddress(
        &mut self,
        array: quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::Expression>,
        index: quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::Expression>,
        argumentIndex: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Linq::Expressions::Interpreter::ByRefUpdater,
        >,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        crate::System::Linq::Expressions::Expression,
                    >,
                    quest_hook::libil2cpp::Gc<
                        crate::System::Linq::Expressions::Expression,
                    >,
                    i32,
                ),
                quest_hook::libil2cpp::Gc<
                    crate::System::Linq::Expressions::Interpreter::ByRefUpdater,
                >,
                3usize,
            >("CompileArrayIndexAddress")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "CompileArrayIndexAddress", 3usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Linq::Expressions::Interpreter::ByRefUpdater,
        > = unsafe { method.invoke_unchecked(self, (array, index, argumentIndex)) };
        Ok(__cordl_ret.into())
    }
    pub fn CompileAsVoid(
        &mut self,
        expr: quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::Expression>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::System::Linq::Expressions::Expression,
                >),
                quest_hook::libil2cpp::Void,
                1usize,
            >("CompileAsVoid")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "CompileAsVoid", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (expr))
        };
        Ok(__cordl_ret.into())
    }
    pub fn CompileAssignBinaryExpression(
        &mut self,
        expr: quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::Expression>,
        asVoid: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        crate::System::Linq::Expressions::Expression,
                    >,
                    bool,
                ),
                quest_hook::libil2cpp::Void,
                2usize,
            >("CompileAssignBinaryExpression")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "CompileAssignBinaryExpression", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (expr, asVoid))
        };
        Ok(__cordl_ret.into())
    }
    pub fn CompileBinaryExpression(
        &mut self,
        expr: quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::Expression>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::System::Linq::Expressions::Expression,
                >),
                quest_hook::libil2cpp::Void,
                1usize,
            >("CompileBinaryExpression")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "CompileBinaryExpression", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (expr))
        };
        Ok(__cordl_ret.into())
    }
    pub fn CompileBlockEnd(
        &mut self,
        locals: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                crate::System::Linq::Expressions::Interpreter::LocalDefinition,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<
                    quest_hook::libil2cpp::Il2CppArray<
                        crate::System::Linq::Expressions::Interpreter::LocalDefinition,
                    >,
                >),
                quest_hook::libil2cpp::Void,
                1usize,
            >("CompileBlockEnd")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "CompileBlockEnd", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (locals))
        };
        Ok(__cordl_ret.into())
    }
    pub fn CompileBlockExpression(
        &mut self,
        expr: quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::Expression>,
        asVoid: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        crate::System::Linq::Expressions::Expression,
                    >,
                    bool,
                ),
                quest_hook::libil2cpp::Void,
                2usize,
            >("CompileBlockExpression")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "CompileBlockExpression", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (expr, asVoid))
        };
        Ok(__cordl_ret.into())
    }
    pub fn CompileBlockStart(
        &mut self,
        node: quest_hook::libil2cpp::Gc<
            crate::System::Linq::Expressions::BlockExpression,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                crate::System::Linq::Expressions::Interpreter::LocalDefinition,
            >,
        >,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::System::Linq::Expressions::BlockExpression,
                >),
                quest_hook::libil2cpp::Gc<
                    quest_hook::libil2cpp::Il2CppArray<
                        crate::System::Linq::Expressions::Interpreter::LocalDefinition,
                    >,
                >,
                1usize,
            >("CompileBlockStart")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "CompileBlockStart", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                crate::System::Linq::Expressions::Interpreter::LocalDefinition,
            >,
        > = unsafe { method.invoke_unchecked(self, (node)) };
        Ok(__cordl_ret.into())
    }
    pub fn CompileCoalesceBinaryExpression(
        &mut self,
        expr: quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::Expression>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::System::Linq::Expressions::Expression,
                >),
                quest_hook::libil2cpp::Void,
                1usize,
            >("CompileCoalesceBinaryExpression")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "CompileCoalesceBinaryExpression", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (expr))
        };
        Ok(__cordl_ret.into())
    }
    pub fn CompileComparison(
        &mut self,
        node: quest_hook::libil2cpp::Gc<
            crate::System::Linq::Expressions::BinaryExpression,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::System::Linq::Expressions::BinaryExpression,
                >),
                quest_hook::libil2cpp::Void,
                1usize,
            >("CompileComparison")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "CompileComparison", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (node))
        };
        Ok(__cordl_ret.into())
    }
    pub fn CompileConditionalExpression(
        &mut self,
        expr: quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::Expression>,
        asVoid: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        crate::System::Linq::Expressions::Expression,
                    >,
                    bool,
                ),
                quest_hook::libil2cpp::Void,
                2usize,
            >("CompileConditionalExpression")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "CompileConditionalExpression", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (expr, asVoid))
        };
        Ok(__cordl_ret.into())
    }
    pub fn CompileConstantExpression(
        &mut self,
        expr: quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::Expression>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::System::Linq::Expressions::Expression,
                >),
                quest_hook::libil2cpp::Void,
                1usize,
            >("CompileConstantExpression")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "CompileConstantExpression", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (expr))
        };
        Ok(__cordl_ret.into())
    }
    pub fn CompileConvertToType(
        &mut self,
        typeFrom: quest_hook::libil2cpp::Gc<crate::System::Type>,
        typeTo: quest_hook::libil2cpp::Gc<crate::System::Type>,
        isChecked: bool,
        isLiftedToNull: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<crate::System::Type>,
                    quest_hook::libil2cpp::Gc<crate::System::Type>,
                    bool,
                    bool,
                ),
                quest_hook::libil2cpp::Void,
                4usize,
            >("CompileConvertToType")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "CompileConvertToType", 4usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (typeFrom, typeTo, isChecked, isLiftedToNull))
        };
        Ok(__cordl_ret.into())
    }
    pub fn CompileConvertUnaryExpression(
        &mut self,
        expr: quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::Expression>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::System::Linq::Expressions::Expression,
                >),
                quest_hook::libil2cpp::Void,
                1usize,
            >("CompileConvertUnaryExpression")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "CompileConvertUnaryExpression", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (expr))
        };
        Ok(__cordl_ret.into())
    }
    pub fn CompileDebugInfoExpression(
        &mut self,
        expr: quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::Expression>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::System::Linq::Expressions::Expression,
                >),
                quest_hook::libil2cpp::Void,
                1usize,
            >("CompileDebugInfoExpression")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "CompileDebugInfoExpression", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (expr))
        };
        Ok(__cordl_ret.into())
    }
    pub fn CompileDefaultExpression_Expression0(
        &mut self,
        expr: quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::Expression>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::System::Linq::Expressions::Expression,
                >),
                quest_hook::libil2cpp::Void,
                1usize,
            >("CompileDefaultExpression")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "CompileDefaultExpression", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (expr))
        };
        Ok(__cordl_ret.into())
    }
    pub fn CompileDefaultExpression_Type1(
        &mut self,
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<crate::System::Type>),
                quest_hook::libil2cpp::Void,
                1usize,
            >("CompileDefaultExpression")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "CompileDefaultExpression", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (_cordl_type))
        };
        Ok(__cordl_ret.into())
    }
    pub fn CompileEqual(
        &mut self,
        left: quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::Expression>,
        right: quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::Expression>,
        liftedToNull: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        crate::System::Linq::Expressions::Expression,
                    >,
                    quest_hook::libil2cpp::Gc<
                        crate::System::Linq::Expressions::Expression,
                    >,
                    bool,
                ),
                quest_hook::libil2cpp::Void,
                3usize,
            >("CompileEqual")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "CompileEqual", 3usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (left, right, liftedToNull))
        };
        Ok(__cordl_ret.into())
    }
    pub fn CompileGetBoxedVariable(
        &mut self,
        variable: quest_hook::libil2cpp::Gc<
            crate::System::Linq::Expressions::ParameterExpression,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::System::Linq::Expressions::ParameterExpression,
                >),
                quest_hook::libil2cpp::Void,
                1usize,
            >("CompileGetBoxedVariable")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "CompileGetBoxedVariable", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (variable))
        };
        Ok(__cordl_ret.into())
    }
    pub fn CompileGetVariable(
        &mut self,
        variable: quest_hook::libil2cpp::Gc<
            crate::System::Linq::Expressions::ParameterExpression,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::System::Linq::Expressions::ParameterExpression,
                >),
                quest_hook::libil2cpp::Void,
                1usize,
            >("CompileGetVariable")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "CompileGetVariable", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (variable))
        };
        Ok(__cordl_ret.into())
    }
    pub fn CompileGotoExpression(
        &mut self,
        expr: quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::Expression>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::System::Linq::Expressions::Expression,
                >),
                quest_hook::libil2cpp::Void,
                1usize,
            >("CompileGotoExpression")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "CompileGotoExpression", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (expr))
        };
        Ok(__cordl_ret.into())
    }
    pub fn CompileIndexAssignment(
        &mut self,
        node: quest_hook::libil2cpp::Gc<
            crate::System::Linq::Expressions::BinaryExpression,
        >,
        asVoid: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        crate::System::Linq::Expressions::BinaryExpression,
                    >,
                    bool,
                ),
                quest_hook::libil2cpp::Void,
                2usize,
            >("CompileIndexAssignment")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "CompileIndexAssignment", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (node, asVoid))
        };
        Ok(__cordl_ret.into())
    }
    pub fn CompileIndexExpression(
        &mut self,
        expr: quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::Expression>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::System::Linq::Expressions::Expression,
                >),
                quest_hook::libil2cpp::Void,
                1usize,
            >("CompileIndexExpression")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "CompileIndexExpression", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (expr))
        };
        Ok(__cordl_ret.into())
    }
    pub fn CompileIntSwitchExpression<T>(
        &mut self,
        node: quest_hook::libil2cpp::Gc<
            crate::System::Linq::Expressions::SwitchExpression,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::System::Linq::Expressions::SwitchExpression,
                >),
                quest_hook::libil2cpp::Void,
                1usize,
            >("CompileIntSwitchExpression")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "CompileIntSwitchExpression", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (node))
        };
        Ok(__cordl_ret.into())
    }
    pub fn CompileInvocationExpression(
        &mut self,
        expr: quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::Expression>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::System::Linq::Expressions::Expression,
                >),
                quest_hook::libil2cpp::Void,
                1usize,
            >("CompileInvocationExpression")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "CompileInvocationExpression", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (expr))
        };
        Ok(__cordl_ret.into())
    }
    pub fn CompileLabelExpression(
        &mut self,
        expr: quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::Expression>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::System::Linq::Expressions::Expression,
                >),
                quest_hook::libil2cpp::Void,
                1usize,
            >("CompileLabelExpression")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "CompileLabelExpression", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (expr))
        };
        Ok(__cordl_ret.into())
    }
    pub fn CompileLambdaExpression(
        &mut self,
        expr: quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::Expression>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::System::Linq::Expressions::Expression,
                >),
                quest_hook::libil2cpp::Void,
                1usize,
            >("CompileLambdaExpression")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "CompileLambdaExpression", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (expr))
        };
        Ok(__cordl_ret.into())
    }
    pub fn CompileLiftedLogicalBinaryExpression(
        &mut self,
        node: quest_hook::libil2cpp::Gc<
            crate::System::Linq::Expressions::BinaryExpression,
        >,
        andAlso: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        crate::System::Linq::Expressions::BinaryExpression,
                    >,
                    bool,
                ),
                quest_hook::libil2cpp::Void,
                2usize,
            >("CompileLiftedLogicalBinaryExpression")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "CompileLiftedLogicalBinaryExpression", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (node, andAlso))
        };
        Ok(__cordl_ret.into())
    }
    pub fn CompileListInit(
        &mut self,
        initializers: quest_hook::libil2cpp::Gc<
            crate::System::Collections::ObjectModel::ReadOnlyCollection_1<
                quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::ElementInit>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::System::Collections::ObjectModel::ReadOnlyCollection_1<
                        quest_hook::libil2cpp::Gc<
                            crate::System::Linq::Expressions::ElementInit,
                        >,
                    >,
                >),
                quest_hook::libil2cpp::Void,
                1usize,
            >("CompileListInit")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "CompileListInit", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (initializers))
        };
        Ok(__cordl_ret.into())
    }
    pub fn CompileListInitExpression(
        &mut self,
        expr: quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::Expression>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::System::Linq::Expressions::Expression,
                >),
                quest_hook::libil2cpp::Void,
                1usize,
            >("CompileListInitExpression")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "CompileListInitExpression", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (expr))
        };
        Ok(__cordl_ret.into())
    }
    pub fn CompileLogicalBinaryExpression(
        &mut self,
        b: quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::BinaryExpression>,
        andAlso: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        crate::System::Linq::Expressions::BinaryExpression,
                    >,
                    bool,
                ),
                quest_hook::libil2cpp::Void,
                2usize,
            >("CompileLogicalBinaryExpression")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "CompileLogicalBinaryExpression", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (b, andAlso))
        };
        Ok(__cordl_ret.into())
    }
    pub fn CompileLoopExpression(
        &mut self,
        expr: quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::Expression>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::System::Linq::Expressions::Expression,
                >),
                quest_hook::libil2cpp::Void,
                1usize,
            >("CompileLoopExpression")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "CompileLoopExpression", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (expr))
        };
        Ok(__cordl_ret.into())
    }
    pub fn CompileMember(
        &mut self,
        from: quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::Expression>,
        member: quest_hook::libil2cpp::Gc<crate::System::Reflection::MemberInfo>,
        forBinding: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        crate::System::Linq::Expressions::Expression,
                    >,
                    quest_hook::libil2cpp::Gc<crate::System::Reflection::MemberInfo>,
                    bool,
                ),
                quest_hook::libil2cpp::Void,
                3usize,
            >("CompileMember")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "CompileMember", 3usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (from, member, forBinding))
        };
        Ok(__cordl_ret.into())
    }
    pub fn CompileMemberAssignment_BinaryExpression__cordl_bool0(
        &mut self,
        node: quest_hook::libil2cpp::Gc<
            crate::System::Linq::Expressions::BinaryExpression,
        >,
        asVoid: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        crate::System::Linq::Expressions::BinaryExpression,
                    >,
                    bool,
                ),
                quest_hook::libil2cpp::Void,
                2usize,
            >("CompileMemberAssignment")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "CompileMemberAssignment", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (node, asVoid))
        };
        Ok(__cordl_ret.into())
    }
    pub fn CompileMemberAssignment__cordl_bool_MemberInfo_Expression__cordl_bool1(
        &mut self,
        asVoid: bool,
        refMember: quest_hook::libil2cpp::Gc<crate::System::Reflection::MemberInfo>,
        value: quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::Expression>,
        forBinding: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    bool,
                    quest_hook::libil2cpp::Gc<crate::System::Reflection::MemberInfo>,
                    quest_hook::libil2cpp::Gc<
                        crate::System::Linq::Expressions::Expression,
                    >,
                    bool,
                ),
                quest_hook::libil2cpp::Void,
                4usize,
            >("CompileMemberAssignment")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "CompileMemberAssignment", 4usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (asVoid, refMember, value, forBinding))
        };
        Ok(__cordl_ret.into())
    }
    pub fn CompileMemberExpression(
        &mut self,
        expr: quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::Expression>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::System::Linq::Expressions::Expression,
                >),
                quest_hook::libil2cpp::Void,
                1usize,
            >("CompileMemberExpression")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "CompileMemberExpression", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (expr))
        };
        Ok(__cordl_ret.into())
    }
    pub fn CompileMemberInit(
        &mut self,
        bindings: quest_hook::libil2cpp::Gc<
            crate::System::Collections::ObjectModel::ReadOnlyCollection_1<
                quest_hook::libil2cpp::Gc<
                    crate::System::Linq::Expressions::MemberBinding,
                >,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::System::Collections::ObjectModel::ReadOnlyCollection_1<
                        quest_hook::libil2cpp::Gc<
                            crate::System::Linq::Expressions::MemberBinding,
                        >,
                    >,
                >),
                quest_hook::libil2cpp::Void,
                1usize,
            >("CompileMemberInit")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "CompileMemberInit", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (bindings))
        };
        Ok(__cordl_ret.into())
    }
    pub fn CompileMemberInitExpression(
        &mut self,
        expr: quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::Expression>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::System::Linq::Expressions::Expression,
                >),
                quest_hook::libil2cpp::Void,
                1usize,
            >("CompileMemberInitExpression")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "CompileMemberInitExpression", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (expr))
        };
        Ok(__cordl_ret.into())
    }
    pub fn CompileMethodCallExpression_Expression0(
        &mut self,
        expr: quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::Expression>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::System::Linq::Expressions::Expression,
                >),
                quest_hook::libil2cpp::Void,
                1usize,
            >("CompileMethodCallExpression")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "CompileMethodCallExpression", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (expr))
        };
        Ok(__cordl_ret.into())
    }
    pub fn CompileMethodCallExpression_MethodInfo_IArgumentProvider1(
        &mut self,
        object: quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::Expression>,
        method: quest_hook::libil2cpp::Gc<crate::System::Reflection::MethodInfo>,
        arguments: quest_hook::libil2cpp::Gc<
            crate::System::Linq::Expressions::IArgumentProvider,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        crate::System::Linq::Expressions::Expression,
                    >,
                    quest_hook::libil2cpp::Gc<crate::System::Reflection::MethodInfo>,
                    quest_hook::libil2cpp::Gc<
                        crate::System::Linq::Expressions::IArgumentProvider,
                    >,
                ),
                quest_hook::libil2cpp::Void,
                3usize,
            >("CompileMethodCallExpression")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "CompileMethodCallExpression", 3usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (object, method, arguments))
        };
        Ok(__cordl_ret.into())
    }
    pub fn CompileMethodLogicalBinaryExpression(
        &mut self,
        expr: quest_hook::libil2cpp::Gc<
            crate::System::Linq::Expressions::BinaryExpression,
        >,
        andAlso: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        crate::System::Linq::Expressions::BinaryExpression,
                    >,
                    bool,
                ),
                quest_hook::libil2cpp::Void,
                2usize,
            >("CompileMethodLogicalBinaryExpression")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "CompileMethodLogicalBinaryExpression", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (expr, andAlso))
        };
        Ok(__cordl_ret.into())
    }
    pub fn CompileMultiDimArrayAccess(
        &mut self,
        array: quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::Expression>,
        arguments: quest_hook::libil2cpp::Gc<
            crate::System::Linq::Expressions::IArgumentProvider,
        >,
        index: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Linq::Expressions::Interpreter::ByRefUpdater,
        >,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        crate::System::Linq::Expressions::Expression,
                    >,
                    quest_hook::libil2cpp::Gc<
                        crate::System::Linq::Expressions::IArgumentProvider,
                    >,
                    i32,
                ),
                quest_hook::libil2cpp::Gc<
                    crate::System::Linq::Expressions::Interpreter::ByRefUpdater,
                >,
                3usize,
            >("CompileMultiDimArrayAccess")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "CompileMultiDimArrayAccess", 3usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Linq::Expressions::Interpreter::ByRefUpdater,
        > = unsafe { method.invoke_unchecked(self, (array, arguments, index)) };
        Ok(__cordl_ret.into())
    }
    pub fn CompileNewArrayExpression(
        &mut self,
        expr: quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::Expression>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::System::Linq::Expressions::Expression,
                >),
                quest_hook::libil2cpp::Void,
                1usize,
            >("CompileNewArrayExpression")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "CompileNewArrayExpression", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (expr))
        };
        Ok(__cordl_ret.into())
    }
    pub fn CompileNewExpression(
        &mut self,
        expr: quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::Expression>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::System::Linq::Expressions::Expression,
                >),
                quest_hook::libil2cpp::Void,
                1usize,
            >("CompileNewExpression")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "CompileNewExpression", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (expr))
        };
        Ok(__cordl_ret.into())
    }
    pub fn CompileNoLabelPush(
        &mut self,
        expr: quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::Expression>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::System::Linq::Expressions::Expression,
                >),
                quest_hook::libil2cpp::Void,
                1usize,
            >("CompileNoLabelPush")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "CompileNoLabelPush", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (expr))
        };
        Ok(__cordl_ret.into())
    }
    pub fn CompileNotEqual(
        &mut self,
        left: quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::Expression>,
        right: quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::Expression>,
        liftedToNull: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        crate::System::Linq::Expressions::Expression,
                    >,
                    quest_hook::libil2cpp::Gc<
                        crate::System::Linq::Expressions::Expression,
                    >,
                    bool,
                ),
                quest_hook::libil2cpp::Void,
                3usize,
            >("CompileNotEqual")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "CompileNotEqual", 3usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (left, right, liftedToNull))
        };
        Ok(__cordl_ret.into())
    }
    pub fn CompileNotExpression(
        &mut self,
        node: quest_hook::libil2cpp::Gc<
            crate::System::Linq::Expressions::UnaryExpression,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::System::Linq::Expressions::UnaryExpression,
                >),
                quest_hook::libil2cpp::Void,
                1usize,
            >("CompileNotExpression")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "CompileNotExpression", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (node))
        };
        Ok(__cordl_ret.into())
    }
    pub fn CompileOrElseBinaryExpression(
        &mut self,
        expr: quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::Expression>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::System::Linq::Expressions::Expression,
                >),
                quest_hook::libil2cpp::Void,
                1usize,
            >("CompileOrElseBinaryExpression")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "CompileOrElseBinaryExpression", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (expr))
        };
        Ok(__cordl_ret.into())
    }
    pub fn CompileParameterExpression(
        &mut self,
        expr: quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::Expression>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::System::Linq::Expressions::Expression,
                >),
                quest_hook::libil2cpp::Void,
                1usize,
            >("CompileParameterExpression")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "CompileParameterExpression", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (expr))
        };
        Ok(__cordl_ret.into())
    }
    pub fn CompileQuoteUnaryExpression(
        &mut self,
        expr: quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::Expression>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::System::Linq::Expressions::Expression,
                >),
                quest_hook::libil2cpp::Void,
                1usize,
            >("CompileQuoteUnaryExpression")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "CompileQuoteUnaryExpression", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (expr))
        };
        Ok(__cordl_ret.into())
    }
    pub fn CompileRuntimeVariablesExpression(
        &mut self,
        expr: quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::Expression>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::System::Linq::Expressions::Expression,
                >),
                quest_hook::libil2cpp::Void,
                1usize,
            >("CompileRuntimeVariablesExpression")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "CompileRuntimeVariablesExpression", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (expr))
        };
        Ok(__cordl_ret.into())
    }
    pub fn CompileSetVariable(
        &mut self,
        variable: quest_hook::libil2cpp::Gc<
            crate::System::Linq::Expressions::ParameterExpression,
        >,
        isVoid: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        crate::System::Linq::Expressions::ParameterExpression,
                    >,
                    bool,
                ),
                quest_hook::libil2cpp::Void,
                2usize,
            >("CompileSetVariable")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "CompileSetVariable", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (variable, isVoid))
        };
        Ok(__cordl_ret.into())
    }
    pub fn CompileStringSwitchExpression(
        &mut self,
        node: quest_hook::libil2cpp::Gc<
            crate::System::Linq::Expressions::SwitchExpression,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::System::Linq::Expressions::SwitchExpression,
                >),
                quest_hook::libil2cpp::Void,
                1usize,
            >("CompileStringSwitchExpression")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "CompileStringSwitchExpression", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (node))
        };
        Ok(__cordl_ret.into())
    }
    pub fn CompileSwitchExpression(
        &mut self,
        expr: quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::Expression>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::System::Linq::Expressions::Expression,
                >),
                quest_hook::libil2cpp::Void,
                1usize,
            >("CompileSwitchExpression")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "CompileSwitchExpression", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (expr))
        };
        Ok(__cordl_ret.into())
    }
    pub fn CompileThrowUnaryExpression(
        &mut self,
        expr: quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::Expression>,
        asVoid: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        crate::System::Linq::Expressions::Expression,
                    >,
                    bool,
                ),
                quest_hook::libil2cpp::Void,
                2usize,
            >("CompileThrowUnaryExpression")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "CompileThrowUnaryExpression", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (expr, asVoid))
        };
        Ok(__cordl_ret.into())
    }
    pub fn CompileTop(
        &mut self,
        node: quest_hook::libil2cpp::Gc<
            crate::System::Linq::Expressions::LambdaExpression,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Linq::Expressions::Interpreter::LightDelegateCreator,
        >,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::System::Linq::Expressions::LambdaExpression,
                >),
                quest_hook::libil2cpp::Gc<
                    crate::System::Linq::Expressions::Interpreter::LightDelegateCreator,
                >,
                1usize,
            >("CompileTop")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "CompileTop", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Linq::Expressions::Interpreter::LightDelegateCreator,
        > = unsafe { method.invoke_unchecked(self, (node)) };
        Ok(__cordl_ret.into())
    }
    pub fn CompileTryExpression(
        &mut self,
        expr: quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::Expression>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::System::Linq::Expressions::Expression,
                >),
                quest_hook::libil2cpp::Void,
                1usize,
            >("CompileTryExpression")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "CompileTryExpression", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (expr))
        };
        Ok(__cordl_ret.into())
    }
    pub fn CompileTryFaultExpression(
        &mut self,
        expr: quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::TryExpression>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::System::Linq::Expressions::TryExpression,
                >),
                quest_hook::libil2cpp::Void,
                1usize,
            >("CompileTryFaultExpression")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "CompileTryFaultExpression", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (expr))
        };
        Ok(__cordl_ret.into())
    }
    pub fn CompileTypeAsExpression(
        &mut self,
        node: quest_hook::libil2cpp::Gc<
            crate::System::Linq::Expressions::UnaryExpression,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::System::Linq::Expressions::UnaryExpression,
                >),
                quest_hook::libil2cpp::Void,
                1usize,
            >("CompileTypeAsExpression")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "CompileTypeAsExpression", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (node))
        };
        Ok(__cordl_ret.into())
    }
    pub fn CompileTypeEqualExpression(
        &mut self,
        expr: quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::Expression>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::System::Linq::Expressions::Expression,
                >),
                quest_hook::libil2cpp::Void,
                1usize,
            >("CompileTypeEqualExpression")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "CompileTypeEqualExpression", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (expr))
        };
        Ok(__cordl_ret.into())
    }
    pub fn CompileTypeIsExpression(
        &mut self,
        expr: quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::Expression>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::System::Linq::Expressions::Expression,
                >),
                quest_hook::libil2cpp::Void,
                1usize,
            >("CompileTypeIsExpression")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "CompileTypeIsExpression", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (expr))
        };
        Ok(__cordl_ret.into())
    }
    pub fn CompileUnaryExpression(
        &mut self,
        expr: quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::Expression>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::System::Linq::Expressions::Expression,
                >),
                quest_hook::libil2cpp::Void,
                1usize,
            >("CompileUnaryExpression")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "CompileUnaryExpression", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (expr))
        };
        Ok(__cordl_ret.into())
    }
    pub fn CompileUnboxUnaryExpression(
        &mut self,
        expr: quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::Expression>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::System::Linq::Expressions::Expression,
                >),
                quest_hook::libil2cpp::Void,
                1usize,
            >("CompileUnboxUnaryExpression")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "CompileUnboxUnaryExpression", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (expr))
        };
        Ok(__cordl_ret.into())
    }
    pub fn CompileUnliftedLogicalBinaryExpression(
        &mut self,
        expr: quest_hook::libil2cpp::Gc<
            crate::System::Linq::Expressions::BinaryExpression,
        >,
        andAlso: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        crate::System::Linq::Expressions::BinaryExpression,
                    >,
                    bool,
                ),
                quest_hook::libil2cpp::Void,
                2usize,
            >("CompileUnliftedLogicalBinaryExpression")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "CompileUnliftedLogicalBinaryExpression", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (expr, andAlso))
        };
        Ok(__cordl_ret.into())
    }
    pub fn CompileVariableAssignment(
        &mut self,
        node: quest_hook::libil2cpp::Gc<
            crate::System::Linq::Expressions::BinaryExpression,
        >,
        asVoid: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        crate::System::Linq::Expressions::BinaryExpression,
                    >,
                    bool,
                ),
                quest_hook::libil2cpp::Void,
                2usize,
            >("CompileVariableAssignment")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "CompileVariableAssignment", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (node, asVoid))
        };
        Ok(__cordl_ret.into())
    }
    pub fn Compile_Expression1(
        &mut self,
        expr: quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::Expression>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::System::Linq::Expressions::Expression,
                >),
                quest_hook::libil2cpp::Void,
                1usize,
            >("Compile")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "Compile", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (expr))
        };
        Ok(__cordl_ret.into())
    }
    pub fn Compile__cordl_bool0(
        &mut self,
        expr: quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::Expression>,
        asVoid: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        crate::System::Linq::Expressions::Expression,
                    >,
                    bool,
                ),
                quest_hook::libil2cpp::Void,
                2usize,
            >("Compile")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "Compile", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (expr, asVoid))
        };
        Ok(__cordl_ret.into())
    }
    pub fn DefineBlockLabels(
        &mut self,
        node: quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::Expression>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::System::Linq::Expressions::Expression,
                >),
                quest_hook::libil2cpp::Void,
                1usize,
            >("DefineBlockLabels")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "DefineBlockLabels", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (node))
        };
        Ok(__cordl_ret.into())
    }
    pub fn DefineLabel(
        &mut self,
        node: quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::LabelTarget>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Linq::Expressions::Interpreter::LabelInfo,
        >,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::System::Linq::Expressions::LabelTarget,
                >),
                quest_hook::libil2cpp::Gc<
                    crate::System::Linq::Expressions::Interpreter::LabelInfo,
                >,
                1usize,
            >("DefineLabel")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "DefineLabel", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Linq::Expressions::Interpreter::LabelInfo,
        > = unsafe { method.invoke_unchecked(self, (node)) };
        Ok(__cordl_ret.into())
    }
    pub fn EmitCopyValueType(
        &mut self,
        valueType: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<crate::System::Type>),
                quest_hook::libil2cpp::Void,
                1usize,
            >("EmitCopyValueType")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "EmitCopyValueType", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (valueType))
        };
        Ok(__cordl_ret.into())
    }
    pub fn EmitIndexGet(
        &mut self,
        index: quest_hook::libil2cpp::Gc<
            crate::System::Linq::Expressions::IndexExpression,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::System::Linq::Expressions::IndexExpression,
                >),
                quest_hook::libil2cpp::Void,
                1usize,
            >("EmitIndexGet")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "EmitIndexGet", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (index))
        };
        Ok(__cordl_ret.into())
    }
    pub fn EmitThisForMethodCall(
        &mut self,
        node: quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::Expression>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::System::Linq::Expressions::Expression,
                >),
                quest_hook::libil2cpp::Void,
                1usize,
            >("EmitThisForMethodCall")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "EmitThisForMethodCall", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (node))
        };
        Ok(__cordl_ret.into())
    }
    pub fn EmitUnaryBoolCheck(
        &mut self,
        node: quest_hook::libil2cpp::Gc<
            crate::System::Linq::Expressions::UnaryExpression,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::System::Linq::Expressions::UnaryExpression,
                >),
                quest_hook::libil2cpp::Void,
                1usize,
            >("EmitUnaryBoolCheck")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "EmitUnaryBoolCheck", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (node))
        };
        Ok(__cordl_ret.into())
    }
    pub fn EmitUnaryMethodCall(
        &mut self,
        node: quest_hook::libil2cpp::Gc<
            crate::System::Linq::Expressions::UnaryExpression,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::System::Linq::Expressions::UnaryExpression,
                >),
                quest_hook::libil2cpp::Void,
                1usize,
            >("EmitUnaryMethodCall")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "EmitUnaryMethodCall", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (node))
        };
        Ok(__cordl_ret.into())
    }
    pub fn EnsureAvailableForClosure(
        &mut self,
        expr: quest_hook::libil2cpp::Gc<
            crate::System::Linq::Expressions::ParameterExpression,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Linq::Expressions::Interpreter::LocalVariable,
        >,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::System::Linq::Expressions::ParameterExpression,
                >),
                quest_hook::libil2cpp::Gc<
                    crate::System::Linq::Expressions::Interpreter::LocalVariable,
                >,
                1usize,
            >("EnsureAvailableForClosure")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "EnsureAvailableForClosure", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Linq::Expressions::Interpreter::LocalVariable,
        > = unsafe { method.invoke_unchecked(self, (expr)) };
        Ok(__cordl_ret.into())
    }
    pub fn EnsureLabel(
        &mut self,
        node: quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::LabelTarget>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Linq::Expressions::Interpreter::LabelInfo,
        >,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::System::Linq::Expressions::LabelTarget,
                >),
                quest_hook::libil2cpp::Gc<
                    crate::System::Linq::Expressions::Interpreter::LabelInfo,
                >,
                1usize,
            >("EnsureLabel")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "EnsureLabel", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Linq::Expressions::Interpreter::LabelInfo,
        > = unsafe { method.invoke_unchecked(self, (node)) };
        Ok(__cordl_ret.into())
    }
    pub fn GetMemberType(
        member: quest_hook::libil2cpp::Gc<crate::System::Reflection::MemberInfo>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<crate::System::Type>> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<crate::System::Reflection::MemberInfo>),
                quest_hook::libil2cpp::Gc<crate::System::Type>,
                1usize,
            >("GetMemberType")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "GetMemberType", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Type> = unsafe {
            method.invoke_unchecked((), (member))
        };
        Ok(__cordl_ret.into())
    }
    pub fn LoadLocalNoValueTypeCopy(
        &mut self,
        variable: quest_hook::libil2cpp::Gc<
            crate::System::Linq::Expressions::ParameterExpression,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::System::Linq::Expressions::ParameterExpression,
                >),
                quest_hook::libil2cpp::Void,
                1usize,
            >("LoadLocalNoValueTypeCopy")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "LoadLocalNoValueTypeCopy", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (variable))
        };
        Ok(__cordl_ret.into())
    }
    pub fn MakeInterpreter(
        &mut self,
        lambdaName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Linq::Expressions::Interpreter::Interpreter,
        >,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>),
                quest_hook::libil2cpp::Gc<
                    crate::System::Linq::Expressions::Interpreter::Interpreter,
                >,
                1usize,
            >("MakeInterpreter")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "MakeInterpreter", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Linq::Expressions::Interpreter::Interpreter,
        > = unsafe { method.invoke_unchecked(self, (lambdaName)) };
        Ok(__cordl_ret.into())
    }
    pub fn MaybeMutableValueType(
        &mut self,
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<crate::System::Type>),
                bool,
                1usize,
            >("MaybeMutableValueType")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "MaybeMutableValueType", 1usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked(self, (_cordl_type)) };
        Ok(__cordl_ret.into())
    }
    pub fn New_0() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn New_LightCompiler1(
        parent: quest_hook::libil2cpp::Gc<
            crate::System::Linq::Expressions::Interpreter::LightCompiler,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (parent))?;
        Ok(__cordl_object.into())
    }
    pub fn PopLabelBlock(
        &mut self,
        kind: crate::System::Linq::Expressions::Interpreter::LabelScopeKind,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (crate::System::Linq::Expressions::Interpreter::LabelScopeKind),
                quest_hook::libil2cpp::Void,
                1usize,
            >("PopLabelBlock")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "PopLabelBlock", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (kind))
        };
        Ok(__cordl_ret.into())
    }
    pub fn PushLabelBlock(
        &mut self,
        _cordl_type: crate::System::Linq::Expressions::Interpreter::LabelScopeKind,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (crate::System::Linq::Expressions::Interpreter::LabelScopeKind),
                quest_hook::libil2cpp::Void,
                1usize,
            >("PushLabelBlock")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "PushLabelBlock", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (_cordl_type))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ReferenceLabel(
        &mut self,
        node: quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::LabelTarget>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Linq::Expressions::Interpreter::LabelInfo,
        >,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::System::Linq::Expressions::LabelTarget,
                >),
                quest_hook::libil2cpp::Gc<
                    crate::System::Linq::Expressions::Interpreter::LabelInfo,
                >,
                1usize,
            >("ReferenceLabel")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ReferenceLabel", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Linq::Expressions::Interpreter::LabelInfo,
        > = unsafe { method.invoke_unchecked(self, (node)) };
        Ok(__cordl_ret.into())
    }
    pub fn ResolveLocal(
        &mut self,
        variable: quest_hook::libil2cpp::Gc<
            crate::System::Linq::Expressions::ParameterExpression,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Linq::Expressions::Interpreter::LocalVariable,
        >,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::System::Linq::Expressions::ParameterExpression,
                >),
                quest_hook::libil2cpp::Gc<
                    crate::System::Linq::Expressions::Interpreter::LocalVariable,
                >,
                1usize,
            >("ResolveLocal")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ResolveLocal", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Linq::Expressions::Interpreter::LocalVariable,
        > = unsafe { method.invoke_unchecked(self, (variable)) };
        Ok(__cordl_ret.into())
    }
    pub fn ShouldWritebackNode(
        node: quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::Expression>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::System::Linq::Expressions::Expression,
                >),
                bool,
                1usize,
            >("ShouldWritebackNode")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ShouldWritebackNode", 1usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked((), (node)) };
        Ok(__cordl_ret.into())
    }
    pub fn TryPushLabelBlock(
        &mut self,
        node: quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::Expression>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::System::Linq::Expressions::Expression,
                >),
                bool,
                1usize,
            >("TryPushLabelBlock")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "TryPushLabelBlock", 1usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked(self, (node)) };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), quest_hook::libil2cpp::Void, 0usize>(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, ".ctor", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_LightCompiler1(
        &mut self,
        parent: quest_hook::libil2cpp::Gc<
            crate::System::Linq::Expressions::Interpreter::LightCompiler,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::System::Linq::Expressions::Interpreter::LightCompiler,
                >),
                quest_hook::libil2cpp::Void,
                1usize,
            >(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, ".ctor", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (parent))
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_Instructions(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Linq::Expressions::Interpreter::InstructionList,
        >,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Gc<
                    crate::System::Linq::Expressions::Interpreter::InstructionList,
                >,
                0usize,
            >("get_Instructions")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "get_Instructions", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Linq::Expressions::Interpreter::InstructionList,
        > = unsafe { method.invoke_unchecked(self, ()) };
        Ok(__cordl_ret.into())
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
    pub _definedParameters: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::Dictionary_2<
            quest_hook::libil2cpp::Gc<
                crate::System::Linq::Expressions::ParameterExpression,
            >,
            i32,
        >,
    >,
    pub _hoistedParameters: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::HashSet_1<
            quest_hook::libil2cpp::Gc<
                crate::System::Linq::Expressions::ParameterExpression,
            >,
        >,
    >,
}
#[cfg(feature = "System+Linq+Expressions+Interpreter+LightCompiler+QuoteVisitor")]
unsafe impl quest_hook::libil2cpp::Type
for crate::System::Linq::Expressions::Interpreter::LightCompiler_QuoteVisitor {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "System.Linq.Expressions.Interpreter";
    const CLASS_NAME: &'static str = "LightCompiler/QuoteVisitor";
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
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn PopParameters(
        &mut self,
        parameters: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<
                quest_hook::libil2cpp::Gc<
                    crate::System::Linq::Expressions::ParameterExpression,
                >,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::System::Collections::Generic::IEnumerable_1<
                        quest_hook::libil2cpp::Gc<
                            crate::System::Linq::Expressions::ParameterExpression,
                        >,
                    >,
                >),
                quest_hook::libil2cpp::Void,
                1usize,
            >("PopParameters")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "PopParameters", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (parameters))
        };
        Ok(__cordl_ret.into())
    }
    pub fn PushParameters(
        &mut self,
        parameters: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<
                quest_hook::libil2cpp::Gc<
                    crate::System::Linq::Expressions::ParameterExpression,
                >,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::System::Collections::Generic::IEnumerable_1<
                        quest_hook::libil2cpp::Gc<
                            crate::System::Linq::Expressions::ParameterExpression,
                        >,
                    >,
                >),
                quest_hook::libil2cpp::Void,
                1usize,
            >("PushParameters")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "PushParameters", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (parameters))
        };
        Ok(__cordl_ret.into())
    }
    pub fn VisitBlock(
        &mut self,
        node: quest_hook::libil2cpp::Gc<
            crate::System::Linq::Expressions::BlockExpression,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::Expression>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::System::Linq::Expressions::BlockExpression,
                >),
                quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::Expression>,
                1usize,
            >("VisitBlock")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "VisitBlock", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Linq::Expressions::Expression,
        > = unsafe { method.invoke_unchecked(self, (node)) };
        Ok(__cordl_ret.into())
    }
    pub fn VisitCatchBlock(
        &mut self,
        node: quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::CatchBlock>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::CatchBlock>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::System::Linq::Expressions::CatchBlock,
                >),
                quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::CatchBlock>,
                1usize,
            >("VisitCatchBlock")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "VisitCatchBlock", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Linq::Expressions::CatchBlock,
        > = unsafe { method.invoke_unchecked(self, (node)) };
        Ok(__cordl_ret.into())
    }
    pub fn VisitLambda<T>(
        &mut self,
        node: quest_hook::libil2cpp::Gc<
            crate::System::Linq::Expressions::Expression_1<T>,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::Expression>,
    >
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::System::Linq::Expressions::Expression_1<T>,
                >),
                quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::Expression>,
                1usize,
            >("VisitLambda")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "VisitLambda", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Linq::Expressions::Expression,
        > = unsafe { method.invoke_unchecked(self, (node)) };
        Ok(__cordl_ret.into())
    }
    pub fn VisitParameter(
        &mut self,
        node: quest_hook::libil2cpp::Gc<
            crate::System::Linq::Expressions::ParameterExpression,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::Expression>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::System::Linq::Expressions::ParameterExpression,
                >),
                quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::Expression>,
                1usize,
            >("VisitParameter")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "VisitParameter", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Linq::Expressions::Expression,
        > = unsafe { method.invoke_unchecked(self, (node)) };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), quest_hook::libil2cpp::Void, 0usize>(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, ".ctor", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())
        };
        Ok(__cordl_ret.into())
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
