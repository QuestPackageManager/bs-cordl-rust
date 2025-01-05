#[cfg(feature = "BurstSliderSpawner")]
#[repr(C)]
#[derive(Debug)]
pub struct BurstSliderSpawner {
    __cordl_parent: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
}
#[cfg(feature = "BurstSliderSpawner")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::BurstSliderSpawner => ""
    ."BurstSliderSpawner"
);
#[cfg(feature = "BurstSliderSpawner")]
impl std::ops::Deref for crate::GlobalNamespace::BurstSliderSpawner {
    type Target = quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BurstSliderSpawner")]
impl std::ops::DerefMut for crate::GlobalNamespace::BurstSliderSpawner {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BurstSliderSpawner")]
impl crate::GlobalNamespace::BurstSliderSpawner {
    #[cfg(feature = "BurstSliderSpawner+ProcessNoteDataDelegate")]
    pub type ProcessNoteDataDelegate = crate::GlobalNamespace::BurstSliderSpawner_ProcessNoteDataDelegate;
    pub fn BezierCurve(
        p0: crate::UnityEngine::Vector2,
        p1: crate::UnityEngine::Vector2,
        p2: crate::UnityEngine::Vector2,
        t: f32,
        pos: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector2>,
        tangent: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector2>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("BezierCurve", (p0, p1, p2, t, pos, tangent))?;
        Ok(__cordl_ret.into())
    }
    pub fn ProcessSliderData(
        sliderData: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::SliderData>,
        sliderSpawnData: quest_hook::libil2cpp::ByRefMut<
            crate::GlobalNamespace::SliderSpawnData,
        >,
        forceIsFirstNote: bool,
        variableMovementDataProvider: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::VariableMovementDataProvider,
        >,
        processNoteData: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::BurstSliderSpawner_ProcessNoteDataDelegate,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "ProcessSliderData",
                (
                    sliderData,
                    sliderSpawnData,
                    forceIsFirstNote,
                    variableMovementDataProvider,
                    processNoteData,
                ),
            )?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "BurstSliderSpawner")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::BurstSliderSpawner {
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
    __cordl_parent: quest_hook::libil2cpp::Gc<crate::System::MulticastDelegate>,
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
    type Target = quest_hook::libil2cpp::Gc<crate::System::MulticastDelegate>;
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
        noteData: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::NoteData>,
        noteSpawnData: quest_hook::libil2cpp::ByRefMut<
            crate::GlobalNamespace::NoteSpawnData,
        >,
        forceIsFirstNote: bool,
        callback: quest_hook::libil2cpp::Gc<crate::System::AsyncCallback>,
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult> = __cordl_object
            .invoke(
                "BeginInvoke",
                (noteData, noteSpawnData, forceIsFirstNote, callback, object),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn EndInvoke(
        &mut self,
        noteSpawnData: quest_hook::libil2cpp::ByRefMut<
            crate::GlobalNamespace::NoteSpawnData,
        >,
        result: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("EndInvoke", (noteSpawnData, result))?;
        Ok(__cordl_ret.into())
    }
    pub fn Invoke(
        &mut self,
        noteData: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::NoteData>,
        noteSpawnData: quest_hook::libil2cpp::ByRefMut<
            crate::GlobalNamespace::NoteSpawnData,
        >,
        forceIsFirstNote: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Invoke", (noteData, noteSpawnData, forceIsFirstNote))?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (object, method))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (object, method))?;
        Ok(__cordl_ret.into())
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
