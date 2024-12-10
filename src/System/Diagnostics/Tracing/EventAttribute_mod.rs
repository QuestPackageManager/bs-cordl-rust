#[cfg(feature = "System+Diagnostics+Tracing+EventAttribute")]
#[repr(C)]
#[derive(Debug)]
pub struct EventAttribute {
    __cordl_parent: crate::System::Attribute,
    pub _EventId_k__BackingField: i32,
    pub _Level_k__BackingField: crate::System::Diagnostics::Tracing::EventLevel,
    pub _Keywords_k__BackingField: crate::System::Diagnostics::Tracing::EventKeywords,
    pub _Message_k__BackingField: *mut quest_hook::libil2cpp::Il2CppString,
}
#[cfg(feature = "System+Diagnostics+Tracing+EventAttribute")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Diagnostics::Tracing::EventAttribute =>
    "System.Diagnostics.Tracing"."EventAttribute"
);
#[cfg(feature = "System+Diagnostics+Tracing+EventAttribute")]
impl std::ops::Deref for crate::System::Diagnostics::Tracing::EventAttribute {
    type Target = crate::System::Attribute;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Diagnostics+Tracing+EventAttribute")]
impl std::ops::DerefMut for crate::System::Diagnostics::Tracing::EventAttribute {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Diagnostics+Tracing+EventAttribute")]
impl crate::System::Diagnostics::Tracing::EventAttribute {
    pub fn New(
        eventId: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (eventId))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        eventId: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (eventId))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_EventId(
        &mut self,
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_EventId", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_Keywords(
        &mut self,
        value: crate::System::Diagnostics::Tracing::EventKeywords,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_Keywords", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_Level(
        &mut self,
        value: crate::System::Diagnostics::Tracing::EventLevel,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_Level", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_Message(
        &mut self,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_Message", (value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Diagnostics+Tracing+EventAttribute")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Diagnostics::Tracing::EventAttribute {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
