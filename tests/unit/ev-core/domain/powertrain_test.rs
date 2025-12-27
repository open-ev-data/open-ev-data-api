use ev_core::{Drivetrain, Motor, MotorPosition, Powertrain};

#[test]
fn test_powertrain_helpers() {
    let pt = Powertrain {
        drivetrain: Drivetrain::Awd,
        system_power_kw: Some(300.0),
        system_torque_nm: Some(600.0),
        motors: Some(vec![
            Motor {
                position: MotorPosition::Front,
                power_kw: Some(100.0),
                torque_nm: Some(200.0),
                motor_type: None,
                cooling: None,
            },
            Motor {
                position: MotorPosition::Rear,
                power_kw: Some(200.0),
                torque_nm: Some(400.0),
                motor_type: None,
                cooling: None,
            },
        ]),
        transmission: None,
    };

    assert_eq!(pt.motor_count(), 2);
    // explicit system_power_kw takes precedence if validation isn't forcing it?
    // helper total_power_kw logic: self.system_power_kw.or_else(|| sum(motors))
    assert_eq!(pt.total_power_kw(), Some(300.0));
}

#[test]
fn test_powertrain_sum_motors() {
    let pt_no_total = Powertrain {
        drivetrain: Drivetrain::Awd,
        system_power_kw: None,
        system_torque_nm: None,
        motors: Some(vec![
            Motor {
                position: MotorPosition::Front,
                power_kw: Some(100.0),
                torque_nm: None,
                motor_type: None,
                cooling: None,
            },
            Motor {
                position: MotorPosition::Rear,
                power_kw: Some(200.0),
                torque_nm: None,
                motor_type: None,
                cooling: None,
            },
        ]),
        transmission: None,
    };

    assert_eq!(pt_no_total.total_power_kw(), Some(300.0));
    assert_eq!(pt_no_total.motor_count(), 2);
}
