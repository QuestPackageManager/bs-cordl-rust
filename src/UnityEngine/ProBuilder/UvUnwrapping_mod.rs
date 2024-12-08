#[cfg(feature = "UnityEngine+ProBuilder+UvUnwrapping+UVTransform")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct UvUnwrapping_UVTransform {
    pub translation: crate::UnityEngine::Vector2,
    pub rotation: f32,
    pub scale: crate::UnityEngine::Vector2,
}
#[cfg(feature = "UnityEngine+ProBuilder+UvUnwrapping+UVTransform")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::ProBuilder::UvUnwrapping_UVTransform => "UnityEngine.ProBuilder"
    ."UvUnwrapping/UVTransform"
);
#[cfg(feature = "UnityEngine+ProBuilder+UvUnwrapping+UVTransform")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::ProBuilder::UvUnwrapping_UVTransform {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+ProBuilder+UvUnwrapping+UVTransform")]
impl crate::UnityEngine::ProBuilder::UvUnwrapping_UVTransform {
    pub fn ToString(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_ret: *mut crate::System::String = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "ToString",
            (),
        )?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "UnityEngine+ProBuilder+UvUnwrapping")]
#[repr(C)]
#[derive(Debug)]
pub struct UvUnwrapping {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "UnityEngine+ProBuilder+UvUnwrapping")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::ProBuilder::UvUnwrapping =>
    "UnityEngine.ProBuilder"."UvUnwrapping"
);
#[cfg(feature = "UnityEngine+ProBuilder+UvUnwrapping")]
impl std::ops::Deref for crate::UnityEngine::ProBuilder::UvUnwrapping {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+ProBuilder+UvUnwrapping")]
impl std::ops::DerefMut for crate::UnityEngine::ProBuilder::UvUnwrapping {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+ProBuilder+UvUnwrapping")]
impl crate::UnityEngine::ProBuilder::UvUnwrapping {
    #[cfg(feature = "UnityEngine+ProBuilder+UvUnwrapping+UVTransform")]
    pub type UVTransform = crate::UnityEngine::ProBuilder::UvUnwrapping_UVTransform;
    #[cfg(feature = "UnityEngine+ProBuilder+UvUnwrapping+__c")]
    pub type __c = crate::UnityEngine::ProBuilder::UvUnwrapping___c;
}
#[cfg(feature = "UnityEngine+ProBuilder+UvUnwrapping")]
impl quest_hook::libil2cpp::ObjectType for crate::UnityEngine::ProBuilder::UvUnwrapping {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
