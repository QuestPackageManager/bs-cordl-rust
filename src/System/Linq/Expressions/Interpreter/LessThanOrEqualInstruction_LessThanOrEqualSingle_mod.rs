#[cfg(
    feature = "System+Linq+Expressions+Interpreter+LessThanOrEqualInstruction+LessThanOrEqualSingle"
)]
#[repr(C)]
#[derive(Debug)]
pub struct LessThanOrEqualInstruction_LessThanOrEqualSingle {
    __cordl_parent: crate::System::Linq::Expressions::Interpreter::LessThanOrEqualInstruction,
}
#[cfg(
    feature = "System+Linq+Expressions+Interpreter+LessThanOrEqualInstruction+LessThanOrEqualSingle"
)]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::LessThanOrEqualInstruction_LessThanOrEqualSingle =>
    "System.Linq.Expressions.Interpreter"
    ."LessThanOrEqualInstruction/LessThanOrEqualSingle"
);
#[cfg(
    feature = "System+Linq+Expressions+Interpreter+LessThanOrEqualInstruction+LessThanOrEqualSingle"
)]
impl std::ops::Deref
for crate::GlobalNamespace::LessThanOrEqualInstruction_LessThanOrEqualSingle {
    type Target = crate::System::Linq::Expressions::Interpreter::LessThanOrEqualInstruction;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(
    feature = "System+Linq+Expressions+Interpreter+LessThanOrEqualInstruction+LessThanOrEqualSingle"
)]
impl std::ops::DerefMut
for crate::GlobalNamespace::LessThanOrEqualInstruction_LessThanOrEqualSingle {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(
    feature = "System+Linq+Expressions+Interpreter+LessThanOrEqualInstruction+LessThanOrEqualSingle"
)]
impl crate::GlobalNamespace::LessThanOrEqualInstruction_LessThanOrEqualSingle {
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
        nullValue: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (nullValue))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        nullValue: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (nullValue))?;
        Ok(__cordl_object)
    }
}
#[cfg(
    feature = "System+Linq+Expressions+Interpreter+LessThanOrEqualInstruction+LessThanOrEqualSingle"
)]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::LessThanOrEqualInstruction_LessThanOrEqualSingle {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
