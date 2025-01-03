#[cfg(feature = "OVRCustomFaceExtensions")]
#[repr(C)]
#[derive(Debug)]
pub struct OVRCustomFaceExtensions {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "OVRCustomFaceExtensions")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::OVRCustomFaceExtensions => ""
    ."OVRCustomFaceExtensions"
);
#[cfg(feature = "OVRCustomFaceExtensions")]
impl std::ops::Deref for crate::GlobalNamespace::OVRCustomFaceExtensions {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVRCustomFaceExtensions")]
impl std::ops::DerefMut for crate::GlobalNamespace::OVRCustomFaceExtensions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVRCustomFaceExtensions")]
impl crate::GlobalNamespace::OVRCustomFaceExtensions {
    pub fn AutoGenerateMapping(
        skinnedMesh: quest_hook::libil2cpp::Gc<crate::UnityEngine::Mesh>,
        blendShapeNames: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<*mut quest_hook::libil2cpp::Il2CppString>,
        >,
        faceExpressions: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                crate::GlobalNamespace::OVRFaceExpressions_FaceExpression,
            >,
        >,
        allowDuplicateMapping: bool,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                crate::GlobalNamespace::OVRFaceExpressions_FaceExpression,
            >,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                crate::GlobalNamespace::OVRFaceExpressions_FaceExpression,
            >,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "AutoGenerateMapping",
                (skinnedMesh, blendShapeNames, faceExpressions, allowDuplicateMapping),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn AutoMapBlendshapes(
        customFace: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::OVRCustomFace>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("AutoMapBlendshapes", (customFace))?;
        Ok(__cordl_ret.into())
    }
    pub fn ClearBlendshapes(
        customFace: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::OVRCustomFace>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ClearBlendshapes", (customFace))?;
        Ok(__cordl_ret.into())
    }
    pub fn CustomAutoGeneratedMapping(
        customFace: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::OVRCustomFace>,
        sharedMesh: quest_hook::libil2cpp::Gc<crate::UnityEngine::Mesh>,
        allowDuplicateMapping: bool,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                crate::GlobalNamespace::OVRFaceExpressions_FaceExpression,
            >,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                crate::GlobalNamespace::OVRFaceExpressions_FaceExpression,
            >,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "CustomAutoGeneratedMapping",
                (customFace, sharedMesh, allowDuplicateMapping),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn FindBestMatch(
        tokenizedOptions: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                *mut crate::System::Collections::Generic::HashSet_1<
                    *mut quest_hook::libil2cpp::Il2CppString,
                >,
            >,
        >,
        searchString: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        expressions: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                crate::GlobalNamespace::OVRFaceExpressions_FaceExpression,
            >,
        >,
        fallback: crate::GlobalNamespace::OVRFaceExpressions_FaceExpression,
    ) -> quest_hook::libil2cpp::Result<
        crate::GlobalNamespace::OVRFaceExpressions_FaceExpression,
    > {
        let __cordl_ret: crate::GlobalNamespace::OVRFaceExpressions_FaceExpression = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "FindBestMatch",
                (tokenizedOptions, searchString, expressions, fallback),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn IsLipsToward(
        blendshapeName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IsLipsToward", (blendshapeName))?;
        Ok(__cordl_ret.into())
    }
    pub fn OculusFaceAutoGenerateMapping(
        sharedMesh: quest_hook::libil2cpp::Gc<crate::UnityEngine::Mesh>,
        allowDuplicateMapping: bool,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                crate::GlobalNamespace::OVRFaceExpressions_FaceExpression,
            >,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                crate::GlobalNamespace::OVRFaceExpressions_FaceExpression,
            >,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "OculusFaceAutoGenerateMapping",
                (sharedMesh, allowDuplicateMapping),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn SplitCamelCase(
        input: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SplitCamelCase", (input))?;
        Ok(__cordl_ret.into())
    }
    pub fn TokenizeString(
        s: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::HashSet_1<
                *mut quest_hook::libil2cpp::Il2CppString,
            >,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::HashSet_1<
                *mut quest_hook::libil2cpp::Il2CppString,
            >,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("TokenizeString", (s))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "OVRCustomFaceExtensions")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::OVRCustomFaceExtensions {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
