#[cfg(feature = "NoteDebris")]
#[repr(C)]
#[derive(Debug)]
pub struct NoteDebris {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
    pub _meshTransform: *mut crate::UnityEngine::Transform,
    pub _physics: *mut crate::GlobalNamespace::NoteDebrisPhysics,
    pub _materialPropertyBlockController: *mut crate::GlobalNamespace::MaterialPropertyBlockController,
    pub _cutoutCurve: *mut crate::UnityEngine::AnimationCurve,
    pub _maxCutPointCenterDistance: f32,
    pub _centroidComputationMesh: *mut crate::UnityEngine::Mesh,
    pub _colorManager: *mut crate::GlobalNamespace::ColorManager,
    pub _elapsedTime: f32,
    pub _lifeTime: f32,
    pub _didFinishEvent: *mut crate::GlobalNamespace::LazyCopyHashSet_1<
        *mut crate::GlobalNamespace::INoteDebrisDidFinishEvent,
    >,
}
#[cfg(feature = "NoteDebris")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::NoteDebris => ""."NoteDebris"
);
#[cfg(feature = "NoteDebris")]
impl std::ops::Deref for crate::GlobalNamespace::NoteDebris {
    type Target = crate::UnityEngine::MonoBehaviour;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "NoteDebris")]
impl std::ops::DerefMut for crate::GlobalNamespace::NoteDebris {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "NoteDebris")]
impl crate::GlobalNamespace::NoteDebris {
    #[cfg(feature = "NoteDebris+Pool")]
    pub type Pool = crate::GlobalNamespace::NoteDebris_Pool;
    pub fn Awake(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Awake", ())?;
        Ok(__cordl_ret)
    }
    pub fn Init(
        &mut self,
        colorType: crate::GlobalNamespace::ColorType,
        notePos: crate::UnityEngine::Vector3,
        noteRot: crate::UnityEngine::Quaternion,
        noteMoveVec: crate::UnityEngine::Vector3,
        noteScale: crate::UnityEngine::Vector3,
        positionOffset: crate::UnityEngine::Vector3,
        rotationOffset: crate::UnityEngine::Quaternion,
        cutPoint: crate::UnityEngine::Vector3,
        cutNormal: crate::UnityEngine::Vector3,
        force: crate::UnityEngine::Vector3,
        torque: crate::UnityEngine::Vector3,
        lifeTime: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "Init",
                (
                    colorType,
                    notePos,
                    noteRot,
                    noteMoveVec,
                    noteScale,
                    positionOffset,
                    rotationOffset,
                    cutPoint,
                    cutNormal,
                    force,
                    torque,
                    lifeTime,
                ),
            )?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn Update(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Update", ())?;
        Ok(__cordl_ret)
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
    pub fn get_didFinishEvent(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::GlobalNamespace::ILazyCopyHashSet_1<
            *mut crate::GlobalNamespace::INoteDebrisDidFinishEvent,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::GlobalNamespace::ILazyCopyHashSet_1<
            *mut crate::GlobalNamespace::INoteDebrisDidFinishEvent,
        > = __cordl_object.invoke("get_didFinishEvent", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "NoteDebris")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::NoteDebris {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "NoteDebris+Pool")]
#[repr(C)]
#[derive(Debug)]
pub struct NoteDebris_Pool {
    __cordl_parent: crate::Zenject::MonoMemoryPool_1<
        *mut crate::GlobalNamespace::NoteDebris,
    >,
}
#[cfg(feature = "NoteDebris+Pool")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::NoteDebris_Pool => ""
    ."NoteDebris/Pool"
);
#[cfg(feature = "NoteDebris+Pool")]
impl std::ops::Deref for crate::GlobalNamespace::NoteDebris_Pool {
    type Target = crate::Zenject::MonoMemoryPool_1<
        *mut crate::GlobalNamespace::NoteDebris,
    >;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "NoteDebris+Pool")]
impl std::ops::DerefMut for crate::GlobalNamespace::NoteDebris_Pool {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "NoteDebris+Pool")]
impl crate::GlobalNamespace::NoteDebris_Pool {
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
#[cfg(feature = "NoteDebris+Pool")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::NoteDebris_Pool {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
