#[cfg(feature = "UnityEngine+ExpressionEvaluator")]
#[repr(C)]
#[derive(Debug)]
pub struct ExpressionEvaluator {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "UnityEngine+ExpressionEvaluator")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::ExpressionEvaluator =>
    "UnityEngine"."ExpressionEvaluator"
);
#[cfg(feature = "UnityEngine+ExpressionEvaluator")]
impl std::ops::Deref for crate::UnityEngine::ExpressionEvaluator {
    type Target = crate::System::Object;
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
    #[cfg(feature = "UnityEngine+ExpressionEvaluator+__c")]
    pub type __c = crate::UnityEngine::ExpressionEvaluator___c;
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
    __cordl_parent: crate::System::Object,
    pub rpnTokens: *mut quest_hook::libil2cpp::Il2CppArray<*mut crate::System::String>,
    pub hasVariables: bool,
}
#[cfg(feature = "UnityEngine+ExpressionEvaluator+Expression")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::ExpressionEvaluator_Expression =>
    "UnityEngine"."ExpressionEvaluator/Expression"
);
#[cfg(feature = "UnityEngine+ExpressionEvaluator+Expression")]
impl std::ops::Deref for crate::UnityEngine::ExpressionEvaluator_Expression {
    type Target = crate::System::Object;
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
        Ok(__cordl_ret)
    }
    pub fn New(
        expression: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (expression))?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
        expression: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (expression))?;
        Ok(__cordl_ret)
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
    __cordl_parent: crate::System::Object,
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
    type Target = crate::System::Object;
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
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (op, precedence, inputs, associativity))?;
        Ok(__cordl_object)
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
        Ok(__cordl_ret)
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
#[derive(Debug, Clone)]
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
        Ok(__cordl_ret)
    }
    pub fn Step(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Step",
            (),
        )?;
        Ok(__cordl_ret)
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
        Ok(__cordl_ret)
    }
}
