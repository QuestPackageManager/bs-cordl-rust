#[cfg(feature = "TMPro+TMP_FontAssetUtilities")]
#[repr(C)]
#[derive(Debug)]
pub struct TMP_FontAssetUtilities {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "TMPro+TMP_FontAssetUtilities")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::TMPro::TMP_FontAssetUtilities => "TMPro"
    ."TMP_FontAssetUtilities"
);
#[cfg(feature = "TMPro+TMP_FontAssetUtilities")]
impl std::ops::Deref for crate::TMPro::TMP_FontAssetUtilities {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "TMPro+TMP_FontAssetUtilities")]
impl std::ops::DerefMut for crate::TMPro::TMP_FontAssetUtilities {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "TMPro+TMP_FontAssetUtilities")]
impl crate::TMPro::TMP_FontAssetUtilities {
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
#[cfg(feature = "TMPro+TMP_FontAssetUtilities")]
impl quest_hook::libil2cpp::ObjectType for crate::TMPro::TMP_FontAssetUtilities {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}