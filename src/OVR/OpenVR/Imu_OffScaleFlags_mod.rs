#[cfg(feature = "OVR+OpenVR+Imu_OffScaleFlags")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Imu_OffScaleFlags {
    OffScale_AccelX = 1i32,
    OffScale_AccelY = 2i32,
    OffScale_AccelZ = 4i32,
    OffScale_GyroX = 8i32,
    OffScale_GyroY = 16i32,
    OffScale_GyroZ = 32i32,
}
#[cfg(feature = "OVR+OpenVR+Imu_OffScaleFlags")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::OVR::OpenVR::Imu_OffScaleFlags => "OVR.OpenVR"
    ."Imu_OffScaleFlags"
);
