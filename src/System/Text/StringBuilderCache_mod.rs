#[cfg(feature = "System+Text+StringBuilderCache")]
#[repr(C)]
#[derive(Debug)]
pub struct StringBuilderCache {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "System+Text+StringBuilderCache")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Text::StringBuilderCache =>
    "System.Text"."StringBuilderCache"
);
#[cfg(feature = "System+Text+StringBuilderCache")]
impl std::ops::Deref for crate::System::Text::StringBuilderCache {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Text+StringBuilderCache")]
impl std::ops::DerefMut for crate::System::Text::StringBuilderCache {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Text+StringBuilderCache")]
impl crate::System::Text::StringBuilderCache {
    pub fn Acquire(
        capacity: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Text::StringBuilder>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Text::StringBuilder> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Acquire", (capacity))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetStringAndRelease(
        sb: quest_hook::libil2cpp::Gc<crate::System::Text::StringBuilder>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetStringAndRelease", (sb))?;
        Ok(__cordl_ret.into())
    }
    pub fn Release(
        sb: quest_hook::libil2cpp::Gc<crate::System::Text::StringBuilder>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Release", (sb))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Text+StringBuilderCache")]
impl quest_hook::libil2cpp::ObjectType for crate::System::Text::StringBuilderCache {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
