#[cfg(feature = "OVRCustomFaceExtensions")]
#[repr(C)]
#[derive(Debug)]
pub struct OVRCustomFaceExtensions {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "OVRCustomFaceExtensions")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::OVRCustomFaceExtensions {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "OVRCustomFaceExtensions";
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
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
            >,
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
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::GlobalNamespace::OVRCustomFaceExtensions as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<crate::UnityEngine::Mesh>,
                    quest_hook::libil2cpp::Gc<
                        quest_hook::libil2cpp::Il2CppArray<
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                        >,
                    >,
                    quest_hook::libil2cpp::Gc<
                        quest_hook::libil2cpp::Il2CppArray<
                            crate::GlobalNamespace::OVRFaceExpressions_FaceExpression,
                        >,
                    >,
                    bool,
                ),
                quest_hook::libil2cpp::Gc<
                    quest_hook::libil2cpp::Il2CppArray<
                        crate::GlobalNamespace::OVRFaceExpressions_FaceExpression,
                    >,
                >,
                4usize,
            >("AutoGenerateMapping")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::GlobalNamespace::OVRCustomFaceExtensions as
                    quest_hook::libil2cpp::Type > ::class(), "AutoGenerateMapping",
                    4usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                crate::GlobalNamespace::OVRFaceExpressions_FaceExpression,
            >,
        > = unsafe {
            method
                .invoke_unchecked(
                    (),
                    (
                        skinnedMesh,
                        blendShapeNames,
                        faceExpressions,
                        allowDuplicateMapping,
                    ),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn AutoMapBlendshapes(
        customFace: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::OVRCustomFace>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::GlobalNamespace::OVRCustomFaceExtensions as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<crate::GlobalNamespace::OVRCustomFace>),
                quest_hook::libil2cpp::Void,
                1usize,
            >("AutoMapBlendshapes")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::GlobalNamespace::OVRCustomFaceExtensions as
                    quest_hook::libil2cpp::Type > ::class(), "AutoMapBlendshapes", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (customFace))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ClearBlendshapes(
        customFace: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::OVRCustomFace>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::GlobalNamespace::OVRCustomFaceExtensions as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<crate::GlobalNamespace::OVRCustomFace>),
                quest_hook::libil2cpp::Void,
                1usize,
            >("ClearBlendshapes")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::GlobalNamespace::OVRCustomFaceExtensions as
                    quest_hook::libil2cpp::Type > ::class(), "ClearBlendshapes", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (customFace))?
        };
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
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::GlobalNamespace::OVRCustomFaceExtensions as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<crate::GlobalNamespace::OVRCustomFace>,
                    quest_hook::libil2cpp::Gc<crate::UnityEngine::Mesh>,
                    bool,
                ),
                quest_hook::libil2cpp::Gc<
                    quest_hook::libil2cpp::Il2CppArray<
                        crate::GlobalNamespace::OVRFaceExpressions_FaceExpression,
                    >,
                >,
                3usize,
            >("CustomAutoGeneratedMapping")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::GlobalNamespace::OVRCustomFaceExtensions as
                    quest_hook::libil2cpp::Type > ::class(),
                    "CustomAutoGeneratedMapping", 3usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                crate::GlobalNamespace::OVRFaceExpressions_FaceExpression,
            >,
        > = unsafe {
            method.invoke_unchecked((), (customFace, sharedMesh, allowDuplicateMapping))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn FindBestMatch(
        tokenizedOptions: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<
                    crate::System::Collections::Generic::HashSet_1<
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    >,
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
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::GlobalNamespace::OVRCustomFaceExtensions as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        quest_hook::libil2cpp::Il2CppArray<
                            quest_hook::libil2cpp::Gc<
                                crate::System::Collections::Generic::HashSet_1<
                                    quest_hook::libil2cpp::Gc<
                                        quest_hook::libil2cpp::Il2CppString,
                                    >,
                                >,
                            >,
                        >,
                    >,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    quest_hook::libil2cpp::Gc<
                        quest_hook::libil2cpp::Il2CppArray<
                            crate::GlobalNamespace::OVRFaceExpressions_FaceExpression,
                        >,
                    >,
                    crate::GlobalNamespace::OVRFaceExpressions_FaceExpression,
                ),
                crate::GlobalNamespace::OVRFaceExpressions_FaceExpression,
                4usize,
            >("FindBestMatch")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::GlobalNamespace::OVRCustomFaceExtensions as
                    quest_hook::libil2cpp::Type > ::class(), "FindBestMatch", 4usize
                )
            });
        let __cordl_ret: crate::GlobalNamespace::OVRFaceExpressions_FaceExpression = unsafe {
            method
                .invoke_unchecked(
                    (),
                    (tokenizedOptions, searchString, expressions, fallback),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn IsLipsToward(
        blendshapeName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::GlobalNamespace::OVRCustomFaceExtensions as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>),
                bool,
                1usize,
            >("IsLipsToward")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::GlobalNamespace::OVRCustomFaceExtensions as
                    quest_hook::libil2cpp::Type > ::class(), "IsLipsToward", 1usize
                )
            });
        let __cordl_ret: bool = unsafe {
            method.invoke_unchecked((), (blendshapeName))?
        };
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
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::GlobalNamespace::OVRCustomFaceExtensions as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<crate::UnityEngine::Mesh>, bool),
                quest_hook::libil2cpp::Gc<
                    quest_hook::libil2cpp::Il2CppArray<
                        crate::GlobalNamespace::OVRFaceExpressions_FaceExpression,
                    >,
                >,
                2usize,
            >("OculusFaceAutoGenerateMapping")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::GlobalNamespace::OVRCustomFaceExtensions as
                    quest_hook::libil2cpp::Type > ::class(),
                    "OculusFaceAutoGenerateMapping", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                crate::GlobalNamespace::OVRFaceExpressions_FaceExpression,
            >,
        > = unsafe { method.invoke_unchecked((), (sharedMesh, allowDuplicateMapping))? };
        Ok(__cordl_ret.into())
    }
    pub fn SplitCamelCase(
        input: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::GlobalNamespace::OVRCustomFaceExtensions as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                1usize,
            >("SplitCamelCase")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::GlobalNamespace::OVRCustomFaceExtensions as
                    quest_hook::libil2cpp::Type > ::class(), "SplitCamelCase", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = unsafe { method.invoke_unchecked((), (input))? };
        Ok(__cordl_ret.into())
    }
    pub fn TokenizeString(
        s: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::HashSet_1<
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
            >,
        >,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::GlobalNamespace::OVRCustomFaceExtensions as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>),
                quest_hook::libil2cpp::Gc<
                    crate::System::Collections::Generic::HashSet_1<
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    >,
                >,
                1usize,
            >("TokenizeString")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::GlobalNamespace::OVRCustomFaceExtensions as
                    quest_hook::libil2cpp::Type > ::class(), "TokenizeString", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::HashSet_1<
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
            >,
        > = unsafe { method.invoke_unchecked((), (s))? };
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
