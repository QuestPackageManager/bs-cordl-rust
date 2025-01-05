#[cfg(feature = "UnityEngine+Timeline+TimelineUndo")]
#[repr(C)]
#[derive(Debug)]
pub struct TimelineUndo {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "UnityEngine+Timeline+TimelineUndo")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::Timeline::TimelineUndo =>
    "UnityEngine.Timeline"."TimelineUndo"
);
#[cfg(feature = "UnityEngine+Timeline+TimelineUndo")]
impl std::ops::Deref for crate::UnityEngine::Timeline::TimelineUndo {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Timeline+TimelineUndo")]
impl std::ops::DerefMut for crate::UnityEngine::Timeline::TimelineUndo {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Timeline+TimelineUndo")]
impl crate::UnityEngine::Timeline::TimelineUndo {
    pub fn PushDestroyUndo(
        timeline: quest_hook::libil2cpp::Gc<crate::UnityEngine::Timeline::TimelineAsset>,
        thingToDirty: quest_hook::libil2cpp::Gc<crate::UnityEngine::Object>,
        objectToDestroy: quest_hook::libil2cpp::Gc<crate::UnityEngine::Object>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("PushDestroyUndo", (timeline, thingToDirty, objectToDestroy))?;
        Ok(__cordl_ret.into())
    }
    pub fn PushUndo_Il2CppArray0(
        thingsToDirty: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<*mut crate::UnityEngine::Object>,
        >,
        operation: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("PushUndo", (thingsToDirty, operation))?;
        Ok(__cordl_ret.into())
    }
    pub fn PushUndo_Object1(
        thingToDirty: quest_hook::libil2cpp::Gc<crate::UnityEngine::Object>,
        operation: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("PushUndo", (thingToDirty, operation))?;
        Ok(__cordl_ret.into())
    }
    pub fn RegisterCreatedObjectUndo(
        thingCreated: quest_hook::libil2cpp::Gc<crate::UnityEngine::Object>,
        operation: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("RegisterCreatedObjectUndo", (thingCreated, operation))?;
        Ok(__cordl_ret.into())
    }
    pub fn UndoName(
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class().invoke("UndoName", (name))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_undoEnabled() -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_undoEnabled", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+Timeline+TimelineUndo")]
impl quest_hook::libil2cpp::ObjectType for crate::UnityEngine::Timeline::TimelineUndo {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
