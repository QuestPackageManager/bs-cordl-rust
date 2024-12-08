#[cfg(feature = "System+Linq+Expressions+Interpreter+ExceptionHandler")]
#[repr(C)]
#[derive(Debug)]
pub struct ExceptionHandler {
    __cordl_parent: crate::System::Object,
    pub _exceptionType: *mut crate::System::Type,
    pub LabelIndex: i32,
    pub HandlerStartIndex: i32,
    pub HandlerEndIndex: i32,
    pub Filter: *mut crate::System::Linq::Expressions::Interpreter::ExceptionFilter,
}
#[cfg(feature = "System+Linq+Expressions+Interpreter+ExceptionHandler")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::Linq::Expressions::Interpreter::ExceptionHandler =>
    "System.Linq.Expressions.Interpreter"."ExceptionHandler"
);
#[cfg(feature = "System+Linq+Expressions+Interpreter+ExceptionHandler")]
impl std::ops::Deref
for crate::System::Linq::Expressions::Interpreter::ExceptionHandler {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Linq+Expressions+Interpreter+ExceptionHandler")]
impl std::ops::DerefMut
for crate::System::Linq::Expressions::Interpreter::ExceptionHandler {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Linq+Expressions+Interpreter+ExceptionHandler")]
impl crate::System::Linq::Expressions::Interpreter::ExceptionHandler {
    pub fn Matches(
        &mut self,
        exceptionType: *mut crate::System::Type,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("Matches", (exceptionType))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        labelIndex: i32,
        handlerStartIndex: i32,
        handlerEndIndex: i32,
        exceptionType: *mut crate::System::Type,
        filter: *mut crate::System::Linq::Expressions::Interpreter::ExceptionFilter,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (labelIndex, handlerStartIndex, handlerEndIndex, exceptionType, filter),
            )?;
        Ok(__cordl_object)
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
        handlerStartIndex: i32,
        handlerEndIndex: i32,
        exceptionType: *mut crate::System::Type,
        filter: *mut crate::System::Linq::Expressions::Interpreter::ExceptionFilter,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                ".ctor",
                (labelIndex, handlerStartIndex, handlerEndIndex, exceptionType, filter),
            )?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "System+Linq+Expressions+Interpreter+ExceptionHandler")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Linq::Expressions::Interpreter::ExceptionHandler {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
