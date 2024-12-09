#[cfg(feature = "System+Linq+Expressions+Interpreter+LightDelegateCreator")]
#[repr(C)]
#[derive(Debug)]
pub struct LightDelegateCreator {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _lambda: *mut crate::System::Linq::Expressions::LambdaExpression,
    pub _Interpreter_k__BackingField: *mut crate::System::Linq::Expressions::Interpreter::Interpreter,
}
#[cfg(feature = "System+Linq+Expressions+Interpreter+LightDelegateCreator")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::Linq::Expressions::Interpreter::LightDelegateCreator =>
    "System.Linq.Expressions.Interpreter"."LightDelegateCreator"
);
#[cfg(feature = "System+Linq+Expressions+Interpreter+LightDelegateCreator")]
impl std::ops::Deref
for crate::System::Linq::Expressions::Interpreter::LightDelegateCreator {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Linq+Expressions+Interpreter+LightDelegateCreator")]
impl std::ops::DerefMut
for crate::System::Linq::Expressions::Interpreter::LightDelegateCreator {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Linq+Expressions+Interpreter+LightDelegateCreator")]
impl crate::System::Linq::Expressions::Interpreter::LightDelegateCreator {
    pub fn CreateDelegate_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Delegate> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Delegate = __cordl_object
            .invoke("CreateDelegate", ())?;
        Ok(__cordl_ret)
    }
    pub fn CreateDelegate_Il2CppArray1(
        &mut self,
        closure: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::System::Runtime::CompilerServices::IStrongBox,
        >,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Delegate> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Delegate = __cordl_object
            .invoke("CreateDelegate", (closure))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        interpreter: *mut crate::System::Linq::Expressions::Interpreter::Interpreter,
        lambda: *mut crate::System::Linq::Expressions::LambdaExpression,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (interpreter, lambda))?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
        interpreter: *mut crate::System::Linq::Expressions::Interpreter::Interpreter,
        lambda: *mut crate::System::Linq::Expressions::LambdaExpression,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (interpreter, lambda))?;
        Ok(__cordl_ret)
    }
    pub fn get_Interpreter(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Linq::Expressions::Interpreter::Interpreter,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Linq::Expressions::Interpreter::Interpreter = __cordl_object
            .invoke("get_Interpreter", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "System+Linq+Expressions+Interpreter+LightDelegateCreator")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Linq::Expressions::Interpreter::LightDelegateCreator {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
