#[cfg(feature = "System+Linq+Expressions+Interpreter+LocalAccessInstruction")]
#[repr(C)]
#[derive(Debug)]
pub struct LocalAccessInstruction {
    __cordl_parent: crate::System::Linq::Expressions::Interpreter::Instruction,
    pub _index: i32,
}
#[cfg(feature = "System+Linq+Expressions+Interpreter+LocalAccessInstruction")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::Linq::Expressions::Interpreter::LocalAccessInstruction =>
    "System.Linq.Expressions.Interpreter"."LocalAccessInstruction"
);
#[cfg(feature = "System+Linq+Expressions+Interpreter+LocalAccessInstruction")]
impl std::ops::Deref
for crate::System::Linq::Expressions::Interpreter::LocalAccessInstruction {
    type Target = crate::System::Linq::Expressions::Interpreter::Instruction;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Linq+Expressions+Interpreter+LocalAccessInstruction")]
impl std::ops::DerefMut
for crate::System::Linq::Expressions::Interpreter::LocalAccessInstruction {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Linq+Expressions+Interpreter+LocalAccessInstruction")]
impl crate::System::Linq::Expressions::Interpreter::LocalAccessInstruction {
    pub fn New(index: i32) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (index))?;
        Ok(__cordl_object)
    }
    pub fn ToDebugString(
        &mut self,
        instructionIndex: i32,
        cookie: *mut quest_hook::libil2cpp::Il2CppObject,
        labelIndexer: *mut crate::System::Func_2<i32, i32>,
        objects: *mut crate::System::Collections::Generic::IReadOnlyList_1<
            *mut quest_hook::libil2cpp::Il2CppObject,
        >,
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppString> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppString = __cordl_object
            .invoke("ToDebugString", (instructionIndex, cookie, labelIndexer, objects))?;
        Ok(__cordl_ret)
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
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "System+Linq+Expressions+Interpreter+LocalAccessInstruction")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Linq::Expressions::Interpreter::LocalAccessInstruction {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
