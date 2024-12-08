#[cfg(feature = "System+Linq+Expressions+Interpreter+StringSwitchInstruction")]
#[repr(C)]
#[derive(Debug)]
pub struct StringSwitchInstruction {
    __cordl_parent: crate::System::Linq::Expressions::Interpreter::Instruction,
    pub _cases: *mut crate::System::Collections::Generic::Dictionary_2<
        *mut crate::System::String,
        i32,
    >,
    pub _nullCase: *mut crate::System::Runtime::CompilerServices::StrongBox_1<i32>,
}
#[cfg(feature = "System+Linq+Expressions+Interpreter+StringSwitchInstruction")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::Linq::Expressions::Interpreter::StringSwitchInstruction =>
    "System.Linq.Expressions.Interpreter"."StringSwitchInstruction"
);
#[cfg(feature = "System+Linq+Expressions+Interpreter+StringSwitchInstruction")]
impl std::ops::Deref
for crate::System::Linq::Expressions::Interpreter::StringSwitchInstruction {
    type Target = crate::System::Linq::Expressions::Interpreter::Instruction;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Linq+Expressions+Interpreter+StringSwitchInstruction")]
impl std::ops::DerefMut
for crate::System::Linq::Expressions::Interpreter::StringSwitchInstruction {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Linq+Expressions+Interpreter+StringSwitchInstruction")]
impl crate::System::Linq::Expressions::Interpreter::StringSwitchInstruction {
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
    pub fn get_ConsumedStack(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_ConsumedStack", ())?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        cases: *mut crate::System::Collections::Generic::Dictionary_2<
            *mut crate::System::String,
            i32,
        >,
        nullCase: *mut crate::System::Runtime::CompilerServices::StrongBox_1<i32>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (cases, nullCase))?;
        Ok(__cordl_ret)
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
    pub fn New(
        cases: *mut crate::System::Collections::Generic::Dictionary_2<
            *mut crate::System::String,
            i32,
        >,
        nullCase: *mut crate::System::Runtime::CompilerServices::StrongBox_1<i32>,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (cases, nullCase))?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "System+Linq+Expressions+Interpreter+StringSwitchInstruction")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Linq::Expressions::Interpreter::StringSwitchInstruction {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
