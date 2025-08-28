pub const K_TOPIC_JOINT_CTRL: &str = "rt/joint_ctrl";
pub const K_TOPIC_LOW_STATE: &str = "rt/low_state";
pub const K_TOPIC_FALL_DOWN: &str = "rt/fall_down";
pub const K_TOPIC_ODOMETER_STATE: &str = "rt/odometer_state";
pub const K_TOPIC_BOOSTER_HAND_DATA: &str = "rt/booster_hand_data";
pub const K_TOPIC_TF: &str = "rt/tf";

pub const K_JOINT_CNT: usize = 23;
pub const K_JOINT_CNT7_DOF_ARM: usize = 29;

#[cxx::bridge]
pub mod ffi {
    #[repr(i32)]
    pub enum JointIndex {
        // head
        kHeadYaw = 0,
        kHeadPitch = 1,

        // Left arm
        kLeftShoulderPitch = 2,
        kLeftShoulderRoll = 3,
        kLeftElbowPitch = 4,
        kLeftElbowYaw = 5,

        // Right arm
        kRightShoulderPitch = 6,
        kRightShoulderRoll = 7,
        kRightElbowPitch = 8,
        kRightElbowYaw = 9,

        // waist
        kWaist = 10,

        // left leg
        kLeftHipPitch = 11,
        kLeftHipRoll = 12,
        kLeftHipYaw = 13,
        kLeftKneePitch = 14,
        kCrankUpLeft = 15,
        kCrankDownLeft = 16,

        // right leg
        kRightHipPitch = 17,
        kRightHipRoll = 18,
        kRightHipYaw = 19,
        kRightKneePitch = 20,
        kCrankUpRight = 21,
        kCrankDownRight = 22,
    }

    #[repr(i32)]
    pub enum JointIndexWith7DofArm {
        // head
        kHeadYaw = 0,
        kHeadPitch = 1,

        // Left arm
        kLeftShoulderPitch = 2,
        kLeftShoulderRoll = 3,
        kLeftElbowPitch = 4,
        kLeftElbowYaw = 5,
        kLeftWristPitch = 6,
        kLeftWristYaw = 7,
        kLeftHandRoll = 8,

        // Right arm
        kRightShoulderPitch = 9,
        kRightShoulderRoll = 10,
        kRightElbowPitch = 11,
        kRightElbowYaw = 12,
        kRightWristPitch = 13,
        kRightWristYaw = 14,
        kRightHandRoll = 15,

        // waist
        kWaist = 16,

        // left leg
        kLeftHipPitch = 17,
        kLeftHipRoll = 18,
        kLeftHipYaw = 19,
        kLeftKneePitch = 20,
        kCrankUpLeft = 21,
        kCrankDownLeft = 22,

        // right leg
        kRightHipPitch = 23,
        kRightHipRoll = 24,
        kRightHipYaw = 25,
        kRightKneePitch = 26,
        kCrankUpRight = 27,
        kCrankDownRight = 28,
    }

    #[repr(i32)]
    pub enum HandIndex {
        kLeftHand = 0,
        kRightHand = 1,
    }

    #[repr(i32)]
    pub enum HandAction {
        kHandOpen = 0,
        kHandClose = 1,
    }

    #[repr(i32)]
    pub enum RemoteControllerEvent {
        NONE = 0,            // no event
        AXIS = 0x600,        // axis motion
        HAT = 0x602,         // hat position change
        BUTTON_DOWN = 0x603, // button pressed
        BUTTON_UP = 0x604,   // button released
        REMOVE = 0x606,      // device has been removed
    }

    unsafe extern "C++" {
        include!("wrapper.hpp");

        type JointIndex;
        type JointIndexWith7DofArm;
        type HandIndex;
        type HandAction;
        type RemoteControllerEvent;
    }
}
