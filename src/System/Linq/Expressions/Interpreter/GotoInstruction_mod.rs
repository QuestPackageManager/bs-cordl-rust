#[cfg(feature = "System+Linq+Expressions+Interpreter+GotoInstruction")]
#[repr(C)]
#[derive(Debug)]
pub struct GotoInstruction {
    __cordl_parent: crate::System::Linq::Expressions::Interpreter::IndexedBranchInstruction,
    pub _hasResult: bool,
    pub _hasValue: bool,
    pub _labelTargetGetsValue: bool,
}
#[cfg(feature = "System+Linq+Expressions+Interpreter+GotoInstruction")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::Linq::Expressions::Interpreter::GotoInstruction =>
    "System.Linq.Expressions.Interpreter"."GotoInstruction"
);
#[cfg(feature = "System+Linq+Expressions+Interpreter+GotoInstruction")]
impl std::ops::Deref for crate::System::Linq::Expressions::Interpreter::GotoInstruction {
    type Target = crate::System::Linq::Expressions::Interpreter::IndexedBranchInstruction;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Linq+Expressions+Interpreter+GotoInstruction")]
impl std::ops::DerefMut
for crate::System::Linq::Expressions::Interpreter::GotoInstruction {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Linq+Expressions+Interpreter+GotoInstruction")]
impl crate::System::Linq::Expressions::Interpreter::GotoInstruction {
    pub fn Create(
        labelIndex: i32,
        hasResult: bool,
        hasValue: bool,
        labelTargetGetsValue: bool,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Linq::Expressions::Interpreter::GotoInstruction,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Linq::Expressions::Interpreter::GotoInstruction,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Create", (labelIndex, hasResult, hasValue, labelTargetGetsValue))?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        targetIndex: i32,
        hasResult: bool,
        hasValue: bool,
        labelTargetGetsValue: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (targetIndex, hasResult, hasValue, labelTargetGetsValue),
            )?;
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
        targetIndex: i32,
        hasResult: bool,
        hasValue: bool,
        labelTargetGetsValue: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (targetIndex, hasResult, hasValue, labelTargetGetsValue))?;
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
#[cfg(feature = "System+Linq+Expressions+Interpreter+GotoInstruction")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Linq::Expressions::Interpreter::GotoInstruction {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
