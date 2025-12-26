use std::io::Write;
use std::path::Path;

use anyhow::{Context, Result};
use ev_core::Vehicle;

pub fn generate(vehicles: &[Vehicle], output_path: &Path) -> Result<()> {
    let mut file = std::fs::File::create(output_path)
        .with_context(|| format!("Failed to create XML file at {:?}", output_path))?;

    writeln!(file, r#"<?xml version="1.0" encoding="UTF-8"?>"#)?;
    writeln!(file, r#"<vehicles xmlns="https://openevdata.org/schema/1.0" count="{}">"#, vehicles.len())?;

    for vehicle in vehicles {
        write_vehicle(&mut file, vehicle)?;
    }

    writeln!(file, "</vehicles>")?;

    Ok(())
}

fn write_vehicle(file: &mut std::fs::File, vehicle: &Vehicle) -> Result<()> {
    let id = vehicle.id().canonical_id();

    writeln!(file, r#"  <vehicle id="{}">"#, escape_xml(&id))?;
    writeln!(file, "    <make slug=\"{}\">{}</make>", escape_xml(&vehicle.make.slug), escape_xml(&vehicle.make.name))?;
    writeln!(file, "    <model slug=\"{}\">{}</model>", escape_xml(&vehicle.model.slug), escape_xml(&vehicle.model.name))?;
    writeln!(file, "    <year>{}</year>", vehicle.year)?;
    writeln!(file, "    <trim slug=\"{}\">{}</trim>", escape_xml(&vehicle.trim.slug), escape_xml(&vehicle.trim.name))?;
    writeln!(file, "    <vehicleType>{:?}</vehicleType>", vehicle.vehicle_type)?;

    writeln!(file, "    <powertrain>")?;
    writeln!(file, "      <drivetrain>{:?}</drivetrain>", vehicle.powertrain.drivetrain)?;
    if let Some(power) = vehicle.powertrain.system_power_kw {
        writeln!(file, "      <systemPowerKw>{}</systemPowerKw>", power)?;
    }
    if let Some(torque) = vehicle.powertrain.system_torque_nm {
        writeln!(file, "      <systemTorqueNm>{}</systemTorqueNm>", torque)?;
    }
    writeln!(file, "    </powertrain>")?;

    writeln!(file, "    <battery>")?;
    if let Some(gross) = vehicle.battery.pack_capacity_kwh_gross {
        writeln!(file, "      <packCapacityKwhGross>{}</packCapacityKwhGross>", gross)?;
    }
    if let Some(net) = vehicle.battery.pack_capacity_kwh_net {
        writeln!(file, "      <packCapacityKwhNet>{}</packCapacityKwhNet>", net)?;
    }
    if let Some(ref chem) = vehicle.battery.chemistry {
        writeln!(file, "      <chemistry>{}</chemistry>", escape_xml(chem))?;
    }
    writeln!(file, "    </battery>")?;

    writeln!(file, "    <chargePorts>")?;
    for port in &vehicle.charge_ports {
        writeln!(file, "      <port kind=\"{:?}\" connector=\"{:?}\"/>", port.kind, port.connector)?;
    }
    writeln!(file, "    </chargePorts>")?;

    writeln!(file, "    <charging>")?;
    if let Some(ref dc) = vehicle.charging.dc {
        writeln!(file, "      <dcMaxPowerKw>{}</dcMaxPowerKw>", dc.max_power_kw)?;
    }
    if let Some(ref ac) = vehicle.charging.ac {
        writeln!(file, "      <acMaxPowerKw>{}</acMaxPowerKw>", ac.max_power_kw)?;
    }
    writeln!(file, "    </charging>")?;

    writeln!(file, "    <range>")?;
    for rating in &vehicle.range.rated {
        writeln!(file, "      <rated cycle=\"{:?}\" km=\"{}\"/>", rating.cycle, rating.range_km)?;
    }
    writeln!(file, "    </range>")?;

    writeln!(file, "    <sources>")?;
    for source in &vehicle.sources {
        writeln!(file, "      <source type=\"{:?}\" url=\"{}\"/>", source.source_type, escape_xml(&source.url))?;
    }
    writeln!(file, "    </sources>")?;

    writeln!(file, "  </vehicle>")?;

    Ok(())
}

fn escape_xml(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&apos;")
}
