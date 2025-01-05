#[cfg(
    feature = "System+Linq+Expressions+Interpreter+InitializeLocalInstruction+ImmutableValue"
)]
#[repr(C)]
#[derive(Debug)]
pub struct InitializeLocalInstruction_ImmutableValue {
    __cordl_parent: crate::System::Linq::Expressions::Interpreter::InitializeLocalInstruction,
    pub _defaultValue: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
}
#[cfg(
    feature = "System+Linq+Expressions+Interpreter+InitializeLocalInstruction+ImmutableValue"
)]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::InitializeLocalInstruction_ImmutableValue =>
    "System.Linq.Expressions.Interpreter"."InitializeLocalInstruction/ImmutableValue"
);
#[cfg(
    feature = "System+Linq+Expressions+Interpreter+InitializeLocalInstruction+ImmutableValue"
)]
impl std::ops::Deref
for crate::GlobalNamespace::InitializeLocalInstruction_ImmutableValue {
    type Target = crate::System::Linq::Expressions::Interpreter::InitializeLocalInstruction;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(
    feature = "System+Linq+Expressions+Interpreter+InitializeLocalInstruction+ImmutableValue"
)]
impl std::ops::DerefMut
for crate::GlobalNamespace::InitializeLocalInstruction_ImmutableValue {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(
    feature = "System+Linq+Expressions+Interpreter+InitializeLocalInstruction+ImmutableValue"
)]
impl crate::GlobalNamespace::InitializeLocalInstruction_ImmutableValue {
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
        defaultValue: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (index, defaultValue))?;
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
        defaultValue: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (index, defaultValue))?;
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
    feature = "System+Linq+Expressions+Interpreter+InitializeLocalInstruction+ImmutableValue"
)]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::InitializeLocalInstruction_ImmutableValue {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(
    feature = "System+Linq+Expressions+Interpreter+InitializeLocalInstruction+ImmutableValue"
)]
impl AsRef<crate::System::Linq::Expressions::Interpreter::IBoxableInstruction>
for crate::GlobalNamespace::InitializeLocalInstruction_ImmutableValue {
    fn as_ref(
        &self,
    ) -> &crate::System::Linq::Expressions::Interpreter::IBoxableInstruction {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(
    feature = "System+Linq+Expressions+Interpreter+InitializeLocalInstruction+ImmutableValue"
)]
impl AsMut<crate::System::Linq::Expressions::Interpreter::IBoxableInstruction>
for crate::GlobalNamespace::InitializeLocalInstruction_ImmutableValue {
    fn as_mut(
        &mut self,
    ) -> &mut crate::System::Linq::Expressions::Interpreter::IBoxableInstruction {
        unsafe { std::mem::transmute(self) }
    }
}
