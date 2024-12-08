#[cfg(feature = "AlphabetScrollInfo")]
#[repr(C)]
#[derive(Debug)]
pub struct AlphabetScrollInfo {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "AlphabetScrollInfo")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for AlphabetScrollInfo => ""."AlphabetScrollInfo"
);
#[cfg(feature = "AlphabetScrollInfo")]
impl std::ops::Deref for AlphabetScrollInfo {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "AlphabetScrollInfo")]
impl std::ops::DerefMut for AlphabetScrollInfo {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "AlphabetScrollInfo")]
impl AlphabetScrollInfo {
    #[cfg(feature = "AlphabetScrollInfo+Data")]
    pub type Data = crate::GlobalNamespace::AlphabetScrollInfo_Data;
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "AlphabetScrollInfo")]
impl quest_hook::libil2cpp::ObjectType for AlphabetScrollInfo {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "AlphabetScrollInfo+Data")]
#[repr(C)]
#[derive(Debug)]
pub struct AlphabetScrollInfo_Data {
    __cordl_parent: crate::System::Object,
    pub character: char,
    pub cellIdx: i32,
}
#[cfg(feature = "AlphabetScrollInfo+Data")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::AlphabetScrollInfo_Data => ""
    ."AlphabetScrollInfo/Data"
);
#[cfg(feature = "AlphabetScrollInfo+Data")]
impl std::ops::Deref for crate::GlobalNamespace::AlphabetScrollInfo_Data {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "AlphabetScrollInfo+Data")]
impl std::ops::DerefMut for crate::GlobalNamespace::AlphabetScrollInfo_Data {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "AlphabetScrollInfo+Data")]
impl crate::GlobalNamespace::AlphabetScrollInfo_Data {
    pub fn _ctor(
        &mut self,
        character: char,
        cellIdx: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (character, cellIdx))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        character: char,
        cellIdx: i32,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (character, cellIdx))?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "AlphabetScrollInfo+Data")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::AlphabetScrollInfo_Data {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
