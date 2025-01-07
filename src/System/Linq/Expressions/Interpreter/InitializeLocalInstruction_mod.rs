#[cfg(feature = "System+Linq+Expressions+Interpreter+InitializeLocalInstruction")]
#[repr(C)]
#[derive(Debug)]
pub struct InitializeLocalInstruction {
    __cordl_parent: crate::System::Linq::Expressions::Interpreter::LocalAccessInstruction,
}
#[cfg(feature = "System+Linq+Expressions+Interpreter+InitializeLocalInstruction")]
unsafe impl quest_hook::libil2cpp::Type
for crate::System::Linq::Expressions::Interpreter::InitializeLocalInstruction {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "System.Linq.Expressions.Interpreter";
    const CLASS_NAME: &'static str = "InitializeLocalInstruction";
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
#[cfg(feature = "System+Linq+Expressions+Interpreter+InitializeLocalInstruction")]
impl std::ops::Deref
for crate::System::Linq::Expressions::Interpreter::InitializeLocalInstruction {
    type Target = crate::System::Linq::Expressions::Interpreter::LocalAccessInstruction;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Linq+Expressions+Interpreter+InitializeLocalInstruction")]
impl std::ops::DerefMut
for crate::System::Linq::Expressions::Interpreter::InitializeLocalInstruction {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Linq+Expressions+Interpreter+InitializeLocalInstruction")]
impl crate::System::Linq::Expressions::Interpreter::InitializeLocalInstruction {
    #[cfg(
        feature = "System+Linq+Expressions+Interpreter+InitializeLocalInstruction+ImmutableBox"
    )]
    pub type ImmutableBox = crate::GlobalNamespace::InitializeLocalInstruction_ImmutableBox;
    #[cfg(
        feature = "System+Linq+Expressions+Interpreter+InitializeLocalInstruction+ImmutableRefBox"
    )]
    pub type ImmutableRefBox = crate::GlobalNamespace::InitializeLocalInstruction_ImmutableRefBox;
    #[cfg(
        feature = "System+Linq+Expressions+Interpreter+InitializeLocalInstruction+ImmutableValue"
    )]
    pub type ImmutableValue = crate::GlobalNamespace::InitializeLocalInstruction_ImmutableValue;
    #[cfg(
        feature = "System+Linq+Expressions+Interpreter+InitializeLocalInstruction+MutableBox"
    )]
    pub type MutableBox = crate::GlobalNamespace::InitializeLocalInstruction_MutableBox;
    #[cfg(
        feature = "System+Linq+Expressions+Interpreter+InitializeLocalInstruction+MutableValue"
    )]
    pub type MutableValue = crate::GlobalNamespace::InitializeLocalInstruction_MutableValue;
    #[cfg(
        feature = "System+Linq+Expressions+Interpreter+InitializeLocalInstruction+Parameter"
    )]
    pub type Parameter = crate::GlobalNamespace::InitializeLocalInstruction_Parameter;
    #[cfg(
        feature = "System+Linq+Expressions+Interpreter+InitializeLocalInstruction+ParameterBox"
    )]
    pub type ParameterBox = crate::GlobalNamespace::InitializeLocalInstruction_ParameterBox;
    #[cfg(
        feature = "System+Linq+Expressions+Interpreter+InitializeLocalInstruction+Reference"
    )]
    pub type Reference = crate::GlobalNamespace::InitializeLocalInstruction_Reference;
    pub fn New(
        index: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (index))?;
        Ok(__cordl_object.into())
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
}
#[cfg(feature = "System+Linq+Expressions+Interpreter+InitializeLocalInstruction")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Linq::Expressions::Interpreter::InitializeLocalInstruction {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
