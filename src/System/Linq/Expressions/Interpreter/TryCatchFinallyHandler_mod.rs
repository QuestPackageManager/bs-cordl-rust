#[cfg(feature = "System+Linq+Expressions+Interpreter+TryCatchFinallyHandler")]
#[repr(C)]
#[derive(Debug)]
pub struct TryCatchFinallyHandler {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub TryStartIndex: i32,
    pub TryEndIndex: i32,
    pub FinallyStartIndex: i32,
    pub FinallyEndIndex: i32,
    pub GotoEndTargetIndex: i32,
    pub _handlers: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<
            *mut crate::System::Linq::Expressions::Interpreter::ExceptionHandler,
        >,
    >,
}
#[cfg(feature = "System+Linq+Expressions+Interpreter+TryCatchFinallyHandler")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::Linq::Expressions::Interpreter::TryCatchFinallyHandler =>
    "System.Linq.Expressions.Interpreter"."TryCatchFinallyHandler"
);
#[cfg(feature = "System+Linq+Expressions+Interpreter+TryCatchFinallyHandler")]
impl std::ops::Deref
for crate::System::Linq::Expressions::Interpreter::TryCatchFinallyHandler {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Linq+Expressions+Interpreter+TryCatchFinallyHandler")]
impl std::ops::DerefMut
for crate::System::Linq::Expressions::Interpreter::TryCatchFinallyHandler {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Linq+Expressions+Interpreter+TryCatchFinallyHandler")]
impl crate::System::Linq::Expressions::Interpreter::TryCatchFinallyHandler {
    pub fn FilterPasses(
        frame: quest_hook::libil2cpp::Gc<
            crate::System::Linq::Expressions::Interpreter::InterpretedFrame,
        >,
        exception: quest_hook::libil2cpp::ByRefMut<
            *mut quest_hook::libil2cpp::Il2CppObject,
        >,
        filter: quest_hook::libil2cpp::Gc<
            crate::System::Linq::Expressions::Interpreter::ExceptionFilter,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("FilterPasses", (frame, exception, filter))?;
        Ok(__cordl_ret.into())
    }
    pub fn HasHandler(
        &mut self,
        frame: quest_hook::libil2cpp::Gc<
            crate::System::Linq::Expressions::Interpreter::InterpretedFrame,
        >,
        exception: quest_hook::libil2cpp::Gc<crate::System::Exception>,
        handler: quest_hook::libil2cpp::ByRefMut<
            *mut crate::System::Linq::Expressions::Interpreter::ExceptionHandler,
        >,
        unwrappedException: quest_hook::libil2cpp::ByRefMut<
            *mut quest_hook::libil2cpp::Il2CppObject,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("HasHandler", (frame, exception, handler, unwrappedException))?;
        Ok(__cordl_ret.into())
    }
    pub fn New_Il2CppArray0(
        tryStart: i32,
        tryEnd: i32,
        gotoEndTargetIndex: i32,
        handlers: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                *mut crate::System::Linq::Expressions::Interpreter::ExceptionHandler,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (tryStart, tryEnd, gotoEndTargetIndex, handlers))?;
        Ok(__cordl_object.into())
    }
    pub fn New_i32_i32_Il2CppArray1(
        tryStart: i32,
        tryEnd: i32,
        gotoEndLabelIndex: i32,
        finallyStart: i32,
        finallyEnd: i32,
        handlers: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                *mut crate::System::Linq::Expressions::Interpreter::ExceptionHandler,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (tryStart, tryEnd, gotoEndLabelIndex, finallyStart, finallyEnd, handlers),
            )?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor_Il2CppArray0(
        &mut self,
        tryStart: i32,
        tryEnd: i32,
        gotoEndTargetIndex: i32,
        handlers: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                *mut crate::System::Linq::Expressions::Interpreter::ExceptionHandler,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (tryStart, tryEnd, gotoEndTargetIndex, handlers))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_i32_i32_Il2CppArray1(
        &mut self,
        tryStart: i32,
        tryEnd: i32,
        gotoEndLabelIndex: i32,
        finallyStart: i32,
        finallyEnd: i32,
        handlers: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                *mut crate::System::Linq::Expressions::Interpreter::ExceptionHandler,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                ".ctor",
                (tryStart, tryEnd, gotoEndLabelIndex, finallyStart, finallyEnd, handlers),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Handlers(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                *mut crate::System::Linq::Expressions::Interpreter::ExceptionHandler,
            >,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                *mut crate::System::Linq::Expressions::Interpreter::ExceptionHandler,
            >,
        > = __cordl_object.invoke("get_Handlers", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_IsCatchBlockExist(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_IsCatchBlockExist", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_IsFinallyBlockExist(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_IsFinallyBlockExist", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Linq+Expressions+Interpreter+TryCatchFinallyHandler")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Linq::Expressions::Interpreter::TryCatchFinallyHandler {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
