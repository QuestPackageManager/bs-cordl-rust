#[cfg(feature = "System+Linq+Expressions+Interpreter+AssignLocalInstruction")]
#[repr(C)]
#[derive(Debug)]
pub struct AssignLocalInstruction {
    __cordl_parent: crate::System::Linq::Expressions::Interpreter::LocalAccessInstruction,
}
#[cfg(feature = "System+Linq+Expressions+Interpreter+AssignLocalInstruction")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::Linq::Expressions::Interpreter::AssignLocalInstruction =>
    "System.Linq.Expressions.Interpreter"."AssignLocalInstruction"
);
#[cfg(feature = "System+Linq+Expressions+Interpreter+AssignLocalInstruction")]
impl std::ops::Deref
for crate::System::Linq::Expressions::Interpreter::AssignLocalInstruction {
    type Target = crate::System::Linq::Expressions::Interpreter::LocalAccessInstruction;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Linq+Expressions+Interpreter+AssignLocalInstruction")]
impl std::ops::DerefMut
for crate::System::Linq::Expressions::Interpreter::AssignLocalInstruction {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Linq+Expressions+Interpreter+AssignLocalInstruction")]
impl crate::System::Linq::Expressions::Interpreter::AssignLocalInstruction {
    pub fn BoxIfIndexMatches(
        &mut self,
        index: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Linq::Expressions::Interpreter::Instruction,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Linq::Expressions::Interpreter::Instruction,
        > = __cordl_object.invoke("BoxIfIndexMatches", (index))?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        index: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (index))?;
        Ok(__cordl_object.into())
    }
    pub fn Run(
        &mut self,
        frame: quest_hook::libil2cpp::Gc<
            crate::System::Linq::Expressions::Interpreter::InterpretedFrame,
        >,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("Run", (frame))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        index: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (index))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_ConsumedStack(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_ConsumedStack", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_InstructionName(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("get_InstructionName", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_ProducedStack(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_ProducedStack", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Linq+Expressions+Interpreter+AssignLocalInstruction")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Linq::Expressions::Interpreter::AssignLocalInstruction {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "System+Linq+Expressions+Interpreter+AssignLocalInstruction")]
impl AsRef<crate::System::Linq::Expressions::Interpreter::IBoxableInstruction>
for crate::System::Linq::Expressions::Interpreter::AssignLocalInstruction {
    fn as_ref(
        &self,
    ) -> &crate::System::Linq::Expressions::Interpreter::IBoxableInstruction {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "System+Linq+Expressions+Interpreter+AssignLocalInstruction")]
impl AsMut<crate::System::Linq::Expressions::Interpreter::IBoxableInstruction>
for crate::System::Linq::Expressions::Interpreter::AssignLocalInstruction {
    fn as_mut(
        &mut self,
    ) -> &mut crate::System::Linq::Expressions::Interpreter::IBoxableInstruction {
        unsafe { std::mem::transmute(self) }
    }
}
