#[cfg(feature = "UnityEngine+Object")]
#[repr(C)]
#[derive(Debug)]
pub struct Object {
    __cordl_parent: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    pub m_CachedPtr: crate::System::IntPtr,
}
#[cfg(feature = "UnityEngine+Object")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::Object => "UnityEngine"."Object"
);
#[cfg(feature = "UnityEngine+Object")]
impl std::ops::Deref for crate::UnityEngine::Object {
    type Target = quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Object")]
impl std::ops::DerefMut for crate::UnityEngine::Object {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Object")]
impl crate::UnityEngine::Object {
    pub const cloneDestroyedMessage: &'static str = "Instantiate failed because the clone was destroyed during creation. This can happen if DestroyImmediate is called in MonoBehaviour.Awake.";
    pub const objectIsNullMessage: &'static str = "The Object you want to instantiate is null.";
    pub fn CheckNullArgument(
        arg: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        message: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CheckNullArgument", (arg, message))?;
        Ok(__cordl_ret.into())
    }
    pub fn CompareBaseObjects(
        lhs: quest_hook::libil2cpp::Gc<crate::UnityEngine::Object>,
        rhs: quest_hook::libil2cpp::Gc<crate::UnityEngine::Object>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CompareBaseObjects", (lhs, rhs))?;
        Ok(__cordl_ret.into())
    }
    pub fn CurrentThreadIsMainThread() -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CurrentThreadIsMainThread", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn DestroyImmediate_Gc1(
        obj: quest_hook::libil2cpp::Gc<crate::UnityEngine::Object>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("DestroyImmediate", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn DestroyImmediate__cordl_bool0(
        obj: quest_hook::libil2cpp::Gc<crate::UnityEngine::Object>,
        allowDestroyingAssets: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("DestroyImmediate", (obj, allowDestroyingAssets))?;
        Ok(__cordl_ret.into())
    }
    pub fn DestroyObject_Gc1(
        obj: quest_hook::libil2cpp::Gc<crate::UnityEngine::Object>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("DestroyObject", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn DestroyObject_f32_0(
        obj: quest_hook::libil2cpp::Gc<crate::UnityEngine::Object>,
        t: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("DestroyObject", (obj, t))?;
        Ok(__cordl_ret.into())
    }
    pub fn Destroy_Gc1(
        obj: quest_hook::libil2cpp::Gc<crate::UnityEngine::Object>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Destroy", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn Destroy_f32_0(
        obj: quest_hook::libil2cpp::Gc<crate::UnityEngine::Object>,
        t: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Destroy", (obj, t))?;
        Ok(__cordl_ret.into())
    }
    pub fn DoesObjectWithInstanceIDExist(
        instanceID: i32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("DoesObjectWithInstanceIDExist", (instanceID))?;
        Ok(__cordl_ret.into())
    }
    pub fn DontDestroyOnLoad(
        target: quest_hook::libil2cpp::Gc<crate::UnityEngine::Object>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("DontDestroyOnLoad", (target))?;
        Ok(__cordl_ret.into())
    }
    pub fn EnsureRunningOnMainThread(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("EnsureRunningOnMainThread", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn Equals(
        &mut self,
        other: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("Equals", (other))?;
        Ok(__cordl_ret.into())
    }
    pub fn FindAnyObjectByType_0<T>() -> quest_hook::libil2cpp::Result<T>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: T = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("FindAnyObjectByType", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn FindAnyObjectByType_FindObjectsInactive1<T>(
        findObjectsInactive: crate::UnityEngine::FindObjectsInactive,
    ) -> quest_hook::libil2cpp::Result<T>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: T = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("FindAnyObjectByType", (findObjectsInactive))?;
        Ok(__cordl_ret.into())
    }
    pub fn FindAnyObjectByType_Gc2(
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Object>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::Object> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("FindAnyObjectByType", (_cordl_type))?;
        Ok(__cordl_ret.into())
    }
    pub fn FindAnyObjectByType_Gc_FindObjectsInactive3(
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
        findObjectsInactive: crate::UnityEngine::FindObjectsInactive,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Object>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::Object> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("FindAnyObjectByType", (_cordl_type, findObjectsInactive))?;
        Ok(__cordl_ret.into())
    }
    pub fn FindFirstObjectByType_0<T>() -> quest_hook::libil2cpp::Result<T>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: T = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("FindFirstObjectByType", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn FindFirstObjectByType_FindObjectsInactive1<T>(
        findObjectsInactive: crate::UnityEngine::FindObjectsInactive,
    ) -> quest_hook::libil2cpp::Result<T>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: T = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("FindFirstObjectByType", (findObjectsInactive))?;
        Ok(__cordl_ret.into())
    }
    pub fn FindFirstObjectByType_Gc2(
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Object>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::Object> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("FindFirstObjectByType", (_cordl_type))?;
        Ok(__cordl_ret.into())
    }
    pub fn FindFirstObjectByType_Gc_FindObjectsInactive3(
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
        findObjectsInactive: crate::UnityEngine::FindObjectsInactive,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Object>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::Object> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("FindFirstObjectByType", (_cordl_type, findObjectsInactive))?;
        Ok(__cordl_ret.into())
    }
    pub fn FindObjectFromInstanceID(
        instanceID: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Object>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::Object> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("FindObjectFromInstanceID", (instanceID))?;
        Ok(__cordl_ret.into())
    }
    pub fn FindObjectOfType_0<T>() -> quest_hook::libil2cpp::Result<T>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: T = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("FindObjectOfType", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn FindObjectOfType_Gc2(
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Object>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::Object> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("FindObjectOfType", (_cordl_type))?;
        Ok(__cordl_ret.into())
    }
    pub fn FindObjectOfType_Gc__cordl_bool3(
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
        includeInactive: bool,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Object>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::Object> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("FindObjectOfType", (_cordl_type, includeInactive))?;
        Ok(__cordl_ret.into())
    }
    pub fn FindObjectOfType__cordl_bool1<T>(
        includeInactive: bool,
    ) -> quest_hook::libil2cpp::Result<T>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: T = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("FindObjectOfType", (includeInactive))?;
        Ok(__cordl_ret.into())
    }
    pub fn FindObjectsByType_FindObjectsInactive_FindObjectsSortMode3<T>(
        findObjectsInactive: crate::UnityEngine::FindObjectsInactive,
        sortMode: crate::UnityEngine::FindObjectsSortMode,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<T>>,
    >
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<T>,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("FindObjectsByType", (findObjectsInactive, sortMode))?;
        Ok(__cordl_ret.into())
    }
    pub fn FindObjectsByType_FindObjectsSortMode2<T>(
        sortMode: crate::UnityEngine::FindObjectsSortMode,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<T>>,
    >
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<T>,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("FindObjectsByType", (sortMode))?;
        Ok(__cordl_ret.into())
    }
    pub fn FindObjectsByType_Gc_FindObjectsInactive_FindObjectsSortMode1(
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
        findObjectsInactive: crate::UnityEngine::FindObjectsInactive,
        sortMode: crate::UnityEngine::FindObjectsSortMode,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<crate::UnityEngine::Object>,
            >,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<crate::UnityEngine::Object>,
            >,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("FindObjectsByType", (_cordl_type, findObjectsInactive, sortMode))?;
        Ok(__cordl_ret.into())
    }
    pub fn FindObjectsByType_Gc_FindObjectsSortMode0(
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
        sortMode: crate::UnityEngine::FindObjectsSortMode,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<crate::UnityEngine::Object>,
            >,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<crate::UnityEngine::Object>,
            >,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("FindObjectsByType", (_cordl_type, sortMode))?;
        Ok(__cordl_ret.into())
    }
    pub fn FindObjectsOfTypeAll(
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<crate::UnityEngine::Object>,
            >,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<crate::UnityEngine::Object>,
            >,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("FindObjectsOfTypeAll", (_cordl_type))?;
        Ok(__cordl_ret.into())
    }
    pub fn FindObjectsOfTypeIncludingAssets(
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<crate::UnityEngine::Object>,
            >,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<crate::UnityEngine::Object>,
            >,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("FindObjectsOfTypeIncludingAssets", (_cordl_type))?;
        Ok(__cordl_ret.into())
    }
    pub fn FindObjectsOfType_2<T>() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<T>>,
    >
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<T>,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("FindObjectsOfType", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn FindObjectsOfType_Gc0(
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<crate::UnityEngine::Object>,
            >,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<crate::UnityEngine::Object>,
            >,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("FindObjectsOfType", (_cordl_type))?;
        Ok(__cordl_ret.into())
    }
    pub fn FindObjectsOfType_Gc__cordl_bool1(
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
        includeInactive: bool,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<crate::UnityEngine::Object>,
            >,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<crate::UnityEngine::Object>,
            >,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("FindObjectsOfType", (_cordl_type, includeInactive))?;
        Ok(__cordl_ret.into())
    }
    pub fn FindObjectsOfType__cordl_bool3<T>(
        includeInactive: bool,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<T>>,
    >
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<T>,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("FindObjectsOfType", (includeInactive))?;
        Ok(__cordl_ret.into())
    }
    pub fn FindSceneObjectsOfType(
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<crate::UnityEngine::Object>,
            >,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<crate::UnityEngine::Object>,
            >,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("FindSceneObjectsOfType", (_cordl_type))?;
        Ok(__cordl_ret.into())
    }
    pub fn ForceLoadFromInstanceID(
        instanceID: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Object>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::Object> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ForceLoadFromInstanceID", (instanceID))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetCachedPtr(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::IntPtr = __cordl_object
            .invoke("GetCachedPtr", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetHashCode(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("GetHashCode", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetInstanceID(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("GetInstanceID", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetName(
        obj: quest_hook::libil2cpp::Gc<crate::UnityEngine::Object>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class().invoke("GetName", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetOffsetOfInstanceIDInCPlusPlusObject() -> quest_hook::libil2cpp::Result<
        i32,
    > {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetOffsetOfInstanceIDInCPlusPlusObject", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn InstantiateAsync_Gc1<T>(
        original: T,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::Transform>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<T>>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Gc<T> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("InstantiateAsync", (original, parent))?;
        Ok(__cordl_ret.into())
    }
    pub fn InstantiateAsync_Gc_Vector3_Quaternion3<T>(
        original: T,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::Transform>,
        position: crate::UnityEngine::Vector3,
        rotation: crate::UnityEngine::Quaternion,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<T>>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Gc<T> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("InstantiateAsync", (original, parent, position, rotation))?;
        Ok(__cordl_ret.into())
    }
    pub fn InstantiateAsync_T0<T>(
        original: T,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<T>>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Gc<T> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("InstantiateAsync", (original))?;
        Ok(__cordl_ret.into())
    }
    pub fn InstantiateAsync_Vector3_Quaternion2<T>(
        original: T,
        position: crate::UnityEngine::Vector3,
        rotation: crate::UnityEngine::Quaternion,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<T>>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Gc<T> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("InstantiateAsync", (original, position, rotation))?;
        Ok(__cordl_ret.into())
    }
    pub fn InstantiateAsync_i32_4<T>(
        original: T,
        count: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<T>>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Gc<T> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("InstantiateAsync", (original, count))?;
        Ok(__cordl_ret.into())
    }
    pub fn InstantiateAsync_i32_Gc5<T>(
        original: T,
        count: i32,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::Transform>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<T>>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Gc<T> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("InstantiateAsync", (original, count, parent))?;
        Ok(__cordl_ret.into())
    }
    pub fn InstantiateAsync_i32_Gc_ReadOnlySpan_1_ReadOnlySpan_1_9<T>(
        original: T,
        count: i32,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::Transform>,
        positions: crate::System::ReadOnlySpan_1<crate::UnityEngine::Vector3>,
        rotations: crate::System::ReadOnlySpan_1<crate::UnityEngine::Quaternion>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<T>>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Gc<T> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "InstantiateAsync",
                (original, count, parent, positions, rotations),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn InstantiateAsync_i32_Gc_Vector3_Quaternion8<T>(
        original: T,
        count: i32,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::Transform>,
        position: crate::UnityEngine::Vector3,
        rotation: crate::UnityEngine::Quaternion,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<T>>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Gc<T> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("InstantiateAsync", (original, count, parent, position, rotation))?;
        Ok(__cordl_ret.into())
    }
    pub fn InstantiateAsync_i32_ReadOnlySpan_1_ReadOnlySpan_1_7<T>(
        original: T,
        count: i32,
        positions: crate::System::ReadOnlySpan_1<crate::UnityEngine::Vector3>,
        rotations: crate::System::ReadOnlySpan_1<crate::UnityEngine::Quaternion>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<T>>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Gc<T> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("InstantiateAsync", (original, count, positions, rotations))?;
        Ok(__cordl_ret.into())
    }
    pub fn InstantiateAsync_i32_Vector3_Quaternion6<T>(
        original: T,
        count: i32,
        position: crate::UnityEngine::Vector3,
        rotation: crate::UnityEngine::Quaternion,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<T>>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Gc<T> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("InstantiateAsync", (original, count, position, rotation))?;
        Ok(__cordl_ret.into())
    }
    pub fn Instantiate_Gc2(
        original: quest_hook::libil2cpp::Gc<crate::UnityEngine::Object>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Object>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::Object> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Instantiate", (original))?;
        Ok(__cordl_ret.into())
    }
    pub fn Instantiate_Gc_Gc4(
        original: quest_hook::libil2cpp::Gc<crate::UnityEngine::Object>,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::Transform>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Object>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::Object> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Instantiate", (original, parent))?;
        Ok(__cordl_ret.into())
    }
    pub fn Instantiate_Gc_Gc__cordl_bool5(
        original: quest_hook::libil2cpp::Gc<crate::UnityEngine::Object>,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::Transform>,
        instantiateInWorldSpace: bool,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Object>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::Object> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Instantiate", (original, parent, instantiateInWorldSpace))?;
        Ok(__cordl_ret.into())
    }
    pub fn Instantiate_Gc_Scene3(
        original: quest_hook::libil2cpp::Gc<crate::UnityEngine::Object>,
        scene: crate::UnityEngine::SceneManagement::Scene,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Object>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::Object> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Instantiate", (original, scene))?;
        Ok(__cordl_ret.into())
    }
    pub fn Instantiate_Gc_Vector3_Quaternion0(
        original: quest_hook::libil2cpp::Gc<crate::UnityEngine::Object>,
        position: crate::UnityEngine::Vector3,
        rotation: crate::UnityEngine::Quaternion,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Object>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::Object> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Instantiate", (original, position, rotation))?;
        Ok(__cordl_ret.into())
    }
    pub fn Instantiate_Gc_Vector3_Quaternion_Gc1(
        original: quest_hook::libil2cpp::Gc<crate::UnityEngine::Object>,
        position: crate::UnityEngine::Vector3,
        rotation: crate::UnityEngine::Quaternion,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::Transform>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Object>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::Object> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Instantiate", (original, position, rotation, parent))?;
        Ok(__cordl_ret.into())
    }
    pub fn Instantiate_T6<T>(original: T) -> quest_hook::libil2cpp::Result<T>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: T = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Instantiate", (original))?;
        Ok(__cordl_ret.into())
    }
    pub fn Instantiate_T_Gc9<T>(
        original: T,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::Transform>,
    ) -> quest_hook::libil2cpp::Result<T>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: T = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Instantiate", (original, parent))?;
        Ok(__cordl_ret.into())
    }
    pub fn Instantiate_T_Gc__cordl_bool10<T>(
        original: T,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::Transform>,
        worldPositionStays: bool,
    ) -> quest_hook::libil2cpp::Result<T>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: T = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Instantiate", (original, parent, worldPositionStays))?;
        Ok(__cordl_ret.into())
    }
    pub fn Instantiate_T_Vector3_Quaternion7<T>(
        original: T,
        position: crate::UnityEngine::Vector3,
        rotation: crate::UnityEngine::Quaternion,
    ) -> quest_hook::libil2cpp::Result<T>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: T = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Instantiate", (original, position, rotation))?;
        Ok(__cordl_ret.into())
    }
    pub fn Instantiate_T_Vector3_Quaternion_Gc8<T>(
        original: T,
        position: crate::UnityEngine::Vector3,
        rotation: crate::UnityEngine::Quaternion,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::Transform>,
    ) -> quest_hook::libil2cpp::Result<T>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: T = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Instantiate", (original, position, rotation, parent))?;
        Ok(__cordl_ret.into())
    }
    pub fn Internal_CloneSingle(
        data: quest_hook::libil2cpp::Gc<crate::UnityEngine::Object>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Object>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::Object> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Internal_CloneSingle", (data))?;
        Ok(__cordl_ret.into())
    }
    pub fn Internal_CloneSingleWithParent(
        data: quest_hook::libil2cpp::Gc<crate::UnityEngine::Object>,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::Transform>,
        worldPositionStays: bool,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Object>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::Object> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "Internal_CloneSingleWithParent",
                (data, parent, worldPositionStays),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn Internal_CloneSingleWithScene(
        data: quest_hook::libil2cpp::Gc<crate::UnityEngine::Object>,
        scene: crate::UnityEngine::SceneManagement::Scene,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Object>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::Object> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Internal_CloneSingleWithScene", (data, scene))?;
        Ok(__cordl_ret.into())
    }
    pub fn Internal_CloneSingleWithScene_Injected(
        data: quest_hook::libil2cpp::Gc<crate::UnityEngine::Object>,
        scene: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::SceneManagement::Scene,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Object>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::Object> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Internal_CloneSingleWithScene_Injected", (data, scene))?;
        Ok(__cordl_ret.into())
    }
    pub fn Internal_InstantiateAsyncWithParent(
        original: quest_hook::libil2cpp::Gc<crate::UnityEngine::Object>,
        count: i32,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::Transform>,
        positions: crate::System::IntPtr,
        positionsCount: i32,
        rotations: crate::System::IntPtr,
        rotationsCount: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::AsyncInstantiateOperation>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::AsyncInstantiateOperation,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "Internal_InstantiateAsyncWithParent",
                (
                    original,
                    count,
                    parent,
                    positions,
                    positionsCount,
                    rotations,
                    rotationsCount,
                ),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn Internal_InstantiateSingle(
        data: quest_hook::libil2cpp::Gc<crate::UnityEngine::Object>,
        pos: crate::UnityEngine::Vector3,
        rot: crate::UnityEngine::Quaternion,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Object>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::Object> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Internal_InstantiateSingle", (data, pos, rot))?;
        Ok(__cordl_ret.into())
    }
    pub fn Internal_InstantiateSingleWithParent(
        data: quest_hook::libil2cpp::Gc<crate::UnityEngine::Object>,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::Transform>,
        pos: crate::UnityEngine::Vector3,
        rot: crate::UnityEngine::Quaternion,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Object>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::Object> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Internal_InstantiateSingleWithParent", (data, parent, pos, rot))?;
        Ok(__cordl_ret.into())
    }
    pub fn Internal_InstantiateSingleWithParent_Injected(
        data: quest_hook::libil2cpp::Gc<crate::UnityEngine::Object>,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::Transform>,
        pos: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector3>,
        rot: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Quaternion>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Object>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::Object> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "Internal_InstantiateSingleWithParent_Injected",
                (data, parent, pos, rot),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn Internal_InstantiateSingle_Injected(
        data: quest_hook::libil2cpp::Gc<crate::UnityEngine::Object>,
        pos: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector3>,
        rot: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Quaternion>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Object>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::Object> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Internal_InstantiateSingle_Injected", (data, pos, rot))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsNativeObjectAlive(
        o: quest_hook::libil2cpp::Gc<crate::UnityEngine::Object>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IsNativeObjectAlive", (o))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsPersistent(
        obj: quest_hook::libil2cpp::Gc<crate::UnityEngine::Object>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IsPersistent", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn MarkDirty(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("MarkDirty", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn SetName(
        obj: quest_hook::libil2cpp::Gc<crate::UnityEngine::Object>,
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SetName", (obj, name))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToString_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("ToString", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ToString_Gc1(
        obj: quest_hook::libil2cpp::Gc<crate::UnityEngine::Object>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class().invoke("ToString", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_hideFlags(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::HideFlags> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::HideFlags = __cordl_object
            .invoke("get_hideFlags", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_name(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("get_name", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Equality(
        x: quest_hook::libil2cpp::Gc<crate::UnityEngine::Object>,
        y: quest_hook::libil2cpp::Gc<crate::UnityEngine::Object>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Equality", (x, y))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Implicit(
        exists: quest_hook::libil2cpp::Gc<crate::UnityEngine::Object>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Implicit", (exists))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Inequality(
        x: quest_hook::libil2cpp::Gc<crate::UnityEngine::Object>,
        y: quest_hook::libil2cpp::Gc<crate::UnityEngine::Object>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Inequality", (x, y))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_hideFlags(
        &mut self,
        value: crate::UnityEngine::HideFlags,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_hideFlags", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_name(
        &mut self,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_name", (value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+Object")]
impl quest_hook::libil2cpp::ObjectType for crate::UnityEngine::Object {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
