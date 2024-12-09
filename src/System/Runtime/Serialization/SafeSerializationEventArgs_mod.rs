#[cfg(feature = "System+Runtime+Serialization+SafeSerializationEventArgs")]
#[repr(C)]
#[derive(Debug)]
pub struct SafeSerializationEventArgs {
    __cordl_parent: crate::System::EventArgs,
    pub m_streamingContext: crate::System::Runtime::Serialization::StreamingContext,
    pub m_serializedStates: *mut crate::System::Collections::Generic::List_1<
        *mut quest_hook::libil2cpp::Il2CppObject,
    >,
}
#[cfg(feature = "System+Runtime+Serialization+SafeSerializationEventArgs")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::Runtime::Serialization::SafeSerializationEventArgs =>
    "System.Runtime.Serialization"."SafeSerializationEventArgs"
);
#[cfg(feature = "System+Runtime+Serialization+SafeSerializationEventArgs")]
impl std::ops::Deref
for crate::System::Runtime::Serialization::SafeSerializationEventArgs {
    type Target = crate::System::EventArgs;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Runtime+Serialization+SafeSerializationEventArgs")]
impl std::ops::DerefMut
for crate::System::Runtime::Serialization::SafeSerializationEventArgs {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Runtime+Serialization+SafeSerializationEventArgs")]
impl crate::System::Runtime::Serialization::SafeSerializationEventArgs {
    pub fn New(
        streamingContext: crate::System::Runtime::Serialization::StreamingContext,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (streamingContext))?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
        streamingContext: crate::System::Runtime::Serialization::StreamingContext,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (streamingContext))?;
        Ok(__cordl_ret)
    }
    pub fn get_SerializedStates(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Collections::Generic::IList_1<
            *mut quest_hook::libil2cpp::Il2CppObject,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::Generic::IList_1<
            *mut quest_hook::libil2cpp::Il2CppObject,
        > = __cordl_object.invoke("get_SerializedStates", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "System+Runtime+Serialization+SafeSerializationEventArgs")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Runtime::Serialization::SafeSerializationEventArgs {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
