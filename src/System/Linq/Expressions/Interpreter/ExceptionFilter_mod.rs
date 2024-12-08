#[cfg(feature = "System+Linq+Expressions+Interpreter+ExceptionFilter")]
#[repr(C)]
#[derive(Debug)]
pub struct ExceptionFilter {
    __cordl_parent: crate::System::Object,
    pub LabelIndex: i32,
    pub StartIndex: i32,
    pub EndIndex: i32,
}
#[cfg(feature = "System+Linq+Expressions+Interpreter+ExceptionFilter")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::Linq::Expressions::Interpreter::ExceptionFilter =>
    "System.Linq.Expressions.Interpreter"."ExceptionFilter"
);
#[cfg(feature = "System+Linq+Expressions+Interpreter+ExceptionFilter")]
impl std::ops::Deref for crate::System::Linq::Expressions::Interpreter::ExceptionFilter {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Linq+Expressions+Interpreter+ExceptionFilter")]
impl std::ops::DerefMut
for crate::System::Linq::Expressions::Interpreter::ExceptionFilter {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Linq+Expressions+Interpreter+ExceptionFilter")]
impl crate::System::Linq::Expressions::Interpreter::ExceptionFilter {
    pub fn _ctor(
        &mut self,
        labelIndex: i32,
        start: i32,
        end: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (labelIndex, start, end))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        labelIndex: i32,
        start: i32,
        end: i32,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (labelIndex, start, end))?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "System+Linq+Expressions+Interpreter+ExceptionFilter")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Linq::Expressions::Interpreter::ExceptionFilter {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
