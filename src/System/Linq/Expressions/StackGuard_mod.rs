#[cfg(feature = "System+Linq+Expressions+StackGuard")]
#[repr(C)]
#[derive(Debug)]
pub struct StackGuard {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _executionStackCount: i32,
}
#[cfg(feature = "System+Linq+Expressions+StackGuard")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Linq::Expressions::StackGuard =>
    "System.Linq.Expressions"."StackGuard"
);
#[cfg(feature = "System+Linq+Expressions+StackGuard")]
impl std::ops::Deref for crate::System::Linq::Expressions::StackGuard {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Linq+Expressions+StackGuard")]
impl std::ops::DerefMut for crate::System::Linq::Expressions::StackGuard {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Linq+Expressions+StackGuard")]
impl crate::System::Linq::Expressions::StackGuard {
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn RunOnEmptyStack<T1, T2>(
        &mut self,
        action: quest_hook::libil2cpp::Gc<crate::System::Action_2<T1, T2>>,
        arg1: T1,
        arg2: T2,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T1: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        T2: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RunOnEmptyStack", (action, arg1, arg2))?;
        Ok(__cordl_ret.into())
    }
    pub fn RunOnEmptyStackCore<R>(
        &mut self,
        action: quest_hook::libil2cpp::Gc<
            crate::System::Func_2<
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                R,
            >,
        >,
        state: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<R>
    where
        R: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: R = __cordl_object
            .invoke("RunOnEmptyStackCore", (action, state))?;
        Ok(__cordl_ret.into())
    }
    pub fn TryEnterOnCurrentStack(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("TryEnterOnCurrentStack", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Linq+Expressions+StackGuard")]
impl quest_hook::libil2cpp::ObjectType for crate::System::Linq::Expressions::StackGuard {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
