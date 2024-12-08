#[cfg(feature = "PackedPlayModeBuildLogs")]
#[repr(C)]
#[derive(Debug)]
pub struct PackedPlayModeBuildLogs {
    __cordl_parent: crate::System::Object,
    pub m_RuntimeBuildLogs: *mut crate::System::Collections::Generic::List_1<
        crate::GlobalNamespace::PackedPlayModeBuildLogs_RuntimeBuildLog,
    >,
}
#[cfg(feature = "PackedPlayModeBuildLogs")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for PackedPlayModeBuildLogs => ""."PackedPlayModeBuildLogs"
);
#[cfg(feature = "PackedPlayModeBuildLogs")]
impl std::ops::Deref for PackedPlayModeBuildLogs {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "PackedPlayModeBuildLogs")]
impl std::ops::DerefMut for PackedPlayModeBuildLogs {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "PackedPlayModeBuildLogs")]
impl PackedPlayModeBuildLogs {
    #[cfg(feature = "PackedPlayModeBuildLogs+RuntimeBuildLog")]
    pub type RuntimeBuildLog = crate::GlobalNamespace::PackedPlayModeBuildLogs_RuntimeBuildLog;
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_RuntimeBuildLogs(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Collections::Generic::List_1<
            crate::GlobalNamespace::PackedPlayModeBuildLogs_RuntimeBuildLog,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::Generic::List_1<
            crate::GlobalNamespace::PackedPlayModeBuildLogs_RuntimeBuildLog,
        > = __cordl_object.invoke("get_RuntimeBuildLogs", ())?;
        Ok(__cordl_ret)
    }
    pub fn set_RuntimeBuildLogs(
        &mut self,
        value: *mut crate::System::Collections::Generic::List_1<
            crate::GlobalNamespace::PackedPlayModeBuildLogs_RuntimeBuildLog,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_RuntimeBuildLogs", (value))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "PackedPlayModeBuildLogs")]
impl quest_hook::libil2cpp::ObjectType for PackedPlayModeBuildLogs {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "PackedPlayModeBuildLogs+RuntimeBuildLog")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct PackedPlayModeBuildLogs_RuntimeBuildLog {
    pub Type: crate::UnityEngine::LogType,
    pub Message: *mut crate::System::String,
}
#[cfg(feature = "PackedPlayModeBuildLogs+RuntimeBuildLog")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::PackedPlayModeBuildLogs_RuntimeBuildLog => ""
    ."PackedPlayModeBuildLogs/RuntimeBuildLog"
);
#[cfg(feature = "PackedPlayModeBuildLogs+RuntimeBuildLog")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::GlobalNamespace::PackedPlayModeBuildLogs_RuntimeBuildLog {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "PackedPlayModeBuildLogs+RuntimeBuildLog")]
impl crate::GlobalNamespace::PackedPlayModeBuildLogs_RuntimeBuildLog {
    pub fn _ctor(
        &mut self,
        _cordl_type: crate::UnityEngine::LogType,
        message: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (_cordl_type, message),
        )?;
        Ok(__cordl_ret)
    }
}