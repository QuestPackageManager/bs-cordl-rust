#[cfg(
    feature = "System+Linq+Expressions+Interpreter+InitializeLocalInstruction+MutableBox"
)]
#[repr(C)]
#[derive(Debug)]
pub struct InitializeLocalInstruction_MutableBox {
    __cordl_parent: crate::System::Linq::Expressions::Interpreter::InitializeLocalInstruction,
    pub _type: quest_hook::libil2cpp::Gc<crate::System::Type>,
}
#[cfg(
    feature = "System+Linq+Expressions+Interpreter+InitializeLocalInstruction+MutableBox"
)]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::InitializeLocalInstruction_MutableBox =>
    "System.Linq.Expressions.Interpreter"."InitializeLocalInstruction/MutableBox"
);
#[cfg(
    feature = "System+Linq+Expressions+Interpreter+InitializeLocalInstruction+MutableBox"
)]
impl std::ops::Deref for crate::GlobalNamespace::InitializeLocalInstruction_MutableBox {
    type Target = crate::System::Linq::Expressions::Interpreter::InitializeLocalInstruction;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(
    feature = "System+Linq+Expressions+Interpreter+InitializeLocalInstruction+MutableBox"
)]
impl std::ops::DerefMut
for crate::GlobalNamespace::InitializeLocalInstruction_MutableBox {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(
    feature = "System+Linq+Expressions+Interpreter+InitializeLocalInstruction+MutableBox"
)]
impl crate::GlobalNamespace::InitializeLocalInstruction_MutableBox {
    pub fn New(
        index: i32,
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (index, _cordl_type))?;
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
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (index, _cordl_type))?;
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
}
#[cfg(
    feature = "System+Linq+Expressions+Interpreter+InitializeLocalInstruction+MutableBox"
)]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::InitializeLocalInstruction_MutableBox {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
