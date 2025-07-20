#[cfg(feature = "UnityEngine+UI+ReflectionMethodsCache")]
#[repr(C)]
#[derive(Debug)]
pub struct ReflectionMethodsCache {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub raycast3D: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::UI::ReflectionMethodsCache_Raycast3DCallback,
    >,
    pub raycast3DAll: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::UI::ReflectionMethodsCache_RaycastAllCallback,
    >,
    pub getRaycastNonAlloc: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::UI::ReflectionMethodsCache_GetRaycastNonAllocCallback,
    >,
    pub raycast2D: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::UI::ReflectionMethodsCache_Raycast2DCallback,
    >,
    pub getRayIntersectionAll: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::UI::ReflectionMethodsCache_GetRayIntersectionAllCallback,
    >,
    pub getRayIntersectionAllNonAlloc: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::UI::ReflectionMethodsCache_GetRayIntersectionAllNonAllocCallback,
    >,
}
#[cfg(feature = "UnityEngine+UI+ReflectionMethodsCache")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::UI::ReflectionMethodsCache {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.UI";
    const CLASS_NAME: &'static str = "ReflectionMethodsCache";
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
#[cfg(feature = "UnityEngine+UI+ReflectionMethodsCache")]
impl std::ops::Deref for crate::UnityEngine::UI::ReflectionMethodsCache {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UI+ReflectionMethodsCache")]
impl std::ops::DerefMut for crate::UnityEngine::UI::ReflectionMethodsCache {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UI+ReflectionMethodsCache")]
impl crate::UnityEngine::UI::ReflectionMethodsCache {
    #[cfg(
        feature = "UnityEngine+UI+ReflectionMethodsCache+GetRayIntersectionAllCallback"
    )]
    pub type GetRayIntersectionAllCallback = crate::UnityEngine::UI::ReflectionMethodsCache_GetRayIntersectionAllCallback;
    #[cfg(
        feature = "UnityEngine+UI+ReflectionMethodsCache+GetRayIntersectionAllNonAllocCallback"
    )]
    pub type GetRayIntersectionAllNonAllocCallback = crate::UnityEngine::UI::ReflectionMethodsCache_GetRayIntersectionAllNonAllocCallback;
    #[cfg(feature = "UnityEngine+UI+ReflectionMethodsCache+GetRaycastNonAllocCallback")]
    pub type GetRaycastNonAllocCallback = crate::UnityEngine::UI::ReflectionMethodsCache_GetRaycastNonAllocCallback;
    #[cfg(feature = "UnityEngine+UI+ReflectionMethodsCache+Raycast2DCallback")]
    pub type Raycast2DCallback = crate::UnityEngine::UI::ReflectionMethodsCache_Raycast2DCallback;
    #[cfg(feature = "UnityEngine+UI+ReflectionMethodsCache+Raycast3DCallback")]
    pub type Raycast3DCallback = crate::UnityEngine::UI::ReflectionMethodsCache_Raycast3DCallback;
    #[cfg(feature = "UnityEngine+UI+ReflectionMethodsCache+RaycastAllCallback")]
    pub type RaycastAllCallback = crate::UnityEngine::UI::ReflectionMethodsCache_RaycastAllCallback;
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::UI::ReflectionMethodsCache as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), quest_hook::libil2cpp::Void, 0usize>(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::UI::ReflectionMethodsCache as
                    quest_hook::libil2cpp::Type > ::class(), ".ctor", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_Singleton() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::UI::ReflectionMethodsCache>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::UI::ReflectionMethodsCache as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (),
                quest_hook::libil2cpp::Gc<
                    crate::UnityEngine::UI::ReflectionMethodsCache,
                >,
                0usize,
            >("get_Singleton")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::UI::ReflectionMethodsCache as
                    quest_hook::libil2cpp::Type > ::class(), "get_Singleton", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UI::ReflectionMethodsCache,
        > = unsafe { method.invoke_unchecked((), ())? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+UI+ReflectionMethodsCache")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::UI::ReflectionMethodsCache {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "UnityEngine+UI+ReflectionMethodsCache+GetRayIntersectionAllCallback")]
#[repr(C)]
#[derive(Debug)]
pub struct ReflectionMethodsCache_GetRayIntersectionAllCallback {
    __cordl_parent: crate::System::MulticastDelegate,
}
#[cfg(feature = "UnityEngine+UI+ReflectionMethodsCache+GetRayIntersectionAllCallback")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::UI::ReflectionMethodsCache_GetRayIntersectionAllCallback {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.UI";
    const CLASS_NAME: &'static str = "ReflectionMethodsCache/GetRayIntersectionAllCallback";
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
#[cfg(feature = "UnityEngine+UI+ReflectionMethodsCache+GetRayIntersectionAllCallback")]
impl std::ops::Deref
for crate::UnityEngine::UI::ReflectionMethodsCache_GetRayIntersectionAllCallback {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UI+ReflectionMethodsCache+GetRayIntersectionAllCallback")]
impl std::ops::DerefMut
for crate::UnityEngine::UI::ReflectionMethodsCache_GetRayIntersectionAllCallback {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UI+ReflectionMethodsCache+GetRayIntersectionAllCallback")]
impl crate::UnityEngine::UI::ReflectionMethodsCache_GetRayIntersectionAllCallback {
    pub fn BeginInvoke(
        &mut self,
        r: crate::UnityEngine::Ray,
        f: f32,
        i: i32,
        callback: quest_hook::libil2cpp::Gc<crate::System::AsyncCallback>,
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::UI::ReflectionMethodsCache_GetRayIntersectionAllCallback as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    crate::UnityEngine::Ray,
                    f32,
                    i32,
                    quest_hook::libil2cpp::Gc<crate::System::AsyncCallback>,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                ),
                quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
                5usize,
            >("BeginInvoke")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate
                    ::UnityEngine::UI::ReflectionMethodsCache_GetRayIntersectionAllCallback
                    as quest_hook::libil2cpp::Type > ::class(), "BeginInvoke", 5usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult> = unsafe {
            method.invoke_unchecked(self, (r, f, i, callback, object))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn EndInvoke(
        &mut self,
        result: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::RaycastHit2D>,
        >,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::UI::ReflectionMethodsCache_GetRayIntersectionAllCallback as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>),
                quest_hook::libil2cpp::Gc<
                    quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::RaycastHit2D>,
                >,
                1usize,
            >("EndInvoke")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate
                    ::UnityEngine::UI::ReflectionMethodsCache_GetRayIntersectionAllCallback
                    as quest_hook::libil2cpp::Type > ::class(), "EndInvoke", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::RaycastHit2D>,
        > = unsafe { method.invoke_unchecked(self, (result))? };
        Ok(__cordl_ret.into())
    }
    pub fn Invoke(
        &mut self,
        r: crate::UnityEngine::Ray,
        f: f32,
        i: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::RaycastHit2D>,
        >,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::UI::ReflectionMethodsCache_GetRayIntersectionAllCallback as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (crate::UnityEngine::Ray, f32, i32),
                quest_hook::libil2cpp::Gc<
                    quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::RaycastHit2D>,
                >,
                3usize,
            >("Invoke")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate
                    ::UnityEngine::UI::ReflectionMethodsCache_GetRayIntersectionAllCallback
                    as quest_hook::libil2cpp::Type > ::class(), "Invoke", 3usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::RaycastHit2D>,
        > = unsafe { method.invoke_unchecked(self, (r, f, i))? };
        Ok(__cordl_ret.into())
    }
    pub fn New(
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (object, method))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::UI::ReflectionMethodsCache_GetRayIntersectionAllCallback as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                    crate::System::IntPtr,
                ),
                quest_hook::libil2cpp::Void,
                2usize,
            >(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate
                    ::UnityEngine::UI::ReflectionMethodsCache_GetRayIntersectionAllCallback
                    as quest_hook::libil2cpp::Type > ::class(), ".ctor", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (object, method))?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+UI+ReflectionMethodsCache+GetRayIntersectionAllCallback")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::UI::ReflectionMethodsCache_GetRayIntersectionAllCallback {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(
    feature = "UnityEngine+UI+ReflectionMethodsCache+GetRayIntersectionAllNonAllocCallback"
)]
#[repr(C)]
#[derive(Debug)]
pub struct ReflectionMethodsCache_GetRayIntersectionAllNonAllocCallback {
    __cordl_parent: crate::System::MulticastDelegate,
}
#[cfg(
    feature = "UnityEngine+UI+ReflectionMethodsCache+GetRayIntersectionAllNonAllocCallback"
)]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::UI::ReflectionMethodsCache_GetRayIntersectionAllNonAllocCallback {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.UI";
    const CLASS_NAME: &'static str = "ReflectionMethodsCache/GetRayIntersectionAllNonAllocCallback";
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
#[cfg(
    feature = "UnityEngine+UI+ReflectionMethodsCache+GetRayIntersectionAllNonAllocCallback"
)]
impl std::ops::Deref
for crate::UnityEngine::UI::ReflectionMethodsCache_GetRayIntersectionAllNonAllocCallback {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(
    feature = "UnityEngine+UI+ReflectionMethodsCache+GetRayIntersectionAllNonAllocCallback"
)]
impl std::ops::DerefMut
for crate::UnityEngine::UI::ReflectionMethodsCache_GetRayIntersectionAllNonAllocCallback {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(
    feature = "UnityEngine+UI+ReflectionMethodsCache+GetRayIntersectionAllNonAllocCallback"
)]
impl crate::UnityEngine::UI::ReflectionMethodsCache_GetRayIntersectionAllNonAllocCallback {
    pub fn BeginInvoke(
        &mut self,
        r: crate::UnityEngine::Ray,
        results: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::RaycastHit2D>,
        >,
        f: f32,
        i: i32,
        callback: quest_hook::libil2cpp::Gc<crate::System::AsyncCallback>,
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::UI::ReflectionMethodsCache_GetRayIntersectionAllNonAllocCallback as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    crate::UnityEngine::Ray,
                    quest_hook::libil2cpp::Gc<
                        quest_hook::libil2cpp::Il2CppArray<
                            crate::UnityEngine::RaycastHit2D,
                        >,
                    >,
                    f32,
                    i32,
                    quest_hook::libil2cpp::Gc<crate::System::AsyncCallback>,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                ),
                quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
                6usize,
            >("BeginInvoke")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate
                    ::UnityEngine::UI::ReflectionMethodsCache_GetRayIntersectionAllNonAllocCallback
                    as quest_hook::libil2cpp::Type > ::class(), "BeginInvoke", 6usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult> = unsafe {
            method.invoke_unchecked(self, (r, results, f, i, callback, object))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn EndInvoke(
        &mut self,
        result: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::UI::ReflectionMethodsCache_GetRayIntersectionAllNonAllocCallback as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>),
                i32,
                1usize,
            >("EndInvoke")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate
                    ::UnityEngine::UI::ReflectionMethodsCache_GetRayIntersectionAllNonAllocCallback
                    as quest_hook::libil2cpp::Type > ::class(), "EndInvoke", 1usize
                )
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked(self, (result))? };
        Ok(__cordl_ret.into())
    }
    pub fn Invoke(
        &mut self,
        r: crate::UnityEngine::Ray,
        results: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::RaycastHit2D>,
        >,
        f: f32,
        i: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::UI::ReflectionMethodsCache_GetRayIntersectionAllNonAllocCallback as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    crate::UnityEngine::Ray,
                    quest_hook::libil2cpp::Gc<
                        quest_hook::libil2cpp::Il2CppArray<
                            crate::UnityEngine::RaycastHit2D,
                        >,
                    >,
                    f32,
                    i32,
                ),
                i32,
                4usize,
            >("Invoke")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate
                    ::UnityEngine::UI::ReflectionMethodsCache_GetRayIntersectionAllNonAllocCallback
                    as quest_hook::libil2cpp::Type > ::class(), "Invoke", 4usize
                )
            });
        let __cordl_ret: i32 = unsafe {
            method.invoke_unchecked(self, (r, results, f, i))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn New(
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (object, method))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::UI::ReflectionMethodsCache_GetRayIntersectionAllNonAllocCallback as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                    crate::System::IntPtr,
                ),
                quest_hook::libil2cpp::Void,
                2usize,
            >(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate
                    ::UnityEngine::UI::ReflectionMethodsCache_GetRayIntersectionAllNonAllocCallback
                    as quest_hook::libil2cpp::Type > ::class(), ".ctor", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (object, method))?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(
    feature = "UnityEngine+UI+ReflectionMethodsCache+GetRayIntersectionAllNonAllocCallback"
)]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::UI::ReflectionMethodsCache_GetRayIntersectionAllNonAllocCallback {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "UnityEngine+UI+ReflectionMethodsCache+GetRaycastNonAllocCallback")]
#[repr(C)]
#[derive(Debug)]
pub struct ReflectionMethodsCache_GetRaycastNonAllocCallback {
    __cordl_parent: crate::System::MulticastDelegate,
}
#[cfg(feature = "UnityEngine+UI+ReflectionMethodsCache+GetRaycastNonAllocCallback")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::UI::ReflectionMethodsCache_GetRaycastNonAllocCallback {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.UI";
    const CLASS_NAME: &'static str = "ReflectionMethodsCache/GetRaycastNonAllocCallback";
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
#[cfg(feature = "UnityEngine+UI+ReflectionMethodsCache+GetRaycastNonAllocCallback")]
impl std::ops::Deref
for crate::UnityEngine::UI::ReflectionMethodsCache_GetRaycastNonAllocCallback {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UI+ReflectionMethodsCache+GetRaycastNonAllocCallback")]
impl std::ops::DerefMut
for crate::UnityEngine::UI::ReflectionMethodsCache_GetRaycastNonAllocCallback {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UI+ReflectionMethodsCache+GetRaycastNonAllocCallback")]
impl crate::UnityEngine::UI::ReflectionMethodsCache_GetRaycastNonAllocCallback {
    pub fn BeginInvoke(
        &mut self,
        r: crate::UnityEngine::Ray,
        results: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::RaycastHit>,
        >,
        f: f32,
        i: i32,
        callback: quest_hook::libil2cpp::Gc<crate::System::AsyncCallback>,
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::UI::ReflectionMethodsCache_GetRaycastNonAllocCallback as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    crate::UnityEngine::Ray,
                    quest_hook::libil2cpp::Gc<
                        quest_hook::libil2cpp::Il2CppArray<
                            crate::UnityEngine::RaycastHit,
                        >,
                    >,
                    f32,
                    i32,
                    quest_hook::libil2cpp::Gc<crate::System::AsyncCallback>,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                ),
                quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
                6usize,
            >("BeginInvoke")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate
                    ::UnityEngine::UI::ReflectionMethodsCache_GetRaycastNonAllocCallback
                    as quest_hook::libil2cpp::Type > ::class(), "BeginInvoke", 6usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult> = unsafe {
            method.invoke_unchecked(self, (r, results, f, i, callback, object))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn EndInvoke(
        &mut self,
        result: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::UI::ReflectionMethodsCache_GetRaycastNonAllocCallback as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>),
                i32,
                1usize,
            >("EndInvoke")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate
                    ::UnityEngine::UI::ReflectionMethodsCache_GetRaycastNonAllocCallback
                    as quest_hook::libil2cpp::Type > ::class(), "EndInvoke", 1usize
                )
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked(self, (result))? };
        Ok(__cordl_ret.into())
    }
    pub fn Invoke(
        &mut self,
        r: crate::UnityEngine::Ray,
        results: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::RaycastHit>,
        >,
        f: f32,
        i: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::UI::ReflectionMethodsCache_GetRaycastNonAllocCallback as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    crate::UnityEngine::Ray,
                    quest_hook::libil2cpp::Gc<
                        quest_hook::libil2cpp::Il2CppArray<
                            crate::UnityEngine::RaycastHit,
                        >,
                    >,
                    f32,
                    i32,
                ),
                i32,
                4usize,
            >("Invoke")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate
                    ::UnityEngine::UI::ReflectionMethodsCache_GetRaycastNonAllocCallback
                    as quest_hook::libil2cpp::Type > ::class(), "Invoke", 4usize
                )
            });
        let __cordl_ret: i32 = unsafe {
            method.invoke_unchecked(self, (r, results, f, i))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn New(
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (object, method))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::UI::ReflectionMethodsCache_GetRaycastNonAllocCallback as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                    crate::System::IntPtr,
                ),
                quest_hook::libil2cpp::Void,
                2usize,
            >(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate
                    ::UnityEngine::UI::ReflectionMethodsCache_GetRaycastNonAllocCallback
                    as quest_hook::libil2cpp::Type > ::class(), ".ctor", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (object, method))?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+UI+ReflectionMethodsCache+GetRaycastNonAllocCallback")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::UI::ReflectionMethodsCache_GetRaycastNonAllocCallback {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "UnityEngine+UI+ReflectionMethodsCache+Raycast2DCallback")]
#[repr(C)]
#[derive(Debug)]
pub struct ReflectionMethodsCache_Raycast2DCallback {
    __cordl_parent: crate::System::MulticastDelegate,
}
#[cfg(feature = "UnityEngine+UI+ReflectionMethodsCache+Raycast2DCallback")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::UI::ReflectionMethodsCache_Raycast2DCallback {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.UI";
    const CLASS_NAME: &'static str = "ReflectionMethodsCache/Raycast2DCallback";
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
#[cfg(feature = "UnityEngine+UI+ReflectionMethodsCache+Raycast2DCallback")]
impl std::ops::Deref
for crate::UnityEngine::UI::ReflectionMethodsCache_Raycast2DCallback {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UI+ReflectionMethodsCache+Raycast2DCallback")]
impl std::ops::DerefMut
for crate::UnityEngine::UI::ReflectionMethodsCache_Raycast2DCallback {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UI+ReflectionMethodsCache+Raycast2DCallback")]
impl crate::UnityEngine::UI::ReflectionMethodsCache_Raycast2DCallback {
    pub fn BeginInvoke(
        &mut self,
        p1: crate::UnityEngine::Vector2,
        p2: crate::UnityEngine::Vector2,
        f: f32,
        i: i32,
        callback: quest_hook::libil2cpp::Gc<crate::System::AsyncCallback>,
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::UI::ReflectionMethodsCache_Raycast2DCallback as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    crate::UnityEngine::Vector2,
                    crate::UnityEngine::Vector2,
                    f32,
                    i32,
                    quest_hook::libil2cpp::Gc<crate::System::AsyncCallback>,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                ),
                quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
                6usize,
            >("BeginInvoke")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::UI::ReflectionMethodsCache_Raycast2DCallback as
                    quest_hook::libil2cpp::Type > ::class(), "BeginInvoke", 6usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult> = unsafe {
            method.invoke_unchecked(self, (p1, p2, f, i, callback, object))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn EndInvoke(
        &mut self,
        result: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::RaycastHit2D> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::UI::ReflectionMethodsCache_Raycast2DCallback as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>),
                crate::UnityEngine::RaycastHit2D,
                1usize,
            >("EndInvoke")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::UI::ReflectionMethodsCache_Raycast2DCallback as
                    quest_hook::libil2cpp::Type > ::class(), "EndInvoke", 1usize
                )
            });
        let __cordl_ret: crate::UnityEngine::RaycastHit2D = unsafe {
            method.invoke_unchecked(self, (result))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Invoke(
        &mut self,
        p1: crate::UnityEngine::Vector2,
        p2: crate::UnityEngine::Vector2,
        f: f32,
        i: i32,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::RaycastHit2D> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::UI::ReflectionMethodsCache_Raycast2DCallback as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (crate::UnityEngine::Vector2, crate::UnityEngine::Vector2, f32, i32),
                crate::UnityEngine::RaycastHit2D,
                4usize,
            >("Invoke")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::UI::ReflectionMethodsCache_Raycast2DCallback as
                    quest_hook::libil2cpp::Type > ::class(), "Invoke", 4usize
                )
            });
        let __cordl_ret: crate::UnityEngine::RaycastHit2D = unsafe {
            method.invoke_unchecked(self, (p1, p2, f, i))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn New(
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (object, method))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::UI::ReflectionMethodsCache_Raycast2DCallback as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                    crate::System::IntPtr,
                ),
                quest_hook::libil2cpp::Void,
                2usize,
            >(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::UI::ReflectionMethodsCache_Raycast2DCallback as
                    quest_hook::libil2cpp::Type > ::class(), ".ctor", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (object, method))?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+UI+ReflectionMethodsCache+Raycast2DCallback")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::UI::ReflectionMethodsCache_Raycast2DCallback {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "UnityEngine+UI+ReflectionMethodsCache+Raycast3DCallback")]
#[repr(C)]
#[derive(Debug)]
pub struct ReflectionMethodsCache_Raycast3DCallback {
    __cordl_parent: crate::System::MulticastDelegate,
}
#[cfg(feature = "UnityEngine+UI+ReflectionMethodsCache+Raycast3DCallback")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::UI::ReflectionMethodsCache_Raycast3DCallback {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.UI";
    const CLASS_NAME: &'static str = "ReflectionMethodsCache/Raycast3DCallback";
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
#[cfg(feature = "UnityEngine+UI+ReflectionMethodsCache+Raycast3DCallback")]
impl std::ops::Deref
for crate::UnityEngine::UI::ReflectionMethodsCache_Raycast3DCallback {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UI+ReflectionMethodsCache+Raycast3DCallback")]
impl std::ops::DerefMut
for crate::UnityEngine::UI::ReflectionMethodsCache_Raycast3DCallback {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UI+ReflectionMethodsCache+Raycast3DCallback")]
impl crate::UnityEngine::UI::ReflectionMethodsCache_Raycast3DCallback {
    pub fn BeginInvoke(
        &mut self,
        r: crate::UnityEngine::Ray,
        hit: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::RaycastHit>,
        f: f32,
        i: i32,
        callback: quest_hook::libil2cpp::Gc<crate::System::AsyncCallback>,
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::UI::ReflectionMethodsCache_Raycast3DCallback as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    crate::UnityEngine::Ray,
                    quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::RaycastHit>,
                    f32,
                    i32,
                    quest_hook::libil2cpp::Gc<crate::System::AsyncCallback>,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                ),
                quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
                6usize,
            >("BeginInvoke")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::UI::ReflectionMethodsCache_Raycast3DCallback as
                    quest_hook::libil2cpp::Type > ::class(), "BeginInvoke", 6usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult> = unsafe {
            method.invoke_unchecked(self, (r, hit, f, i, callback, object))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn EndInvoke(
        &mut self,
        hit: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::RaycastHit>,
        result: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::UI::ReflectionMethodsCache_Raycast3DCallback as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::RaycastHit>,
                    quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
                ),
                bool,
                2usize,
            >("EndInvoke")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::UI::ReflectionMethodsCache_Raycast3DCallback as
                    quest_hook::libil2cpp::Type > ::class(), "EndInvoke", 2usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked(self, (hit, result))? };
        Ok(__cordl_ret.into())
    }
    pub fn Invoke(
        &mut self,
        r: crate::UnityEngine::Ray,
        hit: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::RaycastHit>,
        f: f32,
        i: i32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::UI::ReflectionMethodsCache_Raycast3DCallback as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    crate::UnityEngine::Ray,
                    quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::RaycastHit>,
                    f32,
                    i32,
                ),
                bool,
                4usize,
            >("Invoke")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::UI::ReflectionMethodsCache_Raycast3DCallback as
                    quest_hook::libil2cpp::Type > ::class(), "Invoke", 4usize
                )
            });
        let __cordl_ret: bool = unsafe {
            method.invoke_unchecked(self, (r, hit, f, i))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn New(
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (object, method))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::UI::ReflectionMethodsCache_Raycast3DCallback as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                    crate::System::IntPtr,
                ),
                quest_hook::libil2cpp::Void,
                2usize,
            >(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::UI::ReflectionMethodsCache_Raycast3DCallback as
                    quest_hook::libil2cpp::Type > ::class(), ".ctor", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (object, method))?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+UI+ReflectionMethodsCache+Raycast3DCallback")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::UI::ReflectionMethodsCache_Raycast3DCallback {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "UnityEngine+UI+ReflectionMethodsCache+RaycastAllCallback")]
#[repr(C)]
#[derive(Debug)]
pub struct ReflectionMethodsCache_RaycastAllCallback {
    __cordl_parent: crate::System::MulticastDelegate,
}
#[cfg(feature = "UnityEngine+UI+ReflectionMethodsCache+RaycastAllCallback")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::UI::ReflectionMethodsCache_RaycastAllCallback {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.UI";
    const CLASS_NAME: &'static str = "ReflectionMethodsCache/RaycastAllCallback";
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
#[cfg(feature = "UnityEngine+UI+ReflectionMethodsCache+RaycastAllCallback")]
impl std::ops::Deref
for crate::UnityEngine::UI::ReflectionMethodsCache_RaycastAllCallback {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UI+ReflectionMethodsCache+RaycastAllCallback")]
impl std::ops::DerefMut
for crate::UnityEngine::UI::ReflectionMethodsCache_RaycastAllCallback {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UI+ReflectionMethodsCache+RaycastAllCallback")]
impl crate::UnityEngine::UI::ReflectionMethodsCache_RaycastAllCallback {
    pub fn BeginInvoke(
        &mut self,
        r: crate::UnityEngine::Ray,
        f: f32,
        i: i32,
        callback: quest_hook::libil2cpp::Gc<crate::System::AsyncCallback>,
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::UI::ReflectionMethodsCache_RaycastAllCallback as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    crate::UnityEngine::Ray,
                    f32,
                    i32,
                    quest_hook::libil2cpp::Gc<crate::System::AsyncCallback>,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                ),
                quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
                5usize,
            >("BeginInvoke")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::UI::ReflectionMethodsCache_RaycastAllCallback as
                    quest_hook::libil2cpp::Type > ::class(), "BeginInvoke", 5usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult> = unsafe {
            method.invoke_unchecked(self, (r, f, i, callback, object))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn EndInvoke(
        &mut self,
        result: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::RaycastHit>,
        >,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::UI::ReflectionMethodsCache_RaycastAllCallback as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>),
                quest_hook::libil2cpp::Gc<
                    quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::RaycastHit>,
                >,
                1usize,
            >("EndInvoke")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::UI::ReflectionMethodsCache_RaycastAllCallback as
                    quest_hook::libil2cpp::Type > ::class(), "EndInvoke", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::RaycastHit>,
        > = unsafe { method.invoke_unchecked(self, (result))? };
        Ok(__cordl_ret.into())
    }
    pub fn Invoke(
        &mut self,
        r: crate::UnityEngine::Ray,
        f: f32,
        i: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::RaycastHit>,
        >,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::UI::ReflectionMethodsCache_RaycastAllCallback as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (crate::UnityEngine::Ray, f32, i32),
                quest_hook::libil2cpp::Gc<
                    quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::RaycastHit>,
                >,
                3usize,
            >("Invoke")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::UI::ReflectionMethodsCache_RaycastAllCallback as
                    quest_hook::libil2cpp::Type > ::class(), "Invoke", 3usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::RaycastHit>,
        > = unsafe { method.invoke_unchecked(self, (r, f, i))? };
        Ok(__cordl_ret.into())
    }
    pub fn New(
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (object, method))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::UI::ReflectionMethodsCache_RaycastAllCallback as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                    crate::System::IntPtr,
                ),
                quest_hook::libil2cpp::Void,
                2usize,
            >(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::UI::ReflectionMethodsCache_RaycastAllCallback as
                    quest_hook::libil2cpp::Type > ::class(), ".ctor", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (object, method))?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+UI+ReflectionMethodsCache+RaycastAllCallback")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::UI::ReflectionMethodsCache_RaycastAllCallback {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
