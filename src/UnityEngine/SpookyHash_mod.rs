#[cfg(feature = "UnityEngine+SpookyHash")]
#[repr(C)]
#[derive(Debug)]
pub struct SpookyHash {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "UnityEngine+SpookyHash")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::SpookyHash => "UnityEngine"
    ."SpookyHash"
);
#[cfg(feature = "UnityEngine+SpookyHash")]
impl std::ops::Deref for crate::UnityEngine::SpookyHash {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+SpookyHash")]
impl std::ops::DerefMut for crate::UnityEngine::SpookyHash {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+SpookyHash")]
impl crate::UnityEngine::SpookyHash {
    #[cfg(feature = "UnityEngine+SpookyHash+U")]
    pub type U = crate::UnityEngine::SpookyHash_U;
}
#[cfg(feature = "UnityEngine+SpookyHash")]
impl quest_hook::libil2cpp::ObjectType for crate::UnityEngine::SpookyHash {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "UnityEngine+SpookyHash+U")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct SpookyHash_U {
    padding: [u8; 8usize],
}
#[cfg(feature = "UnityEngine+SpookyHash+U")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::SpookyHash_U => "UnityEngine"
    ."SpookyHash/U"
);
#[cfg(feature = "UnityEngine+SpookyHash+U")]
unsafe impl quest_hook::libil2cpp::ThisArgument for crate::UnityEngine::SpookyHash_U {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+SpookyHash+U")]
impl crate::UnityEngine::SpookyHash_U {
    pub fn _ctor(
        &mut self,
        p8: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (p8),
        )?;
        Ok(__cordl_ret)
    }
}
