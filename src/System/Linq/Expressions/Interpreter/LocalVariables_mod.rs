#[cfg(feature = "System+Linq+Expressions+Interpreter+LocalVariables")]
#[repr(C)]
#[derive(Debug)]
pub struct LocalVariables {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _variables: quest_hook::libil2cpp::Gc<
        crate::System::Linq::Expressions::Interpreter::HybridReferenceDictionary_2<
            quest_hook::libil2cpp::Gc<
                crate::System::Linq::Expressions::ParameterExpression,
            >,
            quest_hook::libil2cpp::Gc<
                crate::System::Linq::Expressions::Interpreter::LocalVariables_VariableScope,
            >,
        >,
    >,
    pub _closureVariables: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::Dictionary_2<
            quest_hook::libil2cpp::Gc<
                crate::System::Linq::Expressions::ParameterExpression,
            >,
            quest_hook::libil2cpp::Gc<
                crate::System::Linq::Expressions::Interpreter::LocalVariable,
            >,
        >,
    >,
    pub _localCount: i32,
    pub _maxLocalCount: i32,
}
#[cfg(feature = "System+Linq+Expressions+Interpreter+LocalVariables")]
unsafe impl quest_hook::libil2cpp::Type
for crate::System::Linq::Expressions::Interpreter::LocalVariables {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "System.Linq.Expressions.Interpreter";
    const CLASS_NAME: &'static str = "LocalVariables";
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
#[cfg(feature = "System+Linq+Expressions+Interpreter+LocalVariables")]
impl std::ops::Deref for crate::System::Linq::Expressions::Interpreter::LocalVariables {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Linq+Expressions+Interpreter+LocalVariables")]
impl std::ops::DerefMut
for crate::System::Linq::Expressions::Interpreter::LocalVariables {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Linq+Expressions+Interpreter+LocalVariables")]
impl crate::System::Linq::Expressions::Interpreter::LocalVariables {
    #[cfg(feature = "System+Linq+Expressions+Interpreter+LocalVariables+VariableScope")]
    pub type VariableScope = crate::System::Linq::Expressions::Interpreter::LocalVariables_VariableScope;
    pub fn AddClosureVariable(
        &mut self,
        variable: quest_hook::libil2cpp::Gc<
            crate::System::Linq::Expressions::ParameterExpression,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Linq::Expressions::Interpreter::LocalVariable,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Linq::Expressions::Interpreter::LocalVariable,
        > = __cordl_object.invoke("AddClosureVariable", (variable))?;
        Ok(__cordl_ret.into())
    }
    pub fn Box(
        &mut self,
        variable: quest_hook::libil2cpp::Gc<
            crate::System::Linq::Expressions::ParameterExpression,
        >,
        instructions: quest_hook::libil2cpp::Gc<
            crate::System::Linq::Expressions::Interpreter::InstructionList,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Box", (variable, instructions))?;
        Ok(__cordl_ret.into())
    }
    pub fn DefineLocal(
        &mut self,
        variable: quest_hook::libil2cpp::Gc<
            crate::System::Linq::Expressions::ParameterExpression,
        >,
        start: i32,
    ) -> quest_hook::libil2cpp::Result<
        crate::System::Linq::Expressions::Interpreter::LocalDefinition,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::Linq::Expressions::Interpreter::LocalDefinition = __cordl_object
            .invoke("DefineLocal", (variable, start))?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn TryGetLocalOrClosure(
        &mut self,
        var: quest_hook::libil2cpp::Gc<
            crate::System::Linq::Expressions::ParameterExpression,
        >,
        local: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<
                crate::System::Linq::Expressions::Interpreter::LocalVariable,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("TryGetLocalOrClosure", (var, local))?;
        Ok(__cordl_ret.into())
    }
    pub fn UndefineLocal(
        &mut self,
        definition: crate::System::Linq::Expressions::Interpreter::LocalDefinition,
        end: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UndefineLocal", (definition, end))?;
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
    pub fn get_ClosureVariables(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::Dictionary_2<
                quest_hook::libil2cpp::Gc<
                    crate::System::Linq::Expressions::ParameterExpression,
                >,
                quest_hook::libil2cpp::Gc<
                    crate::System::Linq::Expressions::Interpreter::LocalVariable,
                >,
            >,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::Dictionary_2<
                quest_hook::libil2cpp::Gc<
                    crate::System::Linq::Expressions::ParameterExpression,
                >,
                quest_hook::libil2cpp::Gc<
                    crate::System::Linq::Expressions::Interpreter::LocalVariable,
                >,
            >,
        > = __cordl_object.invoke("get_ClosureVariables", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_LocalCount(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_LocalCount", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Linq+Expressions+Interpreter+LocalVariables")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Linq::Expressions::Interpreter::LocalVariables {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "System+Linq+Expressions+Interpreter+LocalVariables+VariableScope")]
#[repr(C)]
#[derive(Debug)]
pub struct LocalVariables_VariableScope {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub Start: i32,
    pub Stop: i32,
    pub Variable: quest_hook::libil2cpp::Gc<
        crate::System::Linq::Expressions::Interpreter::LocalVariable,
    >,
    pub Parent: quest_hook::libil2cpp::Gc<
        crate::System::Linq::Expressions::Interpreter::LocalVariables_VariableScope,
    >,
    pub ChildScopes: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::List_1<
            quest_hook::libil2cpp::Gc<
                crate::System::Linq::Expressions::Interpreter::LocalVariables_VariableScope,
            >,
        >,
    >,
}
#[cfg(feature = "System+Linq+Expressions+Interpreter+LocalVariables+VariableScope")]
unsafe impl quest_hook::libil2cpp::Type
for crate::System::Linq::Expressions::Interpreter::LocalVariables_VariableScope {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "System.Linq.Expressions.Interpreter";
    const CLASS_NAME: &'static str = "VariableScope";
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
#[cfg(feature = "System+Linq+Expressions+Interpreter+LocalVariables+VariableScope")]
impl std::ops::Deref
for crate::System::Linq::Expressions::Interpreter::LocalVariables_VariableScope {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Linq+Expressions+Interpreter+LocalVariables+VariableScope")]
impl std::ops::DerefMut
for crate::System::Linq::Expressions::Interpreter::LocalVariables_VariableScope {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Linq+Expressions+Interpreter+LocalVariables+VariableScope")]
impl crate::System::Linq::Expressions::Interpreter::LocalVariables_VariableScope {
    pub fn New(
        variable: quest_hook::libil2cpp::Gc<
            crate::System::Linq::Expressions::Interpreter::LocalVariable,
        >,
        start: i32,
        parent: quest_hook::libil2cpp::Gc<
            crate::System::Linq::Expressions::Interpreter::LocalVariables_VariableScope,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (variable, start, parent))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        variable: quest_hook::libil2cpp::Gc<
            crate::System::Linq::Expressions::Interpreter::LocalVariable,
        >,
        start: i32,
        parent: quest_hook::libil2cpp::Gc<
            crate::System::Linq::Expressions::Interpreter::LocalVariables_VariableScope,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (variable, start, parent))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Linq+Expressions+Interpreter+LocalVariables+VariableScope")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Linq::Expressions::Interpreter::LocalVariables_VariableScope {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
