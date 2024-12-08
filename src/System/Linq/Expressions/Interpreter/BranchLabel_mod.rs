#[cfg(feature = "System+Linq+Expressions+Interpreter+BranchLabel")]
#[repr(C)]
#[derive(Debug)]
pub struct BranchLabel {
    __cordl_parent: crate::System::Object,
    pub _targetIndex: i32,
    pub _stackDepth: i32,
    pub _continuationStackDepth: i32,
    pub _forwardBranchFixups: *mut crate::System::Collections::Generic::List_1<i32>,
    pub _LabelIndex_k__BackingField: i32,
}
#[cfg(feature = "System+Linq+Expressions+Interpreter+BranchLabel")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::Linq::Expressions::Interpreter::BranchLabel =>
    "System.Linq.Expressions.Interpreter"."BranchLabel"
);
#[cfg(feature = "System+Linq+Expressions+Interpreter+BranchLabel")]
impl std::ops::Deref for crate::System::Linq::Expressions::Interpreter::BranchLabel {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Linq+Expressions+Interpreter+BranchLabel")]
impl std::ops::DerefMut for crate::System::Linq::Expressions::Interpreter::BranchLabel {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Linq+Expressions+Interpreter+BranchLabel")]
impl crate::System::Linq::Expressions::Interpreter::BranchLabel {
    pub fn ToRuntimeLabel(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::System::Linq::Expressions::Interpreter::RuntimeLabel,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::Linq::Expressions::Interpreter::RuntimeLabel = __cordl_object
            .invoke("ToRuntimeLabel", ())?;
        Ok(__cordl_ret)
    }
    pub fn Mark(
        &mut self,
        instructions: *mut crate::System::Linq::Expressions::Interpreter::InstructionList,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Mark", (instructions))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret)
    }
    pub fn FixupBranch(
        &mut self,
        instructions: *mut crate::System::Linq::Expressions::Interpreter::InstructionList,
        branchIndex: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("FixupBranch", (instructions, branchIndex))?;
        Ok(__cordl_ret)
    }
    pub fn get_TargetIndex(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_TargetIndex", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_LabelIndex(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_LabelIndex", ())?;
        Ok(__cordl_ret)
    }
    pub fn AddBranch(
        &mut self,
        instructions: *mut crate::System::Linq::Expressions::Interpreter::InstructionList,
        branchIndex: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddBranch", (instructions, branchIndex))?;
        Ok(__cordl_ret)
    }
    pub fn get_HasRuntimeLabel(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_HasRuntimeLabel", ())?;
        Ok(__cordl_ret)
    }
    pub fn set_LabelIndex(
        &mut self,
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_LabelIndex", (value))?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "System+Linq+Expressions+Interpreter+BranchLabel")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Linq::Expressions::Interpreter::BranchLabel {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
