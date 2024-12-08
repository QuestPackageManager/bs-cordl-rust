#[cfg(
    feature = "System+Linq+Expressions+Interpreter+GreaterThanInstruction+GreaterThanSByte"
)]
#[repr(C)]
#[derive(Debug)]
pub struct GreaterThanInstruction_GreaterThanSByte {
    __cordl_parent: crate::System::Linq::Expressions::Interpreter::GreaterThanInstruction,
}
#[cfg(
    feature = "System+Linq+Expressions+Interpreter+GreaterThanInstruction+GreaterThanSByte"
)]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::GreaterThanInstruction_GreaterThanSByte =>
    "System.Linq.Expressions.Interpreter"."GreaterThanInstruction/GreaterThanSByte"
);
#[cfg(
    feature = "System+Linq+Expressions+Interpreter+GreaterThanInstruction+GreaterThanSByte"
)]
impl std::ops::Deref
for crate::GlobalNamespace::GreaterThanInstruction_GreaterThanSByte {
    type Target = crate::System::Linq::Expressions::Interpreter::GreaterThanInstruction;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(
    feature = "System+Linq+Expressions+Interpreter+GreaterThanInstruction+GreaterThanSByte"
)]
impl std::ops::DerefMut
for crate::GlobalNamespace::GreaterThanInstruction_GreaterThanSByte {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(
    feature = "System+Linq+Expressions+Interpreter+GreaterThanInstruction+GreaterThanSByte"
)]
impl crate::GlobalNamespace::GreaterThanInstruction_GreaterThanSByte {
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
    feature = "System+Linq+Expressions+Interpreter+GreaterThanInstruction+GreaterThanSByte"
)]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::GreaterThanInstruction_GreaterThanSByte {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
