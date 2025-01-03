#[cfg(feature = "System+Diagnostics+StackTrace")]
#[repr(C)]
#[derive(Debug)]
pub struct StackTrace {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub frames: *mut quest_hook::libil2cpp::Il2CppArray<
        *mut crate::System::Diagnostics::StackFrame,
    >,
    pub captured_traces: *mut quest_hook::libil2cpp::Il2CppArray<
        *mut crate::System::Diagnostics::StackTrace,
    >,
    pub debug_info: bool,
}
#[cfg(feature = "System+Diagnostics+StackTrace")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Diagnostics::StackTrace =>
    "System.Diagnostics"."StackTrace"
);
#[cfg(feature = "System+Diagnostics+StackTrace")]
impl std::ops::Deref for crate::System::Diagnostics::StackTrace {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Diagnostics+StackTrace")]
impl std::ops::DerefMut for crate::System::Diagnostics::StackTrace {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Diagnostics+StackTrace")]
impl crate::System::Diagnostics::StackTrace {
    pub const METHODS_TO_SKIP: i32 = 0i32;
    pub const prefix: &'static str = "  at ";
    #[cfg(feature = "System+Diagnostics+StackTrace+TraceFormat")]
    pub type TraceFormat = crate::System::Diagnostics::StackTrace_TraceFormat;
    pub fn AddFrames(
        &mut self,
        sb: quest_hook::libil2cpp::Gc<crate::System::Text::StringBuilder>,
        separator: bool,
        isAsync: quest_hook::libil2cpp::ByRefMut<bool>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("AddFrames", (sb, separator, isAsync))?;
        Ok(__cordl_ret.into())
    }
    pub fn ConvertAsyncStateMachineMethod(
        method: quest_hook::libil2cpp::ByRefMut<
            *mut crate::System::Reflection::MethodBase,
        >,
        declaringType: quest_hook::libil2cpp::ByRefMut<*mut crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ConvertAsyncStateMachineMethod", (method, declaringType))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetAotId() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class().invoke("GetAotId", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetFrame(
        &mut self,
        index: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Diagnostics::StackFrame>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Diagnostics::StackFrame,
        > = __cordl_object.invoke("GetFrame", (index))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetFullNameForStackTrace(
        &mut self,
        sb: quest_hook::libil2cpp::Gc<crate::System::Text::StringBuilder>,
        mi: quest_hook::libil2cpp::Gc<crate::System::Reflection::MethodBase>,
        needsNewLine: bool,
        skipped: quest_hook::libil2cpp::ByRefMut<bool>,
        isAsync: quest_hook::libil2cpp::ByRefMut<bool>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "GetFullNameForStackTrace",
                (sb, mi, needsNewLine, skipped, isAsync),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn New_0() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn New_Exception__cordl_bool4(
        e: quest_hook::libil2cpp::Gc<crate::System::Exception>,
        fNeedFileInfo: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (e, fNeedFileInfo))?;
        Ok(__cordl_object.into())
    }
    pub fn New_Exception_i32__cordl_bool5(
        e: quest_hook::libil2cpp::Gc<crate::System::Exception>,
        skipFrames: i32,
        fNeedFileInfo: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (e, skipFrames, fNeedFileInfo))?;
        Ok(__cordl_object.into())
    }
    pub fn New__cordl_bool1(
        fNeedFileInfo: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (fNeedFileInfo))?;
        Ok(__cordl_object.into())
    }
    pub fn New_i32_2(
        skipFrames: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (skipFrames))?;
        Ok(__cordl_object.into())
    }
    pub fn New_i32__cordl_bool3(
        skipFrames: i32,
        fNeedFileInfo: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (skipFrames, fNeedFileInfo))?;
        Ok(__cordl_object.into())
    }
    pub fn ToString_0(
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
    pub fn ToString_StackTrace_TraceFormat1(
        &mut self,
        traceFormat: crate::System::Diagnostics::StackTrace_TraceFormat,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("ToString", (traceFormat))?;
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
    pub fn _ctor_Exception__cordl_bool4(
        &mut self,
        e: quest_hook::libil2cpp::Gc<crate::System::Exception>,
        fNeedFileInfo: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (e, fNeedFileInfo))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_Exception_i32__cordl_bool5(
        &mut self,
        e: quest_hook::libil2cpp::Gc<crate::System::Exception>,
        skipFrames: i32,
        fNeedFileInfo: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (e, skipFrames, fNeedFileInfo))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor__cordl_bool1(
        &mut self,
        fNeedFileInfo: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (fNeedFileInfo))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_i32_2(
        &mut self,
        skipFrames: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (skipFrames))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_i32__cordl_bool3(
        &mut self,
        skipFrames: i32,
        fNeedFileInfo: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (skipFrames, fNeedFileInfo))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_FrameCount(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_FrameCount", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_trace(
        e: quest_hook::libil2cpp::Gc<crate::System::Exception>,
        skipFrames: i32,
        fNeedFileInfo: bool,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                *mut crate::System::Diagnostics::StackFrame,
            >,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                *mut crate::System::Diagnostics::StackFrame,
            >,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_trace", (e, skipFrames, fNeedFileInfo))?;
        Ok(__cordl_ret.into())
    }
    pub fn init_frames(
        &mut self,
        skipFrames: i32,
        fNeedFileInfo: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("init_frames", (skipFrames, fNeedFileInfo))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Diagnostics+StackTrace")]
impl quest_hook::libil2cpp::ObjectType for crate::System::Diagnostics::StackTrace {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "System+Diagnostics+StackTrace+TraceFormat")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StackTrace_TraceFormat {
    NoResourceLookup = 2i32,
    Normal = 0i32,
    TrailingNewLine = 1i32,
}
#[cfg(feature = "System+Diagnostics+StackTrace+TraceFormat")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::System::Diagnostics::StackTrace_TraceFormat =>
    "System.Diagnostics"."StackTrace/TraceFormat"
);
