#[cfg(feature = "BurstSliderSpawner")]
#[repr(C)]
#[derive(Debug)]
pub struct BurstSliderSpawner {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "BurstSliderSpawner")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for BurstSliderSpawner => ""."BurstSliderSpawner"
);
#[cfg(feature = "BurstSliderSpawner")]
impl std::ops::Deref for BurstSliderSpawner {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BurstSliderSpawner")]
impl std::ops::DerefMut for BurstSliderSpawner {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BurstSliderSpawner")]
impl BurstSliderSpawner {
    #[cfg(feature = "BurstSliderSpawner+ProcessNoteDataDelegate")]
    pub type ProcessNoteDataDelegate = crate::GlobalNamespace::BurstSliderSpawner_ProcessNoteDataDelegate;
}
#[cfg(feature = "BurstSliderSpawner")]
impl quest_hook::libil2cpp::ObjectType for BurstSliderSpawner {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "BurstSliderSpawner+ProcessNoteDataDelegate")]
#[repr(C)]
#[derive(Debug)]
pub struct BurstSliderSpawner_ProcessNoteDataDelegate {
    __cordl_parent: crate::System::MulticastDelegate,
}
#[cfg(feature = "BurstSliderSpawner+ProcessNoteDataDelegate")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::BurstSliderSpawner_ProcessNoteDataDelegate => ""
    ."BurstSliderSpawner/ProcessNoteDataDelegate"
);
#[cfg(feature = "BurstSliderSpawner+ProcessNoteDataDelegate")]
impl std::ops::Deref
for crate::GlobalNamespace::BurstSliderSpawner_ProcessNoteDataDelegate {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BurstSliderSpawner+ProcessNoteDataDelegate")]
impl std::ops::DerefMut
for crate::GlobalNamespace::BurstSliderSpawner_ProcessNoteDataDelegate {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BurstSliderSpawner+ProcessNoteDataDelegate")]
impl crate::GlobalNamespace::BurstSliderSpawner_ProcessNoteDataDelegate {
    pub fn BeginInvoke(
        &mut self,
        noteData: *mut NoteData,
        noteSpawnData: quest_hook::libil2cpp::ByRefMut<
            crate::GlobalNamespace::BeatmapObjectSpawnMovementData_NoteSpawnData,
        >,
        rotation: f32,
        forceIsFirstNote: bool,
        callback: *mut crate::System::AsyncCallback,
        object: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::IAsyncResult> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::IAsyncResult = __cordl_object
            .invoke(
                "BeginInvoke",
                (noteData, noteSpawnData, rotation, forceIsFirstNote, callback, object),
            )?;
        Ok(__cordl_ret)
    }
    pub fn EndInvoke(
        &mut self,
        noteSpawnData: quest_hook::libil2cpp::ByRefMut<
            crate::GlobalNamespace::BeatmapObjectSpawnMovementData_NoteSpawnData,
        >,
        result: *mut crate::System::IAsyncResult,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("EndInvoke", (noteSpawnData, result))?;
        Ok(__cordl_ret)
    }
    pub fn Invoke(
        &mut self,
        noteData: *mut NoteData,
        noteSpawnData: quest_hook::libil2cpp::ByRefMut<
            crate::GlobalNamespace::BeatmapObjectSpawnMovementData_NoteSpawnData,
        >,
        rotation: f32,
        forceIsFirstNote: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Invoke", (noteData, noteSpawnData, rotation, forceIsFirstNote))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        object: *mut crate::System::Object,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (object, method))?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
        object: *mut crate::System::Object,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (object, method))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "BurstSliderSpawner+ProcessNoteDataDelegate")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::BurstSliderSpawner_ProcessNoteDataDelegate {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
