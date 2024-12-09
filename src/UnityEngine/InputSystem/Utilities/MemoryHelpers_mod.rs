#[cfg(feature = "UnityEngine+InputSystem+Utilities+MemoryHelpers")]
#[repr(C)]
#[derive(Debug)]
pub struct MemoryHelpers {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "UnityEngine+InputSystem+Utilities+MemoryHelpers")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::InputSystem::Utilities::MemoryHelpers =>
    "UnityEngine.InputSystem.Utilities"."MemoryHelpers"
);
#[cfg(feature = "UnityEngine+InputSystem+Utilities+MemoryHelpers")]
impl std::ops::Deref for crate::UnityEngine::InputSystem::Utilities::MemoryHelpers {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+Utilities+MemoryHelpers")]
impl std::ops::DerefMut for crate::UnityEngine::InputSystem::Utilities::MemoryHelpers {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+Utilities+MemoryHelpers")]
impl crate::UnityEngine::InputSystem::Utilities::MemoryHelpers {
    #[cfg(feature = "UnityEngine+InputSystem+Utilities+MemoryHelpers+BitRegion")]
    pub type BitRegion = crate::UnityEngine::InputSystem::Utilities::MemoryHelpers_BitRegion;
}
#[cfg(feature = "UnityEngine+InputSystem+Utilities+MemoryHelpers")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::InputSystem::Utilities::MemoryHelpers {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "UnityEngine+InputSystem+Utilities+MemoryHelpers+BitRegion")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct MemoryHelpers_BitRegion {
    pub bitOffset: u32,
    pub sizeInBits: u32,
}
#[cfg(feature = "UnityEngine+InputSystem+Utilities+MemoryHelpers+BitRegion")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::InputSystem::Utilities::MemoryHelpers_BitRegion =>
    "UnityEngine.InputSystem.Utilities"."MemoryHelpers/BitRegion"
);
#[cfg(feature = "UnityEngine+InputSystem+Utilities+MemoryHelpers+BitRegion")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::InputSystem::Utilities::MemoryHelpers_BitRegion {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+Utilities+MemoryHelpers+BitRegion")]
impl crate::UnityEngine::InputSystem::Utilities::MemoryHelpers_BitRegion {
    pub fn Overlap(
        &mut self,
        other: crate::UnityEngine::InputSystem::Utilities::MemoryHelpers_BitRegion,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::InputSystem::Utilities::MemoryHelpers_BitRegion,
    > {
        let __cordl_ret: crate::UnityEngine::InputSystem::Utilities::MemoryHelpers_BitRegion = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Overlap",
            (other),
        )?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_u32_1(
        &mut self,
        byteOffset: u32,
        bitOffset: u32,
        sizeInBits: u32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (byteOffset, bitOffset, sizeInBits),
        )?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_u32_u32_0(
        &mut self,
        bitOffset: u32,
        sizeInBits: u32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (bitOffset, sizeInBits),
        )?;
        Ok(__cordl_ret)
    }
    pub fn get_isEmpty(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_isEmpty",
            (),
        )?;
        Ok(__cordl_ret)
    }
}
