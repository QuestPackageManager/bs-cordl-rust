#[cfg(feature = "System+Linq+Expressions+Expression")]
#[repr(C)]
#[derive(Debug)]
pub struct Expression {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "System+Linq+Expressions+Expression")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Linq::Expressions::Expression =>
    "System.Linq.Expressions"."Expression"
);
#[cfg(feature = "System+Linq+Expressions+Expression")]
impl std::ops::Deref for crate::System::Linq::Expressions::Expression {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Linq+Expressions+Expression")]
impl std::ops::DerefMut for crate::System::Linq::Expressions::Expression {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Linq+Expressions+Expression")]
impl crate::System::Linq::Expressions::Expression {
    #[cfg(feature = "System+Linq+Expressions+Expression+BinaryExpressionProxy")]
    pub type BinaryExpressionProxy = crate::System::Linq::Expressions::Expression_BinaryExpressionProxy;
    #[cfg(feature = "System+Linq+Expressions+Expression+BlockExpressionProxy")]
    pub type BlockExpressionProxy = crate::System::Linq::Expressions::Expression_BlockExpressionProxy;
    #[cfg(feature = "System+Linq+Expressions+Expression+CatchBlockProxy")]
    pub type CatchBlockProxy = crate::System::Linq::Expressions::Expression_CatchBlockProxy;
    #[cfg(feature = "System+Linq+Expressions+Expression+ConditionalExpressionProxy")]
    pub type ConditionalExpressionProxy = crate::System::Linq::Expressions::Expression_ConditionalExpressionProxy;
    #[cfg(feature = "System+Linq+Expressions+Expression+ConstantExpressionProxy")]
    pub type ConstantExpressionProxy = crate::System::Linq::Expressions::Expression_ConstantExpressionProxy;
    #[cfg(feature = "System+Linq+Expressions+Expression+DebugInfoExpressionProxy")]
    pub type DebugInfoExpressionProxy = crate::System::Linq::Expressions::Expression_DebugInfoExpressionProxy;
    #[cfg(feature = "System+Linq+Expressions+Expression+DefaultExpressionProxy")]
    pub type DefaultExpressionProxy = crate::System::Linq::Expressions::Expression_DefaultExpressionProxy;
    #[cfg(feature = "System+Linq+Expressions+Expression+ExtensionInfo")]
    pub type ExtensionInfo = crate::System::Linq::Expressions::Expression_ExtensionInfo;
    #[cfg(feature = "System+Linq+Expressions+Expression+GotoExpressionProxy")]
    pub type GotoExpressionProxy = crate::System::Linq::Expressions::Expression_GotoExpressionProxy;
    #[cfg(feature = "System+Linq+Expressions+Expression+IndexExpressionProxy")]
    pub type IndexExpressionProxy = crate::System::Linq::Expressions::Expression_IndexExpressionProxy;
    #[cfg(feature = "System+Linq+Expressions+Expression+InvocationExpressionProxy")]
    pub type InvocationExpressionProxy = crate::System::Linq::Expressions::Expression_InvocationExpressionProxy;
    #[cfg(feature = "System+Linq+Expressions+Expression+LabelExpressionProxy")]
    pub type LabelExpressionProxy = crate::System::Linq::Expressions::Expression_LabelExpressionProxy;
    #[cfg(feature = "System+Linq+Expressions+Expression+LambdaExpressionProxy")]
    pub type LambdaExpressionProxy = crate::System::Linq::Expressions::Expression_LambdaExpressionProxy;
    #[cfg(feature = "System+Linq+Expressions+Expression+ListInitExpressionProxy")]
    pub type ListInitExpressionProxy = crate::System::Linq::Expressions::Expression_ListInitExpressionProxy;
    #[cfg(feature = "System+Linq+Expressions+Expression+LoopExpressionProxy")]
    pub type LoopExpressionProxy = crate::System::Linq::Expressions::Expression_LoopExpressionProxy;
    #[cfg(feature = "System+Linq+Expressions+Expression+MemberExpressionProxy")]
    pub type MemberExpressionProxy = crate::System::Linq::Expressions::Expression_MemberExpressionProxy;
    #[cfg(feature = "System+Linq+Expressions+Expression+MemberInitExpressionProxy")]
    pub type MemberInitExpressionProxy = crate::System::Linq::Expressions::Expression_MemberInitExpressionProxy;
    #[cfg(feature = "System+Linq+Expressions+Expression+MethodCallExpressionProxy")]
    pub type MethodCallExpressionProxy = crate::System::Linq::Expressions::Expression_MethodCallExpressionProxy;
    #[cfg(feature = "System+Linq+Expressions+Expression+NewArrayExpressionProxy")]
    pub type NewArrayExpressionProxy = crate::System::Linq::Expressions::Expression_NewArrayExpressionProxy;
    #[cfg(feature = "System+Linq+Expressions+Expression+NewExpressionProxy")]
    pub type NewExpressionProxy = crate::System::Linq::Expressions::Expression_NewExpressionProxy;
    #[cfg(feature = "System+Linq+Expressions+Expression+ParameterExpressionProxy")]
    pub type ParameterExpressionProxy = crate::System::Linq::Expressions::Expression_ParameterExpressionProxy;
    #[cfg(
        feature = "System+Linq+Expressions+Expression+RuntimeVariablesExpressionProxy"
    )]
    pub type RuntimeVariablesExpressionProxy = crate::System::Linq::Expressions::Expression_RuntimeVariablesExpressionProxy;
    #[cfg(feature = "System+Linq+Expressions+Expression+SwitchCaseProxy")]
    pub type SwitchCaseProxy = crate::System::Linq::Expressions::Expression_SwitchCaseProxy;
    #[cfg(feature = "System+Linq+Expressions+Expression+SwitchExpressionProxy")]
    pub type SwitchExpressionProxy = crate::System::Linq::Expressions::Expression_SwitchExpressionProxy;
    #[cfg(feature = "System+Linq+Expressions+Expression+TryExpressionProxy")]
    pub type TryExpressionProxy = crate::System::Linq::Expressions::Expression_TryExpressionProxy;
    #[cfg(feature = "System+Linq+Expressions+Expression+TypeBinaryExpressionProxy")]
    pub type TypeBinaryExpressionProxy = crate::System::Linq::Expressions::Expression_TypeBinaryExpressionProxy;
    #[cfg(feature = "System+Linq+Expressions+Expression+UnaryExpressionProxy")]
    pub type UnaryExpressionProxy = crate::System::Linq::Expressions::Expression_UnaryExpressionProxy;
    pub fn Accept(
        &mut self,
        visitor: quest_hook::libil2cpp::Gc<
            crate::System::Linq::Expressions::ExpressionVisitor,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::Expression>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Linq::Expressions::Expression,
        > = __cordl_object.invoke("Accept", (visitor))?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn Reduce(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::Expression>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Linq::Expressions::Expression,
        > = __cordl_object.invoke("Reduce", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ReduceAndCheck(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::Expression>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Linq::Expressions::Expression,
        > = __cordl_object.invoke("ReduceAndCheck", ())?;
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
    pub fn VisitChildren(
        &mut self,
        visitor: quest_hook::libil2cpp::Gc<
            crate::System::Linq::Expressions::ExpressionVisitor,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::Expression>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Linq::Expressions::Expression,
        > = __cordl_object.invoke("VisitChildren", (visitor))?;
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
    pub fn get_CanReduce(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_CanReduce", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_NodeType(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::System::Linq::Expressions::ExpressionType,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::Linq::Expressions::ExpressionType = __cordl_object
            .invoke("get_NodeType", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Type(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<crate::System::Type>> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Type> = __cordl_object
            .invoke("get_Type", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Linq+Expressions+Expression")]
impl quest_hook::libil2cpp::ObjectType for crate::System::Linq::Expressions::Expression {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "System+Linq+Expressions+Expression+BinaryExpressionProxy")]
#[repr(C)]
#[derive(Debug)]
pub struct Expression_BinaryExpressionProxy {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "System+Linq+Expressions+Expression+BinaryExpressionProxy")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::Linq::Expressions::Expression_BinaryExpressionProxy =>
    "System.Linq.Expressions"."Expression/BinaryExpressionProxy"
);
#[cfg(feature = "System+Linq+Expressions+Expression+BinaryExpressionProxy")]
impl std::ops::Deref
for crate::System::Linq::Expressions::Expression_BinaryExpressionProxy {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Linq+Expressions+Expression+BinaryExpressionProxy")]
impl std::ops::DerefMut
for crate::System::Linq::Expressions::Expression_BinaryExpressionProxy {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Linq+Expressions+Expression+BinaryExpressionProxy")]
impl crate::System::Linq::Expressions::Expression_BinaryExpressionProxy {}
#[cfg(feature = "System+Linq+Expressions+Expression+BinaryExpressionProxy")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Linq::Expressions::Expression_BinaryExpressionProxy {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "System+Linq+Expressions+Expression+BlockExpressionProxy")]
#[repr(C)]
#[derive(Debug)]
pub struct Expression_BlockExpressionProxy {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "System+Linq+Expressions+Expression+BlockExpressionProxy")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::Linq::Expressions::Expression_BlockExpressionProxy =>
    "System.Linq.Expressions"."Expression/BlockExpressionProxy"
);
#[cfg(feature = "System+Linq+Expressions+Expression+BlockExpressionProxy")]
impl std::ops::Deref
for crate::System::Linq::Expressions::Expression_BlockExpressionProxy {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Linq+Expressions+Expression+BlockExpressionProxy")]
impl std::ops::DerefMut
for crate::System::Linq::Expressions::Expression_BlockExpressionProxy {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Linq+Expressions+Expression+BlockExpressionProxy")]
impl crate::System::Linq::Expressions::Expression_BlockExpressionProxy {}
#[cfg(feature = "System+Linq+Expressions+Expression+BlockExpressionProxy")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Linq::Expressions::Expression_BlockExpressionProxy {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "System+Linq+Expressions+Expression+CatchBlockProxy")]
#[repr(C)]
#[derive(Debug)]
pub struct Expression_CatchBlockProxy {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "System+Linq+Expressions+Expression+CatchBlockProxy")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::Linq::Expressions::Expression_CatchBlockProxy => "System.Linq.Expressions"
    ."Expression/CatchBlockProxy"
);
#[cfg(feature = "System+Linq+Expressions+Expression+CatchBlockProxy")]
impl std::ops::Deref for crate::System::Linq::Expressions::Expression_CatchBlockProxy {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Linq+Expressions+Expression+CatchBlockProxy")]
impl std::ops::DerefMut
for crate::System::Linq::Expressions::Expression_CatchBlockProxy {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Linq+Expressions+Expression+CatchBlockProxy")]
impl crate::System::Linq::Expressions::Expression_CatchBlockProxy {}
#[cfg(feature = "System+Linq+Expressions+Expression+CatchBlockProxy")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Linq::Expressions::Expression_CatchBlockProxy {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "System+Linq+Expressions+Expression+ConditionalExpressionProxy")]
#[repr(C)]
#[derive(Debug)]
pub struct Expression_ConditionalExpressionProxy {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "System+Linq+Expressions+Expression+ConditionalExpressionProxy")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::Linq::Expressions::Expression_ConditionalExpressionProxy =>
    "System.Linq.Expressions"."Expression/ConditionalExpressionProxy"
);
#[cfg(feature = "System+Linq+Expressions+Expression+ConditionalExpressionProxy")]
impl std::ops::Deref
for crate::System::Linq::Expressions::Expression_ConditionalExpressionProxy {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Linq+Expressions+Expression+ConditionalExpressionProxy")]
impl std::ops::DerefMut
for crate::System::Linq::Expressions::Expression_ConditionalExpressionProxy {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Linq+Expressions+Expression+ConditionalExpressionProxy")]
impl crate::System::Linq::Expressions::Expression_ConditionalExpressionProxy {}
#[cfg(feature = "System+Linq+Expressions+Expression+ConditionalExpressionProxy")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Linq::Expressions::Expression_ConditionalExpressionProxy {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "System+Linq+Expressions+Expression+ConstantExpressionProxy")]
#[repr(C)]
#[derive(Debug)]
pub struct Expression_ConstantExpressionProxy {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "System+Linq+Expressions+Expression+ConstantExpressionProxy")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::Linq::Expressions::Expression_ConstantExpressionProxy =>
    "System.Linq.Expressions"."Expression/ConstantExpressionProxy"
);
#[cfg(feature = "System+Linq+Expressions+Expression+ConstantExpressionProxy")]
impl std::ops::Deref
for crate::System::Linq::Expressions::Expression_ConstantExpressionProxy {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Linq+Expressions+Expression+ConstantExpressionProxy")]
impl std::ops::DerefMut
for crate::System::Linq::Expressions::Expression_ConstantExpressionProxy {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Linq+Expressions+Expression+ConstantExpressionProxy")]
impl crate::System::Linq::Expressions::Expression_ConstantExpressionProxy {}
#[cfg(feature = "System+Linq+Expressions+Expression+ConstantExpressionProxy")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Linq::Expressions::Expression_ConstantExpressionProxy {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "System+Linq+Expressions+Expression+DebugInfoExpressionProxy")]
#[repr(C)]
#[derive(Debug)]
pub struct Expression_DebugInfoExpressionProxy {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "System+Linq+Expressions+Expression+DebugInfoExpressionProxy")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::Linq::Expressions::Expression_DebugInfoExpressionProxy =>
    "System.Linq.Expressions"."Expression/DebugInfoExpressionProxy"
);
#[cfg(feature = "System+Linq+Expressions+Expression+DebugInfoExpressionProxy")]
impl std::ops::Deref
for crate::System::Linq::Expressions::Expression_DebugInfoExpressionProxy {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Linq+Expressions+Expression+DebugInfoExpressionProxy")]
impl std::ops::DerefMut
for crate::System::Linq::Expressions::Expression_DebugInfoExpressionProxy {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Linq+Expressions+Expression+DebugInfoExpressionProxy")]
impl crate::System::Linq::Expressions::Expression_DebugInfoExpressionProxy {}
#[cfg(feature = "System+Linq+Expressions+Expression+DebugInfoExpressionProxy")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Linq::Expressions::Expression_DebugInfoExpressionProxy {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "System+Linq+Expressions+Expression+DefaultExpressionProxy")]
#[repr(C)]
#[derive(Debug)]
pub struct Expression_DefaultExpressionProxy {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "System+Linq+Expressions+Expression+DefaultExpressionProxy")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::Linq::Expressions::Expression_DefaultExpressionProxy =>
    "System.Linq.Expressions"."Expression/DefaultExpressionProxy"
);
#[cfg(feature = "System+Linq+Expressions+Expression+DefaultExpressionProxy")]
impl std::ops::Deref
for crate::System::Linq::Expressions::Expression_DefaultExpressionProxy {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Linq+Expressions+Expression+DefaultExpressionProxy")]
impl std::ops::DerefMut
for crate::System::Linq::Expressions::Expression_DefaultExpressionProxy {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Linq+Expressions+Expression+DefaultExpressionProxy")]
impl crate::System::Linq::Expressions::Expression_DefaultExpressionProxy {}
#[cfg(feature = "System+Linq+Expressions+Expression+DefaultExpressionProxy")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Linq::Expressions::Expression_DefaultExpressionProxy {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "System+Linq+Expressions+Expression+ExtensionInfo")]
#[repr(C)]
#[derive(Debug)]
pub struct Expression_ExtensionInfo {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub NodeType: crate::System::Linq::Expressions::ExpressionType,
    pub Type: *mut crate::System::Type,
}
#[cfg(feature = "System+Linq+Expressions+Expression+ExtensionInfo")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::Linq::Expressions::Expression_ExtensionInfo => "System.Linq.Expressions"
    ."Expression/ExtensionInfo"
);
#[cfg(feature = "System+Linq+Expressions+Expression+ExtensionInfo")]
impl std::ops::Deref for crate::System::Linq::Expressions::Expression_ExtensionInfo {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Linq+Expressions+Expression+ExtensionInfo")]
impl std::ops::DerefMut for crate::System::Linq::Expressions::Expression_ExtensionInfo {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Linq+Expressions+Expression+ExtensionInfo")]
impl crate::System::Linq::Expressions::Expression_ExtensionInfo {}
#[cfg(feature = "System+Linq+Expressions+Expression+ExtensionInfo")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Linq::Expressions::Expression_ExtensionInfo {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "System+Linq+Expressions+Expression+GotoExpressionProxy")]
#[repr(C)]
#[derive(Debug)]
pub struct Expression_GotoExpressionProxy {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "System+Linq+Expressions+Expression+GotoExpressionProxy")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::Linq::Expressions::Expression_GotoExpressionProxy =>
    "System.Linq.Expressions"."Expression/GotoExpressionProxy"
);
#[cfg(feature = "System+Linq+Expressions+Expression+GotoExpressionProxy")]
impl std::ops::Deref
for crate::System::Linq::Expressions::Expression_GotoExpressionProxy {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Linq+Expressions+Expression+GotoExpressionProxy")]
impl std::ops::DerefMut
for crate::System::Linq::Expressions::Expression_GotoExpressionProxy {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Linq+Expressions+Expression+GotoExpressionProxy")]
impl crate::System::Linq::Expressions::Expression_GotoExpressionProxy {}
#[cfg(feature = "System+Linq+Expressions+Expression+GotoExpressionProxy")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Linq::Expressions::Expression_GotoExpressionProxy {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "System+Linq+Expressions+Expression+IndexExpressionProxy")]
#[repr(C)]
#[derive(Debug)]
pub struct Expression_IndexExpressionProxy {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "System+Linq+Expressions+Expression+IndexExpressionProxy")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::Linq::Expressions::Expression_IndexExpressionProxy =>
    "System.Linq.Expressions"."Expression/IndexExpressionProxy"
);
#[cfg(feature = "System+Linq+Expressions+Expression+IndexExpressionProxy")]
impl std::ops::Deref
for crate::System::Linq::Expressions::Expression_IndexExpressionProxy {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Linq+Expressions+Expression+IndexExpressionProxy")]
impl std::ops::DerefMut
for crate::System::Linq::Expressions::Expression_IndexExpressionProxy {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Linq+Expressions+Expression+IndexExpressionProxy")]
impl crate::System::Linq::Expressions::Expression_IndexExpressionProxy {}
#[cfg(feature = "System+Linq+Expressions+Expression+IndexExpressionProxy")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Linq::Expressions::Expression_IndexExpressionProxy {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "System+Linq+Expressions+Expression+InvocationExpressionProxy")]
#[repr(C)]
#[derive(Debug)]
pub struct Expression_InvocationExpressionProxy {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "System+Linq+Expressions+Expression+InvocationExpressionProxy")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::Linq::Expressions::Expression_InvocationExpressionProxy =>
    "System.Linq.Expressions"."Expression/InvocationExpressionProxy"
);
#[cfg(feature = "System+Linq+Expressions+Expression+InvocationExpressionProxy")]
impl std::ops::Deref
for crate::System::Linq::Expressions::Expression_InvocationExpressionProxy {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Linq+Expressions+Expression+InvocationExpressionProxy")]
impl std::ops::DerefMut
for crate::System::Linq::Expressions::Expression_InvocationExpressionProxy {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Linq+Expressions+Expression+InvocationExpressionProxy")]
impl crate::System::Linq::Expressions::Expression_InvocationExpressionProxy {}
#[cfg(feature = "System+Linq+Expressions+Expression+InvocationExpressionProxy")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Linq::Expressions::Expression_InvocationExpressionProxy {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "System+Linq+Expressions+Expression+LabelExpressionProxy")]
#[repr(C)]
#[derive(Debug)]
pub struct Expression_LabelExpressionProxy {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "System+Linq+Expressions+Expression+LabelExpressionProxy")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::Linq::Expressions::Expression_LabelExpressionProxy =>
    "System.Linq.Expressions"."Expression/LabelExpressionProxy"
);
#[cfg(feature = "System+Linq+Expressions+Expression+LabelExpressionProxy")]
impl std::ops::Deref
for crate::System::Linq::Expressions::Expression_LabelExpressionProxy {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Linq+Expressions+Expression+LabelExpressionProxy")]
impl std::ops::DerefMut
for crate::System::Linq::Expressions::Expression_LabelExpressionProxy {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Linq+Expressions+Expression+LabelExpressionProxy")]
impl crate::System::Linq::Expressions::Expression_LabelExpressionProxy {}
#[cfg(feature = "System+Linq+Expressions+Expression+LabelExpressionProxy")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Linq::Expressions::Expression_LabelExpressionProxy {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "System+Linq+Expressions+Expression+LambdaExpressionProxy")]
#[repr(C)]
#[derive(Debug)]
pub struct Expression_LambdaExpressionProxy {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "System+Linq+Expressions+Expression+LambdaExpressionProxy")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::Linq::Expressions::Expression_LambdaExpressionProxy =>
    "System.Linq.Expressions"."Expression/LambdaExpressionProxy"
);
#[cfg(feature = "System+Linq+Expressions+Expression+LambdaExpressionProxy")]
impl std::ops::Deref
for crate::System::Linq::Expressions::Expression_LambdaExpressionProxy {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Linq+Expressions+Expression+LambdaExpressionProxy")]
impl std::ops::DerefMut
for crate::System::Linq::Expressions::Expression_LambdaExpressionProxy {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Linq+Expressions+Expression+LambdaExpressionProxy")]
impl crate::System::Linq::Expressions::Expression_LambdaExpressionProxy {}
#[cfg(feature = "System+Linq+Expressions+Expression+LambdaExpressionProxy")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Linq::Expressions::Expression_LambdaExpressionProxy {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "System+Linq+Expressions+Expression+ListInitExpressionProxy")]
#[repr(C)]
#[derive(Debug)]
pub struct Expression_ListInitExpressionProxy {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "System+Linq+Expressions+Expression+ListInitExpressionProxy")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::Linq::Expressions::Expression_ListInitExpressionProxy =>
    "System.Linq.Expressions"."Expression/ListInitExpressionProxy"
);
#[cfg(feature = "System+Linq+Expressions+Expression+ListInitExpressionProxy")]
impl std::ops::Deref
for crate::System::Linq::Expressions::Expression_ListInitExpressionProxy {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Linq+Expressions+Expression+ListInitExpressionProxy")]
impl std::ops::DerefMut
for crate::System::Linq::Expressions::Expression_ListInitExpressionProxy {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Linq+Expressions+Expression+ListInitExpressionProxy")]
impl crate::System::Linq::Expressions::Expression_ListInitExpressionProxy {}
#[cfg(feature = "System+Linq+Expressions+Expression+ListInitExpressionProxy")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Linq::Expressions::Expression_ListInitExpressionProxy {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "System+Linq+Expressions+Expression+LoopExpressionProxy")]
#[repr(C)]
#[derive(Debug)]
pub struct Expression_LoopExpressionProxy {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "System+Linq+Expressions+Expression+LoopExpressionProxy")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::Linq::Expressions::Expression_LoopExpressionProxy =>
    "System.Linq.Expressions"."Expression/LoopExpressionProxy"
);
#[cfg(feature = "System+Linq+Expressions+Expression+LoopExpressionProxy")]
impl std::ops::Deref
for crate::System::Linq::Expressions::Expression_LoopExpressionProxy {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Linq+Expressions+Expression+LoopExpressionProxy")]
impl std::ops::DerefMut
for crate::System::Linq::Expressions::Expression_LoopExpressionProxy {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Linq+Expressions+Expression+LoopExpressionProxy")]
impl crate::System::Linq::Expressions::Expression_LoopExpressionProxy {}
#[cfg(feature = "System+Linq+Expressions+Expression+LoopExpressionProxy")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Linq::Expressions::Expression_LoopExpressionProxy {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "System+Linq+Expressions+Expression+MemberExpressionProxy")]
#[repr(C)]
#[derive(Debug)]
pub struct Expression_MemberExpressionProxy {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "System+Linq+Expressions+Expression+MemberExpressionProxy")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::Linq::Expressions::Expression_MemberExpressionProxy =>
    "System.Linq.Expressions"."Expression/MemberExpressionProxy"
);
#[cfg(feature = "System+Linq+Expressions+Expression+MemberExpressionProxy")]
impl std::ops::Deref
for crate::System::Linq::Expressions::Expression_MemberExpressionProxy {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Linq+Expressions+Expression+MemberExpressionProxy")]
impl std::ops::DerefMut
for crate::System::Linq::Expressions::Expression_MemberExpressionProxy {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Linq+Expressions+Expression+MemberExpressionProxy")]
impl crate::System::Linq::Expressions::Expression_MemberExpressionProxy {}
#[cfg(feature = "System+Linq+Expressions+Expression+MemberExpressionProxy")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Linq::Expressions::Expression_MemberExpressionProxy {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "System+Linq+Expressions+Expression+MemberInitExpressionProxy")]
#[repr(C)]
#[derive(Debug)]
pub struct Expression_MemberInitExpressionProxy {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "System+Linq+Expressions+Expression+MemberInitExpressionProxy")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::Linq::Expressions::Expression_MemberInitExpressionProxy =>
    "System.Linq.Expressions"."Expression/MemberInitExpressionProxy"
);
#[cfg(feature = "System+Linq+Expressions+Expression+MemberInitExpressionProxy")]
impl std::ops::Deref
for crate::System::Linq::Expressions::Expression_MemberInitExpressionProxy {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Linq+Expressions+Expression+MemberInitExpressionProxy")]
impl std::ops::DerefMut
for crate::System::Linq::Expressions::Expression_MemberInitExpressionProxy {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Linq+Expressions+Expression+MemberInitExpressionProxy")]
impl crate::System::Linq::Expressions::Expression_MemberInitExpressionProxy {}
#[cfg(feature = "System+Linq+Expressions+Expression+MemberInitExpressionProxy")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Linq::Expressions::Expression_MemberInitExpressionProxy {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "System+Linq+Expressions+Expression+MethodCallExpressionProxy")]
#[repr(C)]
#[derive(Debug)]
pub struct Expression_MethodCallExpressionProxy {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "System+Linq+Expressions+Expression+MethodCallExpressionProxy")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::Linq::Expressions::Expression_MethodCallExpressionProxy =>
    "System.Linq.Expressions"."Expression/MethodCallExpressionProxy"
);
#[cfg(feature = "System+Linq+Expressions+Expression+MethodCallExpressionProxy")]
impl std::ops::Deref
for crate::System::Linq::Expressions::Expression_MethodCallExpressionProxy {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Linq+Expressions+Expression+MethodCallExpressionProxy")]
impl std::ops::DerefMut
for crate::System::Linq::Expressions::Expression_MethodCallExpressionProxy {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Linq+Expressions+Expression+MethodCallExpressionProxy")]
impl crate::System::Linq::Expressions::Expression_MethodCallExpressionProxy {}
#[cfg(feature = "System+Linq+Expressions+Expression+MethodCallExpressionProxy")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Linq::Expressions::Expression_MethodCallExpressionProxy {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "System+Linq+Expressions+Expression+NewArrayExpressionProxy")]
#[repr(C)]
#[derive(Debug)]
pub struct Expression_NewArrayExpressionProxy {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "System+Linq+Expressions+Expression+NewArrayExpressionProxy")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::Linq::Expressions::Expression_NewArrayExpressionProxy =>
    "System.Linq.Expressions"."Expression/NewArrayExpressionProxy"
);
#[cfg(feature = "System+Linq+Expressions+Expression+NewArrayExpressionProxy")]
impl std::ops::Deref
for crate::System::Linq::Expressions::Expression_NewArrayExpressionProxy {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Linq+Expressions+Expression+NewArrayExpressionProxy")]
impl std::ops::DerefMut
for crate::System::Linq::Expressions::Expression_NewArrayExpressionProxy {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Linq+Expressions+Expression+NewArrayExpressionProxy")]
impl crate::System::Linq::Expressions::Expression_NewArrayExpressionProxy {}
#[cfg(feature = "System+Linq+Expressions+Expression+NewArrayExpressionProxy")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Linq::Expressions::Expression_NewArrayExpressionProxy {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "System+Linq+Expressions+Expression+NewExpressionProxy")]
#[repr(C)]
#[derive(Debug)]
pub struct Expression_NewExpressionProxy {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "System+Linq+Expressions+Expression+NewExpressionProxy")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::Linq::Expressions::Expression_NewExpressionProxy =>
    "System.Linq.Expressions"."Expression/NewExpressionProxy"
);
#[cfg(feature = "System+Linq+Expressions+Expression+NewExpressionProxy")]
impl std::ops::Deref
for crate::System::Linq::Expressions::Expression_NewExpressionProxy {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Linq+Expressions+Expression+NewExpressionProxy")]
impl std::ops::DerefMut
for crate::System::Linq::Expressions::Expression_NewExpressionProxy {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Linq+Expressions+Expression+NewExpressionProxy")]
impl crate::System::Linq::Expressions::Expression_NewExpressionProxy {}
#[cfg(feature = "System+Linq+Expressions+Expression+NewExpressionProxy")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Linq::Expressions::Expression_NewExpressionProxy {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "System+Linq+Expressions+Expression+ParameterExpressionProxy")]
#[repr(C)]
#[derive(Debug)]
pub struct Expression_ParameterExpressionProxy {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "System+Linq+Expressions+Expression+ParameterExpressionProxy")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::Linq::Expressions::Expression_ParameterExpressionProxy =>
    "System.Linq.Expressions"."Expression/ParameterExpressionProxy"
);
#[cfg(feature = "System+Linq+Expressions+Expression+ParameterExpressionProxy")]
impl std::ops::Deref
for crate::System::Linq::Expressions::Expression_ParameterExpressionProxy {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Linq+Expressions+Expression+ParameterExpressionProxy")]
impl std::ops::DerefMut
for crate::System::Linq::Expressions::Expression_ParameterExpressionProxy {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Linq+Expressions+Expression+ParameterExpressionProxy")]
impl crate::System::Linq::Expressions::Expression_ParameterExpressionProxy {}
#[cfg(feature = "System+Linq+Expressions+Expression+ParameterExpressionProxy")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Linq::Expressions::Expression_ParameterExpressionProxy {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "System+Linq+Expressions+Expression+RuntimeVariablesExpressionProxy")]
#[repr(C)]
#[derive(Debug)]
pub struct Expression_RuntimeVariablesExpressionProxy {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "System+Linq+Expressions+Expression+RuntimeVariablesExpressionProxy")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::Linq::Expressions::Expression_RuntimeVariablesExpressionProxy =>
    "System.Linq.Expressions"."Expression/RuntimeVariablesExpressionProxy"
);
#[cfg(feature = "System+Linq+Expressions+Expression+RuntimeVariablesExpressionProxy")]
impl std::ops::Deref
for crate::System::Linq::Expressions::Expression_RuntimeVariablesExpressionProxy {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Linq+Expressions+Expression+RuntimeVariablesExpressionProxy")]
impl std::ops::DerefMut
for crate::System::Linq::Expressions::Expression_RuntimeVariablesExpressionProxy {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Linq+Expressions+Expression+RuntimeVariablesExpressionProxy")]
impl crate::System::Linq::Expressions::Expression_RuntimeVariablesExpressionProxy {}
#[cfg(feature = "System+Linq+Expressions+Expression+RuntimeVariablesExpressionProxy")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Linq::Expressions::Expression_RuntimeVariablesExpressionProxy {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "System+Linq+Expressions+Expression+SwitchCaseProxy")]
#[repr(C)]
#[derive(Debug)]
pub struct Expression_SwitchCaseProxy {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "System+Linq+Expressions+Expression+SwitchCaseProxy")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::Linq::Expressions::Expression_SwitchCaseProxy => "System.Linq.Expressions"
    ."Expression/SwitchCaseProxy"
);
#[cfg(feature = "System+Linq+Expressions+Expression+SwitchCaseProxy")]
impl std::ops::Deref for crate::System::Linq::Expressions::Expression_SwitchCaseProxy {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Linq+Expressions+Expression+SwitchCaseProxy")]
impl std::ops::DerefMut
for crate::System::Linq::Expressions::Expression_SwitchCaseProxy {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Linq+Expressions+Expression+SwitchCaseProxy")]
impl crate::System::Linq::Expressions::Expression_SwitchCaseProxy {}
#[cfg(feature = "System+Linq+Expressions+Expression+SwitchCaseProxy")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Linq::Expressions::Expression_SwitchCaseProxy {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "System+Linq+Expressions+Expression+SwitchExpressionProxy")]
#[repr(C)]
#[derive(Debug)]
pub struct Expression_SwitchExpressionProxy {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "System+Linq+Expressions+Expression+SwitchExpressionProxy")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::Linq::Expressions::Expression_SwitchExpressionProxy =>
    "System.Linq.Expressions"."Expression/SwitchExpressionProxy"
);
#[cfg(feature = "System+Linq+Expressions+Expression+SwitchExpressionProxy")]
impl std::ops::Deref
for crate::System::Linq::Expressions::Expression_SwitchExpressionProxy {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Linq+Expressions+Expression+SwitchExpressionProxy")]
impl std::ops::DerefMut
for crate::System::Linq::Expressions::Expression_SwitchExpressionProxy {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Linq+Expressions+Expression+SwitchExpressionProxy")]
impl crate::System::Linq::Expressions::Expression_SwitchExpressionProxy {}
#[cfg(feature = "System+Linq+Expressions+Expression+SwitchExpressionProxy")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Linq::Expressions::Expression_SwitchExpressionProxy {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "System+Linq+Expressions+Expression+TryExpressionProxy")]
#[repr(C)]
#[derive(Debug)]
pub struct Expression_TryExpressionProxy {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "System+Linq+Expressions+Expression+TryExpressionProxy")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::Linq::Expressions::Expression_TryExpressionProxy =>
    "System.Linq.Expressions"."Expression/TryExpressionProxy"
);
#[cfg(feature = "System+Linq+Expressions+Expression+TryExpressionProxy")]
impl std::ops::Deref
for crate::System::Linq::Expressions::Expression_TryExpressionProxy {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Linq+Expressions+Expression+TryExpressionProxy")]
impl std::ops::DerefMut
for crate::System::Linq::Expressions::Expression_TryExpressionProxy {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Linq+Expressions+Expression+TryExpressionProxy")]
impl crate::System::Linq::Expressions::Expression_TryExpressionProxy {}
#[cfg(feature = "System+Linq+Expressions+Expression+TryExpressionProxy")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Linq::Expressions::Expression_TryExpressionProxy {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "System+Linq+Expressions+Expression+TypeBinaryExpressionProxy")]
#[repr(C)]
#[derive(Debug)]
pub struct Expression_TypeBinaryExpressionProxy {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "System+Linq+Expressions+Expression+TypeBinaryExpressionProxy")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::Linq::Expressions::Expression_TypeBinaryExpressionProxy =>
    "System.Linq.Expressions"."Expression/TypeBinaryExpressionProxy"
);
#[cfg(feature = "System+Linq+Expressions+Expression+TypeBinaryExpressionProxy")]
impl std::ops::Deref
for crate::System::Linq::Expressions::Expression_TypeBinaryExpressionProxy {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Linq+Expressions+Expression+TypeBinaryExpressionProxy")]
impl std::ops::DerefMut
for crate::System::Linq::Expressions::Expression_TypeBinaryExpressionProxy {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Linq+Expressions+Expression+TypeBinaryExpressionProxy")]
impl crate::System::Linq::Expressions::Expression_TypeBinaryExpressionProxy {}
#[cfg(feature = "System+Linq+Expressions+Expression+TypeBinaryExpressionProxy")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Linq::Expressions::Expression_TypeBinaryExpressionProxy {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "System+Linq+Expressions+Expression+UnaryExpressionProxy")]
#[repr(C)]
#[derive(Debug)]
pub struct Expression_UnaryExpressionProxy {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "System+Linq+Expressions+Expression+UnaryExpressionProxy")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::Linq::Expressions::Expression_UnaryExpressionProxy =>
    "System.Linq.Expressions"."Expression/UnaryExpressionProxy"
);
#[cfg(feature = "System+Linq+Expressions+Expression+UnaryExpressionProxy")]
impl std::ops::Deref
for crate::System::Linq::Expressions::Expression_UnaryExpressionProxy {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Linq+Expressions+Expression+UnaryExpressionProxy")]
impl std::ops::DerefMut
for crate::System::Linq::Expressions::Expression_UnaryExpressionProxy {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Linq+Expressions+Expression+UnaryExpressionProxy")]
impl crate::System::Linq::Expressions::Expression_UnaryExpressionProxy {}
#[cfg(feature = "System+Linq+Expressions+Expression+UnaryExpressionProxy")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Linq::Expressions::Expression_UnaryExpressionProxy {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
