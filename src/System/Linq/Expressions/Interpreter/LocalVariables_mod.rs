#[cfg(feature = "System+Linq+Expressions+Interpreter+LocalVariables")]
#[repr(C)]
#[derive(Debug)]
pub struct LocalVariables {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _variables: *mut crate::System::Linq::Expressions::Interpreter::HybridReferenceDictionary_2<
        *mut crate::System::Linq::Expressions::ParameterExpression,
        *mut crate::System::Linq::Expressions::Interpreter::LocalVariables_VariableScope,
    >,
    pub _closureVariables: *mut crate::System::Collections::Generic::Dictionary_2<
        *mut crate::System::Linq::Expressions::ParameterExpression,
        *mut crate::System::Linq::Expressions::Interpreter::LocalVariable,
    >,
    pub _localCount: i32,
    pub _maxLocalCount: i32,
}
#[cfg(feature = "System+Linq+Expressions+Interpreter+LocalVariables")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::Linq::Expressions::Interpreter::LocalVariables =>
    "System.Linq.Expressions.Interpreter"."LocalVariables"
);
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
        variable: *mut crate::System::Linq::Expressions::ParameterExpression,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Linq::Expressions::Interpreter::LocalVariable,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Linq::Expressions::Interpreter::LocalVariable = __cordl_object
            .invoke("AddClosureVariable", (variable))?;
        Ok(__cordl_ret)
    }
    pub fn Box(
        &mut self,
        variable: *mut crate::System::Linq::Expressions::ParameterExpression,
        instructions: *mut crate::System::Linq::Expressions::Interpreter::InstructionList,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Box", (variable, instructions))?;
        Ok(__cordl_ret)
    }
    pub fn DefineLocal(
        &mut self,
        variable: *mut crate::System::Linq::Expressions::ParameterExpression,
        start: i32,
    ) -> quest_hook::libil2cpp::Result<
        crate::System::Linq::Expressions::Interpreter::LocalDefinition,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::Linq::Expressions::Interpreter::LocalDefinition = __cordl_object
            .invoke("DefineLocal", (variable, start))?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn TryGetLocalOrClosure(
        &mut self,
        var: *mut crate::System::Linq::Expressions::ParameterExpression,
        local: quest_hook::libil2cpp::ByRefMut<
            *mut crate::System::Linq::Expressions::Interpreter::LocalVariable,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("TryGetLocalOrClosure", (var, local))?;
        Ok(__cordl_ret)
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
    pub fn get_ClosureVariables(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Collections::Generic::Dictionary_2<
            *mut crate::System::Linq::Expressions::ParameterExpression,
            *mut crate::System::Linq::Expressions::Interpreter::LocalVariable,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::Generic::Dictionary_2<
            *mut crate::System::Linq::Expressions::ParameterExpression,
            *mut crate::System::Linq::Expressions::Interpreter::LocalVariable,
        > = __cordl_object.invoke("get_ClosureVariables", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_LocalCount(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_LocalCount", ())?;
        Ok(__cordl_ret)
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
    pub Variable: *mut crate::System::Linq::Expressions::Interpreter::LocalVariable,
    pub Parent: *mut crate::System::Linq::Expressions::Interpreter::LocalVariables_VariableScope,
    pub ChildScopes: *mut crate::System::Collections::Generic::List_1<
        *mut crate::System::Linq::Expressions::Interpreter::LocalVariables_VariableScope,
    >,
}
#[cfg(feature = "System+Linq+Expressions+Interpreter+LocalVariables+VariableScope")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::Linq::Expressions::Interpreter::LocalVariables_VariableScope =>
    "System.Linq.Expressions.Interpreter"."LocalVariables/VariableScope"
);
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
        variable: *mut crate::System::Linq::Expressions::Interpreter::LocalVariable,
        start: i32,
        parent: *mut crate::System::Linq::Expressions::Interpreter::LocalVariables_VariableScope,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (variable, start, parent))?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
        variable: *mut crate::System::Linq::Expressions::Interpreter::LocalVariable,
        start: i32,
        parent: *mut crate::System::Linq::Expressions::Interpreter::LocalVariables_VariableScope,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (variable, start, parent))?;
        Ok(__cordl_ret)
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
