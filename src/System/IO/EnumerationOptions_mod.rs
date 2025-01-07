#[cfg(feature = "System+IO+EnumerationOptions")]
#[repr(C)]
#[derive(Debug)]
pub struct EnumerationOptions {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _RecurseSubdirectories_k__BackingField: bool,
    pub _IgnoreInaccessible_k__BackingField: bool,
    pub _AttributesToSkip_k__BackingField: crate::System::IO::FileAttributes,
    pub _MatchType_k__BackingField: crate::System::IO::MatchType,
    pub _MatchCasing_k__BackingField: crate::System::IO::MatchCasing,
    pub _ReturnSpecialDirectories_k__BackingField: bool,
}
#[cfg(feature = "System+IO+EnumerationOptions")]
unsafe impl quest_hook::libil2cpp::Type for crate::System::IO::EnumerationOptions {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "System.IO";
    const CLASS_NAME: &'static str = "EnumerationOptions";
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
#[cfg(feature = "System+IO+EnumerationOptions")]
impl std::ops::Deref for crate::System::IO::EnumerationOptions {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+IO+EnumerationOptions")]
impl std::ops::DerefMut for crate::System::IO::EnumerationOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+IO+EnumerationOptions")]
impl crate::System::IO::EnumerationOptions {
    pub fn FromSearchOption(
        searchOption: crate::System::IO::SearchOption,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::IO::EnumerationOptions>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::IO::EnumerationOptions,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("FromSearchOption", (searchOption))?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_AttributesToSkip(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::System::IO::FileAttributes> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::IO::FileAttributes = __cordl_object
            .invoke("get_AttributesToSkip", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Compatible() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::IO::EnumerationOptions>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::IO::EnumerationOptions,
        > = <Self as quest_hook::libil2cpp::Type>::class().invoke("get_Compatible", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_CompatibleRecursive() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::IO::EnumerationOptions>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::IO::EnumerationOptions,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_CompatibleRecursive", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Default() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::IO::EnumerationOptions>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::IO::EnumerationOptions,
        > = <Self as quest_hook::libil2cpp::Type>::class().invoke("get_Default", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_IgnoreInaccessible(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_IgnoreInaccessible", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_MatchCasing(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::System::IO::MatchCasing> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::IO::MatchCasing = __cordl_object
            .invoke("get_MatchCasing", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_MatchType(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::System::IO::MatchType> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::IO::MatchType = __cordl_object
            .invoke("get_MatchType", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_RecurseSubdirectories(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_RecurseSubdirectories", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_ReturnSpecialDirectories(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("get_ReturnSpecialDirectories", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn set_AttributesToSkip(
        &mut self,
        value: crate::System::IO::FileAttributes,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_AttributesToSkip", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_IgnoreInaccessible(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_IgnoreInaccessible", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_MatchType(
        &mut self,
        value: crate::System::IO::MatchType,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_MatchType", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_RecurseSubdirectories(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_RecurseSubdirectories", (value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+IO+EnumerationOptions")]
impl quest_hook::libil2cpp::ObjectType for crate::System::IO::EnumerationOptions {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
