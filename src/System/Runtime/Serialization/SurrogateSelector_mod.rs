#[cfg(feature = "System+Runtime+Serialization+SurrogateSelector")]
#[repr(C)]
#[derive(Debug)]
pub struct SurrogateSelector {
    __cordl_parent: crate::System::Object,
    pub m_surrogates: *mut crate::System::Runtime::Serialization::SurrogateHashtable,
    pub m_nextSelector: *mut crate::System::Runtime::Serialization::ISurrogateSelector,
}
#[cfg(feature = "System+Runtime+Serialization+SurrogateSelector")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::Runtime::Serialization::SurrogateSelector => "System.Runtime.Serialization"
    ."SurrogateSelector"
);
#[cfg(feature = "System+Runtime+Serialization+SurrogateSelector")]
impl std::ops::Deref for crate::System::Runtime::Serialization::SurrogateSelector {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Runtime+Serialization+SurrogateSelector")]
impl std::ops::DerefMut for crate::System::Runtime::Serialization::SurrogateSelector {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Runtime+Serialization+SurrogateSelector")]
impl crate::System::Runtime::Serialization::SurrogateSelector {
    pub fn AddSurrogate(
        &mut self,
        _cordl_type: *mut crate::System::Type,
        context: crate::System::Runtime::Serialization::StreamingContext,
        surrogate: *mut crate::System::Runtime::Serialization::ISerializationSurrogate,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddSurrogate", (_cordl_type, context, surrogate))?;
        Ok(__cordl_ret)
    }
    pub fn GetSurrogate(
        &mut self,
        _cordl_type: *mut crate::System::Type,
        context: crate::System::Runtime::Serialization::StreamingContext,
        selector: quest_hook::libil2cpp::ByRefMut<
            *mut crate::System::Runtime::Serialization::ISurrogateSelector,
        >,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Runtime::Serialization::ISerializationSurrogate,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Runtime::Serialization::ISerializationSurrogate = __cordl_object
            .invoke("GetSurrogate", (_cordl_type, context, selector))?;
        Ok(__cordl_ret)
    }
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
}
#[cfg(feature = "System+Runtime+Serialization+SurrogateSelector")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Runtime::Serialization::SurrogateSelector {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}