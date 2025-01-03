#[cfg(feature = "System+Runtime+Serialization+Formatters+Binary+BinaryFormatter")]
#[repr(C)]
#[derive(Debug)]
pub struct BinaryFormatter {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub m_surrogates: *mut crate::System::Runtime::Serialization::ISurrogateSelector,
    pub m_context: crate::System::Runtime::Serialization::StreamingContext,
    pub m_binder: *mut crate::System::Runtime::Serialization::SerializationBinder,
    pub m_typeFormat: crate::System::Runtime::Serialization::Formatters::FormatterTypeStyle,
    pub m_assemblyFormat: crate::System::Runtime::Serialization::Formatters::FormatterAssemblyStyle,
    pub m_securityLevel: crate::System::Runtime::Serialization::Formatters::TypeFilterLevel,
    pub m_crossAppDomainArray: *mut quest_hook::libil2cpp::Il2CppArray<
        *mut quest_hook::libil2cpp::Il2CppObject,
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
    type Target = quest_hook::libil2cpp::Il2CppObject;
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
        serializationStream: quest_hook::libil2cpp::Gc<crate::System::IO::Stream>,
        handler: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Remoting::Messaging::HeaderHandler,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = __cordl_object.invoke("Deserialize", (serializationStream, handler))?;
        Ok(__cordl_ret.into())
    }
    pub fn Deserialize_HeaderHandler__cordl_bool1(
        &mut self,
        serializationStream: quest_hook::libil2cpp::Gc<crate::System::IO::Stream>,
        handler: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Remoting::Messaging::HeaderHandler,
        >,
        fCheck: bool,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = __cordl_object
            .invoke("Deserialize", (serializationStream, handler, fCheck))?;
        Ok(__cordl_ret.into())
    }
    pub fn Deserialize_Stream0(
        &mut self,
        serializationStream: quest_hook::libil2cpp::Gc<crate::System::IO::Stream>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = __cordl_object.invoke("Deserialize", (serializationStream))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetTypeInformation(
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::Formatters::Binary::TypeInformation,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::Formatters::Binary::TypeInformation,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetTypeInformation", (_cordl_type))?;
        Ok(__cordl_ret.into())
    }
    pub fn New_0() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn New_ISurrogateSelector_StreamingContext1(
        selector: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::ISurrogateSelector,
        >,
        context: crate::System::Runtime::Serialization::StreamingContext,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (selector, context))?;
        Ok(__cordl_object.into())
    }
    pub fn Serialize_Il2CppArray1(
        &mut self,
        serializationStream: quest_hook::libil2cpp::Gc<crate::System::IO::Stream>,
        graph: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        headers: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                *mut crate::System::Runtime::Remoting::Messaging::Header,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Serialize", (serializationStream, graph, headers))?;
        Ok(__cordl_ret.into())
    }
    pub fn Serialize_Il2CppArray__cordl_bool2(
        &mut self,
        serializationStream: quest_hook::libil2cpp::Gc<crate::System::IO::Stream>,
        graph: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        headers: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                *mut crate::System::Runtime::Remoting::Messaging::Header,
            >,
        >,
        fCheck: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Serialize", (serializationStream, graph, headers, fCheck))?;
        Ok(__cordl_ret.into())
    }
    pub fn Serialize_Stream_Il2CppObject0(
        &mut self,
        serializationStream: quest_hook::libil2cpp::Gc<crate::System::IO::Stream>,
        graph: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Serialize", (serializationStream, graph))?;
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
    pub fn _ctor_ISurrogateSelector_StreamingContext1(
        &mut self,
        selector: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::ISurrogateSelector,
        >,
        context: crate::System::Runtime::Serialization::StreamingContext,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (selector, context))?;
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
    }
    pub fn set_Binder(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::SerializationBinder,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_Binder", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_SurrogateSelector(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::ISurrogateSelector,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_SurrogateSelector", (value))?;
        Ok(__cordl_ret.into())
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
#[cfg(feature = "System+Runtime+Serialization+Formatters+Binary+BinaryFormatter")]
impl AsRef<crate::System::Runtime::Serialization::IFormatter>
for crate::System::Runtime::Serialization::Formatters::Binary::BinaryFormatter {
    fn as_ref(&self) -> &crate::System::Runtime::Serialization::IFormatter {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "System+Runtime+Serialization+Formatters+Binary+BinaryFormatter")]
impl AsMut<crate::System::Runtime::Serialization::IFormatter>
for crate::System::Runtime::Serialization::Formatters::Binary::BinaryFormatter {
    fn as_mut(&mut self) -> &mut crate::System::Runtime::Serialization::IFormatter {
        unsafe { std::mem::transmute(self) }
    }
}
