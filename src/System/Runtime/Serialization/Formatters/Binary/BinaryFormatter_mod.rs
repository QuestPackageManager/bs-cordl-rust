#[cfg(feature = "System+Runtime+Serialization+Formatters+Binary+BinaryFormatter")]
#[repr(C)]
#[derive(Debug)]
pub struct BinaryFormatter {
    __cordl_parent: crate::System::Object,
    pub m_surrogates: *mut crate::System::Runtime::Serialization::ISurrogateSelector,
    pub m_context: crate::System::Runtime::Serialization::StreamingContext,
    pub m_binder: *mut crate::System::Runtime::Serialization::SerializationBinder,
    pub m_typeFormat: crate::System::Runtime::Serialization::Formatters::FormatterTypeStyle,
    pub m_assemblyFormat: crate::System::Runtime::Serialization::Formatters::FormatterAssemblyStyle,
    pub m_securityLevel: crate::System::Runtime::Serialization::Formatters::TypeFilterLevel,
    pub m_crossAppDomainArray: *mut quest_hook::libil2cpp::Il2CppArray<
        *mut crate::System::Object,
    >,
}
#[cfg(feature = "System+Runtime+Serialization+Formatters+Binary+BinaryFormatter")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::Runtime::Serialization::Formatters::Binary::BinaryFormatter =>
    "System.Runtime.Serialization.Formatters.Binary"."BinaryFormatter"
);
#[cfg(feature = "System+Runtime+Serialization+Formatters+Binary+BinaryFormatter")]
impl std::ops::Deref
for crate::System::Runtime::Serialization::Formatters::Binary::BinaryFormatter {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Runtime+Serialization+Formatters+Binary+BinaryFormatter")]
impl std::ops::DerefMut
for crate::System::Runtime::Serialization::Formatters::Binary::BinaryFormatter {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Runtime+Serialization+Formatters+Binary+BinaryFormatter")]
impl crate::System::Runtime::Serialization::Formatters::Binary::BinaryFormatter {
    pub fn Deserialize_HeaderHandler2(
        &mut self,
        serializationStream: *mut crate::System::IO::Stream,
        handler: *mut crate::System::Runtime::Remoting::Messaging::HeaderHandler,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Object> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Object = __cordl_object
            .invoke("Deserialize", (serializationStream, handler))?;
        Ok(__cordl_ret)
    }
    pub fn Deserialize_HeaderHandler__cordl_bool1(
        &mut self,
        serializationStream: *mut crate::System::IO::Stream,
        handler: *mut crate::System::Runtime::Remoting::Messaging::HeaderHandler,
        fCheck: bool,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Object> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Object = __cordl_object
            .invoke("Deserialize", (serializationStream, handler, fCheck))?;
        Ok(__cordl_ret)
    }
    pub fn Deserialize_Stream0(
        &mut self,
        serializationStream: *mut crate::System::IO::Stream,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Object> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Object = __cordl_object
            .invoke("Deserialize", (serializationStream))?;
        Ok(__cordl_ret)
    }
    pub fn New_0() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn New_ISurrogateSelector_StreamingContext1(
        selector: *mut crate::System::Runtime::Serialization::ISurrogateSelector,
        context: crate::System::Runtime::Serialization::StreamingContext,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (selector, context))?;
        Ok(__cordl_object)
    }
    pub fn Serialize_Il2CppArray1(
        &mut self,
        serializationStream: *mut crate::System::IO::Stream,
        graph: *mut crate::System::Object,
        headers: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::System::Runtime::Remoting::Messaging::Header,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Serialize", (serializationStream, graph, headers))?;
        Ok(__cordl_ret)
    }
    pub fn Serialize_Il2CppArray__cordl_bool2(
        &mut self,
        serializationStream: *mut crate::System::IO::Stream,
        graph: *mut crate::System::Object,
        headers: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::System::Runtime::Remoting::Messaging::Header,
        >,
        fCheck: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Serialize", (serializationStream, graph, headers, fCheck))?;
        Ok(__cordl_ret)
    }
    pub fn Serialize_Stream_Object0(
        &mut self,
        serializationStream: *mut crate::System::IO::Stream,
        graph: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Serialize", (serializationStream, graph))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_ISurrogateSelector_StreamingContext1(
        &mut self,
        selector: *mut crate::System::Runtime::Serialization::ISurrogateSelector,
        context: crate::System::Runtime::Serialization::StreamingContext,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (selector, context))?;
        Ok(__cordl_ret)
    }
    pub fn set_AssemblyFormat(
        &mut self,
        value: crate::System::Runtime::Serialization::Formatters::FormatterAssemblyStyle,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_AssemblyFormat", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_Binder(
        &mut self,
        value: *mut crate::System::Runtime::Serialization::SerializationBinder,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_Binder", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_SurrogateSelector(
        &mut self,
        value: *mut crate::System::Runtime::Serialization::ISurrogateSelector,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_SurrogateSelector", (value))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "System+Runtime+Serialization+Formatters+Binary+BinaryFormatter")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Runtime::Serialization::Formatters::Binary::BinaryFormatter {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}