#[cfg(feature = "System+Linq+Expressions+ExpressionVisitor")]
#[repr(C)]
#[derive(Debug)]
pub struct ExpressionVisitor {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "System+Linq+Expressions+ExpressionVisitor")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Linq::Expressions::ExpressionVisitor =>
    "System.Linq.Expressions"."ExpressionVisitor"
);
#[cfg(feature = "System+Linq+Expressions+ExpressionVisitor")]
impl std::ops::Deref for crate::System::Linq::Expressions::ExpressionVisitor {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Linq+Expressions+ExpressionVisitor")]
impl std::ops::DerefMut for crate::System::Linq::Expressions::ExpressionVisitor {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Linq+Expressions+ExpressionVisitor")]
impl crate::System::Linq::Expressions::ExpressionVisitor {
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn VisitAndConvert_ReadOnlyCollection_1_1<T>(
        &mut self,
        nodes: *mut crate::System::Collections::ObjectModel::ReadOnlyCollection_1<T>,
        callerName: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Collections::ObjectModel::ReadOnlyCollection_1<T>,
    >
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::ObjectModel::ReadOnlyCollection_1<
            T,
        > = __cordl_object.invoke("VisitAndConvert", (nodes, callerName))?;
        Ok(__cordl_ret)
    }
    pub fn VisitAndConvert_T0<T>(
        &mut self,
        node: T,
        callerName: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<T>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: T = __cordl_object
            .invoke("VisitAndConvert", (node, callerName))?;
        Ok(__cordl_ret)
    }
    pub fn VisitArguments(
        &mut self,
        nodes: *mut crate::System::Linq::Expressions::IArgumentProvider,
    ) -> quest_hook::libil2cpp::Result<
        *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::System::Linq::Expressions::Expression,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::System::Linq::Expressions::Expression,
        > = __cordl_object.invoke("VisitArguments", (nodes))?;
        Ok(__cordl_ret)
    }
    pub fn VisitBinary(
        &mut self,
        node: *mut crate::System::Linq::Expressions::BinaryExpression,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Linq::Expressions::Expression,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Linq::Expressions::Expression = __cordl_object
            .invoke("VisitBinary", (node))?;
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
    pub fn VisitConditional(
        &mut self,
        node: *mut crate::System::Linq::Expressions::ConditionalExpression,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Linq::Expressions::Expression,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Linq::Expressions::Expression = __cordl_object
            .invoke("VisitConditional", (node))?;
        Ok(__cordl_ret)
    }
    pub fn VisitConstant(
        &mut self,
        node: *mut crate::System::Linq::Expressions::ConstantExpression,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Linq::Expressions::Expression,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Linq::Expressions::Expression = __cordl_object
            .invoke("VisitConstant", (node))?;
        Ok(__cordl_ret)
    }
    pub fn VisitDefault(
        &mut self,
        node: *mut crate::System::Linq::Expressions::DefaultExpression,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Linq::Expressions::Expression,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Linq::Expressions::Expression = __cordl_object
            .invoke("VisitDefault", (node))?;
        Ok(__cordl_ret)
    }
    pub fn VisitExtension(
        &mut self,
        node: *mut crate::System::Linq::Expressions::Expression,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Linq::Expressions::Expression,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Linq::Expressions::Expression = __cordl_object
            .invoke("VisitExtension", (node))?;
        Ok(__cordl_ret)
    }
    pub fn VisitGoto(
        &mut self,
        node: *mut crate::System::Linq::Expressions::GotoExpression,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Linq::Expressions::Expression,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Linq::Expressions::Expression = __cordl_object
            .invoke("VisitGoto", (node))?;
        Ok(__cordl_ret)
    }
    pub fn VisitIndex(
        &mut self,
        node: *mut crate::System::Linq::Expressions::IndexExpression,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Linq::Expressions::Expression,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Linq::Expressions::Expression = __cordl_object
            .invoke("VisitIndex", (node))?;
        Ok(__cordl_ret)
    }
    pub fn VisitInvocation(
        &mut self,
        node: *mut crate::System::Linq::Expressions::InvocationExpression,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Linq::Expressions::Expression,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Linq::Expressions::Expression = __cordl_object
            .invoke("VisitInvocation", (node))?;
        Ok(__cordl_ret)
    }
    pub fn VisitLabel(
        &mut self,
        node: *mut crate::System::Linq::Expressions::LabelExpression,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Linq::Expressions::Expression,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Linq::Expressions::Expression = __cordl_object
            .invoke("VisitLabel", (node))?;
        Ok(__cordl_ret)
    }
    pub fn VisitLabelTarget(
        &mut self,
        node: *mut crate::System::Linq::Expressions::LabelTarget,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Linq::Expressions::LabelTarget,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Linq::Expressions::LabelTarget = __cordl_object
            .invoke("VisitLabelTarget", (node))?;
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
    pub fn VisitLoop(
        &mut self,
        node: *mut crate::System::Linq::Expressions::LoopExpression,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Linq::Expressions::Expression,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Linq::Expressions::Expression = __cordl_object
            .invoke("VisitLoop", (node))?;
        Ok(__cordl_ret)
    }
    pub fn VisitMember(
        &mut self,
        node: *mut crate::System::Linq::Expressions::MemberExpression,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Linq::Expressions::Expression,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Linq::Expressions::Expression = __cordl_object
            .invoke("VisitMember", (node))?;
        Ok(__cordl_ret)
    }
    pub fn VisitMethodCall(
        &mut self,
        node: *mut crate::System::Linq::Expressions::MethodCallExpression,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Linq::Expressions::Expression,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Linq::Expressions::Expression = __cordl_object
            .invoke("VisitMethodCall", (node))?;
        Ok(__cordl_ret)
    }
    pub fn VisitNewArray(
        &mut self,
        node: *mut crate::System::Linq::Expressions::NewArrayExpression,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Linq::Expressions::Expression,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Linq::Expressions::Expression = __cordl_object
            .invoke("VisitNewArray", (node))?;
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
    pub fn VisitParameters(
        &mut self,
        nodes: *mut crate::System::Linq::Expressions::IParameterProvider,
        callerName: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<
        *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::System::Linq::Expressions::ParameterExpression,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::System::Linq::Expressions::ParameterExpression,
        > = __cordl_object.invoke("VisitParameters", (nodes, callerName))?;
        Ok(__cordl_ret)
    }
    pub fn VisitTry(
        &mut self,
        node: *mut crate::System::Linq::Expressions::TryExpression,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Linq::Expressions::Expression,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Linq::Expressions::Expression = __cordl_object
            .invoke("VisitTry", (node))?;
        Ok(__cordl_ret)
    }
    pub fn VisitTypeBinary(
        &mut self,
        node: *mut crate::System::Linq::Expressions::TypeBinaryExpression,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Linq::Expressions::Expression,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Linq::Expressions::Expression = __cordl_object
            .invoke("VisitTypeBinary", (node))?;
        Ok(__cordl_ret)
    }
    pub fn VisitUnary(
        &mut self,
        node: *mut crate::System::Linq::Expressions::UnaryExpression,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Linq::Expressions::Expression,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Linq::Expressions::Expression = __cordl_object
            .invoke("VisitUnary", (node))?;
        Ok(__cordl_ret)
    }
    pub fn Visit_Expression0(
        &mut self,
        node: *mut crate::System::Linq::Expressions::Expression,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Linq::Expressions::Expression,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Linq::Expressions::Expression = __cordl_object
            .invoke("Visit", (node))?;
        Ok(__cordl_ret)
    }
    pub fn Visit_ReadOnlyCollection_1_1(
        &mut self,
        nodes: *mut crate::System::Collections::ObjectModel::ReadOnlyCollection_1<
            *mut crate::System::Linq::Expressions::Expression,
        >,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Collections::ObjectModel::ReadOnlyCollection_1<
            *mut crate::System::Linq::Expressions::Expression,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::ObjectModel::ReadOnlyCollection_1<
            *mut crate::System::Linq::Expressions::Expression,
        > = __cordl_object.invoke("Visit", (nodes))?;
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
}
#[cfg(feature = "System+Linq+Expressions+ExpressionVisitor")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Linq::Expressions::ExpressionVisitor {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}