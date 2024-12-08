#[cfg(feature = "System+Linq+Expressions+ExpressionStringBuilder")]
#[repr(C)]
#[derive(Debug)]
pub struct ExpressionStringBuilder {
    __cordl_parent: crate::System::Linq::Expressions::ExpressionVisitor,
    pub _out: *mut crate::System::Text::StringBuilder,
    pub _ids: *mut crate::System::Collections::Generic::Dictionary_2<
        *mut crate::System::Object,
        i32,
    >,
}
#[cfg(feature = "System+Linq+Expressions+ExpressionStringBuilder")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::Linq::Expressions::ExpressionStringBuilder => "System.Linq.Expressions"
    ."ExpressionStringBuilder"
);
#[cfg(feature = "System+Linq+Expressions+ExpressionStringBuilder")]
impl std::ops::Deref for crate::System::Linq::Expressions::ExpressionStringBuilder {
    type Target = crate::System::Linq::Expressions::ExpressionVisitor;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Linq+Expressions+ExpressionStringBuilder")]
impl std::ops::DerefMut for crate::System::Linq::Expressions::ExpressionStringBuilder {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Linq+Expressions+ExpressionStringBuilder")]
impl crate::System::Linq::Expressions::ExpressionStringBuilder {
    pub fn DumpLabel(
        &mut self,
        target: *mut crate::System::Linq::Expressions::LabelTarget,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("DumpLabel", (target))?;
        Ok(__cordl_ret)
    }
    pub fn GetId(
        &mut self,
        o: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("GetId", (o))?;
        Ok(__cordl_ret)
    }
    pub fn GetLabelId(
        &mut self,
        label: *mut crate::System::Linq::Expressions::LabelTarget,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("GetLabelId", (label))?;
        Ok(__cordl_ret)
    }
    pub fn GetParamId(
        &mut self,
        p: *mut crate::System::Linq::Expressions::ParameterExpression,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("GetParamId", (p))?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn OutMember(
        &mut self,
        instance: *mut crate::System::Linq::Expressions::Expression,
        member: *mut crate::System::Reflection::MemberInfo,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OutMember", (instance, member))?;
        Ok(__cordl_ret)
    }
    pub fn Out_String0(
        &mut self,
        s: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Out", (s))?;
        Ok(__cordl_ret)
    }
    pub fn Out__cordl_char1(
        &mut self,
        c: char,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Out", (c))?;
        Ok(__cordl_ret)
    }
    pub fn ToString(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("ToString", ())?;
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
    pub fn VisitExpressions_String1<T>(
        &mut self,
        open: char,
        expressions: *mut crate::System::Collections::ObjectModel::ReadOnlyCollection_1<
            T,
        >,
        close: char,
        seperator: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("VisitExpressions", (open, expressions, close, seperator))?;
        Ok(__cordl_ret)
    }
    pub fn VisitExpressions__cordl_char_ReadOnlyCollection_1__cordl_char0<T>(
        &mut self,
        open: char,
        expressions: *mut crate::System::Collections::ObjectModel::ReadOnlyCollection_1<
            T,
        >,
        close: char,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("VisitExpressions", (open, expressions, close))?;
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
#[cfg(feature = "System+Linq+Expressions+ExpressionStringBuilder")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Linq::Expressions::ExpressionStringBuilder {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}