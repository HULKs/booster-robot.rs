use cxx::let_cxx_string;

use booster_robot_sys::robot::{b1::b1_loco_client_new, common::RobotMode, init_channel_factory};

fn main() {
    let_cxx_string!(network_interface = "127.0.0.1");
    init_channel_factory(&network_interface);

    let mut client = b1_loco_client_new();

    client.pin_mut().ChangeMode(RobotMode::kDamping);
}
