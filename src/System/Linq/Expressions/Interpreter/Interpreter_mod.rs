#[cfg(feature = "System+Linq+Expressions+Interpreter+Interpreter")]
#[repr(C)]
#[derive(Debug)]
pub struct Interpreter {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _instructions: crate::System::Linq::Expressions::Interpreter::InstructionArray,
    pub _objects: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<*mut quest_hook::libil2cpp::Il2CppObject>,
    >,
    pub _labels: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<
            crate::System::Linq::Expressions::Interpreter::RuntimeLabel,
        >,
    >,
    pub _debugInfos: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<
            *mut crate::System::Linq::Expressions::Interpreter::DebugInfo,
        >,
    >,
    pub _Name_k__BackingField: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppString,
    >,
    pub _LocalCount_k__BackingField: i32,
    pub _ClosureVariables_k__BackingField: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::Dictionary_2<
            *mut crate::System::Linq::Expressions::ParameterExpression,
            *mut crate::System::Linq::Expressions::Interpreter::LocalVariable,
        >,
    >,
}
#[cfg(feature = "System+Linq+Expressions+Interpreter+Interpreter")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::Linq::Expressions::Interpreter::Interpreter =>
    "System.Linq.Expressions.Interpreter"."Interpreter"
);
#[cfg(feature = "System+Linq+Expressions+Interpreter+Interpreter")]
impl std::ops::Deref for crate::System::Linq::Expressions::Interpreter::Interpreter {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Linq+Expressions+Interpreter+Interpreter")]
impl std::ops::DerefMut for crate::System::Linq::Expressions::Interpreter::Interpreter {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Linq+Expressions+Interpreter+Interpreter")]
impl crate::System::Linq::Expressions::Interpreter::Interpreter {
    pub fn New(
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        locals: quest_hook::libil2cpp::Gc<
            crate::System::Linq::Expressions::Interpreter::LocalVariables,
        >,
        instructions: crate::System::Linq::Expressions::Interpreter::InstructionArray,
        debugInfos: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                *mut crate::System::Linq::Expressions::Interpreter::DebugInfo,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (name, locals, instructions, debugInfos))?;
        Ok(__cordl_object.into())
    }
    pub fn Run(
        &mut self,
        frame: quest_hook::libil2cpp::Gc<
            crate::System::Linq::Expressions::Interpreter::InterpretedFrame,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Run", (frame))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        locals: quest_hook::libil2cpp::Gc<
            crate::System::Linq::Expressions::Interpreter::LocalVariables,
        >,
        instructions: crate::System::Linq::Expressions::Interpreter::InstructionArray,
        debugInfos: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                *mut crate::System::Linq::Expressions::Interpreter::DebugInfo,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (name, locals, instructions, debugInfos))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_ClosureSize(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_ClosureSize", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_ClosureVariables(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::Dictionary_2<
                *mut crate::System::Linq::Expressions::ParameterExpression,
                *mut crate::System::Linq::Expressions::Interpreter::LocalVariable,
            >,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::Dictionary_2<
                *mut crate::System::Linq::Expressions::ParameterExpression,
                *mut crate::System::Linq::Expressions::Interpreter::LocalVariable,
            >,
        > = __cordl_object.invoke("get_ClosureVariables", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Instructions(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::System::Linq::Expressions::Interpreter::InstructionArray,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::Linq::Expressions::Interpreter::InstructionArray = __cordl_object
            .invoke("get_Instructions", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_LocalCount(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_LocalCount", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Name(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("get_Name", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Linq+Expressions+Interpreter+Interpreter")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Linq::Expressions::Interpreter::Interpreter {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
