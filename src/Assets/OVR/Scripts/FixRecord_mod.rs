#[cfg(feature = "Assets+OVR+Scripts+FixRecord")]
#[repr(C)]
#[derive(Debug)]
pub struct FixRecord {
    __cordl_parent: quest_hook::libil2cpp::Gc<crate::Assets::OVR::Scripts::Record>,
    pub fixMethod: quest_hook::libil2cpp::Gc<
        crate::Assets::OVR::Scripts::FixMethodDelegate,
    >,
    pub targetObject: quest_hook::libil2cpp::Gc<crate::UnityEngine::Object>,
    pub buttonNames: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        >,
    >,
    pub editModeRequired: bool,
    pub complete: bool,
}
#[cfg(feature = "Assets+OVR+Scripts+FixRecord")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Assets::OVR::Scripts::FixRecord =>
    "Assets.OVR.Scripts"."FixRecord"
);
#[cfg(feature = "Assets+OVR+Scripts+FixRecord")]
impl std::ops::Deref for crate::Assets::OVR::Scripts::FixRecord {
    type Target = quest_hook::libil2cpp::Gc<crate::Assets::OVR::Scripts::Record>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Assets+OVR+Scripts+FixRecord")]
impl std::ops::DerefMut for crate::Assets::OVR::Scripts::FixRecord {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Assets+OVR+Scripts+FixRecord")]
impl crate::Assets::OVR::Scripts::FixRecord {
    pub fn New(
        order: i32,
        cat: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        msg: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        fix: quest_hook::libil2cpp::Gc<crate::Assets::OVR::Scripts::FixMethodDelegate>,
        target: quest_hook::libil2cpp::Gc<crate::UnityEngine::Object>,
        editRequired: bool,
        buttons: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (order, cat, msg, fix, target, editRequired, buttons),
            )?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        order: i32,
        cat: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        msg: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        fix: quest_hook::libil2cpp::Gc<crate::Assets::OVR::Scripts::FixMethodDelegate>,
        target: quest_hook::libil2cpp::Gc<crate::UnityEngine::Object>,
        editRequired: bool,
        buttons: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (order, cat, msg, fix, target, editRequired, buttons))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Assets+OVR+Scripts+FixRecord")]
impl quest_hook::libil2cpp::ObjectType for crate::Assets::OVR::Scripts::FixRecord {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
