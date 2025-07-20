#[cfg(feature = "OVRGLTFAnimationNodeMorphTargetHandler")]
#[repr(C)]
#[derive(Debug)]
pub struct OVRGLTFAnimationNodeMorphTargetHandler {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _MeshData_k__BackingField: crate::GlobalNamespace::OVRMeshData,
    pub Weights: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<f32>>,
    pub modified: bool,
}
#[cfg(feature = "OVRGLTFAnimationNodeMorphTargetHandler")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::OVRGLTFAnimationNodeMorphTargetHandler {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "OVRGLTFAnimationNodeMorphTargetHandler";
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
#[cfg(feature = "OVRGLTFAnimationNodeMorphTargetHandler")]
impl std::ops::Deref for crate::GlobalNamespace::OVRGLTFAnimationNodeMorphTargetHandler {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVRGLTFAnimationNodeMorphTargetHandler")]
impl std::ops::DerefMut
for crate::GlobalNamespace::OVRGLTFAnimationNodeMorphTargetHandler {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVRGLTFAnimationNodeMorphTargetHandler")]
impl crate::GlobalNamespace::OVRGLTFAnimationNodeMorphTargetHandler {
    pub fn MarkModified(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::GlobalNamespace::OVRGLTFAnimationNodeMorphTargetHandler as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("MarkModified")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::GlobalNamespace::OVRGLTFAnimationNodeMorphTargetHandler as
                    quest_hook::libil2cpp::Type > ::class(), "MarkModified", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn New(
        meshData: crate::GlobalNamespace::OVRMeshData,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (meshData))?;
        Ok(__cordl_object.into())
    }
    pub fn Update(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::GlobalNamespace::OVRGLTFAnimationNodeMorphTargetHandler as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("Update")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::GlobalNamespace::OVRGLTFAnimationNodeMorphTargetHandler as
                    quest_hook::libil2cpp::Type > ::class(), "Update", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        meshData: crate::GlobalNamespace::OVRMeshData,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::GlobalNamespace::OVRGLTFAnimationNodeMorphTargetHandler as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (crate::GlobalNamespace::OVRMeshData),
                quest_hook::libil2cpp::Void,
                1usize,
            >(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::GlobalNamespace::OVRGLTFAnimationNodeMorphTargetHandler as
                    quest_hook::libil2cpp::Type > ::class(), ".ctor", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (meshData))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_MeshData(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRMeshData> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::GlobalNamespace::OVRGLTFAnimationNodeMorphTargetHandler as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                crate::GlobalNamespace::OVRMeshData,
                0usize,
            >("get_MeshData")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::GlobalNamespace::OVRGLTFAnimationNodeMorphTargetHandler as
                    quest_hook::libil2cpp::Type > ::class(), "get_MeshData", 0usize
                )
            });
        let __cordl_ret: crate::GlobalNamespace::OVRMeshData = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn set_MeshData(
        &mut self,
        value: crate::GlobalNamespace::OVRMeshData,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::GlobalNamespace::OVRGLTFAnimationNodeMorphTargetHandler as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (crate::GlobalNamespace::OVRMeshData),
                quest_hook::libil2cpp::Void,
                1usize,
            >("set_MeshData")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::GlobalNamespace::OVRGLTFAnimationNodeMorphTargetHandler as
                    quest_hook::libil2cpp::Type > ::class(), "set_MeshData", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (value))?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "OVRGLTFAnimationNodeMorphTargetHandler")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::OVRGLTFAnimationNodeMorphTargetHandler {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
