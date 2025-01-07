#[cfg(feature = "System+Runtime+ExceptionServices+ExceptionDispatchInfo")]
#[repr(C)]
#[derive(Debug)]
pub struct ExceptionDispatchInfo {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub m_Exception: quest_hook::libil2cpp::Gc<crate::System::Exception>,
    pub m_stackTrace: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
}
#[cfg(feature = "System+Runtime+ExceptionServices+ExceptionDispatchInfo")]
unsafe impl quest_hook::libil2cpp::Type
for crate::System::Runtime::ExceptionServices::ExceptionDispatchInfo {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "System.Runtime.ExceptionServices";
    const CLASS_NAME: &'static str = "ExceptionDispatchInfo";
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
#[cfg(feature = "System+Runtime+ExceptionServices+ExceptionDispatchInfo")]
impl std::ops::Deref
for crate::System::Runtime::ExceptionServices::ExceptionDispatchInfo {
    type Target = quest_hook::libil2cpp::Il2CppObject;
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
    pub fn Capture(
        source: quest_hook::libil2cpp::Gc<crate::System::Exception>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Runtime::ExceptionServices::ExceptionDispatchInfo,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::ExceptionServices::ExceptionDispatchInfo,
        > = <Self as quest_hook::libil2cpp::Type>::class().invoke("Capture", (source))?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        exception: quest_hook::libil2cpp::Gc<crate::System::Exception>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (exception))?;
        Ok(__cordl_object.into())
    }
    pub fn Throw_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Throw", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn Throw_Exception1(
        source: quest_hook::libil2cpp::Gc<crate::System::Exception>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Throw", (source))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        exception: quest_hook::libil2cpp::Gc<crate::System::Exception>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (exception))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_BinaryStackTraceArray(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = __cordl_object.invoke("get_BinaryStackTraceArray", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_SourceException(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Exception>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Exception> = __cordl_object
            .invoke("get_SourceException", ())?;
        Ok(__cordl_ret.into())
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
