#[cfg(feature = "Newtonsoft+Json+Serialization+DiagnosticsTraceWriter")]
#[repr(C)]
#[derive(Debug)]
pub struct DiagnosticsTraceWriter {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _LevelFilter_k__BackingField: crate::System::Diagnostics::TraceLevel,
}
#[cfg(feature = "Newtonsoft+Json+Serialization+DiagnosticsTraceWriter")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Newtonsoft::Json::Serialization::DiagnosticsTraceWriter =>
    "Newtonsoft.Json.Serialization"."DiagnosticsTraceWriter"
);
#[cfg(feature = "Newtonsoft+Json+Serialization+DiagnosticsTraceWriter")]
impl std::ops::Deref for crate::Newtonsoft::Json::Serialization::DiagnosticsTraceWriter {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Newtonsoft+Json+Serialization+DiagnosticsTraceWriter")]
impl std::ops::DerefMut
for crate::Newtonsoft::Json::Serialization::DiagnosticsTraceWriter {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Newtonsoft+Json+Serialization+DiagnosticsTraceWriter")]
impl crate::Newtonsoft::Json::Serialization::DiagnosticsTraceWriter {
    pub fn GetTraceEventType(
        &mut self,
        level: crate::System::Diagnostics::TraceLevel,
    ) -> quest_hook::libil2cpp::Result<crate::System::Diagnostics::TraceEventType> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::Diagnostics::TraceEventType = __cordl_object
            .invoke("GetTraceEventType", (level))?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn Trace(
        &mut self,
        level: crate::System::Diagnostics::TraceLevel,
        message: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        ex: quest_hook::libil2cpp::Gc<crate::System::Exception>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Trace", (level, message, ex))?;
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
    pub fn get_LevelFilter(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::System::Diagnostics::TraceLevel> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::Diagnostics::TraceLevel = __cordl_object
            .invoke("get_LevelFilter", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn set_LevelFilter(
        &mut self,
        value: crate::System::Diagnostics::TraceLevel,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_LevelFilter", (value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Newtonsoft+Json+Serialization+DiagnosticsTraceWriter")]
impl quest_hook::libil2cpp::ObjectType
for crate::Newtonsoft::Json::Serialization::DiagnosticsTraceWriter {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Newtonsoft+Json+Serialization+DiagnosticsTraceWriter")]
impl AsRef<crate::Newtonsoft::Json::Serialization::ITraceWriter>
for crate::Newtonsoft::Json::Serialization::DiagnosticsTraceWriter {
    fn as_ref(&self) -> &crate::Newtonsoft::Json::Serialization::ITraceWriter {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "Newtonsoft+Json+Serialization+DiagnosticsTraceWriter")]
impl AsMut<crate::Newtonsoft::Json::Serialization::ITraceWriter>
for crate::Newtonsoft::Json::Serialization::DiagnosticsTraceWriter {
    fn as_mut(&mut self) -> &mut crate::Newtonsoft::Json::Serialization::ITraceWriter {
        unsafe { std::mem::transmute(self) }
    }
}
