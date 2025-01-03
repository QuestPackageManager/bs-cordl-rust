#[cfg(feature = "System+Linq+Expressions+Interpreter+EnterTryFaultInstruction")]
#[repr(C)]
#[derive(Debug)]
pub struct EnterTryFaultInstruction {
    __cordl_parent: crate::System::Linq::Expressions::Interpreter::IndexedBranchInstruction,
    pub _tryHandler: quest_hook::libil2cpp::Gc<
        crate::System::Linq::Expressions::Interpreter::TryFaultHandler,
    >,
}
#[cfg(feature = "System+Linq+Expressions+Interpreter+EnterTryFaultInstruction")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::Linq::Expressions::Interpreter::EnterTryFaultInstruction =>
    "System.Linq.Expressions.Interpreter"."EnterTryFaultInstruction"
);
#[cfg(feature = "System+Linq+Expressions+Interpreter+EnterTryFaultInstruction")]
impl std::ops::Deref
for crate::System::Linq::Expressions::Interpreter::EnterTryFaultInstruction {
    type Target = crate::System::Linq::Expressions::Interpreter::IndexedBranchInstruction;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Linq+Expressions+Interpreter+EnterTryFaultInstruction")]
impl std::ops::DerefMut
for crate::System::Linq::Expressions::Interpreter::EnterTryFaultInstruction {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Linq+Expressions+Interpreter+EnterTryFaultInstruction")]
impl crate::System::Linq::Expressions::Interpreter::EnterTryFaultInstruction {
    pub fn New(
        targetIndex: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (targetIndex))?;
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
    pub fn SetTryHandler(
        &mut self,
        tryHandler: quest_hook::libil2cpp::Gc<
            crate::System::Linq::Expressions::Interpreter::TryFaultHandler,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetTryHandler", (tryHandler))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        targetIndex: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (targetIndex))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Handler(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Linq::Expressions::Interpreter::TryFaultHandler,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Linq::Expressions::Interpreter::TryFaultHandler,
        > = __cordl_object.invoke("get_Handler", ())?;
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
    pub fn get_ProducedContinuations(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_ProducedContinuations", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Linq+Expressions+Interpreter+EnterTryFaultInstruction")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Linq::Expressions::Interpreter::EnterTryFaultInstruction {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
