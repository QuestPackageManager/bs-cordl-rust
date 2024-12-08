#[cfg(
    feature = "System+Linq+Expressions+Interpreter+RightShiftInstruction+RightShiftByte"
)]
#[repr(C)]
#[derive(Debug)]
pub struct RightShiftInstruction_RightShiftByte {
    __cordl_parent: crate::System::Linq::Expressions::Interpreter::RightShiftInstruction,
}
#[cfg(
    feature = "System+Linq+Expressions+Interpreter+RightShiftInstruction+RightShiftByte"
)]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::RightShiftInstruction_RightShiftByte =>
    "System.Linq.Expressions.Interpreter"."RightShiftInstruction/RightShiftByte"
);
#[cfg(
    feature = "System+Linq+Expressions+Interpreter+RightShiftInstruction+RightShiftByte"
)]
impl std::ops::Deref for crate::GlobalNamespace::RightShiftInstruction_RightShiftByte {
    type Target = crate::System::Linq::Expressions::Interpreter::RightShiftInstruction;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(
    feature = "System+Linq+Expressions+Interpreter+RightShiftInstruction+RightShiftByte"
)]
impl std::ops::DerefMut
for crate::GlobalNamespace::RightShiftInstruction_RightShiftByte {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(
    feature = "System+Linq+Expressions+Interpreter+RightShiftInstruction+RightShiftByte"
)]
impl crate::GlobalNamespace::RightShiftInstruction_RightShiftByte {
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
    pub fn New() -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
}
#[cfg(
    feature = "System+Linq+Expressions+Interpreter+RightShiftInstruction+RightShiftByte"
)]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::RightShiftInstruction_RightShiftByte {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
