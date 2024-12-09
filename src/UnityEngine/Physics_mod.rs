#[cfg(feature = "UnityEngine+Physics")]
#[repr(C)]
#[derive(Debug)]
pub struct Physics {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "UnityEngine+Physics")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::Physics => "UnityEngine"."Physics"
);
#[cfg(feature = "UnityEngine+Physics")]
impl std::ops::Deref for crate::UnityEngine::Physics {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Physics")]
impl std::ops::DerefMut for crate::UnityEngine::Physics {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Physics")]
impl crate::UnityEngine::Physics {
    pub const AllLayers: i32 = -1i32;
    pub const DefaultRaycastLayers: i32 = -5i32;
    pub const IgnoreRaycastLayer: i32 = 4i32;
    pub const kAllLayers: i32 = -1i32;
    pub const kDefaultRaycastLayers: i32 = -5i32;
    pub const kIgnoreRaycastLayer: i32 = 4i32;
    pub const k_MaxFloatMinusEpsilon: f32 = 340282330000000000000000000000000000000f32;
    #[cfg(feature = "UnityEngine+Physics+ContactEventDelegate")]
    pub type ContactEventDelegate = crate::UnityEngine::Physics_ContactEventDelegate;
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "UnityEngine+Physics")]
impl quest_hook::libil2cpp::ObjectType for crate::UnityEngine::Physics {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "UnityEngine+Physics+ContactEventDelegate")]
#[repr(C)]
#[derive(Debug)]
pub struct Physics_ContactEventDelegate {
    __cordl_parent: crate::System::MulticastDelegate,
}
#[cfg(feature = "UnityEngine+Physics+ContactEventDelegate")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::Physics_ContactEventDelegate =>
    "UnityEngine"."Physics/ContactEventDelegate"
);
#[cfg(feature = "UnityEngine+Physics+ContactEventDelegate")]
impl std::ops::Deref for crate::UnityEngine::Physics_ContactEventDelegate {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Physics+ContactEventDelegate")]
impl std::ops::DerefMut for crate::UnityEngine::Physics_ContactEventDelegate {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Physics+ContactEventDelegate")]
impl crate::UnityEngine::Physics_ContactEventDelegate {
    pub fn BeginInvoke(
        &mut self,
        scene: crate::UnityEngine::PhysicsScene,
        headerArray: crate::Unity::Collections::NativeArray_1_ReadOnly<
            crate::UnityEngine::ContactPairHeader,
        >,
        callback: *mut crate::System::AsyncCallback,
        object: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::IAsyncResult> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::IAsyncResult = __cordl_object
            .invoke("BeginInvoke", (scene, headerArray, callback, object))?;
        Ok(__cordl_ret)
    }
    pub fn EndInvoke(
        &mut self,
        result: *mut crate::System::IAsyncResult,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("EndInvoke", (result))?;
        Ok(__cordl_ret)
    }
    pub fn Invoke(
        &mut self,
        scene: crate::UnityEngine::PhysicsScene,
        headerArray: crate::Unity::Collections::NativeArray_1_ReadOnly<
            crate::UnityEngine::ContactPairHeader,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Invoke", (scene, headerArray))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        object: *mut quest_hook::libil2cpp::Il2CppObject,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (object, method))?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
        object: *mut quest_hook::libil2cpp::Il2CppObject,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (object, method))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "UnityEngine+Physics+ContactEventDelegate")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::Physics_ContactEventDelegate {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
