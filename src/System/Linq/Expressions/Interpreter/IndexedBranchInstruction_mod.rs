#[cfg(feature = "System+Linq+Expressions+Interpreter+IndexedBranchInstruction")]
#[repr(C)]
#[derive(Debug)]
pub struct IndexedBranchInstruction {
    __cordl_parent: crate::System::Linq::Expressions::Interpreter::Instruction,
    pub _labelIndex: i32,
}
#[cfg(feature = "System+Linq+Expressions+Interpreter+IndexedBranchInstruction")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::Linq::Expressions::Interpreter::IndexedBranchInstruction =>
    "System.Linq.Expressions.Interpreter"."IndexedBranchInstruction"
);
#[cfg(feature = "System+Linq+Expressions+Interpreter+IndexedBranchInstruction")]
impl std::ops::Deref
for crate::System::Linq::Expressions::Interpreter::IndexedBranchInstruction {
    type Target = crate::System::Linq::Expressions::Interpreter::Instruction;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Linq+Expressions+Interpreter+IndexedBranchInstruction")]
impl std::ops::DerefMut
for crate::System::Linq::Expressions::Interpreter::IndexedBranchInstruction {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Linq+Expressions+Interpreter+IndexedBranchInstruction")]
impl crate::System::Linq::Expressions::Interpreter::IndexedBranchInstruction {
    pub fn ToDebugString(
        &mut self,
        instructionIndex: i32,
        cookie: *mut crate::System::Object,
        labelIndexer: *mut crate::System::Func_2<i32, i32>,
        objects: *mut crate::System::Collections::Generic::IReadOnlyList_1<
            *mut crate::System::Object,
        >,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("ToDebugString", (instructionIndex, cookie, labelIndexer, objects))?;
        Ok(__cordl_ret)
    }
    pub fn ToString(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("ToString", ())?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        labelIndex: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (labelIndex))?;
        Ok(__cordl_ret)
    }
    pub fn GetLabel(
        &mut self,
        frame: *mut crate::System::Linq::Expressions::Interpreter::InterpretedFrame,
    ) -> quest_hook::libil2cpp::Result<
        crate::System::Linq::Expressions::Interpreter::RuntimeLabel,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::Linq::Expressions::Interpreter::RuntimeLabel = __cordl_object
            .invoke("GetLabel", (frame))?;
        Ok(__cordl_ret)
    }
    pub fn New(labelIndex: i32) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (labelIndex))?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "System+Linq+Expressions+Interpreter+IndexedBranchInstruction")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Linq::Expressions::Interpreter::IndexedBranchInstruction {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
