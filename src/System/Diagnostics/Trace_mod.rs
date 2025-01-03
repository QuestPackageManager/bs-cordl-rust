#[cfg(feature = "System+Diagnostics+Trace")]
#[repr(C)]
#[derive(Debug)]
pub struct Trace {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "System+Diagnostics+Trace")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Diagnostics::Trace =>
    "System.Diagnostics"."Trace"
);
#[cfg(feature = "System+Diagnostics+Trace")]
impl std::ops::Deref for crate::System::Diagnostics::Trace {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Diagnostics+Trace")]
impl std::ops::DerefMut for crate::System::Diagnostics::Trace {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Diagnostics+Trace")]
impl crate::System::Diagnostics::Trace {
    pub fn get_AutoFlush() -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_AutoFlush", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_CorrelationManager() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Diagnostics::CorrelationManager>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Diagnostics::CorrelationManager,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_CorrelationManager", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Listeners() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Diagnostics::TraceListenerCollection>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Diagnostics::TraceListenerCollection,
        > = <Self as quest_hook::libil2cpp::Type>::class().invoke("get_Listeners", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Diagnostics+Trace")]
impl quest_hook::libil2cpp::ObjectType for crate::System::Diagnostics::Trace {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
