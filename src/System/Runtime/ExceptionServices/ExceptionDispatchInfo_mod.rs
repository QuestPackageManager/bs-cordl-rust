#[cfg(feature = "System+Runtime+ExceptionServices+ExceptionDispatchInfo")]
#[repr(C)]
#[derive(Debug)]
pub struct ExceptionDispatchInfo {
    __cordl_parent: crate::System::Object,
    pub m_Exception: *mut crate::System::Exception,
    pub m_stackTrace: *mut crate::System::Object,
}
#[cfg(feature = "System+Runtime+ExceptionServices+ExceptionDispatchInfo")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::Runtime::ExceptionServices::ExceptionDispatchInfo =>
    "System.Runtime.ExceptionServices"."ExceptionDispatchInfo"
);
#[cfg(feature = "System+Runtime+ExceptionServices+ExceptionDispatchInfo")]
impl std::ops::Deref
for crate::System::Runtime::ExceptionServices::ExceptionDispatchInfo {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Runtime+ExceptionServices+ExceptionDispatchInfo")]
impl std::ops::DerefMut
for crate::System::Runtime::ExceptionServices::ExceptionDispatchInfo {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Runtime+ExceptionServices+ExceptionDispatchInfo")]
impl crate::System::Runtime::ExceptionServices::ExceptionDispatchInfo {
    pub fn New(
        exception: *mut crate::System::Exception,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (exception))?;
        Ok(__cordl_object)
    }
    pub fn Throw(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Throw", ())?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        exception: *mut crate::System::Exception,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (exception))?;
        Ok(__cordl_ret)
    }
    pub fn get_BinaryStackTraceArray(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Object> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Object = __cordl_object
            .invoke("get_BinaryStackTraceArray", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_SourceException(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Exception> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Exception = __cordl_object
            .invoke("get_SourceException", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "System+Runtime+ExceptionServices+ExceptionDispatchInfo")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Runtime::ExceptionServices::ExceptionDispatchInfo {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}