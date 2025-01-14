#[cfg(feature = "UnityEngine+ProBuilder+MeshOperations+ConnectFaceRebuildData")]
#[repr(C)]
#[derive(Debug)]
pub struct ConnectFaceRebuildData {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub faceRebuildData: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::ProBuilder::FaceRebuildData,
    >,
    pub newVertexIndexes: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::List_1<i32>,
    >,
}
#[cfg(feature = "UnityEngine+ProBuilder+MeshOperations+ConnectFaceRebuildData")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::ProBuilder::MeshOperations::ConnectFaceRebuildData {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.ProBuilder.MeshOperations";
    const CLASS_NAME: &'static str = "ConnectFaceRebuildData";
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.class().is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_argument(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
    fn matches_value_parameter(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
}
#[cfg(feature = "UnityEngine+ProBuilder+MeshOperations+ConnectFaceRebuildData")]
impl std::ops::Deref
for crate::UnityEngine::ProBuilder::MeshOperations::ConnectFaceRebuildData {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+ProBuilder+MeshOperations+ConnectFaceRebuildData")]
impl std::ops::DerefMut
for crate::UnityEngine::ProBuilder::MeshOperations::ConnectFaceRebuildData {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+ProBuilder+MeshOperations+ConnectFaceRebuildData")]
impl crate::UnityEngine::ProBuilder::MeshOperations::ConnectFaceRebuildData {
    pub fn New(
        faceRebuildData: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::ProBuilder::FaceRebuildData,
        >,
        newVertexIndexes: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<i32>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (faceRebuildData, newVertexIndexes))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        faceRebuildData: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::ProBuilder::FaceRebuildData,
        >,
        newVertexIndexes: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<i32>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        crate::UnityEngine::ProBuilder::FaceRebuildData,
                    >,
                    quest_hook::libil2cpp::Gc<
                        crate::System::Collections::Generic::List_1<i32>,
                    >,
                ),
                quest_hook::libil2cpp::Void,
                2usize,
            >(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, ".ctor", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (faceRebuildData, newVertexIndexes))
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+ProBuilder+MeshOperations+ConnectFaceRebuildData")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::ProBuilder::MeshOperations::ConnectFaceRebuildData {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
