#[cfg(
    feature = "System+Linq+Expressions+Interpreter+InitializeLocalInstruction+ImmutableBox"
)]
#[repr(C)]
#[derive(Debug)]
pub struct InitializeLocalInstruction_ImmutableBox {
    __cordl_parent: crate::System::Linq::Expressions::Interpreter::InitializeLocalInstruction,
    pub _defaultValue: *mut crate::System::Object,
}
#[cfg(
    feature = "System+Linq+Expressions+Interpreter+InitializeLocalInstruction+ImmutableBox"
)]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::InitializeLocalInstruction_ImmutableBox =>
    "System.Linq.Expressions.Interpreter"."InitializeLocalInstruction/ImmutableBox"
);
#[cfg(
    feature = "System+Linq+Expressions+Interpreter+InitializeLocalInstruction+ImmutableBox"
)]
impl std::ops::Deref
for crate::GlobalNamespace::InitializeLocalInstruction_ImmutableBox {
    type Target = crate::System::Linq::Expressions::Interpreter::InitializeLocalInstruction;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(
    feature = "System+Linq+Expressions+Interpreter+InitializeLocalInstruction+ImmutableBox"
)]
impl std::ops::DerefMut
for crate::GlobalNamespace::InitializeLocalInstruction_ImmutableBox {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(
    feature = "System+Linq+Expressions+Interpreter+InitializeLocalInstruction+ImmutableBox"
)]
impl crate::GlobalNamespace::InitializeLocalInstruction_ImmutableBox {
    pub fn New(
        index: i32,
        defaultValue: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (index, defaultValue))?;
        Ok(__cordl_object)
    }
    pub fn Run(
        &mut self,
        frame: *mut crate::System::Linq::Expressions::Interpreter::InterpretedFrame,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("Run", (frame))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        index: i32,
        defaultValue: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (index, defaultValue))?;
        Ok(__cordl_ret)
    }
    pub fn get_InstructionName(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("get_InstructionName", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(
    feature = "System+Linq+Expressions+Interpreter+InitializeLocalInstruction+ImmutableBox"
)]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::InitializeLocalInstruction_ImmutableBox {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
