#[cfg(feature = "Unity+Burst+Intrinsics+Arm")]
#[repr(C)]
#[derive(Debug)]
pub struct Arm {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "Unity+Burst+Intrinsics+Arm")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Unity::Burst::Intrinsics::Arm =>
    "Unity.Burst.Intrinsics"."Arm"
);
#[cfg(feature = "Unity+Burst+Intrinsics+Arm")]
impl std::ops::Deref for crate::Unity::Burst::Intrinsics::Arm {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+Burst+Intrinsics+Arm")]
impl std::ops::DerefMut for crate::Unity::Burst::Intrinsics::Arm {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+Burst+Intrinsics+Arm")]
impl crate::Unity::Burst::Intrinsics::Arm {
    #[cfg(feature = "Unity+Burst+Intrinsics+Arm+Neon")]
    pub type Neon = crate::Unity::Burst::Intrinsics::Arm_Neon;
}
#[cfg(feature = "Unity+Burst+Intrinsics+Arm")]
impl quest_hook::libil2cpp::ObjectType for crate::Unity::Burst::Intrinsics::Arm {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Unity+Burst+Intrinsics+Arm+Neon")]
#[repr(C)]
#[derive(Debug)]
pub struct Arm_Neon {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "Unity+Burst+Intrinsics+Arm+Neon")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Unity::Burst::Intrinsics::Arm_Neon =>
    "Unity.Burst.Intrinsics"."Arm/Neon"
);
#[cfg(feature = "Unity+Burst+Intrinsics+Arm+Neon")]
impl std::ops::Deref for crate::Unity::Burst::Intrinsics::Arm_Neon {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+Burst+Intrinsics+Arm+Neon")]
impl std::ops::DerefMut for crate::Unity::Burst::Intrinsics::Arm_Neon {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+Burst+Intrinsics+Arm+Neon")]
impl crate::Unity::Burst::Intrinsics::Arm_Neon {
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
}
#[cfg(feature = "Unity+Burst+Intrinsics+Arm+Neon")]
impl quest_hook::libil2cpp::ObjectType for crate::Unity::Burst::Intrinsics::Arm_Neon {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
