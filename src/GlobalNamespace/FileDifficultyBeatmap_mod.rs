#[cfg(feature = "FileDifficultyBeatmap")]
#[repr(C)]
#[derive(Debug)]
pub struct FileDifficultyBeatmap {
    __cordl_parent: crate::System::Object,
    pub _beatmapPath: *mut crate::System::String,
    pub _lightshowPath: *mut crate::System::String,
}
#[cfg(feature = "FileDifficultyBeatmap")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for FileDifficultyBeatmap => ""."FileDifficultyBeatmap"
);
#[cfg(feature = "FileDifficultyBeatmap")]
impl std::ops::Deref for FileDifficultyBeatmap {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "FileDifficultyBeatmap")]
impl std::ops::DerefMut for FileDifficultyBeatmap {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "FileDifficultyBeatmap")]
impl FileDifficultyBeatmap {
    pub fn GetBeatmapString(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("GetBeatmapString", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetBeatmapStringAsync(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Threading::Tasks::Task_1<*mut crate::System::String>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Threading::Tasks::Task_1<
            *mut crate::System::String,
        > = __cordl_object.invoke("GetBeatmapStringAsync", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetLightshowString(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("GetLightshowString", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetLightshowStringAsync(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Threading::Tasks::Task_1<*mut crate::System::String>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Threading::Tasks::Task_1<
            *mut crate::System::String,
        > = __cordl_object.invoke("GetLightshowStringAsync", ())?;
        Ok(__cordl_ret)
    }
    pub fn New(
        beatmapPath: *mut crate::System::String,
        lightshowPath: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (beatmapPath, lightshowPath))?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
        beatmapPath: *mut crate::System::String,
        lightshowPath: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (beatmapPath, lightshowPath))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "FileDifficultyBeatmap")]
impl quest_hook::libil2cpp::ObjectType for FileDifficultyBeatmap {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
