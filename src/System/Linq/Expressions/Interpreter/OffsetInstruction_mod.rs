#[cfg(feature = "System+Linq+Expressions+Interpreter+OffsetInstruction")]
#[repr(C)]
#[derive(Debug)]
pub struct OffsetInstruction {
    __cordl_parent: crate::System::Linq::Expressions::Interpreter::Instruction,
    pub _offset: i32,
}
#[cfg(feature = "System+Linq+Expressions+Interpreter+OffsetInstruction")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::Linq::Expressions::Interpreter::OffsetInstruction =>
    "System.Linq.Expressions.Interpreter"."OffsetInstruction"
);
#[cfg(feature = "System+Linq+Expressions+Interpreter+OffsetInstruction")]
impl std::ops::Deref
for crate::System::Linq::Expressions::Interpreter::OffsetInstruction {
    type Target = crate::System::Linq::Expressions::Interpreter::Instruction;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Linq+Expressions+Interpreter+OffsetInstruction")]
impl std::ops::DerefMut
for crate::System::Linq::Expressions::Interpreter::OffsetInstruction {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Linq+Expressions+Interpreter+OffsetInstruction")]
impl crate::System::Linq::Expressions::Interpreter::OffsetInstruction {
    pub fn Fixup(
        &mut self,
        offset: i32,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Linq::Expressions::Interpreter::Instruction,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Linq::Expressions::Interpreter::Instruction = __cordl_object
            .invoke("Fixup", (offset))?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
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
    pub fn ToString(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppString> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppString = __cordl_object
            .invoke("ToString", ())?;
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
    pub fn get_Cache(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::System::Linq::Expressions::Interpreter::Instruction,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::System::Linq::Expressions::Interpreter::Instruction,
        > = __cordl_object.invoke("get_Cache", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "System+Linq+Expressions+Interpreter+OffsetInstruction")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Linq::Expressions::Interpreter::OffsetInstruction {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
