#[cfg(feature = "System+Diagnostics+TraceInternal")]
#[repr(C)]
#[derive(Debug)]
pub struct TraceInternal {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "System+Diagnostics+TraceInternal")]
unsafe impl quest_hook::libil2cpp::Type for crate::System::Diagnostics::TraceInternal {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "System.Diagnostics";
    const CLASS_NAME: &'static str = "TraceInternal";
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.class().is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_argument(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
    fn matches_value_parameter(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
}
#[cfg(feature = "System+Diagnostics+TraceInternal")]
impl std::ops::Deref for crate::System::Diagnostics::TraceInternal {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Diagnostics+TraceInternal")]
impl std::ops::DerefMut for crate::System::Diagnostics::TraceInternal {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Diagnostics+TraceInternal")]
impl crate::System::Diagnostics::TraceInternal {
    pub fn InitializeSettings() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Void,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("InitializeSettings", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_AutoFlush() -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_AutoFlush", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_IndentLevel() -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_IndentLevel", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_IndentSize() -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_IndentSize", ())?;
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
#[cfg(feature = "System+Diagnostics+TraceInternal")]
impl quest_hook::libil2cpp::ObjectType for crate::System::Diagnostics::TraceInternal {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
