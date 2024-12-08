#[cfg(feature = "System+Linq+Expressions+Interpreter+RuntimeVariables")]
#[repr(C)]
#[derive(Debug)]
pub struct RuntimeVariables {
    __cordl_parent: crate::System::Object,
    pub _boxes: *mut quest_hook::libil2cpp::Il2CppArray<
        *mut crate::System::Runtime::CompilerServices::IStrongBox,
    >,
}
#[cfg(feature = "System+Linq+Expressions+Interpreter+RuntimeVariables")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::Linq::Expressions::Interpreter::RuntimeVariables =>
    "System.Linq.Expressions.Interpreter"."RuntimeVariables"
);
#[cfg(feature = "System+Linq+Expressions+Interpreter+RuntimeVariables")]
impl std::ops::Deref
for crate::System::Linq::Expressions::Interpreter::RuntimeVariables {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Linq+Expressions+Interpreter+RuntimeVariables")]
impl std::ops::DerefMut
for crate::System::Linq::Expressions::Interpreter::RuntimeVariables {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Linq+Expressions+Interpreter+RuntimeVariables")]
impl crate::System::Linq::Expressions::Interpreter::RuntimeVariables {
    pub fn New(
        boxes: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::System::Runtime::CompilerServices::IStrongBox,
        >,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (boxes))?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
        boxes: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::System::Runtime::CompilerServices::IStrongBox,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (boxes))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "System+Linq+Expressions+Interpreter+RuntimeVariables")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Linq::Expressions::Interpreter::RuntimeVariables {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
