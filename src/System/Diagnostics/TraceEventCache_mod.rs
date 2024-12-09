#[cfg(feature = "System+Diagnostics+TraceEventCache")]
#[repr(C)]
#[derive(Debug)]
pub struct TraceEventCache {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub timeStamp: i64,
    pub dateTime: crate::System::DateTime,
    pub stackTrace: *mut quest_hook::libil2cpp::Il2CppString,
}
#[cfg(feature = "System+Diagnostics+TraceEventCache")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Diagnostics::TraceEventCache =>
    "System.Diagnostics"."TraceEventCache"
);
#[cfg(feature = "System+Diagnostics+TraceEventCache")]
impl std::ops::Deref for crate::System::Diagnostics::TraceEventCache {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Diagnostics+TraceEventCache")]
impl std::ops::DerefMut for crate::System::Diagnostics::TraceEventCache {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Diagnostics+TraceEventCache")]
impl crate::System::Diagnostics::TraceEventCache {
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_Callstack(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppString> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppString = __cordl_object
            .invoke("get_Callstack", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_DateTime(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::System::DateTime> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::DateTime = __cordl_object
            .invoke("get_DateTime", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_LogicalOperationStack(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Collections::Stack> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::Stack = __cordl_object
            .invoke("get_LogicalOperationStack", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_ProcessId(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_ProcessId", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_ThreadId(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppString> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppString = __cordl_object
            .invoke("get_ThreadId", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_Timestamp(&mut self) -> quest_hook::libil2cpp::Result<i64> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i64 = __cordl_object.invoke("get_Timestamp", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "System+Diagnostics+TraceEventCache")]
impl quest_hook::libil2cpp::ObjectType for crate::System::Diagnostics::TraceEventCache {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
