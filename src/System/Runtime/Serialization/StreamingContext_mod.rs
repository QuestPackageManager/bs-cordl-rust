#[cfg(feature = "System+Runtime+Serialization+StreamingContext")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct StreamingContext {
    pub m_additionalContext: *mut quest_hook::libil2cpp::Il2CppObject,
    pub m_state: crate::System::Runtime::Serialization::StreamingContextStates,
}
#[cfg(feature = "System+Runtime+Serialization+StreamingContext")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::System::Runtime::Serialization::StreamingContext
    => "System.Runtime.Serialization"."StreamingContext"
);
#[cfg(feature = "System+Runtime+Serialization+StreamingContext")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::System::Runtime::Serialization::StreamingContext {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "System+Runtime+Serialization+StreamingContext")]
impl crate::System::Runtime::Serialization::StreamingContext {
    pub fn Equals(
        &mut self,
        obj: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Equals",
            (obj),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn GetHashCode(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "GetHashCode",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_Il2CppObject1(
        &mut self,
        state: crate::System::Runtime::Serialization::StreamingContextStates,
        additional: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (state, additional),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_StreamingContextStates0(
        &mut self,
        state: crate::System::Runtime::Serialization::StreamingContextStates,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (state),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Context(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = quest_hook::libil2cpp::ValueTypeExt::invoke(self, "get_Context", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_State(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::System::Runtime::Serialization::StreamingContextStates,
    > {
        let __cordl_ret: crate::System::Runtime::Serialization::StreamingContextStates = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_State",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
}
