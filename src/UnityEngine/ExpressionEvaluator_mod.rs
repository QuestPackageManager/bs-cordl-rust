#[cfg(feature = "UnityEngine+ExpressionEvaluator")]
#[repr(C)]
#[derive(Debug)]
pub struct ExpressionEvaluator {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "UnityEngine+ExpressionEvaluator")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::ExpressionEvaluator =>
    "UnityEngine"."ExpressionEvaluator"
);
#[cfg(feature = "UnityEngine+ExpressionEvaluator")]
impl std::ops::Deref for crate::UnityEngine::ExpressionEvaluator {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+ExpressionEvaluator")]
impl std::ops::DerefMut for crate::UnityEngine::ExpressionEvaluator {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+ExpressionEvaluator")]
impl crate::UnityEngine::ExpressionEvaluator {
    #[cfg(feature = "UnityEngine+ExpressionEvaluator+Associativity")]
    pub type Associativity = crate::UnityEngine::ExpressionEvaluator_Associativity;
    #[cfg(feature = "UnityEngine+ExpressionEvaluator+Expression")]
    pub type Expression = crate::UnityEngine::ExpressionEvaluator_Expression;
    #[cfg(feature = "UnityEngine+ExpressionEvaluator+Op")]
    pub type Op = crate::UnityEngine::ExpressionEvaluator_Op;
    #[cfg(feature = "UnityEngine+ExpressionEvaluator+Operator")]
    pub type Operator = crate::UnityEngine::ExpressionEvaluator_Operator;
    #[cfg(feature = "UnityEngine+ExpressionEvaluator+PcgRandom")]
    pub type PcgRandom = crate::UnityEngine::ExpressionEvaluator_PcgRandom;
    pub fn Evaluate<T>(
        expression: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        value: quest_hook::libil2cpp::ByRefMut<T>,
        delayed: quest_hook::libil2cpp::ByRefMut<
            *mut crate::UnityEngine::ExpressionEvaluator_Expression,
        >,
    ) -> quest_hook::libil2cpp::Result<bool>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Evaluate", (expression, value, delayed))?;
        Ok(__cordl_ret.into())
    }
    pub fn EvaluateDouble(
        tokens: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<*mut quest_hook::libil2cpp::Il2CppString>,
        >,
        value: quest_hook::libil2cpp::ByRefMut<f64>,
        index: i32,
        count: i32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("EvaluateDouble", (tokens, value, index, count))?;
        Ok(__cordl_ret.into())
    }
    pub fn EvaluateOp(
        values: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<f64>>,
        op: crate::UnityEngine::ExpressionEvaluator_Op,
        index: i32,
        count: i32,
    ) -> quest_hook::libil2cpp::Result<f64> {
        let __cordl_ret: f64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("EvaluateOp", (values, op, index, count))?;
        Ok(__cordl_ret.into())
    }
    pub fn EvaluateTokens<T>(
        tokens: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<*mut quest_hook::libil2cpp::Il2CppString>,
        >,
        value: quest_hook::libil2cpp::ByRefMut<T>,
        index: i32,
        count: i32,
    ) -> quest_hook::libil2cpp::Result<bool>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("EvaluateTokens", (tokens, value, index, count))?;
        Ok(__cordl_ret.into())
    }
    pub fn ExpressionToTokens(
        expression: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        hasVariables: quest_hook::libil2cpp::ByRefMut<bool>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<*mut quest_hook::libil2cpp::Il2CppString>,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<*mut quest_hook::libil2cpp::Il2CppString>,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ExpressionToTokens", (expression, hasVariables))?;
        Ok(__cordl_ret.into())
    }
    pub fn FixUnaryOperators(
        tokens: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<*mut quest_hook::libil2cpp::Il2CppString>,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<*mut quest_hook::libil2cpp::Il2CppString>,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<*mut quest_hook::libil2cpp::Il2CppString>,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("FixUnaryOperators", (tokens))?;
        Ok(__cordl_ret.into())
    }
    pub fn InfixToRPN(
        tokens: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<*mut quest_hook::libil2cpp::Il2CppString>,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<*mut quest_hook::libil2cpp::Il2CppString>,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<*mut quest_hook::libil2cpp::Il2CppString>,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("InfixToRPN", (tokens))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsCommand(
        token: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IsCommand", (token))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsDelayedFunction(
        token: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IsDelayedFunction", (token))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsOperator(
        token: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IsOperator", (token))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsVariable(
        token: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IsVariable", (token))?;
        Ok(__cordl_ret.into())
    }
    pub fn NeedToPop(
        operatorStack: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::Stack_1<
                *mut quest_hook::libil2cpp::Il2CppString,
            >,
        >,
        newOperator: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::ExpressionEvaluator_Operator,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("NeedToPop", (operatorStack, newOperator))?;
        Ok(__cordl_ret.into())
    }
    pub fn PreFormatExpression(
        expression: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("PreFormatExpression", (expression))?;
        Ok(__cordl_ret.into())
    }
    pub fn TokenToOperator(
        token: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::ExpressionEvaluator_Operator>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::ExpressionEvaluator_Operator,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("TokenToOperator", (token))?;
        Ok(__cordl_ret.into())
    }
    pub fn TryParse<T>(
        expression: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        result: quest_hook::libil2cpp::ByRefMut<T>,
    ) -> quest_hook::libil2cpp::Result<bool>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("TryParse", (expression, result))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+ExpressionEvaluator")]
impl quest_hook::libil2cpp::ObjectType for crate::UnityEngine::ExpressionEvaluator {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "UnityEngine+ExpressionEvaluator+Associativity")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExpressionEvaluator_Associativity {
    Left = 0i32,
    Right = 1i32,
}
#[cfg(feature = "UnityEngine+ExpressionEvaluator+Associativity")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::ExpressionEvaluator_Associativity
    => "UnityEngine"."ExpressionEvaluator/Associativity"
);
#[cfg(feature = "UnityEngine+ExpressionEvaluator+Expression")]
#[repr(C)]
#[derive(Debug)]
pub struct ExpressionEvaluator_Expression {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub rpnTokens: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<*mut quest_hook::libil2cpp::Il2CppString>,
    >,
    pub hasVariables: bool,
}
#[cfg(feature = "UnityEngine+ExpressionEvaluator+Expression")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::ExpressionEvaluator_Expression =>
    "UnityEngine"."ExpressionEvaluator/Expression"
);
#[cfg(feature = "UnityEngine+ExpressionEvaluator+Expression")]
impl std::ops::Deref for crate::UnityEngine::ExpressionEvaluator_Expression {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+ExpressionEvaluator+Expression")]
impl std::ops::DerefMut for crate::UnityEngine::ExpressionEvaluator_Expression {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+ExpressionEvaluator+Expression")]
impl crate::UnityEngine::ExpressionEvaluator_Expression {
    pub fn Evaluate<T>(
        &mut self,
        value: quest_hook::libil2cpp::ByRefMut<T>,
        index: i32,
        count: i32,
    ) -> quest_hook::libil2cpp::Result<bool>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("Evaluate", (value, index, count))?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        expression: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (expression))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        expression: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (expression))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+ExpressionEvaluator+Expression")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::ExpressionEvaluator_Expression {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "UnityEngine+ExpressionEvaluator+Op")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExpressionEvaluator_Op {
    Add = 0i32,
    Ceil = 12i32,
    Cos = 9i32,
    Div = 3i32,
    Floor = 11i32,
    Linear = 15i32,
    _cordl_Mod = 4i32,
    Mul = 2i32,
    Neg = 5i32,
    Pow = 6i32,
    Rand = 14i32,
    Round = 13i32,
    Sin = 8i32,
    Sqrt = 7i32,
    Sub = 1i32,
    Tan = 10i32,
}
#[cfg(feature = "UnityEngine+ExpressionEvaluator+Op")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::ExpressionEvaluator_Op =>
    "UnityEngine"."ExpressionEvaluator/Op"
);
#[cfg(feature = "UnityEngine+ExpressionEvaluator+Operator")]
#[repr(C)]
#[derive(Debug)]
pub struct ExpressionEvaluator_Operator {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub op: crate::UnityEngine::ExpressionEvaluator_Op,
    pub precedence: i32,
    pub associativity: crate::UnityEngine::ExpressionEvaluator_Associativity,
    pub inputs: i32,
}
#[cfg(feature = "UnityEngine+ExpressionEvaluator+Operator")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::ExpressionEvaluator_Operator =>
    "UnityEngine"."ExpressionEvaluator/Operator"
);
#[cfg(feature = "UnityEngine+ExpressionEvaluator+Operator")]
impl std::ops::Deref for crate::UnityEngine::ExpressionEvaluator_Operator {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+ExpressionEvaluator+Operator")]
impl std::ops::DerefMut for crate::UnityEngine::ExpressionEvaluator_Operator {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+ExpressionEvaluator+Operator")]
impl crate::UnityEngine::ExpressionEvaluator_Operator {
    pub fn New(
        op: crate::UnityEngine::ExpressionEvaluator_Op,
        precedence: i32,
        inputs: i32,
        associativity: crate::UnityEngine::ExpressionEvaluator_Associativity,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (op, precedence, inputs, associativity))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        op: crate::UnityEngine::ExpressionEvaluator_Op,
        precedence: i32,
        inputs: i32,
        associativity: crate::UnityEngine::ExpressionEvaluator_Associativity,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (op, precedence, inputs, associativity))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+ExpressionEvaluator+Operator")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::ExpressionEvaluator_Operator {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "UnityEngine+ExpressionEvaluator+PcgRandom")]
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct ExpressionEvaluator_PcgRandom {
    pub increment: u64,
    pub state: u64,
}
#[cfg(feature = "UnityEngine+ExpressionEvaluator+PcgRandom")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::ExpressionEvaluator_PcgRandom =>
    "UnityEngine"."ExpressionEvaluator/PcgRandom"
);
#[cfg(feature = "UnityEngine+ExpressionEvaluator+PcgRandom")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::ExpressionEvaluator_PcgRandom {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+ExpressionEvaluator+PcgRandom")]
impl crate::UnityEngine::ExpressionEvaluator_PcgRandom {
    pub fn GetUInt(&mut self) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_ret: u32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "GetUInt",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn RotateRight(v: u32, rot: i32) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_ret: u32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("RotateRight", (v, rot))?;
        Ok(__cordl_ret.into())
    }
    pub fn Step(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Step",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn XshRr(s: u64) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_ret: u32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("XshRr", (s))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        state: u64,
        sequence: u64,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (state, sequence),
        )?;
        Ok(__cordl_ret.into())
    }
}
