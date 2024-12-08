#[cfg(feature = "UnityEngine+AssetReferenceUILabelRestriction")]
#[repr(C)]
#[derive(Debug)]
pub struct AssetReferenceUILabelRestriction {
    __cordl_parent: crate::UnityEngine::AssetReferenceUIRestriction,
    pub m_AllowedLabels: *mut quest_hook::libil2cpp::Il2CppArray<
        *mut crate::System::String,
    >,
    pub m_CachedToString: *mut crate::System::String,
}
#[cfg(feature = "UnityEngine+AssetReferenceUILabelRestriction")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::AssetReferenceUILabelRestriction =>
    "UnityEngine"."AssetReferenceUILabelRestriction"
);
#[cfg(feature = "UnityEngine+AssetReferenceUILabelRestriction")]
impl std::ops::Deref for crate::UnityEngine::AssetReferenceUILabelRestriction {
    type Target = crate::UnityEngine::AssetReferenceUIRestriction;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+AssetReferenceUILabelRestriction")]
impl std::ops::DerefMut for crate::UnityEngine::AssetReferenceUILabelRestriction {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+AssetReferenceUILabelRestriction")]
impl crate::UnityEngine::AssetReferenceUILabelRestriction {
    pub fn New(
        allowedLabels: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::System::String,
        >,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (allowedLabels))?;
        Ok(__cordl_object)
    }
    pub fn ToString(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("ToString", ())?;
        Ok(__cordl_ret)
    }
    pub fn ValidateAsset_Object0(
        &mut self,
        obj: *mut crate::UnityEngine::Object,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("ValidateAsset", (obj))?;
        Ok(__cordl_ret)
    }
    pub fn ValidateAsset_String1(
        &mut self,
        path: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("ValidateAsset", (path))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        allowedLabels: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::System::String,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (allowedLabels))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "UnityEngine+AssetReferenceUILabelRestriction")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::AssetReferenceUILabelRestriction {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
