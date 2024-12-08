#[cfg(feature = "System+Diagnostics+TraceFilter")]
#[repr(C)]
#[derive(Debug)]
pub struct TraceFilter {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "System+Diagnostics+TraceFilter")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Diagnostics::TraceFilter =>
    "System.Diagnostics"."TraceFilter"
);
#[cfg(feature = "System+Diagnostics+TraceFilter")]
impl std::ops::Deref for crate::System::Diagnostics::TraceFilter {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Diagnostics+TraceFilter")]
impl std::ops::DerefMut for crate::System::Diagnostics::TraceFilter {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Diagnostics+TraceFilter")]
impl crate::System::Diagnostics::TraceFilter {
    pub fn ShouldTrace_Il2CppArray_Object_Il2CppArray0(
        &mut self,
        cache: *mut crate::System::Diagnostics::TraceEventCache,
        source: *mut crate::System::String,
        eventType: crate::System::Diagnostics::TraceEventType,
        id: i32,
        formatOrMessage: *mut crate::System::String,
        args: *mut quest_hook::libil2cpp::Il2CppArray<*mut crate::System::Object>,
        data1: *mut crate::System::Object,
        data: *mut quest_hook::libil2cpp::Il2CppArray<*mut crate::System::Object>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke(
                "ShouldTrace",
                (cache, source, eventType, id, formatOrMessage, args, data1, data),
            )?;
        Ok(__cordl_ret)
    }
    pub fn ShouldTrace_TraceEventCache_String_TraceEventType_i32_String1(
        &mut self,
        cache: *mut crate::System::Diagnostics::TraceEventCache,
        source: *mut crate::System::String,
        eventType: crate::System::Diagnostics::TraceEventType,
        id: i32,
        formatOrMessage: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("ShouldTrace", (cache, source, eventType, id, formatOrMessage))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "System+Diagnostics+TraceFilter")]
impl quest_hook::libil2cpp::ObjectType for crate::System::Diagnostics::TraceFilter {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
