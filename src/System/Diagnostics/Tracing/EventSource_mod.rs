#[cfg(feature = "System+Diagnostics+Tracing+EventSource")]
#[repr(C)]
#[derive(Debug)]
pub struct EventSource {
    __cordl_parent: crate::System::Object,
    pub _Name_k__BackingField: *mut crate::System::String,
}
#[cfg(feature = "System+Diagnostics+Tracing+EventSource")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Diagnostics::Tracing::EventSource =>
    "System.Diagnostics.Tracing"."EventSource"
);
#[cfg(feature = "System+Diagnostics+Tracing+EventSource")]
impl std::ops::Deref for crate::System::Diagnostics::Tracing::EventSource {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Diagnostics+Tracing+EventSource")]
impl std::ops::DerefMut for crate::System::Diagnostics::Tracing::EventSource {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Diagnostics+Tracing+EventSource")]
impl crate::System::Diagnostics::Tracing::EventSource {
    #[cfg(feature = "System+Diagnostics+Tracing+EventSource+EventData")]
    pub type EventData = crate::System::Diagnostics::Tracing::EventSource_EventData;
    pub fn Dispose_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Dispose", ())?;
        Ok(__cordl_ret)
    }
    pub fn Dispose__cordl_bool1(
        &mut self,
        disposing: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Dispose", (disposing))?;
        Ok(__cordl_ret)
    }
    pub fn Finalize(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Finalize", ())?;
        Ok(__cordl_ret)
    }
    pub fn IsEnabled_0(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("IsEnabled", ())?;
        Ok(__cordl_ret)
    }
    pub fn IsEnabled_EventLevel_EventKeywords1(
        &mut self,
        level: crate::System::Diagnostics::Tracing::EventLevel,
        keywords: crate::System::Diagnostics::Tracing::EventKeywords,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("IsEnabled", (level, keywords))?;
        Ok(__cordl_ret)
    }
    pub fn New_0() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn New_Guid_String2(
        eventSourceGuid: crate::System::Guid,
        eventSourceName: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (eventSourceGuid, eventSourceName))?;
        Ok(__cordl_object)
    }
    pub fn New_String1(
        eventSourceName: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (eventSourceName))?;
        Ok(__cordl_object)
    }
    pub fn WriteEventCore(
        &mut self,
        eventId: i32,
        eventDataCount: i32,
        data: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("WriteEventCore", (eventId, eventDataCount, data))?;
        Ok(__cordl_ret)
    }
    pub fn WriteEvent_Il2CppArray7(
        &mut self,
        eventId: i32,
        args: *mut quest_hook::libil2cpp::Il2CppArray<*mut crate::System::Object>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("WriteEvent", (eventId, args))?;
        Ok(__cordl_ret)
    }
    pub fn WriteEvent_String2(
        &mut self,
        eventId: i32,
        arg1: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("WriteEvent", (eventId, arg1))?;
        Ok(__cordl_ret)
    }
    pub fn WriteEvent_String_String_String8(
        &mut self,
        eventId: i32,
        arg1: *mut crate::System::String,
        arg2: *mut crate::System::String,
        arg3: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("WriteEvent", (eventId, arg1, arg2, arg3))?;
        Ok(__cordl_ret)
    }
    pub fn WriteEvent_i32_0(
        &mut self,
        eventId: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("WriteEvent", (eventId))?;
        Ok(__cordl_ret)
    }
    pub fn WriteEvent_i32_1(
        &mut self,
        eventId: i32,
        arg1: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("WriteEvent", (eventId, arg1))?;
        Ok(__cordl_ret)
    }
    pub fn WriteEvent_i32_i32_3(
        &mut self,
        eventId: i32,
        arg1: i32,
        arg2: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("WriteEvent", (eventId, arg1, arg2))?;
        Ok(__cordl_ret)
    }
    pub fn WriteEvent_i32_i32_i32_4(
        &mut self,
        eventId: i32,
        arg1: i32,
        arg2: i32,
        arg3: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("WriteEvent", (eventId, arg1, arg2, arg3))?;
        Ok(__cordl_ret)
    }
    pub fn WriteEvent_i64_5(
        &mut self,
        eventId: i32,
        arg1: i64,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("WriteEvent", (eventId, arg1))?;
        Ok(__cordl_ret)
    }
    pub fn WriteEvent_i64_String6(
        &mut self,
        eventId: i32,
        arg1: i64,
        arg2: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("WriteEvent", (eventId, arg1, arg2))?;
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
    pub fn _ctor_Guid_String2(
        &mut self,
        eventSourceGuid: crate::System::Guid,
        eventSourceName: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (eventSourceGuid, eventSourceName))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_String1(
        &mut self,
        eventSourceName: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (eventSourceName))?;
        Ok(__cordl_ret)
    }
    pub fn set_Name(
        &mut self,
        value: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_Name", (value))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "System+Diagnostics+Tracing+EventSource")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Diagnostics::Tracing::EventSource {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "System+Diagnostics+Tracing+EventSource+EventData")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct EventSource_EventData {
    pub _DataPointer_k__BackingField: crate::System::IntPtr,
    pub _Size_k__BackingField: i32,
    pub _Reserved_k__BackingField: i32,
}
#[cfg(feature = "System+Diagnostics+Tracing+EventSource+EventData")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::System::Diagnostics::Tracing::EventSource_EventData => "System.Diagnostics.Tracing"
    ."EventSource/EventData"
);
#[cfg(feature = "System+Diagnostics+Tracing+EventSource+EventData")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::System::Diagnostics::Tracing::EventSource_EventData {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "System+Diagnostics+Tracing+EventSource+EventData")]
impl crate::System::Diagnostics::Tracing::EventSource_EventData {
    pub fn set_DataPointer(
        &mut self,
        value: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_DataPointer",
            (value),
        )?;
        Ok(__cordl_ret)
    }
    pub fn set_Reserved(
        &mut self,
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_Reserved",
            (value),
        )?;
        Ok(__cordl_ret)
    }
    pub fn set_Size(
        &mut self,
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_Size",
            (value),
        )?;
        Ok(__cordl_ret)
    }
}
