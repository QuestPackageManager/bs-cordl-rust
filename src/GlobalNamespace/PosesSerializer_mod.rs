#[cfg(feature = "PosesSerializer")]
#[repr(C)]
#[derive(Debug)]
pub struct PosesSerializer {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _logger: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::IBeatSaberLogger>,
    pub _recordingConverter: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::RecordingConverter,
    >,
}
#[cfg(feature = "PosesSerializer")]
unsafe impl quest_hook::libil2cpp::Type for crate::GlobalNamespace::PosesSerializer {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "PosesSerializer";
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
#[cfg(feature = "PosesSerializer")]
impl std::ops::Deref for crate::GlobalNamespace::PosesSerializer {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "PosesSerializer")]
impl std::ops::DerefMut for crate::GlobalNamespace::PosesSerializer {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "PosesSerializer")]
impl crate::GlobalNamespace::PosesSerializer {
    pub const kDataFileName: &'static str = "Data.rcd";
    pub const kInfoFileName: &'static str = "Info.json";
    pub fn LoadDataFile(
        &mut self,
        filePath: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                quest_hook::libil2cpp::Gc<
                    crate::GlobalNamespace::PosesRecordingData_TransformsKeyframe,
                >,
            >,
        >,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>),
                quest_hook::libil2cpp::Gc<
                    crate::System::Collections::Generic::List_1<
                        quest_hook::libil2cpp::Gc<
                            crate::GlobalNamespace::PosesRecordingData_TransformsKeyframe,
                        >,
                    >,
                >,
                1usize,
            >("LoadDataFile")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "LoadDataFile", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                quest_hook::libil2cpp::Gc<
                    crate::GlobalNamespace::PosesRecordingData_TransformsKeyframe,
                >,
            >,
        > = unsafe { method.invoke_unchecked(self, (filePath)) };
        Ok(__cordl_ret.into())
    }
    pub fn LoadInfoFile(
        &mut self,
        filePath: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::PosesRecordingInfoSaveData>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>),
                quest_hook::libil2cpp::Gc<
                    crate::GlobalNamespace::PosesRecordingInfoSaveData,
                >,
                1usize,
            >("LoadInfoFile")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "LoadInfoFile", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::PosesRecordingInfoSaveData,
        > = unsafe { method.invoke_unchecked(self, (filePath)) };
        Ok(__cordl_ret.into())
    }
    pub fn LoadRecording(
        &mut self,
        path: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::PosesRecordingData>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>),
                quest_hook::libil2cpp::Gc<crate::GlobalNamespace::PosesRecordingData>,
                1usize,
            >("LoadRecording")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "LoadRecording", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::PosesRecordingData,
        > = unsafe { method.invoke_unchecked(self, (path)) };
        Ok(__cordl_ret.into())
    }
    pub fn LoadRecordingFromDirectory(
        &mut self,
        path: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::PosesRecordingData>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>),
                quest_hook::libil2cpp::Gc<crate::GlobalNamespace::PosesRecordingData>,
                1usize,
            >("LoadRecordingFromDirectory")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "LoadRecordingFromDirectory", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::PosesRecordingData,
        > = unsafe { method.invoke_unchecked(self, (path)) };
        Ok(__cordl_ret.into())
    }
    pub fn New(
        logger: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::IBeatSaberLogger>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (logger))?;
        Ok(__cordl_object.into())
    }
    pub fn RecordingCanBeCreated(
        &mut self,
        path: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>),
                bool,
                1usize,
            >("RecordingCanBeCreated")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "RecordingCanBeCreated", 1usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked(self, (path)) };
        Ok(__cordl_ret.into())
    }
    pub fn RecordingExists(
        &mut self,
        path: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>),
                bool,
                1usize,
            >("RecordingExists")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "RecordingExists", 1usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked(self, (path)) };
        Ok(__cordl_ret.into())
    }
    pub fn SaveDataFile(
        filePath: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        data: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::PosesRecordingData>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    quest_hook::libil2cpp::Gc<crate::GlobalNamespace::PosesRecordingData>,
                ),
                quest_hook::libil2cpp::Void,
                2usize,
            >("SaveDataFile")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "SaveDataFile", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (filePath, data))
        };
        Ok(__cordl_ret.into())
    }
    pub fn SaveInfoFile(
        filePath: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        data: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::PosesRecordingData>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    quest_hook::libil2cpp::Gc<crate::GlobalNamespace::PosesRecordingData>,
                ),
                quest_hook::libil2cpp::Void,
                2usize,
            >("SaveInfoFile")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "SaveInfoFile", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (filePath, data))
        };
        Ok(__cordl_ret.into())
    }
    pub fn SaveRecording(
        &mut self,
        path: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        data: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::PosesRecordingData>,
        saveToOldFormat: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    quest_hook::libil2cpp::Gc<
                        crate::GlobalNamespace::PosesRecordingData,
                    >,
                    bool,
                ),
                quest_hook::libil2cpp::Void,
                3usize,
            >("SaveRecording")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "SaveRecording", 3usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (path, data, saveToOldFormat))
        };
        Ok(__cordl_ret.into())
    }
    pub fn SaveRecordingIntoDirectory(
        &mut self,
        path: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        data: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::PosesRecordingData>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    quest_hook::libil2cpp::Gc<crate::GlobalNamespace::PosesRecordingData>,
                ),
                quest_hook::libil2cpp::Void,
                2usize,
            >("SaveRecordingIntoDirectory")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "SaveRecordingIntoDirectory", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (path, data))
        };
        Ok(__cordl_ret.into())
    }
    pub fn SaveToOldFormat(
        &mut self,
        path: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        data: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::PosesRecordingData>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    quest_hook::libil2cpp::Gc<crate::GlobalNamespace::PosesRecordingData>,
                ),
                quest_hook::libil2cpp::Void,
                2usize,
            >("SaveToOldFormat")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "SaveToOldFormat", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (path, data))
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        logger: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::IBeatSaberLogger>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<crate::GlobalNamespace::IBeatSaberLogger>),
                quest_hook::libil2cpp::Void,
                1usize,
            >(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, ".ctor", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (logger))
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "PosesSerializer")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::PosesSerializer {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "PosesSerializer")]
impl AsRef<crate::GlobalNamespace::IPosesSerializer>
for crate::GlobalNamespace::PosesSerializer {
    fn as_ref(&self) -> &crate::GlobalNamespace::IPosesSerializer {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "PosesSerializer")]
impl AsMut<crate::GlobalNamespace::IPosesSerializer>
for crate::GlobalNamespace::PosesSerializer {
    fn as_mut(&mut self) -> &mut crate::GlobalNamespace::IPosesSerializer {
        unsafe { std::mem::transmute(self) }
    }
}
