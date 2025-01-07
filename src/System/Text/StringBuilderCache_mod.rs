#[cfg(feature = "System+Text+StringBuilderCache")]
#[repr(C)]
#[derive(Debug)]
pub struct StringBuilderCache {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "System+Text+StringBuilderCache")]
unsafe impl quest_hook::libil2cpp::Type for crate::System::Text::StringBuilderCache {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "System.Text";
    const CLASS_NAME: &'static str = "StringBuilderCache";
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
