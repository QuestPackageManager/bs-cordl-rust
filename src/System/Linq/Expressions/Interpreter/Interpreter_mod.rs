#[cfg(feature = "System+Linq+Expressions+Interpreter+Interpreter")]
#[repr(C)]
#[derive(Debug)]
pub struct Interpreter {
    __cordl_parent: crate::System::Object,
    pub _instructions: crate::System::Linq::Expressions::Interpreter::InstructionArray,
    pub _objects: *mut quest_hook::libil2cpp::Il2CppArray<*mut crate::System::Object>,
    pub _labels: *mut quest_hook::libil2cpp::Il2CppArray<
        crate::System::Linq::Expressions::Interpreter::RuntimeLabel,
    >,
    pub _debugInfos: *mut quest_hook::libil2cpp::Il2CppArray<
        *mut crate::System::Linq::Expressions::Interpreter::DebugInfo,
    >,
    pub _Name_k__BackingField: *mut crate::System::String,
    pub _LocalCount_k__BackingField: i32,
    pub _ClosureVariables_k__BackingField: *mut crate::System::Collections::Generic::Dictionary_2<
        *mut crate::System::Linq::Expressions::ParameterExpression,
        *mut crate::System::Linq::Expressions::Interpreter::LocalVariable,
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
    type Target = crate::System::Object;
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
    pub fn _ctor(
        &mut self,
        name: *mut crate::System::String,
        locals: *mut crate::System::Linq::Expressions::Interpreter::LocalVariables,
        instructions: crate::System::Linq::Expressions::Interpreter::InstructionArray,
        debugInfos: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::System::Linq::Expressions::Interpreter::DebugInfo,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (name, locals, instructions, debugInfos))?;
        Ok(__cordl_ret)
    }
    pub fn get_Name(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("get_Name", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_ClosureSize(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_ClosureSize", ())?;
        Ok(__cordl_ret)
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
    pub fn Run(
        &mut self,
        frame: *mut crate::System::Linq::Expressions::Interpreter::InterpretedFrame,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Run", (frame))?;
        Ok(__cordl_ret)
    }
    pub fn get_LocalCount(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_LocalCount", ())?;
        Ok(__cordl_ret)
    }
    pub fn New(
        name: *mut crate::System::String,
        locals: *mut crate::System::Linq::Expressions::Interpreter::LocalVariables,
        instructions: crate::System::Linq::Expressions::Interpreter::InstructionArray,
        debugInfos: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::System::Linq::Expressions::Interpreter::DebugInfo,
        >,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (name, locals, instructions, debugInfos))?;
        Ok(__cordl_object)
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
