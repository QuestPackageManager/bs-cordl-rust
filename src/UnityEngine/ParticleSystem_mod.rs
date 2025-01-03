#[cfg(feature = "UnityEngine+ParticleSystem")]
#[repr(C)]
#[derive(Debug)]
pub struct ParticleSystem {
    __cordl_parent: crate::UnityEngine::Component,
}
#[cfg(feature = "UnityEngine+ParticleSystem")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::ParticleSystem => "UnityEngine"
    ."ParticleSystem"
);
#[cfg(feature = "UnityEngine+ParticleSystem")]
impl std::ops::Deref for crate::UnityEngine::ParticleSystem {
    type Target = crate::UnityEngine::Component;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+ParticleSystem")]
impl std::ops::DerefMut for crate::UnityEngine::ParticleSystem {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+ParticleSystem")]
impl crate::UnityEngine::ParticleSystem {
    #[cfg(feature = "UnityEngine+ParticleSystem+CollisionModule")]
    pub type CollisionModule = crate::UnityEngine::ParticleSystem_CollisionModule;
    #[cfg(feature = "UnityEngine+ParticleSystem+ColorBySpeedModule")]
    pub type ColorBySpeedModule = crate::UnityEngine::ParticleSystem_ColorBySpeedModule;
    #[cfg(feature = "UnityEngine+ParticleSystem+ColorOverLifetimeModule")]
    pub type ColorOverLifetimeModule = crate::UnityEngine::ParticleSystem_ColorOverLifetimeModule;
    #[cfg(feature = "UnityEngine+ParticleSystem+CustomDataModule")]
    pub type CustomDataModule = crate::UnityEngine::ParticleSystem_CustomDataModule;
    #[cfg(feature = "UnityEngine+ParticleSystem+EmissionModule")]
    pub type EmissionModule = crate::UnityEngine::ParticleSystem_EmissionModule;
    #[cfg(feature = "UnityEngine+ParticleSystem+EmitParams")]
    pub type EmitParams = crate::UnityEngine::ParticleSystem_EmitParams;
    #[cfg(feature = "UnityEngine+ParticleSystem+ExternalForcesModule")]
    pub type ExternalForcesModule = crate::UnityEngine::ParticleSystem_ExternalForcesModule;
    #[cfg(feature = "UnityEngine+ParticleSystem+ForceOverLifetimeModule")]
    pub type ForceOverLifetimeModule = crate::UnityEngine::ParticleSystem_ForceOverLifetimeModule;
    #[cfg(feature = "UnityEngine+ParticleSystem+InheritVelocityModule")]
    pub type InheritVelocityModule = crate::UnityEngine::ParticleSystem_InheritVelocityModule;
    #[cfg(feature = "UnityEngine+ParticleSystem+LifetimeByEmitterSpeedModule")]
    pub type LifetimeByEmitterSpeedModule = crate::UnityEngine::ParticleSystem_LifetimeByEmitterSpeedModule;
    #[cfg(feature = "UnityEngine+ParticleSystem+LightsModule")]
    pub type LightsModule = crate::UnityEngine::ParticleSystem_LightsModule;
    #[cfg(feature = "UnityEngine+ParticleSystem+LimitVelocityOverLifetimeModule")]
    pub type LimitVelocityOverLifetimeModule = crate::UnityEngine::ParticleSystem_LimitVelocityOverLifetimeModule;
    #[cfg(feature = "UnityEngine+ParticleSystem+MainModule")]
    pub type MainModule = crate::UnityEngine::ParticleSystem_MainModule;
    #[cfg(feature = "UnityEngine+ParticleSystem+MinMaxCurve")]
    pub type MinMaxCurve = crate::UnityEngine::ParticleSystem_MinMaxCurve;
    #[cfg(feature = "UnityEngine+ParticleSystem+MinMaxGradient")]
    pub type MinMaxGradient = crate::UnityEngine::ParticleSystem_MinMaxGradient;
    #[cfg(feature = "UnityEngine+ParticleSystem+NoiseModule")]
    pub type NoiseModule = crate::UnityEngine::ParticleSystem_NoiseModule;
    #[cfg(feature = "UnityEngine+ParticleSystem+Particle")]
    pub type Particle = crate::UnityEngine::ParticleSystem_Particle;
    #[cfg(feature = "UnityEngine+ParticleSystem+PlaybackState")]
    pub type PlaybackState = crate::UnityEngine::ParticleSystem_PlaybackState;
    #[cfg(feature = "UnityEngine+ParticleSystem+RotationBySpeedModule")]
    pub type RotationBySpeedModule = crate::UnityEngine::ParticleSystem_RotationBySpeedModule;
    #[cfg(feature = "UnityEngine+ParticleSystem+RotationOverLifetimeModule")]
    pub type RotationOverLifetimeModule = crate::UnityEngine::ParticleSystem_RotationOverLifetimeModule;
    #[cfg(feature = "UnityEngine+ParticleSystem+ShapeModule")]
    pub type ShapeModule = crate::UnityEngine::ParticleSystem_ShapeModule;
    #[cfg(feature = "UnityEngine+ParticleSystem+SizeBySpeedModule")]
    pub type SizeBySpeedModule = crate::UnityEngine::ParticleSystem_SizeBySpeedModule;
    #[cfg(feature = "UnityEngine+ParticleSystem+SizeOverLifetimeModule")]
    pub type SizeOverLifetimeModule = crate::UnityEngine::ParticleSystem_SizeOverLifetimeModule;
    #[cfg(feature = "UnityEngine+ParticleSystem+SubEmittersModule")]
    pub type SubEmittersModule = crate::UnityEngine::ParticleSystem_SubEmittersModule;
    #[cfg(feature = "UnityEngine+ParticleSystem+TextureSheetAnimationModule")]
    pub type TextureSheetAnimationModule = crate::UnityEngine::ParticleSystem_TextureSheetAnimationModule;
    #[cfg(feature = "UnityEngine+ParticleSystem+TrailModule")]
    pub type TrailModule = crate::UnityEngine::ParticleSystem_TrailModule;
    #[cfg(feature = "UnityEngine+ParticleSystem+Trails")]
    pub type Trails = crate::UnityEngine::ParticleSystem_Trails;
    #[cfg(feature = "UnityEngine+ParticleSystem+TriggerModule")]
    pub type TriggerModule = crate::UnityEngine::ParticleSystem_TriggerModule;
    #[cfg(feature = "UnityEngine+ParticleSystem+VelocityOverLifetimeModule")]
    pub type VelocityOverLifetimeModule = crate::UnityEngine::ParticleSystem_VelocityOverLifetimeModule;
    pub fn AllocateAxisOfRotationAttribute(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AllocateAxisOfRotationAttribute", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn AllocateCustomDataAttribute(
        &mut self,
        stream: crate::UnityEngine::ParticleSystemCustomData,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AllocateCustomDataAttribute", (stream))?;
        Ok(__cordl_ret.into())
    }
    pub fn AllocateMeshIndexAttribute(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AllocateMeshIndexAttribute", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn Clear_1(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Clear", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn Clear__cordl_bool0(
        &mut self,
        withChildren: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Clear", (withChildren))?;
        Ok(__cordl_ret.into())
    }
    pub fn CopyManagedJobData(
        systemPtr: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        particleData: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::ParticleSystemJobs::NativeParticleData,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CopyManagedJobData", (systemPtr, particleData))?;
        Ok(__cordl_ret.into())
    }
    pub fn EmitOld_Internal(
        &mut self,
        particle: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::ParticleSystem_Particle,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("EmitOld_Internal", (particle))?;
        Ok(__cordl_ret.into())
    }
    pub fn Emit_Injected(
        &mut self,
        emitParams: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::ParticleSystem_EmitParams,
        >,
        count: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Emit_Injected", (emitParams, count))?;
        Ok(__cordl_ret.into())
    }
    pub fn Emit_Internal(
        &mut self,
        count: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Emit_Internal", (count))?;
        Ok(__cordl_ret.into())
    }
    pub fn Emit_ParticleSystem_EmitParams_i32_3(
        &mut self,
        emitParams: crate::UnityEngine::ParticleSystem_EmitParams,
        count: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Emit", (emitParams, count))?;
        Ok(__cordl_ret.into())
    }
    pub fn Emit_ParticleSystem_Particle1(
        &mut self,
        particle: crate::UnityEngine::ParticleSystem_Particle,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Emit", (particle))?;
        Ok(__cordl_ret.into())
    }
    pub fn Emit_Vector3_Vector3_f32_f32_Color32_0(
        &mut self,
        position: crate::UnityEngine::Vector3,
        velocity: crate::UnityEngine::Vector3,
        _cordl_size: f32,
        lifetime: f32,
        color: crate::UnityEngine::Color32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Emit", (position, velocity, _cordl_size, lifetime, color))?;
        Ok(__cordl_ret.into())
    }
    pub fn Emit_i32_2(
        &mut self,
        count: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Emit", (count))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetCustomParticleData(
        &mut self,
        customData: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<crate::UnityEngine::Vector4>,
        >,
        streamIndex: crate::UnityEngine::ParticleSystemCustomData,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke("GetCustomParticleData", (customData, streamIndex))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetManagedJobData(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = __cordl_object.invoke("GetManagedJobData", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetManagedJobHandle(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Jobs::JobHandle> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::Unity::Jobs::JobHandle = __cordl_object
            .invoke("GetManagedJobHandle", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetManagedJobHandle_Injected(
        &mut self,
        ret: quest_hook::libil2cpp::ByRefMut<crate::Unity::Jobs::JobHandle>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("GetManagedJobHandle_Injected", (ret))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetParticleCurrentColor(
        &mut self,
        particle: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::ParticleSystem_Particle,
        >,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Color32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Color32 = __cordl_object
            .invoke("GetParticleCurrentColor", (particle))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetParticleCurrentColor_Injected(
        &mut self,
        particle: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::ParticleSystem_Particle,
        >,
        ret: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Color32>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("GetParticleCurrentColor_Injected", (particle, ret))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetParticleCurrentSize(
        &mut self,
        particle: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::ParticleSystem_Particle,
        >,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object
            .invoke("GetParticleCurrentSize", (particle))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetParticleCurrentSize3D(
        &mut self,
        particle: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::ParticleSystem_Particle,
        >,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector3> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Vector3 = __cordl_object
            .invoke("GetParticleCurrentSize3D", (particle))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetParticleCurrentSize3D_Injected(
        &mut self,
        particle: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::ParticleSystem_Particle,
        >,
        ret: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector3>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("GetParticleCurrentSize3D_Injected", (particle, ret))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetParticleMeshIndex(
        &mut self,
        particle: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::ParticleSystem_Particle,
        >,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke("GetParticleMeshIndex", (particle))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetParticlesWithNativeArray(
        &mut self,
        particles: crate::System::IntPtr,
        particlesLength: i32,
        _cordl_size: i32,
        offset: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke(
                "GetParticlesWithNativeArray",
                (particles, particlesLength, _cordl_size, offset),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn GetParticles_ByRefMut2(
        &mut self,
        particles: quest_hook::libil2cpp::ByRefMut<
            *mut quest_hook::libil2cpp::Il2CppArray<
                crate::UnityEngine::ParticleSystem_Particle,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("GetParticles", (particles))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetParticles_ByRefMut5(
        &mut self,
        particles: quest_hook::libil2cpp::ByRefMut<
            crate::Unity::Collections::NativeArray_1<
                crate::UnityEngine::ParticleSystem_Particle,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("GetParticles", (particles))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetParticles_i32_1(
        &mut self,
        particles: quest_hook::libil2cpp::ByRefMut<
            *mut quest_hook::libil2cpp::Il2CppArray<
                crate::UnityEngine::ParticleSystem_Particle,
            >,
        >,
        _cordl_size: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke("GetParticles", (particles, _cordl_size))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetParticles_i32_4(
        &mut self,
        particles: quest_hook::libil2cpp::ByRefMut<
            crate::Unity::Collections::NativeArray_1<
                crate::UnityEngine::ParticleSystem_Particle,
            >,
        >,
        _cordl_size: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke("GetParticles", (particles, _cordl_size))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetParticles_i32_i32_0(
        &mut self,
        particles: quest_hook::libil2cpp::ByRefMut<
            *mut quest_hook::libil2cpp::Il2CppArray<
                crate::UnityEngine::ParticleSystem_Particle,
            >,
        >,
        _cordl_size: i32,
        offset: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke("GetParticles", (particles, _cordl_size, offset))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetParticles_i32_i32_3(
        &mut self,
        particles: quest_hook::libil2cpp::ByRefMut<
            crate::Unity::Collections::NativeArray_1<
                crate::UnityEngine::ParticleSystem_Particle,
            >,
        >,
        _cordl_size: i32,
        offset: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke("GetParticles", (particles, _cordl_size, offset))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetPlaybackState(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::ParticleSystem_PlaybackState,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::ParticleSystem_PlaybackState = __cordl_object
            .invoke("GetPlaybackState", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetPlaybackState_Injected(
        &mut self,
        ret: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::ParticleSystem_PlaybackState,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("GetPlaybackState_Injected", (ret))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetTrailDataInternal(
        &mut self,
        trailData: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::ParticleSystem_Trails,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("GetTrailDataInternal", (trailData))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetTrails_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::ParticleSystem_Trails> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::ParticleSystem_Trails = __cordl_object
            .invoke("GetTrails", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetTrails_ByRefMut1(
        &mut self,
        trailData: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::ParticleSystem_Trails,
        >,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("GetTrails", (trailData))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsAlive_1(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("IsAlive", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn IsAlive__cordl_bool0(
        &mut self,
        withChildren: bool,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("IsAlive", (withChildren))?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn Pause_1(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Pause", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn Pause__cordl_bool0(
        &mut self,
        withChildren: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Pause", (withChildren))?;
        Ok(__cordl_ret.into())
    }
    pub fn Play_1(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Play", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn Play__cordl_bool0(
        &mut self,
        withChildren: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Play", (withChildren))?;
        Ok(__cordl_ret.into())
    }
    pub fn ResetPreMappedBufferMemory() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Void,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ResetPreMappedBufferMemory", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ScheduleManagedJob(
        parameters: quest_hook::libil2cpp::ByRefMut<
            crate::Unity::Jobs::LowLevel::Unsafe::JobsUtility_JobScheduleParameters,
        >,
        additionalData: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Jobs::JobHandle> {
        let __cordl_ret: crate::Unity::Jobs::JobHandle = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ScheduleManagedJob", (parameters, additionalData))?;
        Ok(__cordl_ret.into())
    }
    pub fn ScheduleManagedJob_Injected(
        parameters: quest_hook::libil2cpp::ByRefMut<
            crate::Unity::Jobs::LowLevel::Unsafe::JobsUtility_JobScheduleParameters,
        >,
        additionalData: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        ret: quest_hook::libil2cpp::ByRefMut<crate::Unity::Jobs::JobHandle>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ScheduleManagedJob_Injected", (parameters, additionalData, ret))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetCustomParticleData(
        &mut self,
        customData: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<crate::UnityEngine::Vector4>,
        >,
        streamIndex: crate::UnityEngine::ParticleSystemCustomData,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetCustomParticleData", (customData, streamIndex))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetManagedJobHandle(
        &mut self,
        handle: crate::Unity::Jobs::JobHandle,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetManagedJobHandle", (handle))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetManagedJobHandle_Injected(
        &mut self,
        handle: quest_hook::libil2cpp::ByRefMut<crate::Unity::Jobs::JobHandle>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetManagedJobHandle_Injected", (handle))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetMaximumPreMappedBufferCounts(
        vertexBuffersCount: i32,
        indexBuffersCount: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "SetMaximumPreMappedBufferCounts",
                (vertexBuffersCount, indexBuffersCount),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn SetParticlesWithNativeArray(
        &mut self,
        particles: crate::System::IntPtr,
        particlesLength: i32,
        _cordl_size: i32,
        offset: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "SetParticlesWithNativeArray",
                (particles, particlesLength, _cordl_size, offset),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn SetParticles_ByRefMut2(
        &mut self,
        particles: quest_hook::libil2cpp::ByRefMut<
            *mut quest_hook::libil2cpp::Il2CppArray<
                crate::UnityEngine::ParticleSystem_Particle,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetParticles", (particles))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetParticles_ByRefMut5(
        &mut self,
        particles: quest_hook::libil2cpp::ByRefMut<
            crate::Unity::Collections::NativeArray_1<
                crate::UnityEngine::ParticleSystem_Particle,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetParticles", (particles))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetParticles_i32_1(
        &mut self,
        particles: quest_hook::libil2cpp::ByRefMut<
            *mut quest_hook::libil2cpp::Il2CppArray<
                crate::UnityEngine::ParticleSystem_Particle,
            >,
        >,
        _cordl_size: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetParticles", (particles, _cordl_size))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetParticles_i32_4(
        &mut self,
        particles: quest_hook::libil2cpp::ByRefMut<
            crate::Unity::Collections::NativeArray_1<
                crate::UnityEngine::ParticleSystem_Particle,
            >,
        >,
        _cordl_size: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetParticles", (particles, _cordl_size))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetParticles_i32_i32_0(
        &mut self,
        particles: quest_hook::libil2cpp::ByRefMut<
            *mut quest_hook::libil2cpp::Il2CppArray<
                crate::UnityEngine::ParticleSystem_Particle,
            >,
        >,
        _cordl_size: i32,
        offset: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetParticles", (particles, _cordl_size, offset))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetParticles_i32_i32_3(
        &mut self,
        particles: quest_hook::libil2cpp::ByRefMut<
            crate::Unity::Collections::NativeArray_1<
                crate::UnityEngine::ParticleSystem_Particle,
            >,
        >,
        _cordl_size: i32,
        offset: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetParticles", (particles, _cordl_size, offset))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetPlaybackState(
        &mut self,
        playbackState: crate::UnityEngine::ParticleSystem_PlaybackState,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetPlaybackState", (playbackState))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetPlaybackState_Injected(
        &mut self,
        playbackState: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::ParticleSystem_PlaybackState,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetPlaybackState_Injected", (playbackState))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetTrails(
        &mut self,
        trailData: crate::UnityEngine::ParticleSystem_Trails,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetTrails", (trailData))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetTrails_Injected(
        &mut self,
        trailData: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::ParticleSystem_Trails,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetTrails_Injected", (trailData))?;
        Ok(__cordl_ret.into())
    }
    pub fn Simulate__cordl_bool2(
        &mut self,
        t: f32,
        withChildren: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Simulate", (t, withChildren))?;
        Ok(__cordl_ret.into())
    }
    pub fn Simulate__cordl_bool__cordl_bool1(
        &mut self,
        t: f32,
        withChildren: bool,
        restart: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Simulate", (t, withChildren, restart))?;
        Ok(__cordl_ret.into())
    }
    pub fn Simulate__cordl_bool__cordl_bool__cordl_bool0(
        &mut self,
        t: f32,
        withChildren: bool,
        restart: bool,
        fixedTimeStep: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Simulate", (t, withChildren, restart, fixedTimeStep))?;
        Ok(__cordl_ret.into())
    }
    pub fn Simulate_f32_3(
        &mut self,
        t: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Simulate", (t))?;
        Ok(__cordl_ret.into())
    }
    pub fn Stop_2(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Stop", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn Stop__cordl_bool1(
        &mut self,
        withChildren: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Stop", (withChildren))?;
        Ok(__cordl_ret.into())
    }
    pub fn Stop__cordl_bool_ParticleSystemStopBehavior0(
        &mut self,
        withChildren: bool,
        stopBehavior: crate::UnityEngine::ParticleSystemStopBehavior,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Stop", (withChildren, stopBehavior))?;
        Ok(__cordl_ret.into())
    }
    pub fn TriggerSubEmitterForParticle(
        &mut self,
        subEmitterIndex: i32,
        particle: crate::UnityEngine::ParticleSystem_Particle,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("TriggerSubEmitterForParticle", (subEmitterIndex, particle))?;
        Ok(__cordl_ret.into())
    }
    pub fn TriggerSubEmitterForParticle_Injected(
        &mut self,
        subEmitterIndex: i32,
        particle: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::ParticleSystem_Particle,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "TriggerSubEmitterForParticle_Injected",
                (subEmitterIndex, particle),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn TriggerSubEmitter_ByRefMut1(
        &mut self,
        subEmitterIndex: i32,
        particle: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::ParticleSystem_Particle,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("TriggerSubEmitter", (subEmitterIndex, particle))?;
        Ok(__cordl_ret.into())
    }
    pub fn TriggerSubEmitter_List_1_2(
        &mut self,
        subEmitterIndex: i32,
        particles: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                crate::UnityEngine::ParticleSystem_Particle,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("TriggerSubEmitter", (subEmitterIndex, particles))?;
        Ok(__cordl_ret.into())
    }
    pub fn TriggerSubEmitter_i32_0(
        &mut self,
        subEmitterIndex: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("TriggerSubEmitter", (subEmitterIndex))?;
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
    pub fn get_automaticCullingEnabled(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("get_automaticCullingEnabled", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_collision(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::ParticleSystem_CollisionModule,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::ParticleSystem_CollisionModule = __cordl_object
            .invoke("get_collision", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_colorBySpeed(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::ParticleSystem_ColorBySpeedModule,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::ParticleSystem_ColorBySpeedModule = __cordl_object
            .invoke("get_colorBySpeed", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_colorOverLifetime(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::ParticleSystem_ColorOverLifetimeModule,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::ParticleSystem_ColorOverLifetimeModule = __cordl_object
            .invoke("get_colorOverLifetime", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_customData(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::ParticleSystem_CustomDataModule,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::ParticleSystem_CustomDataModule = __cordl_object
            .invoke("get_customData", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_duration(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_duration", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_emission(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::ParticleSystem_EmissionModule,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::ParticleSystem_EmissionModule = __cordl_object
            .invoke("get_emission", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_emissionRate(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_emissionRate", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_enableEmission(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_enableEmission", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_externalForces(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::ParticleSystem_ExternalForcesModule,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::ParticleSystem_ExternalForcesModule = __cordl_object
            .invoke("get_externalForces", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_forceOverLifetime(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::ParticleSystem_ForceOverLifetimeModule,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::ParticleSystem_ForceOverLifetimeModule = __cordl_object
            .invoke("get_forceOverLifetime", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_gravityModifier(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_gravityModifier", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_has3DParticleRotations(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_has3DParticleRotations", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_hasNonUniformParticleSizes(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("get_hasNonUniformParticleSizes", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_inheritVelocity(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::ParticleSystem_InheritVelocityModule,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::ParticleSystem_InheritVelocityModule = __cordl_object
            .invoke("get_inheritVelocity", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_isEmitting(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_isEmitting", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_isPaused(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_isPaused", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_isPlaying(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_isPlaying", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_isStopped(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_isStopped", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_lifetimeByEmitterSpeed(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::ParticleSystem_LifetimeByEmitterSpeedModule,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::ParticleSystem_LifetimeByEmitterSpeedModule = __cordl_object
            .invoke("get_lifetimeByEmitterSpeed", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_lights(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::ParticleSystem_LightsModule> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::ParticleSystem_LightsModule = __cordl_object
            .invoke("get_lights", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_limitVelocityOverLifetime(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::ParticleSystem_LimitVelocityOverLifetimeModule,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::ParticleSystem_LimitVelocityOverLifetimeModule = __cordl_object
            .invoke("get_limitVelocityOverLifetime", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_loop(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_loop", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_main(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::ParticleSystem_MainModule> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::ParticleSystem_MainModule = __cordl_object
            .invoke("get_main", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_maxParticles(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_maxParticles", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_noise(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::ParticleSystem_NoiseModule> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::ParticleSystem_NoiseModule = __cordl_object
            .invoke("get_noise", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_particleCount(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_particleCount", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_playOnAwake(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_playOnAwake", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_playbackSpeed(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_playbackSpeed", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_proceduralSimulationSupported(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("get_proceduralSimulationSupported", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_randomSeed(&mut self) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: u32 = __cordl_object.invoke("get_randomSeed", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_rotationBySpeed(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::ParticleSystem_RotationBySpeedModule,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::ParticleSystem_RotationBySpeedModule = __cordl_object
            .invoke("get_rotationBySpeed", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_rotationOverLifetime(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::ParticleSystem_RotationOverLifetimeModule,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::ParticleSystem_RotationOverLifetimeModule = __cordl_object
            .invoke("get_rotationOverLifetime", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_scalingMode(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::ParticleSystemScalingMode> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::ParticleSystemScalingMode = __cordl_object
            .invoke("get_scalingMode", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_shape(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::ParticleSystem_ShapeModule> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::ParticleSystem_ShapeModule = __cordl_object
            .invoke("get_shape", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_simulationSpace(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::ParticleSystemSimulationSpace,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::ParticleSystemSimulationSpace = __cordl_object
            .invoke("get_simulationSpace", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_sizeBySpeed(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::ParticleSystem_SizeBySpeedModule,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::ParticleSystem_SizeBySpeedModule = __cordl_object
            .invoke("get_sizeBySpeed", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_sizeOverLifetime(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::ParticleSystem_SizeOverLifetimeModule,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::ParticleSystem_SizeOverLifetimeModule = __cordl_object
            .invoke("get_sizeOverLifetime", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_startColor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Color> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Color = __cordl_object
            .invoke("get_startColor", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_startDelay(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_startDelay", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_startLifetime(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_startLifetime", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_startRotation(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_startRotation", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_startRotation3D(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector3> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Vector3 = __cordl_object
            .invoke("get_startRotation3D", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_startSize(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_startSize", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_startSpeed(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_startSpeed", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_subEmitters(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::ParticleSystem_SubEmittersModule,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::ParticleSystem_SubEmittersModule = __cordl_object
            .invoke("get_subEmitters", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_textureSheetAnimation(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::ParticleSystem_TextureSheetAnimationModule,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::ParticleSystem_TextureSheetAnimationModule = __cordl_object
            .invoke("get_textureSheetAnimation", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_time(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_time", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_totalTime(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_totalTime", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_trails(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::ParticleSystem_TrailModule> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::ParticleSystem_TrailModule = __cordl_object
            .invoke("get_trails", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_trigger(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::ParticleSystem_TriggerModule,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::ParticleSystem_TriggerModule = __cordl_object
            .invoke("get_trigger", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_useAutoRandomSeed(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_useAutoRandomSeed", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_velocityOverLifetime(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::ParticleSystem_VelocityOverLifetimeModule,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::ParticleSystem_VelocityOverLifetimeModule = __cordl_object
            .invoke("get_velocityOverLifetime", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn set_emissionRate(
        &mut self,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_emissionRate", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_enableEmission(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_enableEmission", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_gravityModifier(
        &mut self,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_gravityModifier", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_loop(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_loop", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_maxParticles(
        &mut self,
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_maxParticles", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_playOnAwake(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_playOnAwake", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_playbackSpeed(
        &mut self,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_playbackSpeed", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_randomSeed(
        &mut self,
        value: u32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_randomSeed", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_scalingMode(
        &mut self,
        value: crate::UnityEngine::ParticleSystemScalingMode,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_scalingMode", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_simulationSpace(
        &mut self,
        value: crate::UnityEngine::ParticleSystemSimulationSpace,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_simulationSpace", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_startColor(
        &mut self,
        value: crate::UnityEngine::Color,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_startColor", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_startDelay(
        &mut self,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_startDelay", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_startLifetime(
        &mut self,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_startLifetime", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_startRotation(
        &mut self,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_startRotation", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_startRotation3D(
        &mut self,
        value: crate::UnityEngine::Vector3,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_startRotation3D", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_startSize(
        &mut self,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_startSize", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_startSpeed(
        &mut self,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_startSpeed", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_time(
        &mut self,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_time", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_useAutoRandomSeed(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_useAutoRandomSeed", (value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+ParticleSystem")]
impl quest_hook::libil2cpp::ObjectType for crate::UnityEngine::ParticleSystem {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "UnityEngine+ParticleSystem+CollisionModule")]
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct ParticleSystem_CollisionModule {
    pub m_ParticleSystem: quest_hook::libil2cpp::Gc<crate::UnityEngine::ParticleSystem>,
}
#[cfg(feature = "UnityEngine+ParticleSystem+CollisionModule")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::ParticleSystem_CollisionModule =>
    "UnityEngine"."ParticleSystem/CollisionModule"
);
#[cfg(feature = "UnityEngine+ParticleSystem+CollisionModule")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::ParticleSystem_CollisionModule {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+ParticleSystem+CollisionModule")]
impl crate::UnityEngine::ParticleSystem_CollisionModule {
    pub fn _ctor(
        &mut self,
        particleSystem: quest_hook::libil2cpp::Gc<crate::UnityEngine::ParticleSystem>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (particleSystem),
        )?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+ParticleSystem+ColorBySpeedModule")]
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct ParticleSystem_ColorBySpeedModule {
    pub m_ParticleSystem: quest_hook::libil2cpp::Gc<crate::UnityEngine::ParticleSystem>,
}
#[cfg(feature = "UnityEngine+ParticleSystem+ColorBySpeedModule")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::ParticleSystem_ColorBySpeedModule
    => "UnityEngine"."ParticleSystem/ColorBySpeedModule"
);
#[cfg(feature = "UnityEngine+ParticleSystem+ColorBySpeedModule")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::ParticleSystem_ColorBySpeedModule {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+ParticleSystem+ColorBySpeedModule")]
impl crate::UnityEngine::ParticleSystem_ColorBySpeedModule {
    pub fn _ctor(
        &mut self,
        particleSystem: quest_hook::libil2cpp::Gc<crate::UnityEngine::ParticleSystem>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (particleSystem),
        )?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+ParticleSystem+ColorOverLifetimeModule")]
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct ParticleSystem_ColorOverLifetimeModule {
    pub m_ParticleSystem: quest_hook::libil2cpp::Gc<crate::UnityEngine::ParticleSystem>,
}
#[cfg(feature = "UnityEngine+ParticleSystem+ColorOverLifetimeModule")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::ParticleSystem_ColorOverLifetimeModule => "UnityEngine"
    ."ParticleSystem/ColorOverLifetimeModule"
);
#[cfg(feature = "UnityEngine+ParticleSystem+ColorOverLifetimeModule")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::ParticleSystem_ColorOverLifetimeModule {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+ParticleSystem+ColorOverLifetimeModule")]
impl crate::UnityEngine::ParticleSystem_ColorOverLifetimeModule {
    pub fn _ctor(
        &mut self,
        particleSystem: quest_hook::libil2cpp::Gc<crate::UnityEngine::ParticleSystem>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (particleSystem),
        )?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+ParticleSystem+CustomDataModule")]
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct ParticleSystem_CustomDataModule {
    pub m_ParticleSystem: quest_hook::libil2cpp::Gc<crate::UnityEngine::ParticleSystem>,
}
#[cfg(feature = "UnityEngine+ParticleSystem+CustomDataModule")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::ParticleSystem_CustomDataModule =>
    "UnityEngine"."ParticleSystem/CustomDataModule"
);
#[cfg(feature = "UnityEngine+ParticleSystem+CustomDataModule")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::ParticleSystem_CustomDataModule {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+ParticleSystem+CustomDataModule")]
impl crate::UnityEngine::ParticleSystem_CustomDataModule {
    pub fn _ctor(
        &mut self,
        particleSystem: quest_hook::libil2cpp::Gc<crate::UnityEngine::ParticleSystem>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (particleSystem),
        )?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+ParticleSystem+EmissionModule")]
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct ParticleSystem_EmissionModule {
    pub m_ParticleSystem: quest_hook::libil2cpp::Gc<crate::UnityEngine::ParticleSystem>,
}
#[cfg(feature = "UnityEngine+ParticleSystem+EmissionModule")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::ParticleSystem_EmissionModule =>
    "UnityEngine"."ParticleSystem/EmissionModule"
);
#[cfg(feature = "UnityEngine+ParticleSystem+EmissionModule")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::ParticleSystem_EmissionModule {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+ParticleSystem+EmissionModule")]
impl crate::UnityEngine::ParticleSystem_EmissionModule {
    pub fn _ctor(
        &mut self,
        particleSystem: quest_hook::libil2cpp::Gc<crate::UnityEngine::ParticleSystem>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (particleSystem),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_enabled(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_enabled",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_enabled_Injected(
        _unity_self: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::ParticleSystem_EmissionModule,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_enabled_Injected", (_unity_self))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_rateOverTimeMultiplier(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_rateOverTimeMultiplier",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_rateOverTimeMultiplier_Injected(
        _unity_self: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::ParticleSystem_EmissionModule,
        >,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_rateOverTimeMultiplier_Injected", (_unity_self))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_enabled(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_enabled",
            (value),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn set_enabled_Injected(
        _unity_self: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::ParticleSystem_EmissionModule,
        >,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("set_enabled_Injected", (_unity_self, value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_rateOverTime(
        &mut self,
        value: crate::UnityEngine::ParticleSystem_MinMaxCurve,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_rateOverTime",
            (value),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn set_rateOverTime_Injected(
        _unity_self: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::ParticleSystem_EmissionModule,
        >,
        value: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::ParticleSystem_MinMaxCurve,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("set_rateOverTime_Injected", (_unity_self, value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+ParticleSystem+EmitParams")]
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct ParticleSystem_EmitParams {
    pub m_Particle: crate::UnityEngine::ParticleSystem_Particle,
    pub m_PositionSet: bool,
    pub m_VelocitySet: bool,
    pub m_AxisOfRotationSet: bool,
    pub m_RotationSet: bool,
    pub m_AngularVelocitySet: bool,
    pub m_StartSizeSet: bool,
    pub m_StartColorSet: bool,
    pub m_RandomSeedSet: bool,
    pub m_StartLifetimeSet: bool,
    pub m_MeshIndexSet: bool,
    pub m_ApplyShapeToPosition: bool,
}
#[cfg(feature = "UnityEngine+ParticleSystem+EmitParams")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::ParticleSystem_EmitParams =>
    "UnityEngine"."ParticleSystem/EmitParams"
);
#[cfg(feature = "UnityEngine+ParticleSystem+EmitParams")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::ParticleSystem_EmitParams {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+ParticleSystem+EmitParams")]
impl crate::UnityEngine::ParticleSystem_EmitParams {
    pub fn set_applyShapeToPosition(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_applyShapeToPosition",
            (value),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn set_position(
        &mut self,
        value: crate::UnityEngine::Vector3,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_position",
            (value),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn set_rotation3D(
        &mut self,
        value: crate::UnityEngine::Vector3,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_rotation3D",
            (value),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn set_startColor(
        &mut self,
        value: crate::UnityEngine::Color32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_startColor",
            (value),
        )?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+ParticleSystem+ExternalForcesModule")]
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct ParticleSystem_ExternalForcesModule {
    pub m_ParticleSystem: quest_hook::libil2cpp::Gc<crate::UnityEngine::ParticleSystem>,
}
#[cfg(feature = "UnityEngine+ParticleSystem+ExternalForcesModule")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::ParticleSystem_ExternalForcesModule
    => "UnityEngine"."ParticleSystem/ExternalForcesModule"
);
#[cfg(feature = "UnityEngine+ParticleSystem+ExternalForcesModule")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::ParticleSystem_ExternalForcesModule {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+ParticleSystem+ExternalForcesModule")]
impl crate::UnityEngine::ParticleSystem_ExternalForcesModule {
    pub fn _ctor(
        &mut self,
        particleSystem: quest_hook::libil2cpp::Gc<crate::UnityEngine::ParticleSystem>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (particleSystem),
        )?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+ParticleSystem+ForceOverLifetimeModule")]
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct ParticleSystem_ForceOverLifetimeModule {
    pub m_ParticleSystem: quest_hook::libil2cpp::Gc<crate::UnityEngine::ParticleSystem>,
}
#[cfg(feature = "UnityEngine+ParticleSystem+ForceOverLifetimeModule")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::ParticleSystem_ForceOverLifetimeModule => "UnityEngine"
    ."ParticleSystem/ForceOverLifetimeModule"
);
#[cfg(feature = "UnityEngine+ParticleSystem+ForceOverLifetimeModule")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::ParticleSystem_ForceOverLifetimeModule {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+ParticleSystem+ForceOverLifetimeModule")]
impl crate::UnityEngine::ParticleSystem_ForceOverLifetimeModule {
    pub fn _ctor(
        &mut self,
        particleSystem: quest_hook::libil2cpp::Gc<crate::UnityEngine::ParticleSystem>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (particleSystem),
        )?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+ParticleSystem+InheritVelocityModule")]
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct ParticleSystem_InheritVelocityModule {
    pub m_ParticleSystem: quest_hook::libil2cpp::Gc<crate::UnityEngine::ParticleSystem>,
}
#[cfg(feature = "UnityEngine+ParticleSystem+InheritVelocityModule")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::ParticleSystem_InheritVelocityModule => "UnityEngine"
    ."ParticleSystem/InheritVelocityModule"
);
#[cfg(feature = "UnityEngine+ParticleSystem+InheritVelocityModule")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::ParticleSystem_InheritVelocityModule {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+ParticleSystem+InheritVelocityModule")]
impl crate::UnityEngine::ParticleSystem_InheritVelocityModule {
    pub fn _ctor(
        &mut self,
        particleSystem: quest_hook::libil2cpp::Gc<crate::UnityEngine::ParticleSystem>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (particleSystem),
        )?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+ParticleSystem+LifetimeByEmitterSpeedModule")]
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct ParticleSystem_LifetimeByEmitterSpeedModule {
    pub m_ParticleSystem: quest_hook::libil2cpp::Gc<crate::UnityEngine::ParticleSystem>,
}
#[cfg(feature = "UnityEngine+ParticleSystem+LifetimeByEmitterSpeedModule")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::ParticleSystem_LifetimeByEmitterSpeedModule => "UnityEngine"
    ."ParticleSystem/LifetimeByEmitterSpeedModule"
);
#[cfg(feature = "UnityEngine+ParticleSystem+LifetimeByEmitterSpeedModule")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::ParticleSystem_LifetimeByEmitterSpeedModule {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+ParticleSystem+LifetimeByEmitterSpeedModule")]
impl crate::UnityEngine::ParticleSystem_LifetimeByEmitterSpeedModule {
    pub fn _ctor(
        &mut self,
        particleSystem: quest_hook::libil2cpp::Gc<crate::UnityEngine::ParticleSystem>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (particleSystem),
        )?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+ParticleSystem+LightsModule")]
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct ParticleSystem_LightsModule {
    pub m_ParticleSystem: quest_hook::libil2cpp::Gc<crate::UnityEngine::ParticleSystem>,
}
#[cfg(feature = "UnityEngine+ParticleSystem+LightsModule")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::ParticleSystem_LightsModule =>
    "UnityEngine"."ParticleSystem/LightsModule"
);
#[cfg(feature = "UnityEngine+ParticleSystem+LightsModule")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::ParticleSystem_LightsModule {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+ParticleSystem+LightsModule")]
impl crate::UnityEngine::ParticleSystem_LightsModule {
    pub fn _ctor(
        &mut self,
        particleSystem: quest_hook::libil2cpp::Gc<crate::UnityEngine::ParticleSystem>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (particleSystem),
        )?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+ParticleSystem+LimitVelocityOverLifetimeModule")]
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct ParticleSystem_LimitVelocityOverLifetimeModule {
    pub m_ParticleSystem: quest_hook::libil2cpp::Gc<crate::UnityEngine::ParticleSystem>,
}
#[cfg(feature = "UnityEngine+ParticleSystem+LimitVelocityOverLifetimeModule")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::ParticleSystem_LimitVelocityOverLifetimeModule => "UnityEngine"
    ."ParticleSystem/LimitVelocityOverLifetimeModule"
);
#[cfg(feature = "UnityEngine+ParticleSystem+LimitVelocityOverLifetimeModule")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::ParticleSystem_LimitVelocityOverLifetimeModule {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+ParticleSystem+LimitVelocityOverLifetimeModule")]
impl crate::UnityEngine::ParticleSystem_LimitVelocityOverLifetimeModule {
    pub fn _ctor(
        &mut self,
        particleSystem: quest_hook::libil2cpp::Gc<crate::UnityEngine::ParticleSystem>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (particleSystem),
        )?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+ParticleSystem+MainModule")]
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct ParticleSystem_MainModule {
    pub m_ParticleSystem: quest_hook::libil2cpp::Gc<crate::UnityEngine::ParticleSystem>,
}
#[cfg(feature = "UnityEngine+ParticleSystem+MainModule")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::ParticleSystem_MainModule =>
    "UnityEngine"."ParticleSystem/MainModule"
);
#[cfg(feature = "UnityEngine+ParticleSystem+MainModule")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::ParticleSystem_MainModule {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+ParticleSystem+MainModule")]
impl crate::UnityEngine::ParticleSystem_MainModule {
    pub fn _ctor(
        &mut self,
        particleSystem: quest_hook::libil2cpp::Gc<crate::UnityEngine::ParticleSystem>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (particleSystem),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_duration(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_duration",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_duration_Injected(
        _unity_self: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::ParticleSystem_MainModule,
        >,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_duration_Injected", (_unity_self))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_gravityModifierMultiplier(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_gravityModifierMultiplier",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_gravityModifierMultiplier_Injected(
        _unity_self: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::ParticleSystem_MainModule,
        >,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_gravityModifierMultiplier_Injected", (_unity_self))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_loop(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_loop",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_loop_Injected(
        _unity_self: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::ParticleSystem_MainModule,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_loop_Injected", (_unity_self))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_maxParticles(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_maxParticles",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_maxParticles_Injected(
        _unity_self: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::ParticleSystem_MainModule,
        >,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_maxParticles_Injected", (_unity_self))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_playOnAwake(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_playOnAwake",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_playOnAwake_Injected(
        _unity_self: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::ParticleSystem_MainModule,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_playOnAwake_Injected", (_unity_self))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_scalingMode(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::ParticleSystemScalingMode> {
        let __cordl_ret: crate::UnityEngine::ParticleSystemScalingMode = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_scalingMode",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_scalingMode_Injected(
        _unity_self: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::ParticleSystem_MainModule,
        >,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::ParticleSystemScalingMode> {
        let __cordl_ret: crate::UnityEngine::ParticleSystemScalingMode = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_scalingMode_Injected", (_unity_self))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_simulationSpace(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::ParticleSystemSimulationSpace,
    > {
        let __cordl_ret: crate::UnityEngine::ParticleSystemSimulationSpace = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_simulationSpace",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_simulationSpace_Injected(
        _unity_self: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::ParticleSystem_MainModule,
        >,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::ParticleSystemSimulationSpace,
    > {
        let __cordl_ret: crate::UnityEngine::ParticleSystemSimulationSpace = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_simulationSpace_Injected", (_unity_self))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_simulationSpeed(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_simulationSpeed",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_simulationSpeed_Injected(
        _unity_self: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::ParticleSystem_MainModule,
        >,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_simulationSpeed_Injected", (_unity_self))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_startColor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::ParticleSystem_MinMaxGradient,
    > {
        let __cordl_ret: crate::UnityEngine::ParticleSystem_MinMaxGradient = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_startColor",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_startColor_Injected(
        _unity_self: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::ParticleSystem_MainModule,
        >,
        ret: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::ParticleSystem_MinMaxGradient,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_startColor_Injected", (_unity_self, ret))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_startDelayMultiplier(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_startDelayMultiplier",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_startDelayMultiplier_Injected(
        _unity_self: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::ParticleSystem_MainModule,
        >,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_startDelayMultiplier_Injected", (_unity_self))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_startLifetimeMultiplier(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_startLifetimeMultiplier",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_startLifetimeMultiplier_Injected(
        _unity_self: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::ParticleSystem_MainModule,
        >,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_startLifetimeMultiplier_Injected", (_unity_self))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_startRotationMultiplier(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_startRotationMultiplier",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_startRotationMultiplier_Injected(
        _unity_self: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::ParticleSystem_MainModule,
        >,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_startRotationMultiplier_Injected", (_unity_self))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_startRotationXMultiplier(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_startRotationXMultiplier",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_startRotationXMultiplier_Injected(
        _unity_self: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::ParticleSystem_MainModule,
        >,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_startRotationXMultiplier_Injected", (_unity_self))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_startRotationYMultiplier(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_startRotationYMultiplier",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_startRotationYMultiplier_Injected(
        _unity_self: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::ParticleSystem_MainModule,
        >,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_startRotationYMultiplier_Injected", (_unity_self))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_startRotationZMultiplier(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_startRotationZMultiplier",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_startRotationZMultiplier_Injected(
        _unity_self: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::ParticleSystem_MainModule,
        >,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_startRotationZMultiplier_Injected", (_unity_self))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_startSizeMultiplier(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_startSizeMultiplier",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_startSizeMultiplier_Injected(
        _unity_self: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::ParticleSystem_MainModule,
        >,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_startSizeMultiplier_Injected", (_unity_self))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_startSpeedMultiplier(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_startSpeedMultiplier",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_startSpeedMultiplier_Injected(
        _unity_self: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::ParticleSystem_MainModule,
        >,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_startSpeedMultiplier_Injected", (_unity_self))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_gravityModifierMultiplier(
        &mut self,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_gravityModifierMultiplier",
            (value),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn set_gravityModifierMultiplier_Injected(
        _unity_self: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::ParticleSystem_MainModule,
        >,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("set_gravityModifierMultiplier_Injected", (_unity_self, value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_loop(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_loop",
            (value),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn set_loop_Injected(
        _unity_self: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::ParticleSystem_MainModule,
        >,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("set_loop_Injected", (_unity_self, value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_maxParticles(
        &mut self,
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_maxParticles",
            (value),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn set_maxParticles_Injected(
        _unity_self: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::ParticleSystem_MainModule,
        >,
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("set_maxParticles_Injected", (_unity_self, value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_playOnAwake(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_playOnAwake",
            (value),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn set_playOnAwake_Injected(
        _unity_self: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::ParticleSystem_MainModule,
        >,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("set_playOnAwake_Injected", (_unity_self, value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_scalingMode(
        &mut self,
        value: crate::UnityEngine::ParticleSystemScalingMode,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_scalingMode",
            (value),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn set_scalingMode_Injected(
        _unity_self: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::ParticleSystem_MainModule,
        >,
        value: crate::UnityEngine::ParticleSystemScalingMode,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("set_scalingMode_Injected", (_unity_self, value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_simulationSpace(
        &mut self,
        value: crate::UnityEngine::ParticleSystemSimulationSpace,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_simulationSpace",
            (value),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn set_simulationSpace_Injected(
        _unity_self: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::ParticleSystem_MainModule,
        >,
        value: crate::UnityEngine::ParticleSystemSimulationSpace,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("set_simulationSpace_Injected", (_unity_self, value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_simulationSpeed(
        &mut self,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_simulationSpeed",
            (value),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn set_simulationSpeed_Injected(
        _unity_self: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::ParticleSystem_MainModule,
        >,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("set_simulationSpeed_Injected", (_unity_self, value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_startColor(
        &mut self,
        value: crate::UnityEngine::ParticleSystem_MinMaxGradient,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_startColor",
            (value),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn set_startColor_Injected(
        _unity_self: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::ParticleSystem_MainModule,
        >,
        value: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::ParticleSystem_MinMaxGradient,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("set_startColor_Injected", (_unity_self, value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_startDelayMultiplier(
        &mut self,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_startDelayMultiplier",
            (value),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn set_startDelayMultiplier_Injected(
        _unity_self: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::ParticleSystem_MainModule,
        >,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("set_startDelayMultiplier_Injected", (_unity_self, value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_startLifetime(
        &mut self,
        value: crate::UnityEngine::ParticleSystem_MinMaxCurve,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_startLifetime",
            (value),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn set_startLifetimeMultiplier(
        &mut self,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_startLifetimeMultiplier",
            (value),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn set_startLifetimeMultiplier_Injected(
        _unity_self: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::ParticleSystem_MainModule,
        >,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("set_startLifetimeMultiplier_Injected", (_unity_self, value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_startLifetime_Injected(
        _unity_self: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::ParticleSystem_MainModule,
        >,
        value: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::ParticleSystem_MinMaxCurve,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("set_startLifetime_Injected", (_unity_self, value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_startRotationMultiplier(
        &mut self,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_startRotationMultiplier",
            (value),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn set_startRotationMultiplier_Injected(
        _unity_self: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::ParticleSystem_MainModule,
        >,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("set_startRotationMultiplier_Injected", (_unity_self, value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_startRotationXMultiplier(
        &mut self,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_startRotationXMultiplier",
            (value),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn set_startRotationXMultiplier_Injected(
        _unity_self: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::ParticleSystem_MainModule,
        >,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("set_startRotationXMultiplier_Injected", (_unity_self, value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_startRotationYMultiplier(
        &mut self,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_startRotationYMultiplier",
            (value),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn set_startRotationYMultiplier_Injected(
        _unity_self: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::ParticleSystem_MainModule,
        >,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("set_startRotationYMultiplier_Injected", (_unity_self, value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_startRotationZMultiplier(
        &mut self,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_startRotationZMultiplier",
            (value),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn set_startRotationZMultiplier_Injected(
        _unity_self: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::ParticleSystem_MainModule,
        >,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("set_startRotationZMultiplier_Injected", (_unity_self, value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_startSizeMultiplier(
        &mut self,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_startSizeMultiplier",
            (value),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn set_startSizeMultiplier_Injected(
        _unity_self: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::ParticleSystem_MainModule,
        >,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("set_startSizeMultiplier_Injected", (_unity_self, value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_startSpeed(
        &mut self,
        value: crate::UnityEngine::ParticleSystem_MinMaxCurve,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_startSpeed",
            (value),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn set_startSpeedMultiplier(
        &mut self,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_startSpeedMultiplier",
            (value),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn set_startSpeedMultiplier_Injected(
        _unity_self: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::ParticleSystem_MainModule,
        >,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("set_startSpeedMultiplier_Injected", (_unity_self, value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_startSpeed_Injected(
        _unity_self: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::ParticleSystem_MainModule,
        >,
        value: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::ParticleSystem_MinMaxCurve,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("set_startSpeed_Injected", (_unity_self, value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+ParticleSystem+MinMaxCurve")]
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct ParticleSystem_MinMaxCurve {
    pub m_Mode: crate::UnityEngine::ParticleSystemCurveMode,
    pub m_CurveMultiplier: f32,
    pub m_CurveMin: quest_hook::libil2cpp::Gc<crate::UnityEngine::AnimationCurve>,
    pub m_CurveMax: quest_hook::libil2cpp::Gc<crate::UnityEngine::AnimationCurve>,
    pub m_ConstantMin: f32,
    pub m_ConstantMax: f32,
}
#[cfg(feature = "UnityEngine+ParticleSystem+MinMaxCurve")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::ParticleSystem_MinMaxCurve =>
    "UnityEngine"."ParticleSystem/MinMaxCurve"
);
#[cfg(feature = "UnityEngine+ParticleSystem+MinMaxCurve")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::ParticleSystem_MinMaxCurve {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+ParticleSystem+MinMaxCurve")]
impl crate::UnityEngine::ParticleSystem_MinMaxCurve {
    pub fn _ctor_f32_0(
        &mut self,
        constant: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (constant),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_f32_1(
        &mut self,
        min: f32,
        max: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (min, max),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Implicit(
        constant: f32,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::ParticleSystem_MinMaxCurve> {
        let __cordl_ret: crate::UnityEngine::ParticleSystem_MinMaxCurve = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Implicit", (constant))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_constantMax(
        &mut self,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_constantMax",
            (value),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn set_constantMin(
        &mut self,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_constantMin",
            (value),
        )?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+ParticleSystem+MinMaxGradient")]
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct ParticleSystem_MinMaxGradient {
    pub m_Mode: crate::UnityEngine::ParticleSystemGradientMode,
    pub m_GradientMin: quest_hook::libil2cpp::Gc<crate::UnityEngine::Gradient>,
    pub m_GradientMax: quest_hook::libil2cpp::Gc<crate::UnityEngine::Gradient>,
    pub m_ColorMin: crate::UnityEngine::Color,
    pub m_ColorMax: crate::UnityEngine::Color,
}
#[cfg(feature = "UnityEngine+ParticleSystem+MinMaxGradient")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::ParticleSystem_MinMaxGradient =>
    "UnityEngine"."ParticleSystem/MinMaxGradient"
);
#[cfg(feature = "UnityEngine+ParticleSystem+MinMaxGradient")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::ParticleSystem_MinMaxGradient {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+ParticleSystem+MinMaxGradient")]
impl crate::UnityEngine::ParticleSystem_MinMaxGradient {
    pub fn _ctor(
        &mut self,
        color: crate::UnityEngine::Color,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (color),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_color(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Color> {
        let __cordl_ret: crate::UnityEngine::Color = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_color",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Implicit(
        color: crate::UnityEngine::Color,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::ParticleSystem_MinMaxGradient,
    > {
        let __cordl_ret: crate::UnityEngine::ParticleSystem_MinMaxGradient = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Implicit", (color))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+ParticleSystem+NoiseModule")]
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct ParticleSystem_NoiseModule {
    pub m_ParticleSystem: quest_hook::libil2cpp::Gc<crate::UnityEngine::ParticleSystem>,
}
#[cfg(feature = "UnityEngine+ParticleSystem+NoiseModule")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::ParticleSystem_NoiseModule =>
    "UnityEngine"."ParticleSystem/NoiseModule"
);
#[cfg(feature = "UnityEngine+ParticleSystem+NoiseModule")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::ParticleSystem_NoiseModule {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+ParticleSystem+NoiseModule")]
impl crate::UnityEngine::ParticleSystem_NoiseModule {
    pub fn _ctor(
        &mut self,
        particleSystem: quest_hook::libil2cpp::Gc<crate::UnityEngine::ParticleSystem>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (particleSystem),
        )?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+ParticleSystem+Particle")]
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct ParticleSystem_Particle {
    pub m_Position: crate::UnityEngine::Vector3,
    pub m_Velocity: crate::UnityEngine::Vector3,
    pub m_AnimatedVelocity: crate::UnityEngine::Vector3,
    pub m_InitialVelocity: crate::UnityEngine::Vector3,
    pub m_AxisOfRotation: crate::UnityEngine::Vector3,
    pub m_Rotation: crate::UnityEngine::Vector3,
    pub m_AngularVelocity: crate::UnityEngine::Vector3,
    pub m_StartSize: crate::UnityEngine::Vector3,
    pub m_StartColor: crate::UnityEngine::Color32,
    pub m_RandomSeed: u32,
    pub m_ParentRandomSeed: u32,
    pub m_Lifetime: f32,
    pub m_StartLifetime: f32,
    pub m_MeshIndex: i32,
    pub m_EmitAccumulator0: f32,
    pub m_EmitAccumulator1: f32,
    pub m_Flags: u32,
}
#[cfg(feature = "UnityEngine+ParticleSystem+Particle")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::ParticleSystem_Particle =>
    "UnityEngine"."ParticleSystem/Particle"
);
#[cfg(feature = "UnityEngine+ParticleSystem+Particle")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::ParticleSystem_Particle {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+ParticleSystem+Particle")]
impl crate::UnityEngine::ParticleSystem_Particle {
    pub fn set_angularVelocity3D(
        &mut self,
        value: crate::UnityEngine::Vector3,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_angularVelocity3D",
            (value),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn set_lifetime(
        &mut self,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_lifetime",
            (value),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn set_position(
        &mut self,
        value: crate::UnityEngine::Vector3,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_position",
            (value),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn set_randomSeed(
        &mut self,
        value: u32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_randomSeed",
            (value),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn set_remainingLifetime(
        &mut self,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_remainingLifetime",
            (value),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn set_rotation3D(
        &mut self,
        value: crate::UnityEngine::Vector3,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_rotation3D",
            (value),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn set_startColor(
        &mut self,
        value: crate::UnityEngine::Color32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_startColor",
            (value),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn set_startLifetime(
        &mut self,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_startLifetime",
            (value),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn set_startSize(
        &mut self,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_startSize",
            (value),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn set_velocity(
        &mut self,
        value: crate::UnityEngine::Vector3,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_velocity",
            (value),
        )?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+ParticleSystem+PlaybackState")]
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct ParticleSystem_PlaybackState {
    pub m_AccumulatedDt: f32,
    pub m_StartDelay: f32,
    pub m_PlaybackTime: f32,
    pub m_RingBufferIndex: i32,
    pub m_Emission: crate::UnityEngine::PlaybackState_ParticleSystem_Emission,
    pub m_Initial: crate::UnityEngine::PlaybackState_ParticleSystem_Initial,
    pub m_Shape: crate::UnityEngine::PlaybackState_ParticleSystem_Shape,
    pub m_Force: crate::UnityEngine::PlaybackState_ParticleSystem_Force,
    pub m_Collision: crate::UnityEngine::PlaybackState_ParticleSystem_Collision,
    pub m_Noise: crate::UnityEngine::PlaybackState_ParticleSystem_Noise,
    pub m_Lights: crate::UnityEngine::PlaybackState_ParticleSystem_Lights,
    pub m_Trail: crate::UnityEngine::PlaybackState_ParticleSystem_Trail,
}
#[cfg(feature = "UnityEngine+ParticleSystem+PlaybackState")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::ParticleSystem_PlaybackState =>
    "UnityEngine"."ParticleSystem/PlaybackState"
);
#[cfg(feature = "UnityEngine+ParticleSystem+PlaybackState")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::ParticleSystem_PlaybackState {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+ParticleSystem+PlaybackState")]
impl crate::UnityEngine::ParticleSystem_PlaybackState {
    #[cfg(feature = "UnityEngine+ParticleSystem+PlaybackState+Collision")]
    pub type Collision = crate::UnityEngine::PlaybackState_ParticleSystem_Collision;
    #[cfg(feature = "UnityEngine+ParticleSystem+PlaybackState+Emission")]
    pub type Emission = crate::UnityEngine::PlaybackState_ParticleSystem_Emission;
    #[cfg(feature = "UnityEngine+ParticleSystem+PlaybackState+Force")]
    pub type Force = crate::UnityEngine::PlaybackState_ParticleSystem_Force;
    #[cfg(feature = "UnityEngine+ParticleSystem+PlaybackState+Initial")]
    pub type Initial = crate::UnityEngine::PlaybackState_ParticleSystem_Initial;
    #[cfg(feature = "UnityEngine+ParticleSystem+PlaybackState+Lights")]
    pub type Lights = crate::UnityEngine::PlaybackState_ParticleSystem_Lights;
    #[cfg(feature = "UnityEngine+ParticleSystem+PlaybackState+Noise")]
    pub type Noise = crate::UnityEngine::PlaybackState_ParticleSystem_Noise;
    #[cfg(feature = "UnityEngine+ParticleSystem+PlaybackState+Seed")]
    pub type Seed = crate::UnityEngine::PlaybackState_ParticleSystem_Seed;
    #[cfg(feature = "UnityEngine+ParticleSystem+PlaybackState+Seed4")]
    pub type Seed4 = crate::UnityEngine::PlaybackState_ParticleSystem_Seed4;
    #[cfg(feature = "UnityEngine+ParticleSystem+PlaybackState+Shape")]
    pub type Shape = crate::UnityEngine::PlaybackState_ParticleSystem_Shape;
    #[cfg(feature = "UnityEngine+ParticleSystem+PlaybackState+Trail")]
    pub type Trail = crate::UnityEngine::PlaybackState_ParticleSystem_Trail;
}
#[cfg(feature = "UnityEngine+ParticleSystem+RotationBySpeedModule")]
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct ParticleSystem_RotationBySpeedModule {
    pub m_ParticleSystem: quest_hook::libil2cpp::Gc<crate::UnityEngine::ParticleSystem>,
}
#[cfg(feature = "UnityEngine+ParticleSystem+RotationBySpeedModule")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::ParticleSystem_RotationBySpeedModule => "UnityEngine"
    ."ParticleSystem/RotationBySpeedModule"
);
#[cfg(feature = "UnityEngine+ParticleSystem+RotationBySpeedModule")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::ParticleSystem_RotationBySpeedModule {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+ParticleSystem+RotationBySpeedModule")]
impl crate::UnityEngine::ParticleSystem_RotationBySpeedModule {
    pub fn _ctor(
        &mut self,
        particleSystem: quest_hook::libil2cpp::Gc<crate::UnityEngine::ParticleSystem>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (particleSystem),
        )?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+ParticleSystem+RotationOverLifetimeModule")]
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct ParticleSystem_RotationOverLifetimeModule {
    pub m_ParticleSystem: quest_hook::libil2cpp::Gc<crate::UnityEngine::ParticleSystem>,
}
#[cfg(feature = "UnityEngine+ParticleSystem+RotationOverLifetimeModule")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::ParticleSystem_RotationOverLifetimeModule => "UnityEngine"
    ."ParticleSystem/RotationOverLifetimeModule"
);
#[cfg(feature = "UnityEngine+ParticleSystem+RotationOverLifetimeModule")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::ParticleSystem_RotationOverLifetimeModule {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+ParticleSystem+RotationOverLifetimeModule")]
impl crate::UnityEngine::ParticleSystem_RotationOverLifetimeModule {
    pub fn _ctor(
        &mut self,
        particleSystem: quest_hook::libil2cpp::Gc<crate::UnityEngine::ParticleSystem>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (particleSystem),
        )?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+ParticleSystem+ShapeModule")]
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct ParticleSystem_ShapeModule {
    pub m_ParticleSystem: quest_hook::libil2cpp::Gc<crate::UnityEngine::ParticleSystem>,
}
#[cfg(feature = "UnityEngine+ParticleSystem+ShapeModule")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::ParticleSystem_ShapeModule =>
    "UnityEngine"."ParticleSystem/ShapeModule"
);
#[cfg(feature = "UnityEngine+ParticleSystem+ShapeModule")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::ParticleSystem_ShapeModule {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+ParticleSystem+ShapeModule")]
impl crate::UnityEngine::ParticleSystem_ShapeModule {
    pub fn _ctor(
        &mut self,
        particleSystem: quest_hook::libil2cpp::Gc<crate::UnityEngine::ParticleSystem>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (particleSystem),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_meshRenderer(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::MeshRenderer>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::MeshRenderer> = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_meshRenderer",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_meshRenderer_Injected(
        _unity_self: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::ParticleSystem_ShapeModule,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::MeshRenderer>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::MeshRenderer> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_meshRenderer_Injected", (_unity_self))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_meshRenderer(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::UnityEngine::MeshRenderer>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_meshRenderer",
            (value),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn set_meshRenderer_Injected(
        _unity_self: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::ParticleSystem_ShapeModule,
        >,
        value: quest_hook::libil2cpp::Gc<crate::UnityEngine::MeshRenderer>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("set_meshRenderer_Injected", (_unity_self, value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_rotation(
        &mut self,
        value: crate::UnityEngine::Vector3,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_rotation",
            (value),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn set_rotation_Injected(
        _unity_self: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::ParticleSystem_ShapeModule,
        >,
        value: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector3>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("set_rotation_Injected", (_unity_self, value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+ParticleSystem+SizeBySpeedModule")]
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct ParticleSystem_SizeBySpeedModule {
    pub m_ParticleSystem: quest_hook::libil2cpp::Gc<crate::UnityEngine::ParticleSystem>,
}
#[cfg(feature = "UnityEngine+ParticleSystem+SizeBySpeedModule")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::ParticleSystem_SizeBySpeedModule =>
    "UnityEngine"."ParticleSystem/SizeBySpeedModule"
);
#[cfg(feature = "UnityEngine+ParticleSystem+SizeBySpeedModule")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::ParticleSystem_SizeBySpeedModule {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+ParticleSystem+SizeBySpeedModule")]
impl crate::UnityEngine::ParticleSystem_SizeBySpeedModule {
    pub fn _ctor(
        &mut self,
        particleSystem: quest_hook::libil2cpp::Gc<crate::UnityEngine::ParticleSystem>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (particleSystem),
        )?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+ParticleSystem+SizeOverLifetimeModule")]
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct ParticleSystem_SizeOverLifetimeModule {
    pub m_ParticleSystem: quest_hook::libil2cpp::Gc<crate::UnityEngine::ParticleSystem>,
}
#[cfg(feature = "UnityEngine+ParticleSystem+SizeOverLifetimeModule")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::ParticleSystem_SizeOverLifetimeModule => "UnityEngine"
    ."ParticleSystem/SizeOverLifetimeModule"
);
#[cfg(feature = "UnityEngine+ParticleSystem+SizeOverLifetimeModule")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::ParticleSystem_SizeOverLifetimeModule {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+ParticleSystem+SizeOverLifetimeModule")]
impl crate::UnityEngine::ParticleSystem_SizeOverLifetimeModule {
    pub fn _ctor(
        &mut self,
        particleSystem: quest_hook::libil2cpp::Gc<crate::UnityEngine::ParticleSystem>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (particleSystem),
        )?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+ParticleSystem+SubEmittersModule")]
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct ParticleSystem_SubEmittersModule {
    pub m_ParticleSystem: quest_hook::libil2cpp::Gc<crate::UnityEngine::ParticleSystem>,
}
#[cfg(feature = "UnityEngine+ParticleSystem+SubEmittersModule")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::ParticleSystem_SubEmittersModule =>
    "UnityEngine"."ParticleSystem/SubEmittersModule"
);
#[cfg(feature = "UnityEngine+ParticleSystem+SubEmittersModule")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::ParticleSystem_SubEmittersModule {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+ParticleSystem+SubEmittersModule")]
impl crate::UnityEngine::ParticleSystem_SubEmittersModule {
    pub fn GetSubEmitterSystem(
        &mut self,
        index: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::ParticleSystem>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::ParticleSystem> = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "GetSubEmitterSystem",
            (index),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn GetSubEmitterSystem_Injected(
        _unity_self: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::ParticleSystem_SubEmittersModule,
        >,
        index: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::ParticleSystem>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::ParticleSystem> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetSubEmitterSystem_Injected", (_unity_self, index))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        particleSystem: quest_hook::libil2cpp::Gc<crate::UnityEngine::ParticleSystem>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (particleSystem),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_subEmittersCount(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_subEmittersCount",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_subEmittersCount_Injected(
        _unity_self: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::ParticleSystem_SubEmittersModule,
        >,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_subEmittersCount_Injected", (_unity_self))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+ParticleSystem+TextureSheetAnimationModule")]
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct ParticleSystem_TextureSheetAnimationModule {
    pub m_ParticleSystem: quest_hook::libil2cpp::Gc<crate::UnityEngine::ParticleSystem>,
}
#[cfg(feature = "UnityEngine+ParticleSystem+TextureSheetAnimationModule")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::ParticleSystem_TextureSheetAnimationModule => "UnityEngine"
    ."ParticleSystem/TextureSheetAnimationModule"
);
#[cfg(feature = "UnityEngine+ParticleSystem+TextureSheetAnimationModule")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::ParticleSystem_TextureSheetAnimationModule {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+ParticleSystem+TextureSheetAnimationModule")]
impl crate::UnityEngine::ParticleSystem_TextureSheetAnimationModule {
    pub fn _ctor(
        &mut self,
        particleSystem: quest_hook::libil2cpp::Gc<crate::UnityEngine::ParticleSystem>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (particleSystem),
        )?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+ParticleSystem+TrailModule")]
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct ParticleSystem_TrailModule {
    pub m_ParticleSystem: quest_hook::libil2cpp::Gc<crate::UnityEngine::ParticleSystem>,
}
#[cfg(feature = "UnityEngine+ParticleSystem+TrailModule")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::ParticleSystem_TrailModule =>
    "UnityEngine"."ParticleSystem/TrailModule"
);
#[cfg(feature = "UnityEngine+ParticleSystem+TrailModule")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::ParticleSystem_TrailModule {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+ParticleSystem+TrailModule")]
impl crate::UnityEngine::ParticleSystem_TrailModule {
    pub fn _ctor(
        &mut self,
        particleSystem: quest_hook::libil2cpp::Gc<crate::UnityEngine::ParticleSystem>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (particleSystem),
        )?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+ParticleSystem+Trails")]
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct ParticleSystem_Trails {
    pub positions: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::List_1<crate::UnityEngine::Vector4>,
    >,
    pub frontPositions: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::List_1<i32>,
    >,
    pub backPositions: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::List_1<i32>,
    >,
    pub positionCounts: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::List_1<i32>,
    >,
    pub textureOffsets: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::List_1<f32>,
    >,
    pub maxTrailCount: i32,
    pub maxPositionsPerTrailCount: i32,
}
#[cfg(feature = "UnityEngine+ParticleSystem+Trails")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::ParticleSystem_Trails =>
    "UnityEngine"."ParticleSystem/Trails"
);
#[cfg(feature = "UnityEngine+ParticleSystem+Trails")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::ParticleSystem_Trails {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+ParticleSystem+Trails")]
impl crate::UnityEngine::ParticleSystem_Trails {
    pub fn Allocate(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Allocate",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+ParticleSystem+TriggerModule")]
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct ParticleSystem_TriggerModule {
    pub m_ParticleSystem: quest_hook::libil2cpp::Gc<crate::UnityEngine::ParticleSystem>,
}
#[cfg(feature = "UnityEngine+ParticleSystem+TriggerModule")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::ParticleSystem_TriggerModule =>
    "UnityEngine"."ParticleSystem/TriggerModule"
);
#[cfg(feature = "UnityEngine+ParticleSystem+TriggerModule")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::ParticleSystem_TriggerModule {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+ParticleSystem+TriggerModule")]
impl crate::UnityEngine::ParticleSystem_TriggerModule {
    pub fn _ctor(
        &mut self,
        particleSystem: quest_hook::libil2cpp::Gc<crate::UnityEngine::ParticleSystem>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (particleSystem),
        )?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+ParticleSystem+VelocityOverLifetimeModule")]
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct ParticleSystem_VelocityOverLifetimeModule {
    pub m_ParticleSystem: quest_hook::libil2cpp::Gc<crate::UnityEngine::ParticleSystem>,
}
#[cfg(feature = "UnityEngine+ParticleSystem+VelocityOverLifetimeModule")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::ParticleSystem_VelocityOverLifetimeModule => "UnityEngine"
    ."ParticleSystem/VelocityOverLifetimeModule"
);
#[cfg(feature = "UnityEngine+ParticleSystem+VelocityOverLifetimeModule")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::ParticleSystem_VelocityOverLifetimeModule {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+ParticleSystem+VelocityOverLifetimeModule")]
impl crate::UnityEngine::ParticleSystem_VelocityOverLifetimeModule {
    pub fn _ctor(
        &mut self,
        particleSystem: quest_hook::libil2cpp::Gc<crate::UnityEngine::ParticleSystem>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (particleSystem),
        )?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+ParticleSystem+PlaybackState+Collision")]
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct PlaybackState_ParticleSystem_Collision {
    pub m_Random: crate::UnityEngine::PlaybackState_ParticleSystem_Seed4,
}
#[cfg(feature = "UnityEngine+ParticleSystem+PlaybackState+Collision")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::PlaybackState_ParticleSystem_Collision => "UnityEngine"
    ."ParticleSystem/PlaybackState/Collision"
);
#[cfg(feature = "UnityEngine+ParticleSystem+PlaybackState+Collision")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::PlaybackState_ParticleSystem_Collision {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+ParticleSystem+PlaybackState+Collision")]
impl crate::UnityEngine::PlaybackState_ParticleSystem_Collision {}
#[cfg(feature = "UnityEngine+ParticleSystem+PlaybackState+Emission")]
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct PlaybackState_ParticleSystem_Emission {
    pub m_ParticleSpacing: f32,
    pub m_ToEmitAccumulator: f32,
    pub m_Random: crate::UnityEngine::PlaybackState_ParticleSystem_Seed,
}
#[cfg(feature = "UnityEngine+ParticleSystem+PlaybackState+Emission")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::PlaybackState_ParticleSystem_Emission => "UnityEngine"
    ."ParticleSystem/PlaybackState/Emission"
);
#[cfg(feature = "UnityEngine+ParticleSystem+PlaybackState+Emission")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::PlaybackState_ParticleSystem_Emission {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+ParticleSystem+PlaybackState+Emission")]
impl crate::UnityEngine::PlaybackState_ParticleSystem_Emission {}
#[cfg(feature = "UnityEngine+ParticleSystem+PlaybackState+Force")]
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct PlaybackState_ParticleSystem_Force {
    pub m_Random: crate::UnityEngine::PlaybackState_ParticleSystem_Seed4,
}
#[cfg(feature = "UnityEngine+ParticleSystem+PlaybackState+Force")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::PlaybackState_ParticleSystem_Force
    => "UnityEngine"."ParticleSystem/PlaybackState/Force"
);
#[cfg(feature = "UnityEngine+ParticleSystem+PlaybackState+Force")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::PlaybackState_ParticleSystem_Force {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+ParticleSystem+PlaybackState+Force")]
impl crate::UnityEngine::PlaybackState_ParticleSystem_Force {}
#[cfg(feature = "UnityEngine+ParticleSystem+PlaybackState+Initial")]
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct PlaybackState_ParticleSystem_Initial {
    pub m_Random: crate::UnityEngine::PlaybackState_ParticleSystem_Seed4,
}
#[cfg(feature = "UnityEngine+ParticleSystem+PlaybackState+Initial")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::PlaybackState_ParticleSystem_Initial => "UnityEngine"
    ."ParticleSystem/PlaybackState/Initial"
);
#[cfg(feature = "UnityEngine+ParticleSystem+PlaybackState+Initial")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::PlaybackState_ParticleSystem_Initial {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+ParticleSystem+PlaybackState+Initial")]
impl crate::UnityEngine::PlaybackState_ParticleSystem_Initial {}
#[cfg(feature = "UnityEngine+ParticleSystem+PlaybackState+Lights")]
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct PlaybackState_ParticleSystem_Lights {
    pub m_Random: crate::UnityEngine::PlaybackState_ParticleSystem_Seed,
    pub m_ParticleEmissionCounter: f32,
}
#[cfg(feature = "UnityEngine+ParticleSystem+PlaybackState+Lights")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::PlaybackState_ParticleSystem_Lights
    => "UnityEngine"."ParticleSystem/PlaybackState/Lights"
);
#[cfg(feature = "UnityEngine+ParticleSystem+PlaybackState+Lights")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::PlaybackState_ParticleSystem_Lights {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+ParticleSystem+PlaybackState+Lights")]
impl crate::UnityEngine::PlaybackState_ParticleSystem_Lights {}
#[cfg(feature = "UnityEngine+ParticleSystem+PlaybackState+Noise")]
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct PlaybackState_ParticleSystem_Noise {
    pub m_ScrollOffset: f32,
}
#[cfg(feature = "UnityEngine+ParticleSystem+PlaybackState+Noise")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::PlaybackState_ParticleSystem_Noise
    => "UnityEngine"."ParticleSystem/PlaybackState/Noise"
);
#[cfg(feature = "UnityEngine+ParticleSystem+PlaybackState+Noise")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::PlaybackState_ParticleSystem_Noise {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+ParticleSystem+PlaybackState+Noise")]
impl crate::UnityEngine::PlaybackState_ParticleSystem_Noise {}
#[cfg(feature = "UnityEngine+ParticleSystem+PlaybackState+Seed")]
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct PlaybackState_ParticleSystem_Seed {
    pub x: u32,
    pub y: u32,
    pub z: u32,
    pub w: u32,
}
#[cfg(feature = "UnityEngine+ParticleSystem+PlaybackState+Seed")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::PlaybackState_ParticleSystem_Seed
    => "UnityEngine"."ParticleSystem/PlaybackState/Seed"
);
#[cfg(feature = "UnityEngine+ParticleSystem+PlaybackState+Seed")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::PlaybackState_ParticleSystem_Seed {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+ParticleSystem+PlaybackState+Seed")]
impl crate::UnityEngine::PlaybackState_ParticleSystem_Seed {}
#[cfg(feature = "UnityEngine+ParticleSystem+PlaybackState+Seed4")]
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct PlaybackState_ParticleSystem_Seed4 {
    pub x: crate::UnityEngine::PlaybackState_ParticleSystem_Seed,
    pub y: crate::UnityEngine::PlaybackState_ParticleSystem_Seed,
    pub z: crate::UnityEngine::PlaybackState_ParticleSystem_Seed,
    pub w: crate::UnityEngine::PlaybackState_ParticleSystem_Seed,
}
#[cfg(feature = "UnityEngine+ParticleSystem+PlaybackState+Seed4")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::PlaybackState_ParticleSystem_Seed4
    => "UnityEngine"."ParticleSystem/PlaybackState/Seed4"
);
#[cfg(feature = "UnityEngine+ParticleSystem+PlaybackState+Seed4")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::PlaybackState_ParticleSystem_Seed4 {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+ParticleSystem+PlaybackState+Seed4")]
impl crate::UnityEngine::PlaybackState_ParticleSystem_Seed4 {}
#[cfg(feature = "UnityEngine+ParticleSystem+PlaybackState+Shape")]
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct PlaybackState_ParticleSystem_Shape {
    pub m_Random: crate::UnityEngine::PlaybackState_ParticleSystem_Seed4,
    pub m_RadiusTimer: f32,
    pub m_RadiusTimerPrev: f32,
    pub m_ArcTimer: f32,
    pub m_ArcTimerPrev: f32,
    pub m_MeshSpawnTimer: f32,
    pub m_MeshSpawnTimerPrev: f32,
    pub m_OrderedMeshVertexIndex: i32,
}
#[cfg(feature = "UnityEngine+ParticleSystem+PlaybackState+Shape")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::PlaybackState_ParticleSystem_Shape
    => "UnityEngine"."ParticleSystem/PlaybackState/Shape"
);
#[cfg(feature = "UnityEngine+ParticleSystem+PlaybackState+Shape")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::PlaybackState_ParticleSystem_Shape {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+ParticleSystem+PlaybackState+Shape")]
impl crate::UnityEngine::PlaybackState_ParticleSystem_Shape {}
#[cfg(feature = "UnityEngine+ParticleSystem+PlaybackState+Trail")]
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct PlaybackState_ParticleSystem_Trail {
    pub m_Timer: f32,
}
#[cfg(feature = "UnityEngine+ParticleSystem+PlaybackState+Trail")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::PlaybackState_ParticleSystem_Trail
    => "UnityEngine"."ParticleSystem/PlaybackState/Trail"
);
#[cfg(feature = "UnityEngine+ParticleSystem+PlaybackState+Trail")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::PlaybackState_ParticleSystem_Trail {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+ParticleSystem+PlaybackState+Trail")]
impl crate::UnityEngine::PlaybackState_ParticleSystem_Trail {}
