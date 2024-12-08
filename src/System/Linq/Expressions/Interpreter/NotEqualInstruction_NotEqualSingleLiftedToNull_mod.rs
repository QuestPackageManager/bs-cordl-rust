#[cfg(
    feature = "System+Linq+Expressions+Interpreter+NotEqualInstruction+NotEqualSingleLiftedToNull"
)]
#[repr(C)]
#[derive(Debug)]
pub struct NotEqualInstruction_NotEqualSingleLiftedToNull {
    __cordl_parent: crate::System::Linq::Expressions::Interpreter::NotEqualInstruction,
}
#[cfg(
    feature = "System+Linq+Expressions+Interpreter+NotEqualInstruction+NotEqualSingleLiftedToNull"
)]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::NotEqualInstruction_NotEqualSingleLiftedToNull =>
    "System.Linq.Expressions.Interpreter"
    ."NotEqualInstruction/NotEqualSingleLiftedToNull"
);
#[cfg(
    feature = "System+Linq+Expressions+Interpreter+NotEqualInstruction+NotEqualSingleLiftedToNull"
)]
impl std::ops::Deref
for crate::GlobalNamespace::NotEqualInstruction_NotEqualSingleLiftedToNull {
    type Target = crate::System::Linq::Expressions::Interpreter::NotEqualInstruction;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(
    feature = "System+Linq+Expressions+Interpreter+NotEqualInstruction+NotEqualSingleLiftedToNull"
)]
impl std::ops::DerefMut
for crate::GlobalNamespace::NotEqualInstruction_NotEqualSingleLiftedToNull {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(
    feature = "System+Linq+Expressions+Interpreter+NotEqualInstruction+NotEqualSingleLiftedToNull"
)]
impl crate::GlobalNamespace::NotEqualInstruction_NotEqualSingleLiftedToNull {
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
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
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(
    feature = "System+Linq+Expressions+Interpreter+NotEqualInstruction+NotEqualSingleLiftedToNull"
)]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::NotEqualInstruction_NotEqualSingleLiftedToNull {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
