#[cfg(feature = "HoudiniEngineUnity+HEU_Extensions")]
#[repr(C)]
#[derive(Debug)]
pub struct HEU_Extensions {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "HoudiniEngineUnity+HEU_Extensions")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::HoudiniEngineUnity::HEU_Extensions =>
    "HoudiniEngineUnity"."HEU_Extensions"
);
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
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ApproximatelyEquals", (quatA, value))?;
        Ok(__cordl_ret.into())
    }
    pub fn ApproximatelyEquals_f32_f32_f32_1(
        _cordl_self: f32,
        other: f32,
        epsilon: f32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ApproximatelyEquals", (_cordl_self, other, epsilon))?;
        Ok(__cordl_ret.into())
    }
    pub fn AsByteArray(
        _cordl_self: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<u8>,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("AsByteArray", (_cordl_self))?;
        Ok(__cordl_ret.into())
    }
    pub fn AsString(
        buffer: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class().invoke("AsString", (buffer))?;
        Ok(__cordl_ret.into())
    }
    pub fn ConvertArrayToEquivable<T>(
        _cordl_self: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<T>>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                *mut crate::HoudiniEngineUnity::IEquivable_1<T>,
            >,
        >,
    >
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                *mut crate::HoudiniEngineUnity::IEquivable_1<T>,
            >,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ConvertArrayToEquivable", (_cordl_self))?;
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
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<U>,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ConvertList", (_cordl_self))?;
        Ok(__cordl_ret.into())
    }
    pub fn ConvertListToEquivable<T>(
        _cordl_self: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<T>,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                *mut crate::HoudiniEngineUnity::IEquivable_1<T>,
            >,
        >,
    >
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                *mut crate::HoudiniEngineUnity::IEquivable_1<T>,
            >,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ConvertListToEquivable", (_cordl_self))?;
        Ok(__cordl_ret.into())
    }
    pub fn DecomposeToPosition(
        _cordl_self: crate::UnityEngine::Matrix4x4,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector3> {
        let __cordl_ret: crate::UnityEngine::Vector3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("DecomposeToPosition", (_cordl_self))?;
        Ok(__cordl_ret.into())
    }
    pub fn DecomposeToRotation(
        _cordl_self: crate::UnityEngine::Matrix4x4,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Quaternion> {
        let __cordl_ret: crate::UnityEngine::Quaternion = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("DecomposeToRotation", (_cordl_self))?;
        Ok(__cordl_ret.into())
    }
    pub fn DecomposeToScale(
        _cordl_self: crate::UnityEngine::Matrix4x4,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector3> {
        let __cordl_ret: crate::UnityEngine::Vector3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("DecomposeToScale", (_cordl_self))?;
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
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<T>,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Filter", (_cordl_self, predicate))?;
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
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IsEquivalentList", (_cordl_self, other))?;
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
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IsValidIndex", (_cordl_self, index))?;
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
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<R>,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Map", (_cordl_self, selector))?;
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
        let __cordl_ret: T = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Reduce", (_cordl_self, func))?;
        Ok(__cordl_ret.into())
    }
    pub fn SwapXAndY(
        _cordl_self: crate::UnityEngine::Vector3,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector3> {
        let __cordl_ret: crate::UnityEngine::Vector3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SwapXAndY", (_cordl_self))?;
        Ok(__cordl_ret.into())
    }
    pub fn SwapXAndZ(
        _cordl_self: crate::UnityEngine::Vector3,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector3> {
        let __cordl_ret: crate::UnityEngine::Vector3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SwapXAndZ", (_cordl_self))?;
        Ok(__cordl_ret.into())
    }
    pub fn SwapYAndZ(
        _cordl_self: crate::UnityEngine::Vector3,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector3> {
        let __cordl_ret: crate::UnityEngine::Vector3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SwapYAndZ", (_cordl_self))?;
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
