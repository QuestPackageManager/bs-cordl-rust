#[cfg(feature = "HoudiniEngineUnity+HEU_Extensions")]
#[repr(C)]
#[derive(Debug)]
pub struct HEU_Extensions {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "HoudiniEngineUnity+HEU_Extensions")]
unsafe impl quest_hook::libil2cpp::Type for crate::HoudiniEngineUnity::HEU_Extensions {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "HoudiniEngineUnity";
    const CLASS_NAME: &'static str = "HEU_Extensions";
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
#[cfg(feature = "HoudiniEngineUnity+HEU_Extensions")]
impl std::ops::Deref for crate::HoudiniEngineUnity::HEU_Extensions {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "HoudiniEngineUnity+HEU_Extensions")]
impl std::ops::DerefMut for crate::HoudiniEngineUnity::HEU_Extensions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "HoudiniEngineUnity+HEU_Extensions")]
impl crate::HoudiniEngineUnity::HEU_Extensions {
    pub fn ApproximatelyEquals_Quaternion_Quaternion0(
        quatA: crate::UnityEngine::Quaternion,
        value: crate::UnityEngine::Quaternion,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::HoudiniEngineUnity::HEU_Extensions as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::UnityEngine::Quaternion, crate::UnityEngine::Quaternion),
                bool,
                2usize,
            >("ApproximatelyEquals")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::HoudiniEngineUnity::HEU_Extensions as
                    quest_hook::libil2cpp::Type > ::class(), "ApproximatelyEquals",
                    2usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked((), (quatA, value))? };
        Ok(__cordl_ret.into())
    }
    pub fn ApproximatelyEquals_f32_f32_f32_1(
        _cordl_self: f32,
        other: f32,
        epsilon: f32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::HoudiniEngineUnity::HEU_Extensions as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<(f32, f32, f32), bool, 3usize>("ApproximatelyEquals")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::HoudiniEngineUnity::HEU_Extensions as
                    quest_hook::libil2cpp::Type > ::class(), "ApproximatelyEquals",
                    3usize
                )
            });
        let __cordl_ret: bool = unsafe {
            method.invoke_unchecked((), (_cordl_self, other, epsilon))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn AsByteArray(
        _cordl_self: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::HoudiniEngineUnity::HEU_Extensions as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
                1usize,
            >("AsByteArray")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::HoudiniEngineUnity::HEU_Extensions as
                    quest_hook::libil2cpp::Type > ::class(), "AsByteArray", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<u8>,
        > = unsafe { method.invoke_unchecked((), (_cordl_self))? };
        Ok(__cordl_ret.into())
    }
    pub fn AsString(
        buffer: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::HoudiniEngineUnity::HEU_Extensions as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                1usize,
            >("AsString")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::HoudiniEngineUnity::HEU_Extensions as
                    quest_hook::libil2cpp::Type > ::class(), "AsString", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = unsafe { method.invoke_unchecked((), (buffer))? };
        Ok(__cordl_ret.into())
    }
    pub fn ConvertArrayToEquivable<T>(
        _cordl_self: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<T>>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::IEquivable_1<T>>,
            >,
        >,
    >
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::HoudiniEngineUnity::HEU_Extensions as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<T>>),
                quest_hook::libil2cpp::Gc<
                    quest_hook::libil2cpp::Il2CppArray<
                        quest_hook::libil2cpp::Gc<
                            crate::HoudiniEngineUnity::IEquivable_1<T>,
                        >,
                    >,
                >,
                1usize,
            >("ConvertArrayToEquivable")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::HoudiniEngineUnity::HEU_Extensions as
                    quest_hook::libil2cpp::Type > ::class(), "ConvertArrayToEquivable",
                    1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::IEquivable_1<T>>,
            >,
        > = unsafe { method.invoke_unchecked((), (_cordl_self))? };
        Ok(__cordl_ret.into())
    }
    pub fn ConvertList<T, U>(
        _cordl_self: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<T>,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Collections::Generic::List_1<U>>,
    >
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        U: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::HoudiniEngineUnity::HEU_Extensions as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::System::Collections::Generic::List_1<T>,
                >),
                quest_hook::libil2cpp::Gc<
                    crate::System::Collections::Generic::List_1<U>,
                >,
                1usize,
            >("ConvertList")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::HoudiniEngineUnity::HEU_Extensions as
                    quest_hook::libil2cpp::Type > ::class(), "ConvertList", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<U>,
        > = unsafe { method.invoke_unchecked((), (_cordl_self))? };
        Ok(__cordl_ret.into())
    }
    pub fn ConvertListToEquivable<T>(
        _cordl_self: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<T>,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::IEquivable_1<T>>,
            >,
        >,
    >
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::HoudiniEngineUnity::HEU_Extensions as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::System::Collections::Generic::List_1<T>,
                >),
                quest_hook::libil2cpp::Gc<
                    crate::System::Collections::Generic::List_1<
                        quest_hook::libil2cpp::Gc<
                            crate::HoudiniEngineUnity::IEquivable_1<T>,
                        >,
                    >,
                >,
                1usize,
            >("ConvertListToEquivable")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::HoudiniEngineUnity::HEU_Extensions as
                    quest_hook::libil2cpp::Type > ::class(), "ConvertListToEquivable",
                    1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::IEquivable_1<T>>,
            >,
        > = unsafe { method.invoke_unchecked((), (_cordl_self))? };
        Ok(__cordl_ret.into())
    }
    pub fn DecomposeToPosition(
        _cordl_self: crate::UnityEngine::Matrix4x4,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector3> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::HoudiniEngineUnity::HEU_Extensions as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::UnityEngine::Matrix4x4),
                crate::UnityEngine::Vector3,
                1usize,
            >("DecomposeToPosition")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::HoudiniEngineUnity::HEU_Extensions as
                    quest_hook::libil2cpp::Type > ::class(), "DecomposeToPosition",
                    1usize
                )
            });
        let __cordl_ret: crate::UnityEngine::Vector3 = unsafe {
            method.invoke_unchecked((), (_cordl_self))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn DecomposeToRotation(
        _cordl_self: crate::UnityEngine::Matrix4x4,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Quaternion> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::HoudiniEngineUnity::HEU_Extensions as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::UnityEngine::Matrix4x4),
                crate::UnityEngine::Quaternion,
                1usize,
            >("DecomposeToRotation")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::HoudiniEngineUnity::HEU_Extensions as
                    quest_hook::libil2cpp::Type > ::class(), "DecomposeToRotation",
                    1usize
                )
            });
        let __cordl_ret: crate::UnityEngine::Quaternion = unsafe {
            method.invoke_unchecked((), (_cordl_self))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn DecomposeToScale(
        _cordl_self: crate::UnityEngine::Matrix4x4,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector3> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::HoudiniEngineUnity::HEU_Extensions as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::UnityEngine::Matrix4x4),
                crate::UnityEngine::Vector3,
                1usize,
            >("DecomposeToScale")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::HoudiniEngineUnity::HEU_Extensions as
                    quest_hook::libil2cpp::Type > ::class(), "DecomposeToScale", 1usize
                )
            });
        let __cordl_ret: crate::UnityEngine::Vector3 = unsafe {
            method.invoke_unchecked((), (_cordl_self))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Filter<T>(
        _cordl_self: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<T>,
        >,
        predicate: quest_hook::libil2cpp::Gc<crate::System::Func_2<T, bool>>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Collections::Generic::List_1<T>>,
    >
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::HoudiniEngineUnity::HEU_Extensions as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        crate::System::Collections::Generic::IEnumerable_1<T>,
                    >,
                    quest_hook::libil2cpp::Gc<crate::System::Func_2<T, bool>>,
                ),
                quest_hook::libil2cpp::Gc<
                    crate::System::Collections::Generic::List_1<T>,
                >,
                2usize,
            >("Filter")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::HoudiniEngineUnity::HEU_Extensions as
                    quest_hook::libil2cpp::Type > ::class(), "Filter", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<T>,
        > = unsafe { method.invoke_unchecked((), (_cordl_self, predicate))? };
        Ok(__cordl_ret.into())
    }
    pub fn IsEquivalentList<T>(
        _cordl_self: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<T>,
        >,
        other: quest_hook::libil2cpp::Gc<crate::System::Collections::Generic::List_1<T>>,
    ) -> quest_hook::libil2cpp::Result<bool>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::HoudiniEngineUnity::HEU_Extensions as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        crate::System::Collections::Generic::List_1<T>,
                    >,
                    quest_hook::libil2cpp::Gc<
                        crate::System::Collections::Generic::List_1<T>,
                    >,
                ),
                bool,
                2usize,
            >("IsEquivalentList")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::HoudiniEngineUnity::HEU_Extensions as
                    quest_hook::libil2cpp::Type > ::class(), "IsEquivalentList", 2usize
                )
            });
        let __cordl_ret: bool = unsafe {
            method.invoke_unchecked((), (_cordl_self, other))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn IsValidIndex<T>(
        _cordl_self: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<T>,
        >,
        index: i32,
    ) -> quest_hook::libil2cpp::Result<bool>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::HoudiniEngineUnity::HEU_Extensions as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        crate::System::Collections::Generic::List_1<T>,
                    >,
                    i32,
                ),
                bool,
                2usize,
            >("IsValidIndex")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::HoudiniEngineUnity::HEU_Extensions as
                    quest_hook::libil2cpp::Type > ::class(), "IsValidIndex", 2usize
                )
            });
        let __cordl_ret: bool = unsafe {
            method.invoke_unchecked((), (_cordl_self, index))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Map<T, R>(
        _cordl_self: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<T>,
        >,
        selector: quest_hook::libil2cpp::Gc<crate::System::Func_2<T, R>>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Collections::Generic::List_1<R>>,
    >
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        R: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::HoudiniEngineUnity::HEU_Extensions as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        crate::System::Collections::Generic::IEnumerable_1<T>,
                    >,
                    quest_hook::libil2cpp::Gc<crate::System::Func_2<T, R>>,
                ),
                quest_hook::libil2cpp::Gc<
                    crate::System::Collections::Generic::List_1<R>,
                >,
                2usize,
            >("Map")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::HoudiniEngineUnity::HEU_Extensions as
                    quest_hook::libil2cpp::Type > ::class(), "Map", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<R>,
        > = unsafe { method.invoke_unchecked((), (_cordl_self, selector))? };
        Ok(__cordl_ret.into())
    }
    pub fn Reduce<T>(
        _cordl_self: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<T>,
        >,
        func: quest_hook::libil2cpp::Gc<crate::System::Func_3<T, T, T>>,
    ) -> quest_hook::libil2cpp::Result<T>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::HoudiniEngineUnity::HEU_Extensions as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        crate::System::Collections::Generic::IEnumerable_1<T>,
                    >,
                    quest_hook::libil2cpp::Gc<crate::System::Func_3<T, T, T>>,
                ),
                T,
                2usize,
            >("Reduce")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::HoudiniEngineUnity::HEU_Extensions as
                    quest_hook::libil2cpp::Type > ::class(), "Reduce", 2usize
                )
            });
        let __cordl_ret: T = unsafe {
            method.invoke_unchecked((), (_cordl_self, func))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SwapXAndY(
        _cordl_self: crate::UnityEngine::Vector3,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector3> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::HoudiniEngineUnity::HEU_Extensions as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::UnityEngine::Vector3),
                crate::UnityEngine::Vector3,
                1usize,
            >("SwapXAndY")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::HoudiniEngineUnity::HEU_Extensions as
                    quest_hook::libil2cpp::Type > ::class(), "SwapXAndY", 1usize
                )
            });
        let __cordl_ret: crate::UnityEngine::Vector3 = unsafe {
            method.invoke_unchecked((), (_cordl_self))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SwapXAndZ(
        _cordl_self: crate::UnityEngine::Vector3,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector3> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::HoudiniEngineUnity::HEU_Extensions as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::UnityEngine::Vector3),
                crate::UnityEngine::Vector3,
                1usize,
            >("SwapXAndZ")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::HoudiniEngineUnity::HEU_Extensions as
                    quest_hook::libil2cpp::Type > ::class(), "SwapXAndZ", 1usize
                )
            });
        let __cordl_ret: crate::UnityEngine::Vector3 = unsafe {
            method.invoke_unchecked((), (_cordl_self))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SwapYAndZ(
        _cordl_self: crate::UnityEngine::Vector3,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector3> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::HoudiniEngineUnity::HEU_Extensions as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::UnityEngine::Vector3),
                crate::UnityEngine::Vector3,
                1usize,
            >("SwapYAndZ")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::HoudiniEngineUnity::HEU_Extensions as
                    quest_hook::libil2cpp::Type > ::class(), "SwapYAndZ", 1usize
                )
            });
        let __cordl_ret: crate::UnityEngine::Vector3 = unsafe {
            method.invoke_unchecked((), (_cordl_self))?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "HoudiniEngineUnity+HEU_Extensions")]
impl quest_hook::libil2cpp::ObjectType for crate::HoudiniEngineUnity::HEU_Extensions {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
