#[cfg(feature = "UnityEngine+UI+ReflectionMethodsCache")]
#[repr(C)]
#[derive(Debug)]
pub struct ReflectionMethodsCache {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub raycast3D: *mut crate::UnityEngine::UI::ReflectionMethodsCache_Raycast3DCallback,
    pub raycast3DAll: *mut crate::UnityEngine::UI::ReflectionMethodsCache_RaycastAllCallback,
    pub getRaycastNonAlloc: *mut crate::UnityEngine::UI::ReflectionMethodsCache_GetRaycastNonAllocCallback,
    pub raycast2D: *mut crate::UnityEngine::UI::ReflectionMethodsCache_Raycast2DCallback,
    pub getRayIntersectionAll: *mut crate::UnityEngine::UI::ReflectionMethodsCache_GetRayIntersectionAllCallback,
    pub getRayIntersectionAllNonAlloc: *mut crate::UnityEngine::UI::ReflectionMethodsCache_GetRayIntersectionAllNonAllocCallback,
}
#[cfg(feature = "UnityEngine+UI+ReflectionMethodsCache")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::UI::ReflectionMethodsCache =>
    "UnityEngine.UI"."ReflectionMethodsCache"
);
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
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
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
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::UI::ReflectionMethodsCache_GetRayIntersectionAllCallback =>
    "UnityEngine.UI"."ReflectionMethodsCache/GetRayIntersectionAllCallback"
);
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
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult> = __cordl_object
            .invoke("BeginInvoke", (r, f, i, callback, object))?;
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
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::RaycastHit2D>,
        > = __cordl_object.invoke("EndInvoke", (result))?;
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
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::RaycastHit2D>,
        > = __cordl_object.invoke("Invoke", (r, f, i))?;
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
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (object, method))?;
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
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::UI::ReflectionMethodsCache_GetRayIntersectionAllNonAllocCallback =>
    "UnityEngine.UI"."ReflectionMethodsCache/GetRayIntersectionAllNonAllocCallback"
);
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
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult> = __cordl_object
            .invoke("BeginInvoke", (r, results, f, i, callback, object))?;
        Ok(__cordl_ret.into())
    }
    pub fn EndInvoke(
        &mut self,
        result: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("EndInvoke", (result))?;
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
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("Invoke", (r, results, f, i))?;
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
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (object, method))?;
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
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::UI::ReflectionMethodsCache_GetRaycastNonAllocCallback =>
    "UnityEngine.UI"."ReflectionMethodsCache/GetRaycastNonAllocCallback"
);
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
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult> = __cordl_object
            .invoke("BeginInvoke", (r, results, f, i, callback, object))?;
        Ok(__cordl_ret.into())
    }
    pub fn EndInvoke(
        &mut self,
        result: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("EndInvoke", (result))?;
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
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("Invoke", (r, results, f, i))?;
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
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (object, method))?;
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
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::UI::ReflectionMethodsCache_Raycast2DCallback => "UnityEngine.UI"
    ."ReflectionMethodsCache/Raycast2DCallback"
);
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
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult> = __cordl_object
            .invoke("BeginInvoke", (p1, p2, f, i, callback, object))?;
        Ok(__cordl_ret.into())
    }
    pub fn EndInvoke(
        &mut self,
        result: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::RaycastHit2D> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::RaycastHit2D = __cordl_object
            .invoke("EndInvoke", (result))?;
        Ok(__cordl_ret.into())
    }
    pub fn Invoke(
        &mut self,
        p1: crate::UnityEngine::Vector2,
        p2: crate::UnityEngine::Vector2,
        f: f32,
        i: i32,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::RaycastHit2D> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::RaycastHit2D = __cordl_object
            .invoke("Invoke", (p1, p2, f, i))?;
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
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (object, method))?;
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
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::UI::ReflectionMethodsCache_Raycast3DCallback => "UnityEngine.UI"
    ."ReflectionMethodsCache/Raycast3DCallback"
);
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
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult> = __cordl_object
            .invoke("BeginInvoke", (r, hit, f, i, callback, object))?;
        Ok(__cordl_ret.into())
    }
    pub fn EndInvoke(
        &mut self,
        hit: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::RaycastHit>,
        result: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("EndInvoke", (hit, result))?;
        Ok(__cordl_ret.into())
    }
    pub fn Invoke(
        &mut self,
        r: crate::UnityEngine::Ray,
        hit: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::RaycastHit>,
        f: f32,
        i: i32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("Invoke", (r, hit, f, i))?;
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
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (object, method))?;
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
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::UI::ReflectionMethodsCache_RaycastAllCallback => "UnityEngine.UI"
    ."ReflectionMethodsCache/RaycastAllCallback"
);
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
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult> = __cordl_object
            .invoke("BeginInvoke", (r, f, i, callback, object))?;
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
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::RaycastHit>,
        > = __cordl_object.invoke("EndInvoke", (result))?;
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
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::RaycastHit>,
        > = __cordl_object.invoke("Invoke", (r, f, i))?;
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
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (object, method))?;
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
