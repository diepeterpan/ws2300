use serialport::SerialPort;
use std::io::{Read, Write};
use std::path::PathBuf;
use std::time::Duration;
use tracing::{info, warn};

fn encode_temp_bcd(temp_c: f32) -> [u8; 2] {
    let val = ((temp_c + 30.0) * 100.0).round().clamp(0.0, 9999.0) as u32;
    let d3 = ((val / 1000) % 10) as u8;
    let d2 = ((val / 100) % 10) as u8;
    let d1 = ((val / 10) % 10) as u8;
    let d0 = (val % 10) as u8;
    [(d1 << 4) | d0, (d3 << 4) | d2]
}

fn encode_humidity_bcd(hum: u32) -> [u8; 1] {
    let v = hum.min(99);
    let tens = ((v / 10) % 10) as u8;
    let ones = (v % 10) as u8;
    [(tens << 4) | ones]
}

fn encode_wind_speed(speed_ms: f32) -> [u8; 3] {
    let raw = (speed_ms * 10.0).round().clamp(0.0, 4095.0) as u16;
    let b0 = (raw & 0xFF) as u8;
    let b1 = ((raw >> 8) & 0x0F) as u8;
    [b0, b1, 0]
}

fn encode_wind_dir(index: u8) -> [u8; 1] {
    [(index & 0x0F) << 4]
}

fn encode_rain_bcd(mm: f32) -> [u8; 3] {
    let val = (mm * 100.0).round().clamp(0.0, 999999.0) as u32;
    let d5 = ((val / 100_000) % 10) as u8;
    let d4 = ((val / 10_000) % 10) as u8;
    let d3 = ((val / 1_000) % 10) as u8;
    let d2 = ((val / 100) % 10) as u8;
    let d1 = ((val / 10) % 10) as u8;
    let d0 = (val % 10) as u8;
    [(d1 << 4) | d0, (d3 << 4) | d2, (d5 << 4) | d4]
}

fn encode_pressure_bcd(hpa: f32) -> [u8; 3] {
    let val = (hpa * 10.0).round().clamp(0.0, 99999.0) as u32;
    let d4 = ((val / 10_000) % 10) as u8;
    let d3 = ((val / 1_000) % 10) as u8;
    let d2 = ((val / 100) % 10) as u8;
    let d1 = ((val / 10) % 10) as u8;
    let d0 = (val % 10) as u8;
    [(d1 << 4) | d0, (d3 << 4) | d2, d4]
}

fn memory_payload(address: u32, size: usize, read_counter: u64) -> Vec<u8> {
    let drift = ((read_counter / 15) % 8) as f32 * 0.2;
    // Every 4th full poll cycle, inject an out-of-bounds outdoor temperature (68.5 °C > 60 °C max)
    // so the user can observe out-of-bounds replacement with the previous valid value on the UI!
    let poll_cycle = read_counter / 15;
    let outdoor_temp = if poll_cycle > 0 && poll_cycle.is_multiple_of(4) {
        68.5
    } else {
        24.4 + drift
    };

    let raw: Vec<u8> = match address {
        0x346 => encode_temp_bcd(22.1 + drift * 0.5).to_vec(), // temperature_indoor
        0x373 => encode_temp_bcd(outdoor_temp).to_vec(),       // temperature_outdoor
        0x3CE => encode_temp_bcd(16.2 + drift * 0.4).to_vec(), // dewpoint
        0x3FB => encode_humidity_bcd(48).to_vec(),             // humidity_indoor
        0x419 => encode_humidity_bcd(62 + (poll_cycle % 4) as u32).to_vec(), // humidity_outdoor
        0x529 => encode_wind_speed(4.2 + drift).to_vec(),      // wind_speed
        0x52C => encode_wind_dir(6).to_vec(),                  // wind_dir (6 * 22.5 = 135° SE)
        0x3A0 => encode_temp_bcd(23.8 + drift * 0.5).to_vec(), // wind_chill
        0x4B4 => encode_rain_bcd(0.0).to_vec(),                // rain_1h
        0x497 => encode_rain_bcd(1.5).to_vec(),                // rain_24h
        0x4D2 => encode_rain_bcd(142.8).to_vec(),              // rain_total
        0x5E2 => encode_pressure_bcd(1014.2 + drift * 0.5).to_vec(), // pressure
        0x26B => vec![(1 << 4) | 2], // tendency=Rising(1), forecast=Sunny(2)
        _ => vec![0; size],
    };

    let mut out = raw;
    out.resize(size, 0);
    out
}

pub fn spawn_virtual_ws2300_pty(symlink_path: PathBuf) -> Result<(), String> {
    let (mut master, slave) = serialport::TTYPort::pair()
        .map_err(|e| format!("Failed to create virtual PTY pair: {e}"))?;

    let _ = master.set_timeout(Duration::from_secs(3600));

    let slave_name = slave
        .name()
        .ok_or_else(|| String::from("Virtual PTY slave has no device path"))?;

    if symlink_path.exists() || symlink_path.is_symlink() {
        let _ = std::fs::remove_file(&symlink_path);
    }

    #[cfg(unix)]
    std::os::unix::fs::symlink(&slave_name, &symlink_path).map_err(|e| {
        format!(
            "Failed to symlink virtual WS2300 port {} -> {}: {e}",
            symlink_path.display(),
            slave_name
        )
    })?;

    info!(
        "Mock WS2300 hardware emulator listening on virtual serial port {} (slave PTY: {})",
        symlink_path.display(),
        slave_name
    );

    std::thread::spawn(move || {
        // Keep `slave` open for the lifetime of the emulator thread so the PTY stays active
        let _keep_slave_alive = slave;
        let mut seq: usize = 0;
        let mut address: u32 = 0;
        let mut read_counter: u64 = 0;
        let mut buf = [0u8; 1];

        loop {
            match master.read(&mut buf) {
                Ok(1) => {
                    let b = buf[0];
                    if b == 0x06 {
                        seq = 0;
                        address = 0;
                        let _ = master.write_all(&[0x02]);
                        let _ = master.flush();
                        continue;
                    }

                    if seq < 4 && b >= 0x82 {
                        let nibble = ((b - 0x82) / 4) & 0x0F;
                        let ack = (seq as u8) * 16 + nibble;
                        address = (address << 4) | (nibble as u32);
                        let _ = master.write_all(&[ack]);
                        let _ = master.flush();
                        seq += 1;
                    } else if seq == 4 && b >= 0xC2 {
                        let size = ((b - 0xC2) / 4) as usize;
                        let ack = 0x30 + (size as u8);
                        let data = memory_payload(address, size, read_counter);
                        read_counter = read_counter.wrapping_add(1);
                        let checksum: u8 = data.iter().fold(0u32, |acc, &x| acc + (x as u32)) as u8;

                        let mut packet = Vec::with_capacity(1 + size + 1);
                        packet.push(ack);
                        packet.extend_from_slice(&data);
                        packet.push(checksum);

                        let _ = master.write_all(&packet);
                        let _ = master.flush();
                        seq = 0;
                        address = 0;
                    } else {
                        seq = 0;
                        address = 0;
                    }
                }
                Ok(_) => continue,
                Err(e) => {
                    if e.kind() != std::io::ErrorKind::TimedOut {
                        warn!("Mock WS2300 PTY read error: {e}");
                        std::thread::sleep(Duration::from_millis(50));
                    }
                }
            }
        }
    });

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_virtual_pty_ws2300_roundtrip() {
        let pty_link =
            std::env::temp_dir().join(format!("ws2300-mock-test-{}", std::process::id()));
        spawn_virtual_ws2300_pty(pty_link.clone()).expect("spawn mock pty");

        let dev = ws2300::Device::new(pty_link.to_string_lossy().to_string())
            .expect("open mock serial device");
        let data = dev
            .read_all()
            .expect("read_all over virtual serial protocol");

        assert!((data.temperature_indoor - 22.1).abs() < 0.2);
        assert!((data.temperature_outdoor - 24.4).abs() < 0.2);
        assert_eq!(data.humidity_indoor, 48);
        assert_eq!(data.wind_direction.as_deref(), Some("SE"));
        assert_eq!(data.tendency, "Rising");
        assert_eq!(data.forecast, "Sunny");

        let _ = std::fs::remove_file(&pty_link);
    }
}
