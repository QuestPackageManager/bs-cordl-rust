#[cfg(feature = "EnvironmentIntensityReductionOptions+CompressExpandReductionType")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EnvironmentIntensityReductionOptions_CompressExpandReductionType {
    Keep = 0i32,
    RemoveWithStrobeFilter = 1i32,
}
#[cfg(feature = "EnvironmentIntensityReductionOptions+CompressExpandReductionType")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::EnvironmentIntensityReductionOptions_CompressExpandReductionType
    => ""."EnvironmentIntensityReductionOptions/CompressExpandReductionType"
);
#[cfg(feature = "EnvironmentIntensityReductionOptions")]
#[repr(C)]
#[derive(Debug)]
pub struct EnvironmentIntensityReductionOptions {
    __cordl_parent: crate::System::Object,
    pub _compressExpand: crate::GlobalNamespace::EnvironmentIntensityReductionOptions_CompressExpandReductionType,
    pub _rotateRings: crate::GlobalNamespace::EnvironmentIntensityReductionOptions_RotateRingsReductionType,
}
#[cfg(feature = "EnvironmentIntensityReductionOptions")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for EnvironmentIntensityReductionOptions => ""
    ."EnvironmentIntensityReductionOptions"
);
#[cfg(feature = "EnvironmentIntensityReductionOptions")]
impl std::ops::Deref for EnvironmentIntensityReductionOptions {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "EnvironmentIntensityReductionOptions")]
impl std::ops::DerefMut for EnvironmentIntensityReductionOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "EnvironmentIntensityReductionOptions")]
impl EnvironmentIntensityReductionOptions {
    #[cfg(feature = "EnvironmentIntensityReductionOptions+CompressExpandReductionType")]
    pub type CompressExpandReductionType = crate::GlobalNamespace::EnvironmentIntensityReductionOptions_CompressExpandReductionType;
    #[cfg(feature = "EnvironmentIntensityReductionOptions+RotateRingsReductionType")]
    pub type RotateRingsReductionType = crate::GlobalNamespace::EnvironmentIntensityReductionOptions_RotateRingsReductionType;
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
    pub fn get_compressExpand(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::GlobalNamespace::EnvironmentIntensityReductionOptions_CompressExpandReductionType,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::GlobalNamespace::EnvironmentIntensityReductionOptions_CompressExpandReductionType = __cordl_object
            .invoke("get_compressExpand", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_rotateRings(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::GlobalNamespace::EnvironmentIntensityReductionOptions_RotateRingsReductionType,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::GlobalNamespace::EnvironmentIntensityReductionOptions_RotateRingsReductionType = __cordl_object
            .invoke("get_rotateRings", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "EnvironmentIntensityReductionOptions")]
impl quest_hook::libil2cpp::ObjectType for EnvironmentIntensityReductionOptions {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "EnvironmentIntensityReductionOptions+RotateRingsReductionType")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EnvironmentIntensityReductionOptions_RotateRingsReductionType {
    Keep = 0i32,
    RemoveWithStrobeFilter = 1i32,
}
#[cfg(feature = "EnvironmentIntensityReductionOptions+RotateRingsReductionType")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::EnvironmentIntensityReductionOptions_RotateRingsReductionType =>
    ""."EnvironmentIntensityReductionOptions/RotateRingsReductionType"
);
