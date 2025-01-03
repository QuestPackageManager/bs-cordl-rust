#[cfg(feature = "Oculus+Platform+Achievements")]
#[repr(C)]
#[derive(Debug)]
pub struct Achievements {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "Oculus+Platform+Achievements")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Oculus::Platform::Achievements =>
    "Oculus.Platform"."Achievements"
);
#[cfg(feature = "Oculus+Platform+Achievements")]
impl std::ops::Deref for crate::Oculus::Platform::Achievements {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Oculus+Platform+Achievements")]
impl std::ops::DerefMut for crate::Oculus::Platform::Achievements {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Oculus+Platform+Achievements")]
impl crate::Oculus::Platform::Achievements {
    pub fn AddCount(
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        count: u64,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::Oculus::Platform::Request_1<
                *mut crate::Oculus::Platform::Models::AchievementUpdate,
            >,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Oculus::Platform::Request_1<
                *mut crate::Oculus::Platform::Models::AchievementUpdate,
            >,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("AddCount", (name, count))?;
        Ok(__cordl_ret.into())
    }
    pub fn AddFields(
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        fields: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::Oculus::Platform::Request_1<
                *mut crate::Oculus::Platform::Models::AchievementUpdate,
            >,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Oculus::Platform::Request_1<
                *mut crate::Oculus::Platform::Models::AchievementUpdate,
            >,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("AddFields", (name, fields))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetAllDefinitions() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::Oculus::Platform::Request_1<
                *mut crate::Oculus::Platform::Models::AchievementDefinitionList,
            >,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Oculus::Platform::Request_1<
                *mut crate::Oculus::Platform::Models::AchievementDefinitionList,
            >,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetAllDefinitions", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetAllProgress() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::Oculus::Platform::Request_1<
                *mut crate::Oculus::Platform::Models::AchievementProgressList,
            >,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Oculus::Platform::Request_1<
                *mut crate::Oculus::Platform::Models::AchievementProgressList,
            >,
        > = <Self as quest_hook::libil2cpp::Type>::class().invoke("GetAllProgress", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetDefinitionsByName(
        names: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<*mut quest_hook::libil2cpp::Il2CppString>,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::Oculus::Platform::Request_1<
                *mut crate::Oculus::Platform::Models::AchievementDefinitionList,
            >,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Oculus::Platform::Request_1<
                *mut crate::Oculus::Platform::Models::AchievementDefinitionList,
            >,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetDefinitionsByName", (names))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetNextAchievementDefinitionListPage(
        list: quest_hook::libil2cpp::Gc<
            crate::Oculus::Platform::Models::AchievementDefinitionList,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::Oculus::Platform::Request_1<
                *mut crate::Oculus::Platform::Models::AchievementDefinitionList,
            >,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Oculus::Platform::Request_1<
                *mut crate::Oculus::Platform::Models::AchievementDefinitionList,
            >,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetNextAchievementDefinitionListPage", (list))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetNextAchievementProgressListPage(
        list: quest_hook::libil2cpp::Gc<
            crate::Oculus::Platform::Models::AchievementProgressList,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::Oculus::Platform::Request_1<
                *mut crate::Oculus::Platform::Models::AchievementProgressList,
            >,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Oculus::Platform::Request_1<
                *mut crate::Oculus::Platform::Models::AchievementProgressList,
            >,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetNextAchievementProgressListPage", (list))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetProgressByName(
        names: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<*mut quest_hook::libil2cpp::Il2CppString>,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::Oculus::Platform::Request_1<
                *mut crate::Oculus::Platform::Models::AchievementProgressList,
            >,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Oculus::Platform::Request_1<
                *mut crate::Oculus::Platform::Models::AchievementProgressList,
            >,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetProgressByName", (names))?;
        Ok(__cordl_ret.into())
    }
    pub fn Unlock(
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::Oculus::Platform::Request_1<
                *mut crate::Oculus::Platform::Models::AchievementUpdate,
            >,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Oculus::Platform::Request_1<
                *mut crate::Oculus::Platform::Models::AchievementUpdate,
            >,
        > = <Self as quest_hook::libil2cpp::Type>::class().invoke("Unlock", (name))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Oculus+Platform+Achievements")]
impl quest_hook::libil2cpp::ObjectType for crate::Oculus::Platform::Achievements {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
