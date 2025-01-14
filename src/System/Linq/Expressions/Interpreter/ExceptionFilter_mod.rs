#[cfg(feature = "System+Linq+Expressions+Interpreter+ExceptionFilter")]
#[repr(C)]
#[derive(Debug)]
pub struct ExceptionFilter {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub LabelIndex: i32,
    pub StartIndex: i32,
    pub EndIndex: i32,
}
#[cfg(feature = "System+Linq+Expressions+Interpreter+ExceptionFilter")]
unsafe impl quest_hook::libil2cpp::Type
for crate::System::Linq::Expressions::Interpreter::ExceptionFilter {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "System.Linq.Expressions.Interpreter";
    const CLASS_NAME: &'static str = "ExceptionFilter";
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.class().is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_argument(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
    fn matches_value_parameter(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
}
#[cfg(feature = "System+Linq+Expressions+Interpreter+ExceptionFilter")]
impl std::ops::Deref for crate::System::Linq::Expressions::Interpreter::ExceptionFilter {
    type Target = quest_hook::libil2cpp::Il2CppObject;
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
    pub fn New(
        labelIndex: i32,
        start: i32,
        end: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (labelIndex, start, end))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        labelIndex: i32,
        start: i32,
        end: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(i32, i32, i32), quest_hook::libil2cpp::Void, 3usize>(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, ".ctor", 3usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (labelIndex, start, end))
        };
        Ok(__cordl_ret.into())
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
