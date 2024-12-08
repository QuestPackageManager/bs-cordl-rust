#[cfg(
    feature = "System+Linq+Expressions+Interpreter+LessThanInstruction+LessThanSingle"
)]
#[repr(C)]
#[derive(Debug)]
pub struct LessThanInstruction_LessThanSingle {
    __cordl_parent: crate::System::Linq::Expressions::Interpreter::LessThanInstruction,
}
#[cfg(
    feature = "System+Linq+Expressions+Interpreter+LessThanInstruction+LessThanSingle"
)]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::LessThanInstruction_LessThanSingle =>
    "System.Linq.Expressions.Interpreter"."LessThanInstruction/LessThanSingle"
);
#[cfg(
    feature = "System+Linq+Expressions+Interpreter+LessThanInstruction+LessThanSingle"
)]
impl std::ops::Deref for crate::GlobalNamespace::LessThanInstruction_LessThanSingle {
    type Target = crate::System::Linq::Expressions::Interpreter::LessThanInstruction;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(
    feature = "System+Linq+Expressions+Interpreter+LessThanInstruction+LessThanSingle"
)]
impl std::ops::DerefMut for crate::GlobalNamespace::LessThanInstruction_LessThanSingle {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(
    feature = "System+Linq+Expressions+Interpreter+LessThanInstruction+LessThanSingle"
)]
impl crate::GlobalNamespace::LessThanInstruction_LessThanSingle {
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
    feature = "System+Linq+Expressions+Interpreter+LessThanInstruction+LessThanSingle"
)]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::LessThanInstruction_LessThanSingle {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
