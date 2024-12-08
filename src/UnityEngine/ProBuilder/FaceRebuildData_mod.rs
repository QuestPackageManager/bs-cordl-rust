#[cfg(feature = "UnityEngine+ProBuilder+FaceRebuildData")]
#[repr(C)]
#[derive(Debug)]
pub struct FaceRebuildData {
    __cordl_parent: crate::System::Object,
    pub face: *mut crate::UnityEngine::ProBuilder::Face,
    pub vertices: *mut crate::System::Collections::Generic::List_1<
        *mut crate::UnityEngine::ProBuilder::Vertex,
    >,
    pub sharedIndexes: *mut crate::System::Collections::Generic::List_1<i32>,
    pub sharedIndexesUV: *mut crate::System::Collections::Generic::List_1<i32>,
    pub _appliedOffset: i32,
}
#[cfg(feature = "UnityEngine+ProBuilder+FaceRebuildData")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::ProBuilder::FaceRebuildData =>
    "UnityEngine.ProBuilder"."FaceRebuildData"
);
#[cfg(feature = "UnityEngine+ProBuilder+FaceRebuildData")]
impl std::ops::Deref for crate::UnityEngine::ProBuilder::FaceRebuildData {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+ProBuilder+FaceRebuildData")]
impl std::ops::DerefMut for crate::UnityEngine::ProBuilder::FaceRebuildData {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+ProBuilder+FaceRebuildData")]
impl crate::UnityEngine::ProBuilder::FaceRebuildData {
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn Offset(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("Offset", ())?;
        Ok(__cordl_ret)
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
#[cfg(feature = "UnityEngine+ProBuilder+FaceRebuildData")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::ProBuilder::FaceRebuildData {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
