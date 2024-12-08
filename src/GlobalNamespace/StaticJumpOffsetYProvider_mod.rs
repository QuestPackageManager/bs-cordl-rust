#[cfg(feature = "StaticJumpOffsetYProvider+InitData")]
#[repr(C)]
#[derive(Debug)]
pub struct StaticJumpOffsetYProvider_InitData {
    __cordl_parent: crate::System::Object,
    pub jumpyYOffset: f32,
}
#[cfg(feature = "StaticJumpOffsetYProvider+InitData")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::StaticJumpOffsetYProvider_InitData => ""
    ."StaticJumpOffsetYProvider/InitData"
);
#[cfg(feature = "StaticJumpOffsetYProvider+InitData")]
impl std::ops::Deref for crate::GlobalNamespace::StaticJumpOffsetYProvider_InitData {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "StaticJumpOffsetYProvider+InitData")]
impl std::ops::DerefMut for crate::GlobalNamespace::StaticJumpOffsetYProvider_InitData {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "StaticJumpOffsetYProvider+InitData")]
impl crate::GlobalNamespace::StaticJumpOffsetYProvider_InitData {
    pub fn New(jumpyYOffset: f32) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (jumpyYOffset))?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
        jumpyYOffset: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (jumpyYOffset))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "StaticJumpOffsetYProvider+InitData")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::StaticJumpOffsetYProvider_InitData {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "StaticJumpOffsetYProvider")]
#[repr(C)]
#[derive(Debug)]
pub struct StaticJumpOffsetYProvider {
    __cordl_parent: crate::System::Object,
    pub _initData: *mut crate::GlobalNamespace::StaticJumpOffsetYProvider_InitData,
}
#[cfg(feature = "StaticJumpOffsetYProvider")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for StaticJumpOffsetYProvider => ""
    ."StaticJumpOffsetYProvider"
);
#[cfg(feature = "StaticJumpOffsetYProvider")]
impl std::ops::Deref for StaticJumpOffsetYProvider {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "StaticJumpOffsetYProvider")]
impl std::ops::DerefMut for StaticJumpOffsetYProvider {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "StaticJumpOffsetYProvider")]
impl StaticJumpOffsetYProvider {
    #[cfg(feature = "StaticJumpOffsetYProvider+InitData")]
    pub type InitData = crate::GlobalNamespace::StaticJumpOffsetYProvider_InitData;
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
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
    pub fn get_jumpOffsetY(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_jumpOffsetY", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "StaticJumpOffsetYProvider")]
impl quest_hook::libil2cpp::ObjectType for StaticJumpOffsetYProvider {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
