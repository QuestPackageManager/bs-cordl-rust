#[cfg(feature = "UnityEngine+ProBuilder+UvUnwrapping")]
#[repr(C)]
#[derive(Debug)]
pub struct UvUnwrapping {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "UnityEngine+ProBuilder+UvUnwrapping")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::ProBuilder::UvUnwrapping =>
    "UnityEngine.ProBuilder"."UvUnwrapping"
);
#[cfg(feature = "UnityEngine+ProBuilder+UvUnwrapping")]
impl std::ops::Deref for crate::UnityEngine::ProBuilder::UvUnwrapping {
    type Target = quest_hook::libil2cpp::Il2CppObject;
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
    pub fn ApplyUVAnchor(
        uvs: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Vector2>,
        >,
        indexes: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IList_1<i32>,
        >,
        anchor: crate::UnityEngine::ProBuilder::AutoUnwrapSettings_Anchor,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ApplyUVAnchor", (uvs, indexes, anchor))?;
        Ok(__cordl_ret.into())
    }
    pub fn ApplyUVSettings(
        uvs: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Vector2>,
        >,
        indexes: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IList_1<i32>,
        >,
        uvSettings: crate::UnityEngine::ProBuilder::AutoUnwrapSettings,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ApplyUVSettings", (uvs, indexes, uvSettings))?;
        Ok(__cordl_ret.into())
    }
    pub fn CalculateDelta(
        src: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IList_1<crate::UnityEngine::Vector2>,
        >,
        srcIndices: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IList_1<i32>,
        >,
        dst: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IList_1<crate::UnityEngine::Vector2>,
        >,
        dstIndices: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IList_1<i32>,
        >,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::ProBuilder::UvUnwrapping_UVTransform,
    > {
        let __cordl_ret: crate::UnityEngine::ProBuilder::UvUnwrapping_UVTransform = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CalculateDelta", (src, srcIndices, dst, dstIndices))?;
        Ok(__cordl_ret.into())
    }
    pub fn CopyUVs(
        mesh: quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::ProBuilderMesh>,
        source: quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::Face>,
        dest: quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::Face>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CopyUVs", (mesh, source, dest))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetAutoUnwrapSettings(
        mesh: quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::ProBuilderMesh>,
        face: quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::Face>,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::ProBuilder::AutoUnwrapSettings,
    > {
        let __cordl_ret: crate::UnityEngine::ProBuilder::AutoUnwrapSettings = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetAutoUnwrapSettings", (mesh, face))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetIndex(
        collection: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IList_1<i32>,
        >,
        index: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetIndex", (collection, index))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetRotatedSize(
        points: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IList_1<crate::UnityEngine::Vector2>,
        >,
        indices: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IList_1<i32>,
        >,
        center: crate::UnityEngine::Vector2,
        rotation: f32,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector2> {
        let __cordl_ret: crate::UnityEngine::Vector2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetRotatedSize", (points, indices, center, rotation))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetUVTransform(
        mesh: quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::ProBuilderMesh>,
        face: quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::Face>,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::ProBuilder::UvUnwrapping_UVTransform,
    > {
        let __cordl_ret: crate::UnityEngine::ProBuilder::UvUnwrapping_UVTransform = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetUVTransform", (mesh, face))?;
        Ok(__cordl_ret.into())
    }
    pub fn ProjectTextureGroup(
        mesh: quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::ProBuilderMesh>,
        group: i32,
        unwrapSettings: crate::UnityEngine::ProBuilder::AutoUnwrapSettings,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ProjectTextureGroup", (mesh, group, unwrapSettings))?;
        Ok(__cordl_ret.into())
    }
    pub fn ScaleUVs(
        uvs: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Vector2>,
        >,
        indexes: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IList_1<i32>,
        >,
        scale: crate::UnityEngine::Vector2,
        bounds: quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::Bounds2D>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ScaleUVs", (uvs, indexes, scale, bounds))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetAutoAndAlignUnwrapParamsToUVs(
        mesh: quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::ProBuilderMesh>,
        facesToConvert: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<
                quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::Face>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SetAutoAndAlignUnwrapParamsToUVs", (mesh, facesToConvert))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetAutoUV(
        mesh: quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::ProBuilderMesh>,
        faces: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<*mut crate::UnityEngine::ProBuilder::Face>,
        >,
        _cordl_auto: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SetAutoUV", (mesh, faces, _cordl_auto))?;
        Ok(__cordl_ret.into())
    }
    pub fn Unwrap(
        mesh: quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::ProBuilderMesh>,
        face: quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::Face>,
        projection: crate::UnityEngine::Vector3,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Unwrap", (mesh, face, projection))?;
        Ok(__cordl_ret.into())
    }
    pub fn UpgradeAutoUVScaleOffset(
        mesh: quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::ProBuilderMesh>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("UpgradeAutoUVScaleOffset", (mesh))?;
        Ok(__cordl_ret.into())
    }
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
#[cfg(feature = "UnityEngine+ProBuilder+UvUnwrapping+UVTransform")]
#[repr(C)]
#[derive(Debug, Clone, Default)]
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
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = quest_hook::libil2cpp::ValueTypeExt::invoke(self, "ToString", ())?;
        Ok(__cordl_ret.into())
    }
}
