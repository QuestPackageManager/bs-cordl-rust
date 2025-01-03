#[cfg(feature = "System+AggregateException")]
#[repr(C)]
#[derive(Debug)]
pub struct AggregateException {
    __cordl_parent: crate::System::Exception,
    pub m_innerExceptions: quest_hook::libil2cpp::Gc<
        crate::System::Collections::ObjectModel::ReadOnlyCollection_1<
            *mut crate::System::Exception,
        >,
    >,
}
#[cfg(feature = "System+AggregateException")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::AggregateException => "System"
    ."AggregateException"
);
#[cfg(feature = "System+AggregateException")]
impl std::ops::Deref for crate::System::AggregateException {
    type Target = crate::System::Exception;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+AggregateException")]
impl std::ops::DerefMut for crate::System::AggregateException {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+AggregateException")]
impl crate::System::AggregateException {
    pub fn Flatten(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::AggregateException>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::AggregateException> = __cordl_object
            .invoke("Flatten", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetObjectData(
        &mut self,
        info: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::SerializationInfo,
        >,
        context: crate::System::Runtime::Serialization::StreamingContext,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("GetObjectData", (info, context))?;
        Ok(__cordl_ret.into())
    }
    pub fn New_0() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn New_IEnumerable_1_1(
        innerExceptions: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<
                *mut crate::System::Exception,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (innerExceptions))?;
        Ok(__cordl_object.into())
    }
    pub fn New_IEnumerable_1_6(
        innerExceptionInfos: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<
                *mut crate::System::Runtime::ExceptionServices::ExceptionDispatchInfo,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (innerExceptionInfos))?;
        Ok(__cordl_object.into())
    }
    pub fn New_Il2CppArray2(
        innerExceptions: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<*mut crate::System::Exception>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (innerExceptions))?;
        Ok(__cordl_object.into())
    }
    pub fn New_Il2CppString_IEnumerable_1_3(
        message: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        innerExceptions: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<
                *mut crate::System::Exception,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (message, innerExceptions))?;
        Ok(__cordl_object.into())
    }
    pub fn New_Il2CppString_IEnumerable_1_7(
        message: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        innerExceptionInfos: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<
                *mut crate::System::Runtime::ExceptionServices::ExceptionDispatchInfo,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (message, innerExceptionInfos))?;
        Ok(__cordl_object.into())
    }
    pub fn New_Il2CppString_IList_1_5(
        message: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        innerExceptions: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IList_1<*mut crate::System::Exception>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (message, innerExceptions))?;
        Ok(__cordl_object.into())
    }
    pub fn New_Il2CppString_IList_1_8(
        message: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        innerExceptionInfos: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IList_1<
                *mut crate::System::Runtime::ExceptionServices::ExceptionDispatchInfo,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (message, innerExceptionInfos))?;
        Ok(__cordl_object.into())
    }
    pub fn New_Il2CppString_Il2CppArray4(
        message: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        innerExceptions: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<*mut crate::System::Exception>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (message, innerExceptions))?;
        Ok(__cordl_object.into())
    }
    pub fn New_SerializationInfo_StreamingContext9(
        info: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::SerializationInfo,
        >,
        context: crate::System::Runtime::Serialization::StreamingContext,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (info, context))?;
        Ok(__cordl_object.into())
    }
    pub fn ToString(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("ToString", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_IEnumerable_1_1(
        &mut self,
        innerExceptions: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<
                *mut crate::System::Exception,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (innerExceptions))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_IEnumerable_1_6(
        &mut self,
        innerExceptionInfos: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<
                *mut crate::System::Runtime::ExceptionServices::ExceptionDispatchInfo,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (innerExceptionInfos))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_Il2CppArray2(
        &mut self,
        innerExceptions: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<*mut crate::System::Exception>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (innerExceptions))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_Il2CppString_IEnumerable_1_3(
        &mut self,
        message: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        innerExceptions: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<
                *mut crate::System::Exception,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (message, innerExceptions))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_Il2CppString_IEnumerable_1_7(
        &mut self,
        message: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        innerExceptionInfos: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<
                *mut crate::System::Runtime::ExceptionServices::ExceptionDispatchInfo,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (message, innerExceptionInfos))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_Il2CppString_IList_1_5(
        &mut self,
        message: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        innerExceptions: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IList_1<*mut crate::System::Exception>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (message, innerExceptions))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_Il2CppString_IList_1_8(
        &mut self,
        message: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        innerExceptionInfos: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IList_1<
                *mut crate::System::Runtime::ExceptionServices::ExceptionDispatchInfo,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (message, innerExceptionInfos))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_Il2CppString_Il2CppArray4(
        &mut self,
        message: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        innerExceptions: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<*mut crate::System::Exception>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (message, innerExceptions))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_SerializationInfo_StreamingContext9(
        &mut self,
        info: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::SerializationInfo,
        >,
        context: crate::System::Runtime::Serialization::StreamingContext,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (info, context))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_InnerExceptions(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::ObjectModel::ReadOnlyCollection_1<
                *mut crate::System::Exception,
            >,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::ObjectModel::ReadOnlyCollection_1<
                *mut crate::System::Exception,
            >,
        > = __cordl_object.invoke("get_InnerExceptions", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Message(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("get_Message", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+AggregateException")]
impl quest_hook::libil2cpp::ObjectType for crate::System::AggregateException {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
