#[cfg(feature = "System+Linq+Expressions+Interpreter+ByRefMethodInfoCallInstruction")]
#[repr(C)]
#[derive(Debug)]
pub struct ByRefMethodInfoCallInstruction {
    __cordl_parent: crate::System::Linq::Expressions::Interpreter::MethodInfoCallInstruction,
    pub _byrefArgs: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<
            *mut crate::System::Linq::Expressions::Interpreter::ByRefUpdater,
        >,
    >,
}
#[cfg(feature = "System+Linq+Expressions+Interpreter+ByRefMethodInfoCallInstruction")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::Linq::Expressions::Interpreter::ByRefMethodInfoCallInstruction =>
    "System.Linq.Expressions.Interpreter"."ByRefMethodInfoCallInstruction"
);
#[cfg(feature = "System+Linq+Expressions+Interpreter+ByRefMethodInfoCallInstruction")]
impl std::ops::Deref
for crate::System::Linq::Expressions::Interpreter::ByRefMethodInfoCallInstruction {
    type Target = crate::System::Linq::Expressions::Interpreter::MethodInfoCallInstruction;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Linq+Expressions+Interpreter+ByRefMethodInfoCallInstruction")]
impl std::ops::DerefMut
for crate::System::Linq::Expressions::Interpreter::ByRefMethodInfoCallInstruction {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Linq+Expressions+Interpreter+ByRefMethodInfoCallInstruction")]
impl crate::System::Linq::Expressions::Interpreter::ByRefMethodInfoCallInstruction {
    pub fn New(
        target: quest_hook::libil2cpp::Gc<crate::System::Reflection::MethodInfo>,
        argumentCount: i32,
        byrefArgs: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                *mut crate::System::Linq::Expressions::Interpreter::ByRefUpdater,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (target, argumentCount, byrefArgs))?;
        Ok(__cordl_object.into())
    }
    pub fn Run(
        &mut self,
        frame: quest_hook::libil2cpp::Gc<
            crate::System::Linq::Expressions::Interpreter::InterpretedFrame,
        >,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("Run", (frame))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        target: quest_hook::libil2cpp::Gc<crate::System::Reflection::MethodInfo>,
        argumentCount: i32,
        byrefArgs: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                *mut crate::System::Linq::Expressions::Interpreter::ByRefUpdater,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (target, argumentCount, byrefArgs))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_ProducedStack(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_ProducedStack", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Linq+Expressions+Interpreter+ByRefMethodInfoCallInstruction")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Linq::Expressions::Interpreter::ByRefMethodInfoCallInstruction {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
