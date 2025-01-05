#[cfg(feature = "Oculus+Platform+Avatar")]
#[repr(C)]
#[derive(Debug)]
pub struct Avatar {
    __cordl_parent: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
}
#[cfg(feature = "Oculus+Platform+Avatar")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Oculus::Platform::Avatar => "Oculus.Platform"
    ."Avatar"
);
#[cfg(feature = "Oculus+Platform+Avatar")]
impl std::ops::Deref for crate::Oculus::Platform::Avatar {
    type Target = quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Oculus+Platform+Avatar")]
impl std::ops::DerefMut for crate::Oculus::Platform::Avatar {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Oculus+Platform+Avatar")]
impl crate::Oculus::Platform::Avatar {
    pub fn LaunchAvatarEditor(
        options: quest_hook::libil2cpp::Gc<crate::Oculus::Platform::AvatarEditorOptions>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Gc<
                crate::Oculus::Platform::Models::AvatarEditorResult,
            >,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Gc<
                crate::Oculus::Platform::Models::AvatarEditorResult,
            >,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("LaunchAvatarEditor", (options))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Oculus+Platform+Avatar")]
impl quest_hook::libil2cpp::ObjectType for crate::Oculus::Platform::Avatar {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
